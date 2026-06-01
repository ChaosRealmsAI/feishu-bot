use super::*;

mod formatting;
mod links;
mod local;
mod readback;
mod state;

use formatting::*;
use links::*;
pub(super) use local::{office_command_can_run_without_api, run_office_local_command};
use local::{run_office_bootstrap_dry_run, run_office_list, run_office_report_dry_run};
use readback::*;
pub(super) use state::*;
const PROJECT_LOG_TABLE_NAME: &str = "项目日志";

#[derive(Debug)]
struct CreatedDoc {
    document_id: String,
    node_token: Option<String>,
    url: String,
    create_response: Value,
    append_response: Option<Value>,
}

pub(super) async fn run_office_command(
    api: &mut FeishuClient,
    command: OfficeCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OfficeCommand::List(args) => run_office_list(args)?,
        OfficeCommand::Bootstrap(args) => run_office_bootstrap(api, args).await?,
        OfficeCommand::Report(args) => run_office_report(api, args).await?,
        OfficeCommand::Progress(args) => run_office_progress(api, args).await?,
        OfficeCommand::VoiceReport(args) => run_office_voice_report(api, args).await?,
        OfficeCommand::Inbox(args) => run_office_inbox(api, args).await?,
        OfficeCommand::Poll(args) => run_office_poll(api, args).await?,
        OfficeCommand::Status(args) => run_office_status(api, args).await?,
        OfficeCommand::Search(args) => run_office_search(api, args).await?,
        OfficeCommand::Cleanup(args) => run_office_cleanup(api, args).await?,
    };
    print_response(raw_json, "office workflow completed", data)
}

async fn run_office_bootstrap(api: &mut FeishuClient, args: OfficeBootstrapArgs) -> Result<Value> {
    if args.dry_run {
        return run_office_bootstrap_dry_run(args);
    }
    let project_key = office_project_key(&args.project)?;
    let state_path = office_registry_path()?;
    let mut registry = read_office_registry()?;
    let now = office_now();
    let mut project = registry
        .projects
        .get(&project_key)
        .cloned()
        .unwrap_or_else(|| OfficeProject {
            project: project_key.clone(),
            name: args.project.trim().to_string(),
            created_at: Some(now.clone()),
            ..OfficeProject::default()
        });

    let mut next_actions = Vec::new();
    let chat = ensure_office_chat(api, &args, &mut project).await?;
    let wiki = if args.skip_wiki {
        json!({ "status": "skipped" })
    } else {
        ensure_office_wiki_index(api, &args, &mut project, &mut next_actions).await?
    };
    let base = if args.skip_base {
        json!({ "status": "skipped" })
    } else {
        match ensure_office_base(api, &args, &mut project, &mut next_actions).await {
            Ok(value) => value,
            Err(error) => {
                next_actions.push(format!(
                    "Base setup failed after chat setup; grant Base/Wiki permissions or rerun with --skip-base: {error:#}"
                ));
                json!({
                    "status": "error",
                    "error": format!("{error:#}"),
                })
            }
        }
    };
    let tabs = if args.skip_tabs {
        json!({ "status": "skipped" })
    } else {
        add_office_tabs(api, &project).await
    };
    let summary_message = if args.send_summary {
        Some(send_office_summary(api, &mut project).await?)
    } else {
        None
    };

    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;
    sync_legacy_project_chat(&project)?;

    if project.wiki_space_id.is_none() {
        next_actions.push(
            "Set FEISHU_WIKI_SPACE_ID or rerun bootstrap with --space-id to make Wiki the default report route."
                .to_string(),
        );
    }
    if project.base_app_token.is_none() {
        next_actions.push(
            "Rerun without --skip-base after Base/Wiki scopes are granted to enable project log records."
                .to_string(),
        );
    }
    if !args.send_summary {
        next_actions.push(
            "Rerun bootstrap with --send-summary or use office report to send the first visible project update."
                .to_string(),
        );
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "state_file": state_path,
            "chat_id": project.chat_id,
            "wiki_space_id": project.wiki_space_id,
            "wiki_index_node_token": project.wiki_index_node_token,
            "wiki_index_obj_token": project.wiki_index_obj_token,
            "base_node_token": project.base_node_token,
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
            "message_id": project.pinned_summary_message_id,
            "project_state": project,
            "chat": chat,
            "wiki": wiki,
            "base": base,
            "tabs": tabs,
            "summary_message": summary_message,
            "readback": {
                "chat_get": readback_chat(api, project.chat_id.as_deref()).await,
                "wiki_index": readback_wiki_node(api, project.wiki_index_node_token.as_deref(), args.auth).await,
                "base": readback_base(api, project.base_app_token.as_deref(), project.base_table_id.as_deref()).await,
            },
            "next_actions": next_actions,
        }
    }))
}

async fn run_office_report(api: &mut FeishuClient, args: OfficeReportArgs) -> Result<Value> {
    if args.dry_run {
        return run_office_report_dry_run(args);
    }
    let content = read_content(args.content, args.file, args.stdin)?;
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let mut project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let mut next_actions = Vec::new();

    let report = if !args.no_wiki {
        if let Some(space_id) = project.wiki_space_id.as_deref() {
            let parent = project
                .wiki_index_node_token
                .clone()
                .or_else(|| project.wiki_parent_node_token.clone());
            match create_wiki_doc(
                api,
                space_id,
                parent.as_deref(),
                &args.title,
                args.content_type,
                &content,
                args.auth,
            )
            .await
            {
                Ok(created) => json!({
                    "route": "wiki",
                    "document_id": created.document_id,
                    "node_token": created.node_token,
                    "url": created.url,
                    "create_response": created.create_response,
                    "append_response": created.append_response,
                }),
                Err(error) => {
                    next_actions.push(format!(
                        "Wiki report creation failed; created a fallback standalone docx instead: {error:#}"
                    ));
                    let mut fallback = create_standalone_report_doc(
                        api,
                        &args.title,
                        args.content_type,
                        &content,
                        args.auth,
                    )
                    .await?;
                    fallback["route"] = Value::String("docx_fallback".to_string());
                    fallback["wiki_error"] = Value::String(format!("{error:#}"));
                    fallback
                }
            }
        } else {
            next_actions.push(
                "Project has no Wiki space. Run office bootstrap --space-id <space_id>, then rerun report without --no-wiki."
                    .to_string(),
            );
            create_standalone_report_doc(api, &args.title, args.content_type, &content, args.auth)
                .await?
        }
    } else {
        create_standalone_report_doc(api, &args.title, args.content_type, &content, args.auth)
            .await?
    };

    let url = report
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or("<no url returned>");
    let route = report
        .get("route")
        .and_then(Value::as_str)
        .unwrap_or("docx");
    let message_text = format!(
        "{}\n\n状态：报告已写入 {}\n链接：{}",
        args.title, route, url
    );
    let sent = api
        .send_text(chat_id, "chat_id", &message_text, None)
        .await
        .with_context(|| format!("send report notification to project chat {chat_id}"))?;
    let message_id = extract_message_id(&sent);
    let pin = if args.pin {
        pin_message(api, message_id.as_deref()).await
    } else {
        json!({ "status": "skipped" })
    };
    if args.pin {
        project.pinned_summary_message_id = message_id.clone();
    }
    let message_get = readback_message(api, message_id.as_deref()).await;
    let base_record = if args.base_record {
        append_project_base_record(
            api,
            &project,
            "report",
            &args.title,
            "done",
            url,
            truncate_chars(&content, 500).as_str(),
        )
        .await
    } else {
        json!({ "status": "skipped" })
    };
    if args.base_record
        && !base_record
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    {
        next_actions.push(
            "Base record write failed or is not configured. Check project base_app_token/base_table_id with office status --check."
                .to_string(),
        );
    }
    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "message_id": message_id,
            "document_id": report.get("document_id").cloned(),
            "url": url,
            "route": route,
            "report": report,
            "sent_message": sent,
            "pin": pin,
            "message_get": message_get,
            "base_record": base_record,
            "next_actions": next_actions,
        }
    }))
}

async fn run_office_progress(api: &mut FeishuClient, args: OfficeProgressArgs) -> Result<Value> {
    let detail_content =
        read_optional_content(args.content, args.file, args.stdin)?.unwrap_or_default();
    let summary = args
        .summary
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            let fallback = truncate_chars(detail_content.trim(), 500);
            if fallback.is_empty() {
                args.status.clone()
            } else {
                fallback
            }
        });
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let mut project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let mut next_actions = Vec::new();

    let report = if args.wiki_report {
        let report_content = if detail_content.trim().is_empty() {
            format!(
                "# {}\n\n- 状态：{}\n- 摘要：{}\n",
                args.title, args.status, summary
            )
        } else {
            detail_content.clone()
        };
        if let Some(space_id) = project.wiki_space_id.as_deref() {
            let parent = project
                .wiki_index_node_token
                .clone()
                .or_else(|| project.wiki_parent_node_token.clone());
            match create_wiki_doc(
                api,
                space_id,
                parent.as_deref(),
                &args.title,
                args.content_type,
                &report_content,
                args.auth,
            )
            .await
            {
                Ok(created) => json!({
                    "route": "wiki",
                    "document_id": created.document_id,
                    "node_token": created.node_token,
                    "url": created.url,
                    "create_response": created.create_response,
                    "append_response": created.append_response,
                }),
                Err(error) => {
                    next_actions.push(format!(
                        "Wiki progress report creation failed; created a fallback standalone docx instead: {error:#}"
                    ));
                    let mut fallback = create_standalone_report_doc(
                        api,
                        &args.title,
                        args.content_type,
                        &report_content,
                        args.auth,
                    )
                    .await?;
                    fallback["route"] = Value::String("docx_fallback".to_string());
                    fallback["wiki_error"] = Value::String(format!("{error:#}"));
                    fallback
                }
            }
        } else {
            next_actions.push(
                "Project has no Wiki space. Progress message was sent, and the optional detail report used standalone docx fallback."
                    .to_string(),
            );
            create_standalone_report_doc(
                api,
                &args.title,
                args.content_type,
                &report_content,
                args.auth,
            )
            .await?
        }
    } else {
        json!({ "status": "skipped" })
    };

    let report_url = report.get("url").and_then(Value::as_str);
    let message_text = office_progress_message(&args.title, &args.status, &summary, report_url);
    let sent = api
        .send_text(chat_id, "chat_id", &message_text, None)
        .await
        .with_context(|| format!("send progress update to project chat {chat_id}"))?;
    let message_id = extract_message_id(&sent);
    let pin = if args.pin {
        pin_message(api, message_id.as_deref()).await
    } else {
        json!({ "status": "skipped" })
    };
    if args.pin {
        project.pinned_summary_message_id = message_id.clone();
    }
    let message_get = readback_message(api, message_id.as_deref()).await;
    let record_url = report_url
        .map(str::to_string)
        .or_else(|| message_id.as_ref().map(|id| format!("message:{id}")))
        .unwrap_or_default();
    let base_record = if args.no_base_record {
        json!({ "status": "skipped" })
    } else {
        append_project_base_record(
            api,
            &project,
            "progress",
            &args.title,
            &args.status,
            &record_url,
            &summary,
        )
        .await
    };
    if !args.no_base_record
        && !base_record
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    {
        next_actions.push(
            "Base progress record write failed or is not configured. Check office status --check."
                .to_string(),
        );
    }
    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "title": args.title,
            "status": args.status,
            "summary": summary,
            "message_id": message_id,
            "sent_message": sent,
            "pin": pin,
            "message_get": message_get,
            "report": report,
            "base_record": base_record,
            "next_actions": next_actions,
        }
    }))
}

async fn run_office_voice_report(
    api: &mut FeishuClient,
    args: OfficeVoiceReportArgs,
) -> Result<Value> {
    let registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let voice = run_message_send_voice(
        api,
        SendVoiceMessageArgs {
            to: chat_id.to_string(),
            to_type: ReceiveIdTypeArg::ChatId,
            file: args.file,
            text: args.text,
            text_file: args.text_file,
            stdin: args.stdin,
            vox_bin: args.vox_bin,
            voice: args.voice,
            vox_timeout_ms: args.vox_timeout_ms,
            ffmpeg_bin: args.ffmpeg_bin,
            ffprobe_bin: args.ffprobe_bin,
            duration: args.duration,
            name: args.name,
            keep: args.keep,
            readback: true,
            uuid: args.uuid,
        },
    )
    .await?;
    let message_id = extract_message_id(&voice);
    let pin = if args.pin {
        pin_message(api, message_id.as_deref()).await
    } else {
        json!({ "status": "skipped" })
    };
    let reply = if let Some(text) = args.reply_text.filter(|value| !value.trim().is_empty()) {
        probe_value(api.send_text(chat_id, "chat_id", &text, None).await)
    } else {
        json!({ "status": "skipped" })
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "message_id": message_id,
            "voice": voice,
            "pin": pin,
            "reply": reply,
        }
    }))
}

async fn run_office_inbox(api: &mut FeishuClient, args: OfficeInboxArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let data = run_message_poll(
        api,
        MessagePollArgs {
            chat_id: chat_id.to_string(),
            page_size: args.page_size,
            state_file: args.state_file,
            state_key: Some(format!("office:{project_key}")),
            since_position: args.since_position,
            from_now: args.from_now,
            mark_seen: !args.no_mark_seen,
            ack_emoji: if args.no_ack {
                None
            } else {
                Some(args.ack_emoji)
            },
            reply_text: args.reply_text,
            include_app_messages: args.include_app_messages,
            include_system_messages: args.include_system_messages,
        },
    )
    .await?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "inbox": data,
        }
    }))
}

async fn run_office_poll(api: &mut FeishuClient, args: OfficePollArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let data = run_message_poll(
        api,
        MessagePollArgs {
            chat_id: chat_id.to_string(),
            page_size: args.page_size,
            state_file: args.state_file,
            state_key: Some(format!("office:{project_key}")),
            since_position: args.since_position,
            from_now: args.from_now,
            mark_seen: args.mark_seen,
            ack_emoji: args.ack_emoji,
            reply_text: args.reply_text,
            include_app_messages: args.include_app_messages,
            include_system_messages: args.include_system_messages,
        },
    )
    .await?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "poll": data,
        }
    }))
}

async fn run_office_status(api: &mut FeishuClient, args: OfficeStatusArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let state_path = office_registry_path()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let checks = if args.check {
        json!({
            "chat_get": readback_chat(api, project.chat_id.as_deref()).await,
            "wiki_index": readback_wiki_node(api, project.wiki_index_node_token.as_deref(), args.auth).await,
            "base": readback_base(api, project.base_app_token.as_deref(), project.base_table_id.as_deref()).await,
            "pinned_summary": readback_message(api, project.pinned_summary_message_id.as_deref()).await,
        })
    } else {
        json!({ "status": "skipped" })
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "state_file": state_path,
            "chat_id": project.chat_id,
            "wiki_space_id": project.wiki_space_id,
            "wiki_index_node_token": project.wiki_index_node_token,
            "wiki_index_obj_token": project.wiki_index_obj_token,
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
            "message_id": project.pinned_summary_message_id,
            "project_state": project,
            "checks": checks,
        }
    }))
}

async fn run_office_search(api: &mut FeishuClient, args: OfficeSearchArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let run_messages = args.messages || !args.docs;
    let run_docs = args.docs || !args.messages;
    let page_size = args.page_size.max(1);

    let query_text = args.query.clone();
    let message_search = if run_messages {
        if let Some(chat_id) = project.chat_id.as_deref() {
            let mut query = vec![
                ("page_size".to_string(), page_size.min(100).to_string()),
                ("user_id_type".to_string(), "open_id".to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token.clone());
            probe_value(
                api.post_json_user(
                    "/search/v2/message",
                    &query,
                    json!({
                        "query": query_text,
                        "chat_ids": [chat_id],
                    }),
                )
                .await,
            )
        } else {
            json!({ "ok": false, "error": "project has no chat_id" })
        }
    } else {
        json!({ "status": "skipped" })
    };

    let docs_search = if run_docs {
        let mut wiki_filter = Map::new();
        if let Some(space_id) = project.wiki_space_id.clone() {
            wiki_filter.insert("space_ids".to_string(), json!([space_id]));
        }
        let mut body = Map::new();
        body.insert("query".to_string(), Value::String(args.query.clone()));
        body.insert(
            "page_size".to_string(),
            Value::Number(page_size.min(20).into()),
        );
        push_json_opt(&mut body, "page_token", args.page_token);
        body.insert("doc_filter".to_string(), Value::Object(Map::new()));
        body.insert("wiki_filter".to_string(), Value::Object(wiki_filter));
        probe_value(
            api.post_json_user("/search/v2/doc_wiki/search", &[], Value::Object(body))
                .await,
        )
    } else {
        json!({ "status": "skipped" })
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "query": args.query,
            "messages": message_search,
            "docs": docs_search,
        }
    }))
}

async fn run_office_cleanup(api: &mut FeishuClient, args: OfficeCleanupArgs) -> Result<Value> {
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let dry_run = args.dry_run || !args.confirm;
    let mut planned = Vec::new();
    let mut applied = Vec::new();
    let mut api_results = Vec::new();

    planned.push(json!({
        "action": "remove_local_project_state",
        "project": project_key,
    }));
    let delete_messages = args.delete_messages && !args.local_only;
    if delete_messages {
        if let Some(message_id) = project.pinned_summary_message_id.as_deref() {
            planned.push(json!({
                "action": "delete_message",
                "message_id": message_id,
            }));
        }
    }

    if !dry_run {
        if delete_messages {
            if let Some(message_id) = project.pinned_summary_message_id.as_deref() {
                api_results.push(json!({
                    "action": "delete_message",
                    "message_id": message_id,
                    "result": probe_value(api.delete_message(message_id).await),
                }));
            }
        }
        registry.projects.remove(&project_key);
        write_office_registry(&registry)?;
        applied.push(json!({ "action": "remove_local_project_state" }));
    }

    let mut next_actions = vec![
        "Feishu does not expose a personal left-sidebar hide/delete API for conversations; use chat delete only when you want to dissolve the group for everyone.".to_string(),
        "Wiki nodes, Base apps, and project chats are not deleted by office cleanup. Use atomic wiki/base/chat commands for irreversible resource deletion.".to_string(),
    ];
    if args.local_only {
        next_actions.push(
            "--local-only keeps Feishu resources untouched; this command only removes local registry state when --confirm is present."
                .to_string(),
        );
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "dry_run": dry_run,
            "local_only": args.local_only,
            "delete_messages": delete_messages,
            "planned": planned,
            "applied": applied,
            "api_results": api_results,
            "next_actions": next_actions,
        }
    }))
}

async fn ensure_office_chat(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
) -> Result<Value> {
    if let Some(chat_id) = args
        .chat_id
        .clone()
        .or_else(|| project.chat_id.clone())
        .filter(|value| !value.trim().is_empty())
    {
        project.chat_id = Some(chat_id.clone());
        return Ok(json!({
            "status": "reused",
            "chat_id": chat_id,
            "readback": readback_chat(api, project.chat_id.as_deref()).await,
        }));
    }

    let mut users = args.users.clone();
    if users.is_empty() {
        if let Some(default_user_id) = api.config.default_user_id.clone() {
            users.push(default_user_id);
        }
    }
    if users.is_empty() {
        bail!("office bootstrap needs --user or FEISHU_USER_ID to create a project chat");
    }
    let user_type = args.user_id_type.resolve(users.first().map(String::as_str));
    let mut body = json!({
        "name": project.name,
        "description": format!("feishu-bot office project chat: {}", project.name),
        "chat_mode": "group",
        "chat_type": "private",
        "group_message_type": "chat",
        "user_id_list": users,
    });
    if let Some(path) = args.avatar_file.as_ref() {
        let uploaded = api.upload_im_image(path, "avatar").await?;
        if let Some(image_key) = get_string(&uploaded, &["data", "image_key"]) {
            body["avatar"] = Value::String(image_key);
        }
    }
    let created = api
        .post_json(
            "/im/v1/chats",
            &[("user_id_type".to_string(), user_type.to_string())],
            body,
        )
        .await?;
    let chat_id = extract_chat_id(&created)
        .ok_or_else(|| anyhow!("create chat response missing chat_id: {created}"))?;
    project.chat_id = Some(chat_id.clone());
    Ok(json!({
        "status": "created",
        "chat_id": chat_id,
        "create_response": created,
    }))
}

async fn ensure_office_wiki_index(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    let Some(space_id) = args
        .space_id
        .clone()
        .or_else(|| project.wiki_space_id.clone())
        .or_else(|| api.config.default_wiki_space_id.clone())
        .filter(|value| !value.trim().is_empty())
    else {
        next_actions.push(
            "Wiki index skipped because no --space-id or FEISHU_WIKI_SPACE_ID is configured."
                .to_string(),
        );
        return Ok(json!({ "status": "skipped_missing_space" }));
    };
    project.wiki_space_id = Some(space_id.clone());
    project.wiki_parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| project.wiki_parent_node_token.clone())
        .or_else(|| api.config.default_wiki_parent_node_token.clone());

    if project.wiki_index_obj_token.is_some() && project.wiki_index_node_token.is_some() {
        return Ok(json!({
            "status": "reused",
            "space_id": space_id,
            "node_token": project.wiki_index_node_token,
            "document_id": project.wiki_index_obj_token,
            "url": project.wiki_index_node_token.as_deref().map(|token| wiki_url(api, token)),
        }));
    }

    let created = match create_wiki_doc(
        api,
        &space_id,
        project.wiki_parent_node_token.as_deref(),
        &format!("{} 项目主页", project.name),
        ContentTypeArg::Markdown,
        &office_index_markdown(project),
        args.auth,
    )
    .await
    {
        Ok(created) => created,
        Err(error) => {
            next_actions.push(format!(
                "Wiki index creation failed but the project chat can still be used. Grant the app/bot edit permission in the Wiki space, or rerun bootstrap with --auth user/--skip-wiki: {error:#}"
            ));
            return Ok(json!({
                "status": "error",
                "space_id": space_id,
                "error": format!("{error:#}"),
            }));
        }
    };
    project.wiki_index_node_token = created.node_token.clone();
    project.wiki_index_obj_token = Some(created.document_id.clone());
    Ok(json!({
        "status": "created",
        "space_id": space_id,
        "node_token": created.node_token,
        "document_id": created.document_id,
        "url": created.url,
        "create_response": created.create_response,
        "append_response": created.append_response,
    }))
}

async fn ensure_office_base(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    if project.base_app_token.is_some() && project.base_table_id.is_some() {
        return Ok(json!({
            "status": "reused",
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
        }));
    }

    let mut create_route = "base";
    let mut base_node_token = None;
    let app_token = if let Some(space_id) = project.wiki_space_id.as_deref() {
        let path = format!("/wiki/v2/spaces/{}/nodes", encode_path_segment(space_id));
        let body = json!({
            "obj_type": "bitable",
            "node_type": "origin",
            "title": format!("{} 项目多维表格", project.name),
        });
        match wiki_request_json(api, Method::POST, &path, &[], Some(body), args.auth).await {
            Ok(value) => {
                create_route = "wiki_bitable";
                base_node_token = get_string(&value, &["data", "node", "node_token"]);
                get_string(&value, &["data", "node", "obj_token"])
                    .or_else(|| get_string(&value, &["data", "obj_token"]))
                    .ok_or_else(|| anyhow!("wiki bitable response missing obj_token: {value}"))?
            }
            Err(error) => {
                next_actions.push(format!(
                    "Wiki bitable creation failed; fell back to plain Base create: {error:#}"
                ));
                create_plain_base(api, &project.name).await?
            }
        }
    } else {
        create_plain_base(api, &project.name).await?
    };

    let table = create_project_log_table(api, &app_token).await?;
    let table_id = extract_table_id(&table)
        .ok_or_else(|| anyhow!("project log table create response missing table_id: {table}"))?;
    project.base_node_token = base_node_token;
    project.base_app_token = Some(app_token.clone());
    project.base_table_id = Some(table_id.clone());
    Ok(json!({
        "status": "created",
        "route": create_route,
        "app_token": app_token,
        "node_token": project.base_node_token,
        "table_id": table_id,
        "table_response": table,
    }))
}

async fn add_office_tabs(api: &mut FeishuClient, project: &OfficeProject) -> Value {
    let Some(chat_id) = project.chat_id.as_deref() else {
        return json!({ "status": "skipped_missing_chat" });
    };
    let mut items = Vec::new();
    if let Some(node_token) = project.wiki_index_node_token.as_deref() {
        items.push(json!({
            "name": "项目主页",
            "result": probe_value(add_chat_url_tab(api, chat_id, "项目主页", &wiki_url(api, node_token)).await),
        }));
    }
    if let Some(app_token) = project.base_app_token.as_deref() {
        items.push(json!({
            "name": "项目日志",
            "result": probe_value(add_chat_url_tab(api, chat_id, "项目日志", &base_url(api, app_token)).await),
        }));
    }
    json!({ "status": "attempted", "items": items })
}

async fn send_office_summary(api: &mut FeishuClient, project: &mut OfficeProject) -> Result<Value> {
    let chat_id = required_project_field(project.chat_id.as_deref(), &project.project, "chat_id")?;
    let mut lines = vec![
        format!("{} 项目空间已初始化", project.name),
        "后续 AI 汇报会按项目独立写入这个群聊。".to_string(),
    ];
    if let Some(node_token) = project.wiki_index_node_token.as_deref() {
        lines.push(format!("Wiki：{}", wiki_url(api, node_token)));
    }
    if let Some(app_token) = project.base_app_token.as_deref() {
        lines.push(format!("Base：{}", base_url(api, app_token)));
    }
    let sent = api
        .send_text(chat_id, "chat_id", &lines.join("\n"), None)
        .await?;
    let message_id = extract_message_id(&sent);
    let pin = pin_message(api, message_id.as_deref()).await;
    project.pinned_summary_message_id = message_id.clone();
    Ok(json!({
        "sent": sent,
        "message_id": message_id,
        "pin": pin,
        "message_get": readback_message(api, message_id.as_deref()).await,
    }))
}

async fn create_wiki_doc(
    api: &mut FeishuClient,
    space_id: &str,
    parent_node_token: Option<&str>,
    title: &str,
    content_type: ContentTypeArg,
    content: &str,
    auth: ApiAuthArg,
) -> Result<CreatedDoc> {
    let path = format!("/wiki/v2/spaces/{}/nodes", encode_path_segment(space_id));
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String("docx".to_string()));
    body.insert("node_type".to_string(), Value::String("origin".to_string()));
    body.insert("title".to_string(), Value::String(title.to_string()));
    if let Some(parent) = parent_node_token.filter(|value| !value.trim().is_empty()) {
        body.insert(
            "parent_node_token".to_string(),
            Value::String(parent.to_string()),
        );
    }
    let create_response = wiki_request_json(
        api,
        Method::POST,
        &path,
        &[],
        Some(Value::Object(body)),
        auth,
    )
    .await?;
    let document_id = get_string(&create_response, &["data", "node", "obj_token"])
        .or_else(|| get_string(&create_response, &["data", "obj_token"]))
        .ok_or_else(|| {
            anyhow!("wiki create-node response missing docx obj_token: {create_response}")
        })?;
    let node_token = get_string(&create_response, &["data", "node", "node_token"])
        .or_else(|| get_string(&create_response, &["data", "node_token"]));
    let append_response = if content.trim().is_empty() {
        None
    } else {
        Some(
            api.append_converted_content_with_auth(
                &document_id,
                &document_id,
                content_type,
                content,
                auth,
            )
            .await?,
        )
    };
    let url = node_token
        .as_deref()
        .map(|token| wiki_url(api, token))
        .unwrap_or_else(|| api.document_url(&document_id));
    Ok(CreatedDoc {
        document_id,
        node_token,
        url,
        create_response,
        append_response,
    })
}

async fn create_standalone_report_doc(
    api: &mut FeishuClient,
    title: &str,
    content_type: ContentTypeArg,
    content: &str,
    auth: ApiAuthArg,
) -> Result<Value> {
    let created = api.create_document_with_auth(title, None, auth).await?;
    let document_id = get_string(&created, &["data", "document", "document_id"])
        .or_else(|| get_string(&created, &["data", "document_id"]))
        .ok_or_else(|| anyhow!("create document response missing document_id: {created}"))?;
    let append_response = if content.trim().is_empty() {
        None
    } else {
        Some(
            api.append_converted_content_with_auth(
                &document_id,
                &document_id,
                content_type,
                content,
                auth,
            )
            .await?,
        )
    };
    Ok(json!({
        "route": "docx",
        "document_id": document_id,
        "url": api.document_url(&document_id),
        "create_response": created,
        "append_response": append_response,
    }))
}

async fn create_plain_base(api: &mut FeishuClient, project_name: &str) -> Result<String> {
    let created = api
        .post_json(
            "/bitable/v1/apps",
            &[],
            json!({ "name": format!("{project_name} 项目多维表格") }),
        )
        .await?;
    get_string(&created, &["data", "app", "app_token"])
        .or_else(|| get_string(&created, &["data", "app_token"]))
        .ok_or_else(|| anyhow!("create Base response missing app_token: {created}"))
}

async fn create_project_log_table(api: &mut FeishuClient, app_token: &str) -> Result<Value> {
    let path = format!("/bitable/v1/apps/{app_token}/tables");
    let fields = ["类型", "标题", "状态", "链接", "摘要", "创建时间"]
        .into_iter()
        .map(|field_name| json!({ "field_name": field_name, "type": 1 }))
        .collect::<Vec<_>>();
    api.post_json(
        &path,
        &[],
        json!({
            "table": {
                "name": PROJECT_LOG_TABLE_NAME,
                "fields": fields,
            }
        }),
    )
    .await
}

async fn append_project_base_record(
    api: &mut FeishuClient,
    project: &OfficeProject,
    kind: &str,
    title: &str,
    status: &str,
    url: &str,
    summary: &str,
) -> Value {
    let (Some(app_token), Some(table_id)) = (
        project.base_app_token.as_deref(),
        project.base_table_id.as_deref(),
    ) else {
        return json!({ "ok": false, "error": "project has no base_app_token/base_table_id" });
    };
    let path = format!("/bitable/v1/apps/{app_token}/tables/{table_id}/records");
    probe_value(
        api.post_json(
            &path,
            &[],
            json!({
                "fields": {
                    "类型": kind,
                    "标题": title,
                    "状态": status,
                    "链接": url,
                    "摘要": summary,
                    "创建时间": office_now(),
                }
            }),
        )
        .await,
    )
}

async fn add_chat_url_tab(
    api: &mut FeishuClient,
    chat_id: &str,
    name: &str,
    url: &str,
) -> Result<Value> {
    let path = format!("/im/v1/chats/{chat_id}/chat_tabs");
    api.post_json(
        &path,
        &[],
        json!({
            "chat_tabs": [{
                "tab_name": name,
                "tab_type": "url",
                "tab_content": { "url": url },
                "tab_config": { "is_built_in": true },
            }]
        }),
    )
    .await
}

async fn pin_message(api: &mut FeishuClient, message_id: Option<&str>) -> Value {
    let Some(message_id) = message_id else {
        return json!({ "ok": false, "error": "missing message_id" });
    };
    probe_value(
        api.post_json("/im/v1/pins", &[], json!({ "message_id": message_id }))
            .await,
    )
}

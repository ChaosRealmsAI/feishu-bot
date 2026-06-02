use super::*;

mod docs;
mod formatting;
mod interactions;
mod links;
mod local;
mod readback;
mod resources;
mod state;

use docs::*;
use formatting::*;
use interactions::*;
use links::*;
pub(super) use local::{office_command_can_run_without_api, run_office_local_command};
use local::{run_office_bootstrap_dry_run, run_office_list, run_office_report_dry_run};
use readback::*;
use resources::*;
pub(super) use state::*;

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

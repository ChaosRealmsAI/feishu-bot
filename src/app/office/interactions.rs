use super::*;

pub(super) async fn run_office_voice_report(
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

pub(super) async fn run_office_inbox(
    api: &mut FeishuClient,
    args: OfficeInboxArgs,
) -> Result<Value> {
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

pub(super) async fn run_office_poll(api: &mut FeishuClient, args: OfficePollArgs) -> Result<Value> {
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

pub(super) async fn run_office_status(
    api: &mut FeishuClient,
    args: OfficeStatusArgs,
) -> Result<Value> {
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

pub(super) async fn run_office_search(
    api: &mut FeishuClient,
    args: OfficeSearchArgs,
) -> Result<Value> {
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

pub(super) async fn run_office_cleanup(
    api: &mut FeishuClient,
    args: OfficeCleanupArgs,
) -> Result<Value> {
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

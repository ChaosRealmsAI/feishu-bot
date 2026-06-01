use super::*;

pub(in crate::app) fn office_command_can_run_without_api(command: &OfficeCommand) -> bool {
    match command {
        OfficeCommand::List(_) => true,
        OfficeCommand::Bootstrap(args) => args.dry_run,
        OfficeCommand::Report(args) => args.dry_run,
        OfficeCommand::Status(args) => !args.check,
        OfficeCommand::Cleanup(args) => args.dry_run || !args.confirm || args.local_only,
        OfficeCommand::Progress(_)
        | OfficeCommand::VoiceReport(_)
        | OfficeCommand::Inbox(_)
        | OfficeCommand::Poll(_)
        | OfficeCommand::Search(_) => false,
    }
}

pub(in crate::app) fn run_office_local_command(
    command: OfficeCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OfficeCommand::List(args) => run_office_list(args)?,
        OfficeCommand::Bootstrap(args) if args.dry_run => run_office_bootstrap_dry_run(args)?,
        OfficeCommand::Report(args) if args.dry_run => run_office_report_dry_run(args)?,
        OfficeCommand::Status(args) if !args.check => run_office_status_local(args)?,
        OfficeCommand::Cleanup(args) if args.dry_run || !args.confirm || args.local_only => {
            run_office_cleanup_local(args)?
        }
        _ => bail!("office command requires Feishu API credentials"),
    };
    print_response(raw_json, "office workflow completed", data)
}

pub(super) fn run_office_list(args: OfficeListArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let state_path = office_registry_path()?;
    let mut projects: Vec<_> = registry.projects.values().cloned().collect();
    projects.sort_by(|left, right| left.project.cmp(&right.project));
    let items: Vec<Value> = projects
        .into_iter()
        .map(|project| {
            if args.details {
                serde_json::to_value(project).unwrap_or_else(|_| json!({}))
            } else {
                json!({
                    "project": project.project,
                    "name": project.name,
                    "has_chat": project.chat_id.is_some(),
                    "has_wiki": project.wiki_index_node_token.is_some(),
                    "has_base": project.base_app_token.is_some() && project.base_table_id.is_some(),
                    "has_pinned_summary": project.pinned_summary_message_id.is_some(),
                    "updated_at": project.updated_at,
                })
            }
        })
        .collect();
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "state_file": state_path,
            "count": items.len(),
            "projects": items,
        }
    }))
}

pub(super) fn run_office_bootstrap_dry_run(args: OfficeBootstrapArgs) -> Result<Value> {
    let project_key = office_project_key(&args.project)?;
    let state_path = office_registry_path()?;
    let registry = read_office_registry()?;
    let existing = registry.projects.get(&project_key);
    let space_id = args
        .space_id
        .clone()
        .or_else(|| std::env::var("FEISHU_WIKI_SPACE_ID").ok());
    let parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| std::env::var("FEISHU_WIKI_PARENT_NODE_TOKEN").ok());
    let users = if args.users.is_empty() {
        std::env::var("FEISHU_USER_ID")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(|value| vec![value])
            .unwrap_or_default()
    } else {
        args.users.clone()
    };
    let chat_action =
        if args.chat_id.is_some() || existing.and_then(|item| item.chat_id.as_ref()).is_some() {
            "reuse_chat"
        } else {
            "create_chat"
        };
    let mut planned = vec![json!({
        "action": chat_action,
        "requires_api": true,
        "writes_real_feishu_data": chat_action == "create_chat",
        "user_count": users.len(),
        "sets_avatar": args.avatar_file.is_some(),
    })];
    if !args.skip_wiki {
        planned.push(json!({
            "action": "create_or_reuse_wiki_index",
            "requires_api": true,
            "writes_real_feishu_data": true,
            "space_configured": space_id.is_some(),
            "parent_node_configured": parent_node_token.is_some(),
            "auth": format!("{:?}", args.auth).to_lowercase(),
        }));
    }
    if !args.skip_base {
        planned.push(json!({
            "action": "create_or_reuse_base_log",
            "requires_api": true,
            "writes_real_feishu_data": true,
        }));
    }
    if !args.skip_tabs {
        planned.push(json!({
            "action": "add_project_chat_tabs",
            "requires_api": true,
            "writes_real_feishu_data": true,
        }));
    }
    if args.send_summary {
        planned.push(json!({
            "action": "send_and_pin_project_summary",
            "requires_api": true,
            "writes_real_feishu_data": true,
        }));
    }

    let mut next_actions = Vec::new();
    if chat_action == "create_chat" && users.is_empty() {
        next_actions
            .push("Provide --user or FEISHU_USER_ID before running without --dry-run.".to_string());
    }
    if !args.skip_wiki && space_id.is_none() {
        next_actions.push(
            "Set FEISHU_WIKI_SPACE_ID or pass --space-id before running without --dry-run."
                .to_string(),
        );
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "dry_run": true,
            "project": project_key,
            "state_file": state_path,
            "existing_project": existing.is_some(),
            "would_write_local_state": false,
            "planned": planned,
            "next_actions": next_actions,
        }
    }))
}

pub(super) fn run_office_report_dry_run(args: OfficeReportArgs) -> Result<Value> {
    let content = read_content(args.content, args.file, args.stdin)?;
    let registry = read_office_registry()?;
    let state_path = office_registry_path()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let route = if !args.no_wiki && project.wiki_space_id.is_some() {
        "wiki"
    } else if args.no_wiki {
        "docx"
    } else {
        "docx_fallback"
    };
    let planned = vec![
        json!({
            "action": "create_report_document",
            "route": route,
            "content_type": format!("{:?}", args.content_type).to_lowercase(),
            "requires_api": true,
            "writes_real_feishu_data": true,
        }),
        json!({
            "action": "send_project_chat_notification",
            "requires_api": true,
            "writes_real_feishu_data": true,
            "chat_configured": project.chat_id.is_some(),
        }),
        json!({
            "action": "pin_notification",
            "enabled": args.pin,
            "requires_api": args.pin,
            "writes_real_feishu_data": args.pin,
        }),
        json!({
            "action": "append_base_record",
            "enabled": args.base_record,
            "requires_api": args.base_record,
            "writes_real_feishu_data": args.base_record,
            "base_configured": project.base_app_token.is_some() && project.base_table_id.is_some(),
        }),
    ];
    let mut next_actions = Vec::new();
    if project.chat_id.is_none() {
        next_actions
            .push("Run office bootstrap before running report without --dry-run.".to_string());
    }
    if route == "docx_fallback" {
        next_actions.push(
            "Project has no Wiki space; report would create a standalone docx fallback."
                .to_string(),
        );
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "dry_run": true,
            "project": project_key,
            "state_file": state_path,
            "title": args.title,
            "content_chars": content.chars().count(),
            "would_write_local_state": false,
            "planned": planned,
            "next_actions": next_actions,
        }
    }))
}

fn run_office_status_local(args: OfficeStatusArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let state_path = office_registry_path()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
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
            "checks": {
                "status": "skipped",
                "reason": "Run with --check to probe Feishu resources; --check requires API credentials."
            },
        }
    }))
}

fn run_office_cleanup_local(args: OfficeCleanupArgs) -> Result<Value> {
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let project = get_office_project(&registry, &project_key)?;
    let dry_run = args.dry_run || !args.confirm;
    let mut planned = Vec::new();
    let mut applied = Vec::new();

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
                "requires_api": true,
            }));
        }
    }

    if !dry_run {
        if delete_messages {
            bail!("delete message cleanup requires Feishu API credentials; rerun without --local-only through the normal office cleanup path");
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
            "api_results": [],
            "next_actions": next_actions,
        }
    }))
}

use super::*;

pub(in crate::app::office) fn run_office_bootstrap_dry_run(
    args: OfficeBootstrapArgs,
) -> Result<Value> {
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

pub(in crate::app::office) fn run_office_report_dry_run(args: OfficeReportArgs) -> Result<Value> {
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

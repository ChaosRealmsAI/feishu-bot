use super::*;

pub(super) fn build_setup_grant(
    values: &HashMap<String, String>,
    groups: &[String],
    token_type: ApiAuthArg,
) -> Result<Value> {
    let app_id = get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).ok_or_else(|| {
        anyhow!("missing FEISHU_APP_ID or LARK_APP_ID; set it before opening scope grants")
    })?;
    let selected = setup_group_names(groups);
    let scopes = collect_setup_scopes(&selected)?;
    let token_type_text = scope_token_type(token_type);
    let url = format!(
        "https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot-setup&token_type={}",
        app_id,
        scopes.join(","),
        token_type_text
    );
    Ok(json!({
        "ok": true,
        "groups": selected,
        "scope_count": scopes.len(),
        "scopes": scopes,
        "token_type": token_type_text,
        "grant_url": url,
        "browser_command": format!("feishu-bot browser open --url \"{url}\""),
    }))
}

pub(super) fn setup_group_names(groups: &[String]) -> Vec<String> {
    let raw = if groups.is_empty() {
        vec!["office".to_string()]
    } else {
        groups.to_vec()
    };
    let mut out = Vec::new();
    for group in raw {
        let group = group.trim().to_ascii_lowercase();
        if group.is_empty() {
            continue;
        }
        let expanded: Vec<&str> = match group.as_str() {
            "office" => OFFICE_SCOPE_GROUPS.to_vec(),
            other => vec![other],
        };
        for item in expanded {
            if !out.iter().any(|existing| existing == item) {
                out.push(item.to_string());
            }
        }
    }
    out
}

pub(super) fn setup_env_status(values: &HashMap<String, String>) -> Value {
    json!({
        "app_id": get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).as_deref().map(mask_app_id),
        "app_secret_configured": get_any(values, &["FEISHU_APP_SECRET", "LARK_APP_SECRET"]).is_some(),
        "default_user_id_configured": get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_some(),
        "user_access_token_configured": get_any(values, &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"]).is_some(),
        "refresh_token_configured": get_any(values, &["FEISHU_REFRESH_TOKEN", "LARK_REFRESH_TOKEN"]).is_some(),
        "wiki_space_id": get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some(),
        "office_state_file": get_any(values, &["FEISHU_OFFICE_STATE_FILE", "LARK_OFFICE_STATE_FILE"]),
    })
}

pub(super) fn setup_oauth_plan(values: &HashMap<String, String>) -> Value {
    let configured = get_any(
        values,
        &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
    )
    .is_some();
    json!({
        "user_access_token_configured": configured,
        "url_command": "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope search:docs:read --scope search:message --scope wiki:wiki --scope wiki:node:create",
        "token_command": "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
        "refresh_command": "feishu-bot oauth refresh --save-env",
    })
}

pub(super) fn setup_wiki_bot_plan(values: &HashMap<String, String>) -> Value {
    json!({
        "space_configured": get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some(),
        "user_token_configured": get_any(values, &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"]).is_some(),
        "command": "feishu-bot setup wiki-bot --auth user",
        "manual_equivalent": "feishu-bot bot info && feishu-bot wiki member add --auth user --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin",
    })
}

pub(super) fn setup_quickstart_plan(
    values: &HashMap<String, String>,
    project: &str,
    selected_groups: &[String],
) -> Value {
    let project = if project.trim().is_empty() {
        "AI Project"
    } else {
        project.trim()
    };
    let project_arg = shell_quote(project);
    let group_args = if selected_groups.is_empty() {
        "--group office".to_string()
    } else {
        selected_groups
            .iter()
            .map(|group| format!("--group {}", shell_quote(group)))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let user_arg = if get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_some() {
        "--user \"$FEISHU_USER_ID\""
    } else {
        "--user <open_id>"
    };
    let space_arg = if get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some() {
        "--space-id \"$FEISHU_WIKI_SPACE_ID\""
    } else {
        "--space-id <space_id>"
    };
    json!({
        "purpose": "Bring a new Feishu app/account to the common AI office workflow: group chat, Wiki docs, Base project log, message polling, and search.",
        "script": format!("scripts/feishu-bot-setup.sh --project {project_arg}"),
        "commands": {
            "inspect_env": "feishu-bot doctor",
            "plan": "feishu-bot setup plan --json",
            "grant_permissions": format!("feishu-bot setup open-scopes {group_args} --browser --json"),
            "oauth_url": "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope search:docs:read --scope search:message --scope wiki:wiki --scope wiki:node:create",
            "save_oauth_code": "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
            "refresh_oauth": "feishu-bot oauth refresh --save-env",
            "add_wiki_bot": "feishu-bot setup wiki-bot --auth user --json",
            "bootstrap_dry_run": format!("feishu-bot office bootstrap --project {project_arg} --dry-run --json"),
            "bootstrap": format!("feishu-bot office bootstrap --project {project_arg} {user_arg} {space_arg} --send-summary --json"),
            "status_check": format!("feishu-bot office status --project {project_arg} --check --json"),
            "progress_update": format!("feishu-bot office progress --project {project_arg} --title \"进度更新\" --status doing --summary \"当前进展\" --json"),
            "inbox_from_now": format!("feishu-bot office inbox --project {project_arg} --from-now --json"),
            "inbox_process": format!("feishu-bot office inbox --project {project_arg} --reply-text \"收到，我来处理\" --json"),
            "search": format!("feishu-bot office search --project {project_arg} --query \"关键词\" --json")
        },
        "common_order": [
            "doctor",
            "setup quickstart/open scope grant",
            "OAuth user token when user-token APIs are needed",
            "setup wiki-bot when using a Wiki space",
            "office bootstrap",
            "office report/progress/voice-report",
            "office inbox/search/status"
        ]
    })
}

pub(super) fn setup_browser_plan() -> Value {
    json!({
        "ensure_command": "feishu-bot browser ensure",
        "open_scope_command": "feishu-bot setup open-scopes --browser",
        "account_safety": "For multi-account Chrome profiles, bind and verify the intended Playwright MCP profile before approving account-sensitive grants.",
    })
}

pub(super) fn setup_next_actions(values: &HashMap<String, String>) -> Vec<String> {
    let mut actions = Vec::new();
    if get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).is_none() {
        actions.push("Set FEISHU_APP_ID or LARK_APP_ID.".to_string());
    }
    if get_any(values, &["FEISHU_APP_SECRET", "LARK_APP_SECRET"]).is_none() {
        actions.push("Set FEISHU_APP_SECRET or LARK_APP_SECRET.".to_string());
    }
    if get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_none() {
        actions
            .push("Set FEISHU_USER_ID so setup can create one-person project groups.".to_string());
    }
    if get_any(
        values,
        &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
    )
    .is_none()
    {
        actions.push("Run the OAuth URL/token flow to enable user-token APIs such as Wiki member writes and search.".to_string());
    }
    if get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_none() {
        actions.push("Set FEISHU_WIKI_SPACE_ID if reports should default to Wiki.".to_string());
    }
    if actions.is_empty() {
        actions.push("Run `feishu-bot setup quickstart --open-browser`, approve scopes if needed, then run `feishu-bot dogfood verify --include-response`.".to_string());
    }
    actions
}

fn collect_setup_scopes(groups: &[String]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for group in groups {
        for (_name, scopes) in scope_groups(group)? {
            for scope in scopes {
                if !out.iter().any(|existing| existing == scope) {
                    out.push(scope.to_string());
                }
            }
        }
    }
    Ok(out)
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

use super::*;

pub(in crate::app::dogfood::probes) fn dogfood_probe_remediation(
    app_id: &str,
    module: &str,
    scope_group: &str,
    status: &str,
    missing_scopes: Vec<String>,
    rerun_command: &str,
) -> Value {
    match status {
        "missing_scope" => {
            let exact_grant_url =
                (!missing_scopes.is_empty()).then(|| dogfood_grant_url(app_id, &missing_scopes));
            let group_scopes = dogfood_scope_group_scopes(scope_group);
            let group_grant_url =
                (!group_scopes.is_empty()).then(|| dogfood_grant_url(app_id, &group_scopes));
            let preferred_url = exact_grant_url
                .as_ref()
                .or(group_grant_url.as_ref())
                .cloned();
            json!({
                "action": "grant_scopes",
                "scope_group": scope_group,
                "missing_scopes": missing_scopes,
                "grant_url": exact_grant_url,
                "grant_group_url": group_grant_url,
                "scope_command": format!("feishu-bot scopes --group {scope_group}"),
                "browser_command": preferred_url.map(|url| format!("feishu-bot browser open --url \"{url}\"")),
                "rerun_command": rerun_command,
            })
        }
        "missing_user_token" => json!({
            "action": "set_user_access_token",
            "env": ["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
            "oauth_url_command": dogfood_user_token_oauth_command(module),
            "oauth_token_command": dogfood_oauth_token_command(),
            "why": "This Feishu API requires a user_access_token and operates in the human user's visibility context.",
            "rerun_command": rerun_command,
        }),
        "expired_user_token" => json!({
            "action": "refresh_user_access_token",
            "env": ["FEISHU_USER_ACCESS_TOKEN", "FEISHU_REFRESH_TOKEN", "LARK_USER_ACCESS_TOKEN", "LARK_REFRESH_TOKEN"],
            "oauth_refresh_command": dogfood_oauth_refresh_command(),
            "oauth_url_command": dogfood_user_token_oauth_command(module),
            "oauth_token_command": dogfood_oauth_token_command(),
            "why": "Feishu returned 99991677 Authentication token expired. Refresh the user token if a refresh token exists; otherwise rerun OAuth for this module.",
            "rerun_command": rerun_command,
        }),
        "missing_helpdesk_config" => json!({
            "action": "set_helpdesk_config",
            "env": ["FEISHU_HELPDESK_ID", "FEISHU_HELPDESK_TOKEN"],
            "why": "Helpdesk OpenAPI calls require the helpdesk id and helpdesk token header in addition to the app token.",
            "rerun_command": rerun_command,
        }),
        "no_data" => json!({
            "action": "none",
            "why": "The API and permissions are reachable, but this tenant/account has no data for the probe.",
            "rerun_command": rerun_command,
        }),
        "upstream_api_error" => json!({
            "action": "retry_or_check_product_availability",
            "why": "Feishu returned a non-permission server/product error. Recheck later or confirm the product is enabled for this account.",
            "rerun_command": rerun_command,
        }),
        _ => json!({
            "action": "inspect_api_error",
            "rerun_command": rerun_command,
        }),
    }
}

fn dogfood_oauth_token_command() -> String {
    "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env --env-file \"${FEISHU_ENV_FILE:-private/local.env}\"".to_string()
}

fn dogfood_oauth_refresh_command() -> String {
    "feishu-bot oauth refresh --save-env --env-file \"${FEISHU_ENV_FILE:-private/local.env}\""
        .to_string()
}

fn dogfood_user_token_oauth_command(module: &str) -> String {
    let scopes = match module {
        "task" => vec![
            "offline_access",
            "auth:user.id:read",
            "task:task:read",
            "task:task:write",
        ],
        "mail" => vec![
            "offline_access",
            "auth:user.id:read",
            "mail:user_mailbox",
            "mail:user_mailbox:readonly",
            "mail:user_mailbox.message:readonly",
            "mail:user_mailbox.folder:read",
        ],
        "minutes" => vec![
            "offline_access",
            "auth:user.id:read",
            "minutes:minutes",
            "minutes:minutes:readonly",
            "minutes:minutes.search:read",
        ],
        "search" => vec![
            "offline_access",
            "auth:user.id:read",
            "search:docs:read",
            "search:message",
        ],
        "wiki" => vec![
            "offline_access",
            "auth:user.id:read",
            "docx:document:readonly",
            "docx:document:write_only",
            "wiki:wiki",
            "wiki:wiki:readonly",
            "wiki:space:retrieve",
            "wiki:space:read",
            "wiki:space:write_only",
            "wiki:node:retrieve",
            "wiki:node:read",
            "wiki:node:create",
            "wiki:node:move",
            "wiki:node:copy",
            "wiki:node:update",
            "wiki:member:retrieve",
            "wiki:member:create",
            "wiki:member:update",
            "wiki:setting:write_only",
        ],
        _ => vec!["offline_access", "auth:user.id:read"],
    };
    let scope_args = scopes
        .into_iter()
        .map(|scope| format!("--scope {scope}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!("feishu-bot oauth url {scope_args}")
}

fn dogfood_grant_url(app_id: &str, scopes: &[String]) -> String {
    format!(
        "https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot&token_type=tenant",
        app_id,
        scopes.join(",")
    )
}

fn dogfood_scope_group_scopes(group: &str) -> Vec<String> {
    scope_groups(group)
        .ok()
        .and_then(|mut groups| groups.pop())
        .map(|(_, scopes)| scopes.into_iter().map(ToString::to_string).collect())
        .unwrap_or_default()
}

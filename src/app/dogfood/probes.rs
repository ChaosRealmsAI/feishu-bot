use super::*;

pub(in crate::app) fn dogfood_probe_from_result(
    module: &str,
    name: &str,
    command: &str,
    operation: &str,
    scope_group: &str,
    probe: Value,
    include_response: bool,
    app_id: &str,
) -> Value {
    let ok = probe.get("ok").and_then(Value::as_bool).unwrap_or(false);
    let mut object = Map::new();
    object.insert("module".to_string(), Value::String(module.to_string()));
    object.insert("name".to_string(), Value::String(name.to_string()));
    object.insert("command".to_string(), Value::String(command.to_string()));
    object.insert(
        "operation".to_string(),
        Value::String(operation.to_string()),
    );
    object.insert("ok".to_string(), Value::Bool(ok));
    if ok {
        object.insert("status".to_string(), Value::String("ok".to_string()));
    } else {
        let error = probe
            .get("error")
            .and_then(Value::as_str)
            .unwrap_or("unknown error");
        let classified = classify_dogfood_error(error);
        object.insert(
            "status".to_string(),
            Value::String(
                classified
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("api_error")
                    .to_string(),
            ),
        );
        object.insert(
            "error_excerpt".to_string(),
            Value::String(truncate_for_probe(error, 700)),
        );
        if let Some(missing) = classified.get("missing_scopes") {
            object.insert("missing_scopes".to_string(), missing.clone());
        }
        if let Some(log_id) = classified.get("log_id") {
            object.insert("log_id".to_string(), log_id.clone());
        }
        if !scope_group.is_empty() {
            object.insert(
                "grant_hint".to_string(),
                Value::String(format!("feishu-bot scopes --group {scope_group}")),
            );
        }
        let rerun_command = dogfood_probe_rerun_command(module, command);
        object.insert(
            "remediation".to_string(),
            dogfood_probe_remediation(
                app_id,
                module,
                scope_group,
                classified
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("api_error"),
                classified
                    .get("missing_scopes")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                &rerun_command,
            ),
        );
    }
    if include_response {
        object.insert("probe".to_string(), probe);
    }
    Value::Object(object)
}

fn dogfood_probe_rerun_command(module: &str, command: &str) -> String {
    if command.contains("dogfood verify") {
        if command.contains("--include-response") {
            command.to_string()
        } else {
            format!("{command} --include-response")
        }
    } else {
        format!("feishu-bot --json dogfood verify --module {module} --include-response")
    }
}

fn dogfood_probe_remediation(
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
            "oauth_token_command": "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
            "why": "This Feishu API requires a user_access_token and operates in the human user's visibility context.",
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

pub(in crate::app) fn classify_dogfood_error(error: &str) -> Value {
    if error.contains("user_access_token") {
        return json!({ "status": "missing_user_token" });
    }
    if error.contains("helpdesk APIs require") {
        return json!({ "status": "missing_helpdesk_config" });
    }
    if let Some(json) = embedded_feishu_error_json(error) {
        let missing_scopes = missing_scope_subjects(&json);
        let log_id = get_string(&json, &["error", "log_id"]);
        let code = json.get("code").and_then(Value::as_i64);
        let msg = json.get("msg").and_then(Value::as_str).unwrap_or_default();
        if code == Some(99991672) || !missing_scopes.is_empty() {
            return json!({
                "status": "missing_scope",
                "missing_scopes": missing_scopes,
                "log_id": log_id,
            });
        }
        if code == Some(1001004) && msg.contains("data not found") {
            return json!({
                "status": "no_data",
                "log_id": log_id,
                "code": code,
            });
        }
        if code == Some(1230003) && msg.contains("internal server error") {
            return json!({
                "status": "upstream_api_error",
                "log_id": log_id,
                "code": code,
            });
        }
        return json!({
            "status": "api_error",
            "log_id": log_id,
            "code": code,
        });
    }
    json!({ "status": "api_error" })
}

fn embedded_feishu_error_json(error: &str) -> Option<Value> {
    if let Some(index) = error.find("response=") {
        let candidate = error[index + "response=".len()..].trim();
        if let Ok(value) = serde_json::from_str(candidate) {
            return Some(value);
        }
    }
    for (index, ch) in error.char_indices() {
        if ch == '{' {
            let candidate = error[index..].trim();
            if let Ok(value) = serde_json::from_str(candidate) {
                return Some(value);
            }
        }
    }
    None
}

fn missing_scope_subjects(value: &Value) -> Vec<String> {
    value
        .pointer("/error/permission_violations")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get("subject").and_then(Value::as_str))
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub(in crate::app) fn summarize_dogfood_probes(probes: &[Value]) -> Value {
    let mut counts: Map<String, Value> = Map::new();
    for probe in probes {
        let status = probe
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let count = counts.get(status).and_then(Value::as_u64).unwrap_or(0) + 1;
        counts.insert(status.to_string(), Value::Number(count.into()));
    }

    let ok_count = counts.get("ok").and_then(Value::as_u64).unwrap_or(0);
    let usable_count = probes
        .iter()
        .filter(|probe| {
            probe
                .get("status")
                .and_then(Value::as_str)
                .is_some_and(is_dogfood_usable_status)
        })
        .count() as u64;
    let total = probes.len() as u64;
    json!({
        "total": total,
        "ok": ok_count,
        "usable": usable_count,
        "not_ok": total.saturating_sub(usable_count),
        "counts": counts,
        "usable_modules": dogfood_probe_usable_modules(probes),
        "blocked_modules": dogfood_probe_modules_not_ok(probes),
        "next_actions": dogfood_probe_next_actions(probes),
    })
}

fn is_dogfood_usable_status(status: &str) -> bool {
    matches!(status, "ok" | "no_data")
}

fn dogfood_probe_usable_modules(probes: &[Value]) -> Vec<String> {
    let mut modules = Vec::new();
    for probe in probes {
        let usable = probe
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(is_dogfood_usable_status);
        if !usable {
            continue;
        }
        if let Some(module) = probe.get("module").and_then(Value::as_str) {
            if !modules.iter().any(|existing| existing == module) {
                modules.push(module.to_string());
            }
        }
    }
    modules
}

fn dogfood_probe_modules_not_ok(probes: &[Value]) -> Vec<Value> {
    let mut blocked = Vec::new();
    for probe in probes {
        if probe
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(is_dogfood_usable_status)
        {
            continue;
        }
        blocked.push(json!({
            "module": probe.get("module").cloned().unwrap_or(Value::Null),
            "name": probe.get("name").cloned().unwrap_or(Value::Null),
            "status": probe.get("status").cloned().unwrap_or(Value::Null),
            "missing_scopes": probe.get("missing_scopes").cloned().unwrap_or(Value::Null),
            "grant_hint": probe.get("grant_hint").cloned().unwrap_or(Value::Null),
            "remediation": probe.get("remediation").cloned().unwrap_or(Value::Null),
        }));
    }
    blocked
}

fn dogfood_probe_next_actions(probes: &[Value]) -> Vec<Value> {
    let mut actions = Vec::new();
    for probe in probes {
        if probe
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(is_dogfood_usable_status)
        {
            continue;
        }
        let Some(remediation) = probe.get("remediation") else {
            continue;
        };
        let action = remediation
            .get("action")
            .and_then(Value::as_str)
            .unwrap_or("inspect_api_error");
        let module = probe
            .get("module")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let key = match action {
            "grant_scopes" => remediation
                .get("scope_group")
                .and_then(Value::as_str)
                .unwrap_or(module)
                .to_string(),
            _ => action.to_string(),
        };
        if actions.iter().any(|item: &Value| {
            item.get("action").and_then(Value::as_str) == Some(action)
                && item.get("key").and_then(Value::as_str) == Some(key.as_str())
        }) {
            continue;
        }
        actions.push(json!({
            "action": action,
            "key": key,
            "module": module,
            "scope_command": remediation.get("scope_command").cloned().unwrap_or(Value::Null),
            "grant_url": remediation.get("grant_url").cloned().unwrap_or(Value::Null),
            "grant_group_url": remediation.get("grant_group_url").cloned().unwrap_or(Value::Null),
            "browser_command": remediation.get("browser_command").cloned().unwrap_or(Value::Null),
            "env": remediation.get("env").cloned().unwrap_or(Value::Null),
            "oauth_url_command": remediation.get("oauth_url_command").cloned().unwrap_or(Value::Null),
            "oauth_token_command": remediation.get("oauth_token_command").cloned().unwrap_or(Value::Null),
            "rerun_command": remediation.get("rerun_command").cloned().unwrap_or(Value::Null),
        }));
    }
    actions
}

pub(in crate::app) fn dogfood_module_selected(
    filters: &[String],
    module: &str,
    name: &str,
) -> bool {
    if filters.is_empty() {
        return true;
    }
    let module = module.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();
    filters.iter().any(|filter| {
        let filter = filter.trim().to_ascii_lowercase();
        !filter.is_empty() && (module == filter || name == filter || name.starts_with(&filter))
    })
}

fn truncate_for_probe(value: &str, max_chars: usize) -> String {
    let mut output = String::new();
    for ch in value.chars().take(max_chars) {
        output.push(ch);
    }
    if value.chars().count() > max_chars {
        output.push_str("...");
    }
    output
}

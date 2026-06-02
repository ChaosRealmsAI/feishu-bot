use super::*;

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

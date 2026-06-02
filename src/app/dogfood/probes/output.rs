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

use super::*;

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
        if code == Some(99991677) || msg.contains("Authentication token expired") {
            return json!({
                "status": "expired_user_token",
                "log_id": log_id,
                "code": code,
            });
        }
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
    if error.contains("Authentication token expired") {
        return json!({ "status": "expired_user_token" });
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

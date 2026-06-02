use super::env::{oauth_env_lines, save_oauth_tokens_to_env};
use super::*;

pub(super) fn finalize_oauth_token_response(
    response: Value,
    raw: bool,
    print_env: bool,
    save_env: bool,
    env_file: Option<PathBuf>,
) -> Result<Value> {
    let mut output = if raw {
        response.clone()
    } else {
        mask_oauth_token_response(&response)
    };
    let access_token = get_string(&response, &["access_token"]);
    let refresh_token = get_string(&response, &["refresh_token"]);
    if print_env {
        output["env"] = oauth_env_lines(access_token.as_deref(), refresh_token.as_deref());
    }
    if save_env {
        let path = env_file.unwrap_or_else(|| PathBuf::from(".env"));
        save_oauth_tokens_to_env(&path, access_token.as_deref(), refresh_token.as_deref())?;
        output["saved_env_file"] = Value::String(path.display().to_string());
    }
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": output,
    }))
}

pub(in crate::app) fn mask_oauth_token_response(response: &Value) -> Value {
    let mut masked = response.clone();
    for key in ["access_token", "refresh_token", "id_token"] {
        if let Some(value) = masked.get_mut(key) {
            if let Some(token) = value.as_str() {
                *value = Value::String(mask_secret(token));
            }
        }
    }
    masked
}

pub(super) fn print_oauth_response(value: &Value) -> Result<()> {
    let data = value.get("data").unwrap_or(value);
    for (label, path) in [
        ("authorization_url", &["authorization_url"][..]),
        ("browser_command", &["browser_command"][..]),
        ("redirect_uri", &["redirect_uri"][..]),
        ("state", &["state"][..]),
        ("code_verifier", &["code_verifier"][..]),
        ("access_token", &["access_token"][..]),
        ("refresh_token", &["refresh_token"][..]),
        ("scope", &["scope"][..]),
        ("saved_env_file", &["saved_env_file"][..]),
    ] {
        if let Some(output) = get_string(data, path) {
            println!("{label}={output}");
        }
    }
    if let Some(scope) = data.get("scope").and_then(Value::as_array) {
        let items = scope.iter().filter_map(Value::as_str).collect::<Vec<_>>();
        if !items.is_empty() {
            println!("scope={}", items.join(" "));
        }
    }
    if let Some(lines) = data.get("env").and_then(Value::as_array) {
        for line in lines.iter().filter_map(Value::as_str) {
            println!("{line}");
        }
    }
    Ok(())
}

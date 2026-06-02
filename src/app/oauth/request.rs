use super::*;

pub(super) async fn request_oauth_token(config: &Config, body: Value) -> Result<Value> {
    let url = format!("{}/authen/v2/oauth/token", config.base_url);
    let response = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await
        .context("POST /authen/v2/oauth/token")?;
    read_oauth_json(response).await
}

pub(super) async fn read_oauth_json(response: reqwest::Response) -> Result<Value> {
    let status = response.status();
    let text = response.text().await.context("read OAuth response")?;
    let json: Value =
        serde_json::from_str(&text).with_context(|| format!("parse OAuth response: {text}"))?;
    if !status.is_success() {
        bail!(
            "Feishu OAuth HTTP {status}: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    if json
        .get("error")
        .and_then(Value::as_str)
        .is_some_and(|value| !value.is_empty())
    {
        bail!(
            "Feishu OAuth failed: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    Ok(json)
}

use super::*;

pub(in crate::app) async fn read_feishu_json(res: reqwest::Response) -> Result<Value> {
    let status = res.status();
    let text = res.text().await.context("read response")?;
    if status == StatusCode::NO_CONTENT {
        return Ok(Value::Null);
    }
    let json: Value = serde_json::from_str(&text)
        .with_context(|| format!("parse Feishu response JSON: {text}"))?;
    if !status.is_success() {
        bail!(
            "Feishu HTTP {status}: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    if let Some(code) = json.get("code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = json.get("msg").and_then(Value::as_str).unwrap_or("");
            bail!("Feishu API failed: code={code} msg={msg} response={json}");
        }
    }
    Ok(json)
}

pub(in crate::app) async fn read_binary_response(res: reqwest::Response) -> Result<Vec<u8>> {
    let status = res.status();
    if !status.is_success() {
        let text = res.text().await.context("read error response")?;
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            bail!(
                "Feishu HTTP {status}: {}",
                serde_json::to_string_pretty(&json)?
            );
        }
        bail!("Feishu HTTP {status}: {text}");
    }
    Ok(res.bytes().await.context("read binary response")?.to_vec())
}

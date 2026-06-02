use super::*;

mod auth;
mod binary;
mod convenience;
mod helpdesk;
mod json;
mod multipart;

impl FeishuClient {
    fn openapi_request_with_token(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        token: String,
        headers: &[(String, String)],
    ) -> Result<reqwest::RequestBuilder> {
        validate_openapi_path(path)?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self
            .http
            .request(method, url)
            .bearer_auth(token)
            .query(query);
        for (key, value) in headers {
            request = request.header(key.as_str(), value.as_str());
        }
        Ok(request)
    }
}

fn validate_openapi_path(path: &str) -> Result<()> {
    if !path.starts_with('/') {
        bail!("OpenAPI path must start with /: {path}");
    }
    Ok(())
}

async fn send_openapi_request(
    request: reqwest::RequestBuilder,
    method_label: &str,
    path: &str,
    context_suffix: Option<&str>,
) -> Result<reqwest::Response> {
    let context = match context_suffix {
        Some(suffix) if !suffix.is_empty() => format!("{method_label} {path} {suffix}"),
        _ => format!("{method_label} {path}"),
    };
    request.send().await.with_context(|| context)
}

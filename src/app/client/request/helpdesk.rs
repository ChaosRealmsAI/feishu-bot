use super::*;

impl FeishuClient {
    pub(in crate::app) async fn request_helpdesk_json(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        if !path.starts_with('/') {
            bail!("OpenAPI path must start with /: {path}");
        }
        let token = self.tenant_token().await?;
        let helpdesk_auth = self.helpdesk_auth_header()?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self
            .http
            .request(method.clone(), url)
            .bearer_auth(token)
            .header("X-Lark-Helpdesk-Authorization", helpdesk_auth)
            .query(query);
        if let Some(body) = body {
            request = request.json(&body);
        }
        let method_label = method.as_str().to_string();
        let res = request
            .send()
            .await
            .with_context(|| format!("{method_label} {path}"))?;
        read_feishu_json(res).await
    }
}

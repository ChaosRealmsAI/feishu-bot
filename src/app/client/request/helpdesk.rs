use super::*;

impl FeishuClient {
    pub(in crate::app) async fn request_helpdesk_json(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        validate_openapi_path(path)?;
        let method_label = method.as_str().to_string();
        let token = self.tenant_token().await?;
        let helpdesk_auth = self.helpdesk_auth_header()?;
        let headers = [("X-Lark-Helpdesk-Authorization".to_string(), helpdesk_auth)];
        let mut request = self.openapi_request_with_token(method, path, query, token, &headers)?;
        if let Some(body) = body {
            request = request.json(&body);
        }
        let res = send_openapi_request(request, &method_label, path, None).await?;
        read_feishu_json(res).await
    }
}

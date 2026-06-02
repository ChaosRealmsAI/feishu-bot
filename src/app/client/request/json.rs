use super::*;

impl FeishuClient {
    pub(in crate::app) async fn request_json(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        let token = self.tenant_token().await?;
        self.request_json_with_token(method, path, query, body, token)
            .await
    }

    pub(in crate::app) async fn request_json_with_token(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        token: String,
    ) -> Result<Value> {
        self.request_json_with_token_and_headers(method, path, query, body, token, &[])
            .await
    }

    pub(in crate::app) async fn request_json_with_token_and_headers(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        token: String,
        headers: &[(String, String)],
    ) -> Result<Value> {
        let method_label = method.as_str().to_string();
        let mut request = self.openapi_request_with_token(method, path, query, token, headers)?;
        if let Some(body) = body {
            request = request.json(&body);
        }
        let res = send_openapi_request(request, &method_label, path, None).await?;
        read_feishu_json(res).await
    }

    pub(in crate::app) async fn request_json_with_auth(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        auth: ApiAuthArg,
        headers: &[(String, String)],
    ) -> Result<Value> {
        let token = self.token_for_api_auth(auth).await?;
        self.request_json_with_token_and_headers(method, path, query, body, token, headers)
            .await
    }
}

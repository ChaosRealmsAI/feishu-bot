use super::*;

impl FeishuClient {
    pub(in crate::app) async fn request_binary_with_auth(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        auth: ApiAuthArg,
        headers: &[(String, String)],
        range: Option<&str>,
    ) -> Result<Vec<u8>> {
        validate_openapi_path(path)?;
        let method_label = method.as_str().to_string();
        let token = self.token_for_api_auth(auth).await?;
        let mut request = self.openapi_request_with_token(method, path, query, token, headers)?;
        if let Some(range) = range.filter(|value| !value.trim().is_empty()) {
            request = request.header(reqwest::header::RANGE, range);
        }
        let res = send_openapi_request(request, &method_label, path, None).await?;
        read_binary_response(res).await
    }
}

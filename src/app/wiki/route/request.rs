use super::*;

pub(in crate::app) async fn wiki_request_json(
    api: &mut FeishuClient,
    method: Method,
    path: &str,
    query: &[(String, String)],
    body: Option<Value>,
    auth: ApiAuthArg,
) -> Result<Value> {
    api.request_json_with_auth(method, path, query, body, auth, &[])
        .await
}

use super::*;

pub(super) async fn read_wiki_node_for_probe(
    api: &mut FeishuClient,
    node_token: &str,
    auth: ApiAuthArg,
) -> Result<Value> {
    wiki_request_json(
        api,
        Method::GET,
        "/wiki/v2/spaces/get_node",
        &[("token".to_string(), node_token.to_string())],
        None,
        auth,
    )
    .await
}

pub(super) async fn wiki_route_check_call(
    api: &mut FeishuClient,
    name: &str,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    auth: ApiAuthArg,
    required_scopes_hint: &[&str],
) -> Value {
    let method_label = method.as_str().to_string();
    let auth_label = format!("{auth:?}").to_lowercase();
    match wiki_request_json(api, method, &path, &query, None, auth).await {
        Ok(response) => json!({
            "name": name,
            "ok": true,
            "method": method_label,
            "path": path,
            "auth": auth_label,
            "required_scopes_hint": required_scopes_hint,
            "response": response
        }),
        Err(error) => json!({
            "name": name,
            "ok": false,
            "method": method_label,
            "path": path,
            "auth": auth_label,
            "required_scopes_hint": required_scopes_hint,
            "error": format!("{error:#}")
        }),
    }
}

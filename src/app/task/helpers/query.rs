use super::*;

pub(in crate::app) async fn task_request_json(
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

pub(in crate::app) fn task_page_query(
    page_size: u16,
    page_token: Option<String>,
) -> Result<Vec<(String, String)>> {
    if page_size == 0 || page_size > 100 {
        bail!("task page_size must be between 1 and 100");
    }
    let mut query = vec![("page_size".to_string(), page_size.to_string())];
    push_query_opt(&mut query, "page_token", page_token);
    Ok(query)
}

pub(in crate::app) fn build_task_list_query(args: &TaskListArgs) -> Result<Vec<(String, String)>> {
    let mut query = task_page_query(args.page_size, args.page_token.clone())?;
    if let Some(completed) = args.completed {
        query.push(("completed".to_string(), completed.to_string()));
    }
    let list_type = args.list_type.trim();
    if list_type.is_empty() {
        bail!("task list --type cannot be empty");
    }
    query.push(("type".to_string(), list_type.to_string()));
    query.extend(task_user_id_query(args.user_id_type));
    Ok(query)
}

pub(in crate::app) fn task_user_id_query(user_id_type: UserIdTypeArg) -> Vec<(String, String)> {
    vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )]
}

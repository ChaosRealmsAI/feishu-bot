use super::*;

pub(super) async fn run_tasklist_command(
    api: &mut FeishuClient,
    command: TasklistCommand,
) -> Result<Value> {
    match command {
        TasklistCommand::Create(args) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_tasklist_create_body(args)?;
            task_request_json(
                api,
                Method::POST,
                "/task/v2/tasklists",
                &query,
                Some(body),
                auth,
            )
            .await
        }
        TasklistCommand::List(args) => {
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            task_request_json(
                api,
                Method::GET,
                "/task/v2/tasklists",
                &query,
                None,
                args.auth,
            )
            .await
        }
        TasklistCommand::Get(args) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await
        }
        TasklistCommand::Update(args) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_tasklist_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await
        }
        TasklistCommand::Delete(args) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await
        }
        TasklistCommand::Tasks(args) => {
            let path = format!("/task/v2/tasklists/{}/tasks", args.tasklist_guid);
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            if let Some(completed) = args.completed {
                query.push(("completed".to_string(), completed.to_string()));
            }
            push_query_opt(&mut query, "created_from", args.created_from);
            push_query_opt(&mut query, "created_to", args.created_to);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await
        }
        TasklistCommand::AddMember(args) => {
            let path = format!("/task/v2/tasklists/{}/add_members", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_tasklist_member_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await
        }
        TasklistCommand::RemoveMember(args) => {
            let path = format!("/task/v2/tasklists/{}/remove_members", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_tasklist_member_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await
        }
    }
}

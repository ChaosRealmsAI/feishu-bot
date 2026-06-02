use super::*;

pub(super) async fn run_task_section_command(
    api: &mut FeishuClient,
    command: TaskSectionCommand,
) -> Result<Value> {
    match command {
        TaskSectionCommand::List(args) => {
            let mut query = task_page_query(args.page_size, args.page_token)?;
            query.push(("resource_type".to_string(), args.resource_type));
            push_query_opt(&mut query, "resource_id", args.resource_id);
            push_query_opt(&mut query, "update_msec", args.update_msec);
            query.extend(task_user_id_query(args.user_id_type));
            task_request_json(
                api,
                Method::GET,
                "/task/v2/sections",
                &query,
                None,
                args.auth,
            )
            .await
        }
        TaskSectionCommand::Get(args) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await
        }
        TaskSectionCommand::Create(args) => {
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_section_create_body(args)?;
            task_request_json(
                api,
                Method::POST,
                "/task/v2/sections",
                &query,
                Some(body),
                auth,
            )
            .await
        }
        TaskSectionCommand::Update(args) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_section_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await
        }
        TaskSectionCommand::Delete(args) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await
        }
        TaskSectionCommand::Tasks(args) => {
            let path = format!(
                "/task/v2/sections/{}/tasks",
                encode_path_segment(&args.section_guid)
            );
            let mut query = task_page_query(args.page_size, args.page_token)?;
            if let Some(completed) = args.completed {
                query.push(("completed".to_string(), completed.to_string()));
            }
            push_query_opt(&mut query, "created_from", args.created_from);
            push_query_opt(&mut query, "created_to", args.created_to);
            query.extend(task_user_id_query(args.user_id_type));
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await
        }
    }
}

pub(super) async fn run_task_custom_field_command(
    api: &mut FeishuClient,
    command: TaskCustomFieldCommand,
) -> Result<Value> {
    match command {
        TaskCustomFieldCommand::List(args) => {
            let mut query = task_page_query(args.page_size, args.page_token)?;
            push_query_opt(&mut query, "resource_type", args.resource_type);
            push_query_opt(&mut query, "resource_id", args.resource_id);
            query.extend(task_user_id_query(args.user_id_type));
            task_request_json(
                api,
                Method::GET,
                "/task/v2/custom_fields",
                &query,
                None,
                args.auth,
            )
            .await
        }
        TaskCustomFieldCommand::Get(args) => {
            let path = format!(
                "/task/v2/custom_fields/{}",
                encode_path_segment(&args.custom_field_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await
        }
        TaskCustomFieldCommand::Create(args) => {
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_custom_field_create_body(args)?;
            task_request_json(
                api,
                Method::POST,
                "/task/v2/custom_fields",
                &query,
                Some(body),
                auth,
            )
            .await
        }
        TaskCustomFieldCommand::Update(args) => {
            let path = format!(
                "/task/v2/custom_fields/{}",
                encode_path_segment(&args.custom_field_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_custom_field_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await
        }
        TaskCustomFieldCommand::Add(args) => {
            let path = format!(
                "/task/v2/custom_fields/{}/add",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_resource_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await
        }
        TaskCustomFieldCommand::Remove(args) => {
            let path = format!(
                "/task/v2/custom_fields/{}/remove",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_resource_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await
        }
        TaskCustomFieldCommand::SetValue(args) => {
            let path = format!("/task/v2/tasks/{}", encode_path_segment(&args.task_guid));
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_custom_field_value_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await
        }
        TaskCustomFieldCommand::Option(TaskCustomFieldOptionCommand::Create(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}/options",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_option_create_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await
        }
        TaskCustomFieldCommand::Option(TaskCustomFieldOptionCommand::Update(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}/options/{}",
                encode_path_segment(&args.custom_field_guid),
                encode_path_segment(&args.option_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_option_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &[], Some(body), auth).await
        }
    }
}

pub(super) async fn run_task_attachment_command(
    api: &mut FeishuClient,
    command: TaskAttachmentCommand,
) -> Result<Value> {
    match command {
        TaskAttachmentCommand::List(args) => {
            let mut query = task_page_query(args.page_size, args.page_token)?;
            query.push(("resource_type".to_string(), args.resource_type));
            query.push(("resource_id".to_string(), args.resource_id));
            query.extend(task_user_id_query(args.user_id_type));
            task_request_json(
                api,
                Method::GET,
                "/task/v2/attachments",
                &query,
                None,
                args.auth,
            )
            .await
        }
        TaskAttachmentCommand::Upload(args) => {
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let (fields, files) = build_task_attachment_upload_parts(args)?;
            api.request_multipart_with_auth(
                Method::POST,
                "/task/v2/attachments/upload",
                &query,
                fields,
                files,
                auth,
                &[],
            )
            .await
        }
        TaskAttachmentCommand::Delete(args) => {
            let path = format!(
                "/task/v2/attachments/{}",
                encode_path_segment(&args.attachment_guid)
            );
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await
        }
    }
}

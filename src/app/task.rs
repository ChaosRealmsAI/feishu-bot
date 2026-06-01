#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use super::*;

mod structure;

pub(super) use structure::*;

pub(super) async fn run_task_command(
    api: &mut FeishuClient,
    command: TaskCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        TaskCommand::Tasklist(TasklistCommand::Create(args)) => {
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
            .await?
        }
        TaskCommand::Tasklist(TasklistCommand::List(args)) => {
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
            .await?
        }
        TaskCommand::Tasklist(TasklistCommand::Get(args)) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Tasklist(TasklistCommand::Update(args)) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_tasklist_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Tasklist(TasklistCommand::Delete(args)) => {
            let path = format!("/task/v2/tasklists/{}", args.tasklist_guid);
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await?
        }
        TaskCommand::Tasklist(TasklistCommand::Tasks(args)) => {
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
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Tasklist(TasklistCommand::AddMember(args)) => {
            let path = format!("/task/v2/tasklists/{}/add_members", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_tasklist_member_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Tasklist(TasklistCommand::RemoveMember(args)) => {
            let path = format!("/task/v2/tasklists/{}/remove_members", args.tasklist_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_tasklist_member_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Create(args) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_task_create_body(args)?;
            task_request_json(
                api,
                Method::POST,
                "/task/v2/tasks",
                &query,
                Some(body),
                auth,
            )
            .await?
        }
        TaskCommand::List(args) => {
            let query = build_task_list_query(&args)?;
            task_request_json(api, Method::GET, "/task/v2/tasks", &query, None, args.auth).await?
        }
        TaskCommand::Get(args) => {
            let path = format!("/task/v2/tasks/{}", args.guid);
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Update(args) => {
            let path = format!("/task/v2/tasks/{}", args.guid);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_task_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Complete(args) => {
            let path = format!("/task/v2/tasks/{}", args.guid);
            let completed_at = args
                .completed_at
                .unwrap_or_else(|| Local::now().timestamp_millis().to_string());
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            task_request_json(
                api,
                Method::PATCH,
                &path,
                &query,
                Some(json!({
                    "task": { "completed_at": completed_at },
                    "update_fields": ["completed_at"],
                })),
                args.auth,
            )
            .await?
        }
        TaskCommand::Reopen(args) => {
            let path = format!("/task/v2/tasks/{}", args.guid);
            let query = task_user_id_query(args.user_id_type);
            task_request_json(
                api,
                Method::PATCH,
                &path,
                &query,
                Some(json!({
                    "task": { "completed_at": "0" },
                    "update_fields": ["completed_at"],
                })),
                args.auth,
            )
            .await?
        }
        TaskCommand::Delete(args) => {
            let path = format!("/task/v2/tasks/{}", args.guid);
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await?
        }
        TaskCommand::Member(TaskMemberCommand::Add(args)) => {
            let path = format!("/task/v2/tasks/{}/add_members", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_member_body(args, true)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Member(TaskMemberCommand::Remove(args)) => {
            let path = format!("/task/v2/tasks/{}/remove_members", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_member_body(args, false)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Tasklists(args) => {
            let path = format!("/task/v2/tasks/{}/tasklists", args.task_guid);
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::AddTasklist(args) => {
            let path = format!("/task/v2/tasks/{}/add_tasklist", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_tasklist_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::RemoveTasklist(args) => {
            let path = format!("/task/v2/tasks/{}/remove_tasklist", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_tasklist_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Reminder(TaskReminderCommand::Add(args)) => {
            let path = format!("/task/v2/tasks/{}/add_reminders", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_reminder_add_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Reminder(TaskReminderCommand::Remove(args)) => {
            let path = format!("/task/v2/tasks/{}/remove_reminders", args.task_guid);
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_reminder_remove_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Dependency(TaskDependencyCommand::Add(args)) => {
            let path = format!("/task/v2/tasks/{}/add_dependencies", args.task_guid);
            let auth = args.auth;
            let body = build_task_dependency_add_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        TaskCommand::Dependency(TaskDependencyCommand::Remove(args)) => {
            let path = format!("/task/v2/tasks/{}/remove_dependencies", args.task_guid);
            let auth = args.auth;
            let body = build_task_dependency_remove_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        TaskCommand::Comment(TaskCommentCommand::List(args)) => {
            let mut query = vec![
                ("resource_id".to_string(), args.task_guid),
                ("resource_type".to_string(), "task".to_string()),
                ("page_size".to_string(), args.page_size.to_string()),
                ("direction".to_string(), args.direction),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            task_request_json(
                api,
                Method::GET,
                "/task/v2/comments",
                &query,
                None,
                args.auth,
            )
            .await?
        }
        TaskCommand::Comment(TaskCommentCommand::Get(args)) => {
            let path = format!(
                "/task/v2/comments/{}",
                encode_path_segment(&args.comment_id)
            );
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Comment(TaskCommentCommand::Create(args)) => {
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_comment_create_body(args)?;
            task_request_json(
                api,
                Method::POST,
                "/task/v2/comments",
                &query,
                Some(body),
                auth,
            )
            .await?
        }
        TaskCommand::Comment(TaskCommentCommand::Update(args)) => {
            let path = format!(
                "/task/v2/comments/{}",
                encode_path_segment(&args.comment_id)
            );
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_comment_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Comment(TaskCommentCommand::Delete(args)) => {
            let path = format!(
                "/task/v2/comments/{}",
                encode_path_segment(&args.comment_id)
            );
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await?
        }
        TaskCommand::Subtask(TaskSubtaskCommand::Create(args)) => {
            let path = format!("/task/v2/tasks/{}/subtasks", args.task_guid);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_subtask_create_body(args)?;
            task_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Subtask(TaskSubtaskCommand::List(args)) => {
            let path = format!("/task/v2/tasks/{}/subtasks", args.task_guid);
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Section(TaskSectionCommand::List(args)) => {
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
            .await?
        }
        TaskCommand::Section(TaskSectionCommand::Get(args)) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::Section(TaskSectionCommand::Create(args)) => {
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
            .await?
        }
        TaskCommand::Section(TaskSectionCommand::Update(args)) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_section_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::Section(TaskSectionCommand::Delete(args)) => {
            let path = format!(
                "/task/v2/sections/{}",
                encode_path_segment(&args.section_guid)
            );
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await?
        }
        TaskCommand::Section(TaskSectionCommand::Tasks(args)) => {
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
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::List(args)) => {
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
            .await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Get(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}",
                encode_path_segment(&args.custom_field_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            task_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Create(args)) => {
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
            .await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Update(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}",
                encode_path_segment(&args.custom_field_guid)
            );
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_custom_field_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Add(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}/add",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_resource_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Remove(args)) => {
            let path = format!(
                "/task/v2/custom_fields/{}/remove",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_resource_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::SetValue(args)) => {
            let path = format!("/task/v2/tasks/{}", encode_path_segment(&args.task_guid));
            let query = task_user_id_query(args.user_id_type);
            let auth = args.auth;
            let body = build_task_custom_field_value_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &query, Some(body), auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Option(
            TaskCustomFieldOptionCommand::Create(args),
        )) => {
            let path = format!(
                "/task/v2/custom_fields/{}/options",
                encode_path_segment(&args.custom_field_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_option_create_body(args)?;
            task_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        TaskCommand::CustomField(TaskCustomFieldCommand::Option(
            TaskCustomFieldOptionCommand::Update(args),
        )) => {
            let path = format!(
                "/task/v2/custom_fields/{}/options/{}",
                encode_path_segment(&args.custom_field_guid),
                encode_path_segment(&args.option_guid)
            );
            let auth = args.auth;
            let body = build_task_custom_field_option_update_body(args)?;
            task_request_json(api, Method::PATCH, &path, &[], Some(body), auth).await?
        }
        TaskCommand::Attachment(TaskAttachmentCommand::List(args)) => {
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
            .await?
        }
        TaskCommand::Attachment(TaskAttachmentCommand::Upload(args)) => {
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
            .await?
        }
        TaskCommand::Attachment(TaskAttachmentCommand::Delete(args)) => {
            let path = format!(
                "/task/v2/attachments/{}",
                encode_path_segment(&args.attachment_guid)
            );
            task_request_json(api, Method::DELETE, &path, &[], None, args.auth).await?
        }
    };
    print_response(raw_json, "task operation completed", data)
}

pub(super) fn build_task_create_body(args: TaskCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        let mut body = read_json_value(args.body_json, args.file, args.stdin)?;
        if body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }

    let summary = args
        .summary
        .ok_or_else(|| anyhow!("task create needs --summary or --body-json/--file/--stdin"))?;
    let mut body = Map::new();
    body.insert("summary".to_string(), Value::String(summary));
    if let Some(description) = args.description {
        body.insert("description".to_string(), Value::String(description));
    }
    insert_task_time_create(
        &mut body,
        "due",
        args.due_ms,
        args.due_at,
        args.due_date,
        args.due_all_day,
    )?;
    insert_task_time_create(
        &mut body,
        "start",
        args.start_ms,
        args.start_at,
        args.start_date,
        args.start_all_day,
    )?;
    insert_opt_string(&mut body, "completed_at", args.completed_at);
    insert_opt_string(&mut body, "repeat_rule", args.repeat_rule);
    insert_opt_string(&mut body, "extra", args.extra);
    insert_task_optional_u8(&mut body, "mode", args.mode)?;
    if let Some(is_milestone) = args.is_milestone {
        body.insert("is_milestone".to_string(), Value::Bool(is_milestone));
    }
    insert_task_json_object(
        &mut body,
        "custom_complete",
        args.custom_complete_json,
        "custom-complete-json",
    )?;
    insert_task_json_object(&mut body, "origin", args.origin_json, "origin-json")?;
    insert_task_json_object(
        &mut body,
        "docx_source",
        args.docx_source_json,
        "docx-source-json",
    )?;
    insert_task_reminders(
        &mut body,
        args.reminders_json,
        args.reminder_minute,
        "reminders-json",
    )?;
    insert_task_json_array(
        &mut body,
        "custom_fields",
        args.custom_fields_json,
        "custom-fields-json",
    )?;
    let mut members = Vec::new();
    members.extend(task_members(args.assignees, "assignee"));
    members.extend(task_members(args.followers, "follower"));
    if !members.is_empty() {
        body.insert("members".to_string(), Value::Array(members));
    }
    if !args.tasklist_guids.is_empty() {
        body.insert(
            "tasklists".to_string(),
            Value::Array(
                args.tasklist_guids
                    .into_iter()
                    .map(|guid| json!({ "tasklist_guid": guid }))
                    .collect(),
            ),
        );
    }
    body.insert(
        "client_token".to_string(),
        Value::String(args.client_token.unwrap_or_else(random_uuid)),
    );
    Ok(Value::Object(body))
}

pub(super) fn build_tasklist_create_body(args: TasklistCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("tasklist create needs --name or raw body"))?;
    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(name));
    if !args.members.is_empty() {
        body.insert(
            "members".to_string(),
            Value::Array(task_members(args.members, &args.member_role)),
        );
    }
    Ok(Value::Object(body))
}

pub(super) fn build_tasklist_update_body(args: TasklistUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist update body",
        );
    }
    let mut tasklist = Map::new();
    let mut update_fields = Vec::new();
    if let Some(name) = args.name {
        tasklist.insert("name".to_string(), Value::String(name));
        update_fields.push(Value::String("name".to_string()));
    }
    if let Some(owner) = args.owner_json {
        tasklist.insert("owner".to_string(), parse_json_value(&owner, "owner-json")?);
        update_fields.push(Value::String("owner".to_string()));
    }
    if update_fields.is_empty() {
        bail!("tasklist update needs --name, --owner-json, or raw body");
    }
    Ok(json!({
        "tasklist": Value::Object(tasklist),
        "update_fields": Value::Array(update_fields),
        "origin_owner_to_role": args.origin_owner_to_role,
    }))
}

pub(super) fn build_tasklist_member_body(args: TasklistMemberWriteArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist member body",
        );
    }
    if let Some(members_json) = args.members_json {
        let value = parse_json_value(&members_json, "members-json")?;
        if value.get("members").is_some() {
            return ensure_json_object(value, "tasklist member body");
        }
        return Ok(json!({ "members": ensure_json_array(value, "members")? }));
    }
    let member_type = args.member_type.as_deref().unwrap_or("user");
    let mut members = Vec::new();
    members.extend(task_members_typed(args.editors, "editor", member_type));
    members.extend(task_members_typed(args.viewers, "viewer", member_type));
    if members.is_empty() {
        bail!("tasklist member command needs --editor/--viewer, --members-json, or raw body");
    }
    Ok(json!({ "members": members }))
}

pub(super) fn build_task_comment_create_body(args: TaskCommentCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "comment create body",
        );
    }
    let content = args
        .content
        .ok_or_else(|| anyhow!("comment create needs --content or raw body"))?;
    let mut body = Map::new();
    body.insert("content".to_string(), Value::String(content));
    body.insert(
        "resource_type".to_string(),
        Value::String("task".to_string()),
    );
    body.insert("resource_id".to_string(), Value::String(args.task_guid));
    if let Some(reply_to_comment_id) = args.reply_to_comment_id {
        body.insert(
            "reply_to_comment_id".to_string(),
            Value::String(reply_to_comment_id),
        );
    }
    Ok(Value::Object(body))
}

pub(super) fn build_task_comment_update_body(args: TaskCommentUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "comment update body",
        );
    }
    let content = args
        .content
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("comment update needs --content or raw body"))?;
    Ok(json!({
        "comment": {
            "content": content
        },
        "update_fields": ["content"]
    }))
}

pub(super) fn build_subtask_create_body(args: TaskSubtaskCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        let mut body = read_json_value(args.body_json, args.file, args.stdin)?;
        if body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }

    let summary = args
        .summary
        .ok_or_else(|| anyhow!("subtask create needs --summary or --body-json/--file/--stdin"))?;
    let mut body = Map::new();
    body.insert("summary".to_string(), Value::String(summary));
    if let Some(description) = args.description {
        body.insert("description".to_string(), Value::String(description));
    }
    insert_task_time_create(
        &mut body,
        "due",
        args.due_ms,
        args.due_at,
        args.due_date,
        args.due_all_day,
    )?;
    insert_task_time_create(
        &mut body,
        "start",
        args.start_ms,
        args.start_at,
        args.start_date,
        args.start_all_day,
    )?;
    insert_opt_string(&mut body, "completed_at", args.completed_at);
    insert_opt_string(&mut body, "repeat_rule", args.repeat_rule);
    insert_opt_string(&mut body, "extra", args.extra);
    insert_task_optional_u8(&mut body, "mode", args.mode)?;
    if let Some(is_milestone) = args.is_milestone {
        body.insert("is_milestone".to_string(), Value::Bool(is_milestone));
    }
    insert_task_json_object(
        &mut body,
        "custom_complete",
        args.custom_complete_json,
        "custom-complete-json",
    )?;
    insert_task_json_object(&mut body, "origin", args.origin_json, "origin-json")?;
    insert_task_json_object(
        &mut body,
        "docx_source",
        args.docx_source_json,
        "docx-source-json",
    )?;
    insert_task_reminders(
        &mut body,
        args.reminders_json,
        args.reminder_minute,
        "reminders-json",
    )?;
    insert_task_json_array(
        &mut body,
        "custom_fields",
        args.custom_fields_json,
        "custom-fields-json",
    )?;
    body.insert(
        "client_token".to_string(),
        Value::String(args.client_token.unwrap_or_else(random_uuid)),
    );
    Ok(Value::Object(body))
}

pub(super) fn build_task_update_body(args: TaskUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return read_json_value(args.body_json, args.file, args.stdin);
    }

    let mut task = Map::new();
    let mut update_fields = Vec::new();
    if let Some(summary) = args.summary {
        task.insert("summary".to_string(), Value::String(summary));
        update_fields.push(Value::String("summary".to_string()));
    }
    let has_description = args.description.is_some();
    if let Some(description) = args.description {
        task.insert("description".to_string(), Value::String(description));
        update_fields.push(Value::String("description".to_string()));
    }
    if has_description && args.clear_description {
        bail!("task update cannot combine --description with --clear-description");
    }
    if args.clear_description {
        update_fields.push(Value::String("description".to_string()));
    }
    insert_task_time_update(
        &mut task,
        &mut update_fields,
        "due",
        args.due_ms,
        args.due_at,
        args.due_date,
        args.due_all_day,
        args.clear_due,
    )?;
    insert_task_time_update(
        &mut task,
        &mut update_fields,
        "start",
        args.start_ms,
        args.start_at,
        args.start_date,
        args.start_all_day,
        args.clear_start,
    )?;
    if let Some(completed_at) = args.completed_at {
        task.insert("completed_at".to_string(), Value::String(completed_at));
        update_fields.push(Value::String("completed_at".to_string()));
    }
    insert_task_clearable_string(
        &mut task,
        &mut update_fields,
        "repeat_rule",
        args.repeat_rule,
        args.clear_repeat_rule,
    )?;
    insert_task_clearable_string(
        &mut task,
        &mut update_fields,
        "extra",
        args.extra,
        args.clear_extra,
    )?;
    insert_task_clearable_object(
        &mut task,
        &mut update_fields,
        "custom_complete",
        args.custom_complete_json,
        args.clear_custom_complete,
        "custom-complete-json",
    )?;
    if let Some(mode) = args.mode {
        if !(1..=2).contains(&mode) {
            bail!("task mode must be 1 or 2");
        }
        task.insert(
            "mode".to_string(),
            Value::Number(serde_json::Number::from(mode)),
        );
        update_fields.push(Value::String("mode".to_string()));
    }
    if let Some(is_milestone) = args.is_milestone {
        task.insert("is_milestone".to_string(), Value::Bool(is_milestone));
        update_fields.push(Value::String("is_milestone".to_string()));
    }
    if let Some(custom_fields_json) = args.custom_fields_json {
        task.insert(
            "custom_fields".to_string(),
            task_array_from_json(custom_fields_json, "custom-fields-json")?,
        );
        update_fields.push(Value::String("custom_fields".to_string()));
    }
    if update_fields.is_empty() {
        bail!("task update needs a field flag or --body-json/--file/--stdin");
    }
    Ok(json!({
        "task": Value::Object(task),
        "update_fields": Value::Array(update_fields),
    }))
}

fn insert_task_time_create(
    body: &mut Map<String, Value>,
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
) -> Result<()> {
    let Some((timestamp, is_all_day)) =
        resolve_task_time_value(field, timestamp_ms, timestamp_at, date, is_all_day)?
    else {
        if is_all_day {
            bail!("--{field}-all-day requires --{field}-ms, --{field}-at, or --{field}-date");
        }
        return Ok(());
    };
    body.insert(
        field.to_string(),
        json!({ "timestamp": timestamp, "is_all_day": is_all_day }),
    );
    Ok(())
}

fn insert_task_time_update(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
    clear: bool,
) -> Result<()> {
    let timestamp = resolve_task_time_value(field, timestamp_ms, timestamp_at, date, is_all_day)?;
    if timestamp.is_some() && clear {
        bail!("task update cannot combine --{field}-ms/--{field}-at/--{field}-date with --clear-{field}");
    }
    if let Some((timestamp, is_all_day)) = timestamp {
        task.insert(
            field.to_string(),
            json!({ "timestamp": timestamp, "is_all_day": is_all_day }),
        );
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    } else if is_all_day {
        bail!("--{field}-all-day requires --{field}-ms, --{field}-at, or --{field}-date");
    }
    Ok(())
}

fn resolve_task_time_value(
    field: &str,
    timestamp_ms: Option<String>,
    timestamp_at: Option<String>,
    date: Option<String>,
    is_all_day: bool,
) -> Result<Option<(String, bool)>> {
    let timestamp_ms = timestamp_ms.filter(|value| !value.trim().is_empty());
    let timestamp_at = timestamp_at.filter(|value| !value.trim().is_empty());
    let date = date.filter(|value| !value.trim().is_empty());
    let provided =
        timestamp_ms.is_some() as u8 + timestamp_at.is_some() as u8 + date.is_some() as u8;
    if provided > 1 {
        bail!(
            "task {field} time accepts only one of --{field}-ms, --{field}-at, or --{field}-date"
        );
    }
    if let Some(timestamp) = timestamp_ms {
        return Ok(Some((timestamp, is_all_day)));
    }
    if let Some(value) = timestamp_at {
        return Ok(Some((parse_task_timestamp_millis(&value)?, is_all_day)));
    }
    if let Some(value) = date {
        return Ok(Some((parse_task_date_millis(&value)?, true)));
    }
    Ok(None)
}

fn parse_task_timestamp_millis(value: &str) -> Result<String> {
    let value = value.trim();
    if value.chars().all(|char| char.is_ascii_digit()) {
        return Ok(value.to_string());
    }
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.timestamp_millis().to_string());
    }
    for format in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("task time is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis().to_string());
        }
    }
    bail!("task time must be milliseconds, RFC3339, or local 'YYYY-MM-DD HH:MM[:SS]': {value}");
}

fn parse_task_date_millis(value: &str) -> Result<String> {
    let date = NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d")
        .with_context(|| format!("parse task all-day date: {value}"))?;
    let naive = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("invalid task date: {value}"))?;
    let datetime = Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow!("task date is ambiguous in local timezone: {value}"))?;
    Ok(datetime.timestamp_millis().to_string())
}

fn insert_task_clearable_string(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    value: Option<String>,
    clear: bool,
) -> Result<()> {
    if value.is_some() && clear {
        bail!("task update cannot combine --{field} with --clear-{field}");
    }
    if let Some(value) = value {
        task.insert(field.to_string(), Value::String(value));
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    }
    Ok(())
}

fn insert_task_clearable_object(
    task: &mut Map<String, Value>,
    update_fields: &mut Vec<Value>,
    field: &str,
    value: Option<String>,
    clear: bool,
    label: &str,
) -> Result<()> {
    if value.is_some() && clear {
        bail!("task update cannot combine --{field}-json with --clear-{field}");
    }
    if let Some(value) = value {
        task.insert(field.to_string(), task_object_from_json(value, label)?);
        update_fields.push(Value::String(field.to_string()));
    } else if clear {
        update_fields.push(Value::String(field.to_string()));
    }
    Ok(())
}

fn insert_task_json_object(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<String>,
    label: &str,
) -> Result<()> {
    if let Some(value) = value {
        body.insert(field.to_string(), task_object_from_json(value, label)?);
    }
    Ok(())
}

fn insert_task_json_array(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<String>,
    label: &str,
) -> Result<()> {
    if let Some(value) = value {
        body.insert(field.to_string(), task_array_from_json(value, label)?);
    }
    Ok(())
}

fn insert_task_reminders(
    body: &mut Map<String, Value>,
    reminders_json: Option<String>,
    reminder_minute: Option<i64>,
    label: &str,
) -> Result<()> {
    if reminders_json.is_some() && reminder_minute.is_some() {
        bail!("task create cannot combine --reminders-json with --reminder-minute");
    }
    if let Some(value) = reminders_json {
        body.insert("reminders".to_string(), task_array_from_json(value, label)?);
    } else if let Some(reminder_minute) = reminder_minute {
        if body.get("due").is_none() {
            bail!("--reminder-minute requires --due-ms, --due-at, or --due-date");
        }
        body.insert(
            "reminders".to_string(),
            Value::Array(vec![task_relative_reminder(reminder_minute)?]),
        );
    }
    Ok(())
}

fn insert_task_optional_u8(
    body: &mut Map<String, Value>,
    field: &str,
    value: Option<u8>,
) -> Result<()> {
    if let Some(value) = value {
        if field == "mode" && !(1..=2).contains(&value) {
            bail!("task mode must be 1 or 2");
        }
        body.insert(
            field.to_string(),
            Value::Number(serde_json::Number::from(value)),
        );
    }
    Ok(())
}

fn task_object_from_json(value: String, label: &str) -> Result<Value> {
    ensure_json_object(parse_json_value(&value, label)?, label)
}

fn task_array_from_json(value: String, label: &str) -> Result<Value> {
    let value = parse_json_value(&value, label)?;
    if label.contains("reminders") && value.get("reminders").is_some() {
        let array = value.get("reminders").expect("checked reminders key");
        return ensure_json_array(array.clone(), "reminders");
    }
    if label.contains("custom-fields") && value.get("custom_fields").is_some() {
        let array = value
            .get("custom_fields")
            .expect("checked custom_fields key");
        return ensure_json_array(array.clone(), "custom_fields");
    }
    ensure_json_array(value, label)
}

fn task_relative_reminder(relative_fire_minute: i64) -> Result<Value> {
    if relative_fire_minute < 0 {
        bail!("task reminder minute cannot be negative");
    }
    Ok(json!({ "relative_fire_minute": relative_fire_minute }))
}

pub(super) fn build_task_member_body(
    args: TaskMemberWriteArgs,
    include_client_token: bool,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        let mut body = ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task member body",
        )?;
        if include_client_token && body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }
    if let Some(members_json) = args.members_json {
        let value = parse_json_value(&members_json, "members-json")?;
        let mut body = if value.get("members").is_some() {
            ensure_json_object(value, "task member body")?
        } else {
            json!({ "members": ensure_json_array(value, "members")? })
        };
        if include_client_token && body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }
    let member_type = args.member_type.as_deref().unwrap_or("user");
    let mut members = Vec::new();
    members.extend(task_members_typed(args.assignees, "assignee", member_type));
    members.extend(task_members_typed(args.followers, "follower", member_type));
    if members.is_empty() {
        bail!("task member command needs --assignee/--follower, --members-json, or raw body");
    }
    let mut body = json!({ "members": members });
    if include_client_token {
        body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
    }
    Ok(body)
}

pub(super) fn build_task_tasklist_body(args: TaskTasklistWriteArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task tasklist relation body",
        );
    }
    let tasklist_guid = args
        .tasklist_guid
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("task tasklist relation needs --tasklist-guid or raw body"))?;
    let mut body = Map::new();
    body.insert("tasklist_guid".to_string(), Value::String(tasklist_guid));
    insert_opt_string(&mut body, "section_guid", args.section_guid);
    Ok(Value::Object(body))
}

pub(super) fn build_task_reminder_add_body(args: TaskReminderAddArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task reminder add body",
        );
    }
    if let Some(reminders_json) = args.reminders_json {
        let value = parse_json_value(&reminders_json, "reminders-json")?;
        if value.get("reminders").is_some() {
            return ensure_json_object(value, "task reminder add body");
        }
        return Ok(json!({ "reminders": ensure_json_array(value, "reminders")? }));
    }
    let relative_fire_minute = args.relative_fire_minute.ok_or_else(|| {
        anyhow!(
            "task reminder add needs --reminder-minute/--relative-fire-minute, --reminders-json, or raw body"
        )
    })?;
    Ok(json!({ "reminders": [task_relative_reminder(relative_fire_minute)?] }))
}

pub(super) fn build_task_reminder_remove_body(args: TaskReminderRemoveArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task reminder remove body",
        );
    }
    if let Some(reminder_ids_json) = args.reminder_ids_json {
        let value = parse_json_value(&reminder_ids_json, "reminder-ids-json")?;
        if value.get("reminder_ids").is_some() {
            return ensure_json_object(value, "task reminder remove body");
        }
        return Ok(json!({ "reminder_ids": ensure_json_array(value, "reminder_ids")? }));
    }
    let reminder_ids = args
        .reminder_ids
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if reminder_ids.is_empty() {
        bail!("task reminder remove needs --reminder-id, --reminder-ids-json, or raw body");
    }
    Ok(json!({ "reminder_ids": reminder_ids }))
}

pub(super) fn build_task_dependency_add_body(args: TaskDependencyAddArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task dependency add body",
        );
    }
    if let Some(dependencies_json) = args.dependencies_json {
        let value = parse_json_value(&dependencies_json, "dependencies-json")?;
        if value.get("dependencies").is_some() {
            return ensure_json_object(value, "task dependency add body");
        }
        return Ok(json!({ "dependencies": ensure_json_array(value, "dependencies")? }));
    }
    let dependency_type = args.dependency_type.trim();
    if dependency_type.is_empty() {
        bail!("task dependency add --type cannot be empty");
    }
    let dependencies = task_dependency_items(args.dependency_task_guids, Some(dependency_type))?;
    Ok(json!({ "dependencies": dependencies }))
}

pub(super) fn build_task_dependency_remove_body(args: TaskDependencyRemoveArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task dependency remove body",
        );
    }
    if let Some(dependencies_json) = args.dependencies_json {
        let value = parse_json_value(&dependencies_json, "dependencies-json")?;
        if value.get("dependencies").is_some() {
            return ensure_json_object(value, "task dependency remove body");
        }
        return Ok(json!({ "dependencies": ensure_json_array(value, "dependencies")? }));
    }
    let dependencies = task_dependency_items(args.dependency_task_guids, None)?;
    Ok(json!({ "dependencies": dependencies }))
}

fn task_dependency_items(task_guids: Vec<String>, dependency_type: Option<&str>) -> Result<Value> {
    let items = task_guids
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(|task_guid| {
            let mut item = Map::new();
            item.insert("task_guid".to_string(), Value::String(task_guid));
            if let Some(dependency_type) = dependency_type {
                item.insert(
                    "type".to_string(),
                    Value::String(dependency_type.to_string()),
                );
            }
            Value::Object(item)
        })
        .collect::<Vec<_>>();
    if items.is_empty() {
        bail!("task dependency command needs --dependency-task-guid, --dependencies-json, or raw body");
    }
    Ok(Value::Array(items))
}

async fn task_request_json(
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

fn task_page_query(page_size: u16, page_token: Option<String>) -> Result<Vec<(String, String)>> {
    if page_size == 0 || page_size > 100 {
        bail!("task page_size must be between 1 and 100");
    }
    let mut query = vec![("page_size".to_string(), page_size.to_string())];
    push_query_opt(&mut query, "page_token", page_token);
    Ok(query)
}

pub(super) fn build_task_list_query(args: &TaskListArgs) -> Result<Vec<(String, String)>> {
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

fn task_user_id_query(user_id_type: UserIdTypeArg) -> Vec<(String, String)> {
    vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )]
}

fn task_members(ids: Vec<String>, role: &str) -> Vec<Value> {
    task_members_typed(ids, role, "user")
}

fn task_members_typed(ids: Vec<String>, role: &str, member_type: &str) -> Vec<Value> {
    ids.into_iter()
        .map(|id| {
            json!({
                "type": member_type,
                "id": id,
                "role": role,
            })
        })
        .collect()
}

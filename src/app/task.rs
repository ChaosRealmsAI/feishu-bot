#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use super::*;

mod helpers;
mod structure;
mod tasklist;

pub(super) use helpers::*;
pub(super) use structure::*;
use tasklist::run_tasklist_command;

pub(super) async fn run_task_command(
    api: &mut FeishuClient,
    command: TaskCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        TaskCommand::Tasklist(command) => run_tasklist_command(api, command).await?,
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

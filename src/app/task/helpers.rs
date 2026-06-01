use super::*;

mod inputs;

use inputs::*;

pub(in crate::app) fn build_task_create_body(args: TaskCreateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_tasklist_create_body(args: TasklistCreateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_tasklist_update_body(args: TasklistUpdateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_tasklist_member_body(args: TasklistMemberWriteArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_comment_create_body(args: TaskCommentCreateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_comment_update_body(args: TaskCommentUpdateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_subtask_create_body(args: TaskSubtaskCreateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_update_body(args: TaskUpdateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_member_body(
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

pub(in crate::app) fn build_task_tasklist_body(args: TaskTasklistWriteArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_reminder_add_body(args: TaskReminderAddArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_reminder_remove_body(
    args: TaskReminderRemoveArgs,
) -> Result<Value> {
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

pub(in crate::app) fn build_task_dependency_add_body(args: TaskDependencyAddArgs) -> Result<Value> {
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

pub(in crate::app) fn build_task_dependency_remove_body(
    args: TaskDependencyRemoveArgs,
) -> Result<Value> {
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

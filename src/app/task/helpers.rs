use super::*;

mod collaboration;
mod inputs;
mod relations;

pub(in crate::app) use collaboration::*;
use inputs::*;
pub(in crate::app) use relations::*;

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

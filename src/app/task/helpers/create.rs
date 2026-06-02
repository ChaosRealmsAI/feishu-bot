use super::*;

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

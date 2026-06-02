use super::*;

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

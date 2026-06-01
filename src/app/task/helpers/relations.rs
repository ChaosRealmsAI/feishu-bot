use super::*;

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

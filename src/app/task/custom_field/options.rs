use super::*;

pub(in crate::app) fn build_task_custom_field_option_create_body(
    args: TaskCustomFieldOptionCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task custom-field option create body",
        );
    }
    task_validate_insert_position(&args.insert_before, &args.insert_after)?;
    let name = args
        .name
        .ok_or_else(|| anyhow!("task custom-field option create needs --name or raw body"))?;
    let mut option = Map::new();
    option.insert("name".to_string(), Value::String(name));
    insert_opt_i64(&mut option, "color_index", args.color_index);
    insert_opt_string(&mut option, "insert_before", args.insert_before);
    insert_opt_string(&mut option, "insert_after", args.insert_after);
    Ok(Value::Object(option))
}

pub(in crate::app) fn build_task_custom_field_option_update_body(
    args: TaskCustomFieldOptionUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task custom-field option update body",
        );
    }
    task_validate_insert_position(&args.insert_before, &args.insert_after)?;
    let mut option = Map::new();
    let mut update_fields = args
        .update_fields
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if let Some(name) = args.name {
        option.insert("name".to_string(), Value::String(name));
        task_push_update_field(&mut update_fields, "name");
    }
    if let Some(color_index) = args.color_index {
        option.insert("color_index".to_string(), Value::Number(color_index.into()));
        task_push_update_field(&mut update_fields, "color_index");
    }
    if let Some(is_hidden) = args.is_hidden {
        option.insert("is_hidden".to_string(), Value::Bool(is_hidden));
        task_push_update_field(&mut update_fields, "is_hidden");
    }
    if let Some(insert_before) = args.insert_before {
        option.insert("insert_before".to_string(), Value::String(insert_before));
        task_push_update_field(&mut update_fields, "insert_before");
    }
    if let Some(insert_after) = args.insert_after {
        option.insert("insert_after".to_string(), Value::String(insert_after));
        task_push_update_field(&mut update_fields, "insert_after");
    }
    if update_fields.is_empty() {
        bail!("task custom-field option update needs a field flag, --update-field, or raw body");
    }
    Ok(json!({
        "option": Value::Object(option),
        "update_fields": Value::Array(update_fields),
    }))
}

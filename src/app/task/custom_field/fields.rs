use super::settings::task_insert_custom_field_setting;
use super::*;

pub(in crate::app) fn build_task_custom_field_create_body(
    args: TaskCustomFieldCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task custom-field create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("task custom-field create needs --name or raw body"))?;
    let field_type = args
        .field_type
        .ok_or_else(|| anyhow!("task custom-field create needs --type or raw body"))?;
    let resource_id = args.resource_id.ok_or_else(|| {
        anyhow!("task custom-field create needs --resource-id tasklist_guid or raw body")
    })?;
    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(name));
    body.insert("type".to_string(), Value::String(field_type.clone()));
    body.insert(
        "resource_type".to_string(),
        Value::String(args.resource_type),
    );
    body.insert("resource_id".to_string(), Value::String(resource_id));
    task_insert_custom_field_setting(
        &mut body,
        &field_type,
        args.setting_key,
        args.setting_json,
        args.options,
        args.options_json,
        true,
    )?;
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_task_custom_field_update_body(
    args: TaskCustomFieldUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task custom-field update body",
        );
    }
    let mut custom_field = Map::new();
    let mut update_fields = args
        .update_fields
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if let Some(name) = args.name {
        custom_field.insert("name".to_string(), Value::String(name));
        task_push_update_field(&mut update_fields, "name");
    }
    if let Some(setting_json) = args.setting_json {
        let setting_key = args.setting_key.ok_or_else(|| {
            anyhow!("task custom-field update needs --setting-key with --setting-json")
        })?;
        custom_field.insert(
            setting_key.clone(),
            ensure_json_object(
                parse_json_value(&setting_json, "setting-json")?,
                "setting-json",
            )?,
        );
        task_push_update_field(&mut update_fields, &setting_key);
    }
    if update_fields.is_empty() {
        bail!("task custom-field update needs --name, --setting-json/--setting-key, --update-field, or raw body");
    }
    Ok(json!({
        "custom_field": Value::Object(custom_field),
        "update_fields": Value::Array(update_fields),
    }))
}

pub(in crate::app) fn build_task_custom_field_resource_body(
    args: TaskCustomFieldResourceArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task custom-field resource body",
        );
    }
    let resource_id = args
        .resource_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("task custom-field add/remove needs --resource-id or raw body"))?;
    Ok(json!({
        "resource_type": args.resource_type,
        "resource_id": resource_id,
    }))
}

use super::*;

pub(in crate::app) fn build_task_section_create_body(args: TaskSectionCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task section create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("task section create needs --name or raw body"))?;
    task_validate_insert_position(&args.insert_before, &args.insert_after)?;
    if args.resource_type == "tasklist" && args.resource_id.as_deref().unwrap_or("").is_empty() {
        bail!("task section create with --resource-type tasklist needs --resource-id");
    }
    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(name));
    body.insert(
        "resource_type".to_string(),
        Value::String(args.resource_type),
    );
    insert_opt_string(&mut body, "resource_id", args.resource_id);
    insert_opt_string(&mut body, "insert_before", args.insert_before);
    insert_opt_string(&mut body, "insert_after", args.insert_after);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_task_section_update_body(args: TaskSectionUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task section update body",
        );
    }
    task_validate_insert_position(&args.insert_before, &args.insert_after)?;
    let mut section = Map::new();
    let mut update_fields = args
        .update_fields
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if let Some(name) = args.name {
        section.insert("name".to_string(), Value::String(name));
        task_push_update_field(&mut update_fields, "name");
    }
    if let Some(insert_before) = args.insert_before {
        section.insert("insert_before".to_string(), Value::String(insert_before));
        task_push_update_field(&mut update_fields, "insert_before");
    }
    if let Some(insert_after) = args.insert_after {
        section.insert("insert_after".to_string(), Value::String(insert_after));
        task_push_update_field(&mut update_fields, "insert_after");
    }
    if update_fields.is_empty() {
        bail!("task section update needs --name, --insert-before, --insert-after, --update-field, or raw body");
    }
    Ok(json!({
        "section": Value::Object(section),
        "update_fields": Value::Array(update_fields),
    }))
}

pub(in crate::app) fn build_task_attachment_upload_parts(
    args: TaskAttachmentUploadArgs,
) -> Result<(Vec<(String, String)>, Vec<(String, PathBuf)>)> {
    if args.files.is_empty() || args.files.len() > 5 {
        bail!("task attachment upload needs 1..=5 --file values");
    }
    for file in &args.files {
        let metadata =
            fs::metadata(file).with_context(|| format!("read metadata {}", file.display()))?;
        if !metadata.is_file() {
            bail!("task attachment path is not a file: {}", file.display());
        }
        validate_upload_size(metadata.len(), 50 * 1024 * 1024, "task attachment upload")?;
    }
    let fields = vec![
        ("resource_type".to_string(), args.resource_type),
        ("resource_id".to_string(), args.resource_id),
    ];
    let files = args
        .files
        .into_iter()
        .map(|file| ("file".to_string(), file))
        .collect();
    Ok((fields, files))
}

pub(super) fn task_validate_insert_position(
    insert_before: &Option<String>,
    insert_after: &Option<String>,
) -> Result<()> {
    if insert_before.is_some() && insert_after.is_some() {
        bail!("insert_before and insert_after cannot be used together");
    }
    Ok(())
}

pub(super) fn task_push_update_field(update_fields: &mut Vec<Value>, field: &str) {
    if !update_fields
        .iter()
        .any(|value| value.as_str() == Some(field))
    {
        update_fields.push(Value::String(field.to_string()));
    }
}

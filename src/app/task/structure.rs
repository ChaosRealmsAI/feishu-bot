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

pub(in crate::app) fn build_task_custom_field_value_update_body(
    args: TaskCustomFieldSetValueArgs,
) -> Result<Value> {
    let mut field = Map::new();
    field.insert("guid".to_string(), Value::String(args.custom_field_guid));

    match args.value_type {
        TaskCustomFieldValueTypeArg::Text => {
            field.insert(
                "text_value".to_string(),
                Value::String(task_scalar_custom_field_value(
                    args.value, args.clear, "text",
                )?),
            );
        }
        TaskCustomFieldValueTypeArg::Number => {
            field.insert(
                "number_value".to_string(),
                Value::String(task_scalar_custom_field_value(
                    args.value, args.clear, "number",
                )?),
            );
        }
        TaskCustomFieldValueTypeArg::Datetime => {
            field.insert(
                "datetime_value".to_string(),
                Value::String(task_scalar_custom_field_value(
                    args.value, args.clear, "datetime",
                )?),
            );
        }
        TaskCustomFieldValueTypeArg::Member => {
            let members =
                task_custom_field_member_values(args.members, &args.member_type, args.clear)?;
            field.insert("member_value".to_string(), Value::Array(members));
        }
        TaskCustomFieldValueTypeArg::SingleSelect => {
            let option =
                task_single_select_custom_field_value(args.value, args.option_guids, args.clear)?;
            field.insert("single_select_value".to_string(), Value::String(option));
        }
        TaskCustomFieldValueTypeArg::MultiSelect => {
            let options = task_multi_select_custom_field_value(args.option_guids, args.clear)?;
            field.insert("multi_select_value".to_string(), Value::Array(options));
        }
    }

    Ok(json!({
        "task": {
            "custom_fields": [Value::Object(field)]
        },
        "update_fields": ["custom_fields"],
    }))
}

fn task_scalar_custom_field_value(
    value: Option<String>,
    clear: bool,
    label: &str,
) -> Result<String> {
    if clear {
        return Ok(String::new());
    }
    value.ok_or_else(|| anyhow!("task custom-field set-value needs --value for {label} fields"))
}

fn task_custom_field_member_values(
    members: Vec<String>,
    member_type: &str,
    clear: bool,
) -> Result<Vec<Value>> {
    if clear {
        return Ok(Vec::new());
    }
    let members = members
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(|id| {
            json!({
                "id": id,
                "type": member_type,
            })
        })
        .collect::<Vec<_>>();
    if members.is_empty() {
        bail!("task custom-field set-value needs --member for member fields, or --clear");
    }
    Ok(members)
}

fn task_single_select_custom_field_value(
    value: Option<String>,
    option_guids: Vec<String>,
    clear: bool,
) -> Result<String> {
    if clear {
        return Ok(String::new());
    }
    let mut options = option_guids
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        options.push(value);
    }
    if options.len() != 1 {
        bail!("task custom-field set-value single-select needs exactly one --option-guid or --value, or --clear");
    }
    Ok(options.remove(0))
}

fn task_multi_select_custom_field_value(
    option_guids: Vec<String>,
    clear: bool,
) -> Result<Vec<Value>> {
    if clear {
        return Ok(Vec::new());
    }
    let options = option_guids
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if options.is_empty() {
        bail!("task custom-field set-value multi-select needs --option-guid values, or --clear");
    }
    Ok(options)
}

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

fn task_insert_custom_field_setting(
    body: &mut Map<String, Value>,
    field_type: &str,
    setting_key: Option<String>,
    setting_json: Option<String>,
    options: Vec<String>,
    options_json: Option<String>,
    require_setting: bool,
) -> Result<()> {
    let inferred_key = task_custom_field_setting_key(field_type);
    if let Some(setting_json) = setting_json {
        let key = setting_key
            .or_else(|| inferred_key.map(str::to_string))
            .ok_or_else(|| anyhow!("unknown custom-field --type; pass --setting-key"))?;
        body.insert(
            key,
            ensure_json_object(
                parse_json_value(&setting_json, "setting-json")?,
                "setting-json",
            )?,
        );
        return Ok(());
    }

    let select_options = task_custom_field_options(options, options_json)?;
    if let Some(select_options) = select_options {
        let key = match field_type {
            "single_select" => "single_select_setting",
            "multi_select" => "multi_select_setting",
            other => bail!("--option/--options-json only works with single_select or multi_select, got {other}"),
        };
        body.insert(key.to_string(), json!({ "options": select_options }));
        return Ok(());
    }

    if field_type == "text" {
        body.insert("text_setting".to_string(), json!({}));
        return Ok(());
    }

    if require_setting {
        bail!("task custom-field create needs --setting-json for this --type, or --option/--options-json for select fields");
    }
    Ok(())
}

fn task_custom_field_setting_key(field_type: &str) -> Option<&'static str> {
    match field_type {
        "number" => Some("number_setting"),
        "member" => Some("member_setting"),
        "datetime" => Some("datetime_setting"),
        "single_select" => Some("single_select_setting"),
        "multi_select" => Some("multi_select_setting"),
        "text" => Some("text_setting"),
        _ => None,
    }
}

fn task_custom_field_options(
    options: Vec<String>,
    options_json: Option<String>,
) -> Result<Option<Value>> {
    if let Some(options_json) = options_json {
        return Ok(Some(ensure_json_array(
            parse_json_value(&options_json, "options-json")?,
            "options-json",
        )?));
    }
    let options = options
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(|name| json!({ "name": name }))
        .collect::<Vec<_>>();
    if options.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Array(options)))
    }
}

fn task_validate_insert_position(
    insert_before: &Option<String>,
    insert_after: &Option<String>,
) -> Result<()> {
    if insert_before.is_some() && insert_after.is_some() {
        bail!("insert_before and insert_after cannot be used together");
    }
    Ok(())
}

fn task_push_update_field(update_fields: &mut Vec<Value>, field: &str) {
    if !update_fields
        .iter()
        .any(|value| value.as_str() == Some(field))
    {
        update_fields.push(Value::String(field.to_string()));
    }
}

use super::*;

pub(super) fn task_insert_custom_field_setting(
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
            other => bail!(
                "--option/--options-json only works with single_select or multi_select, got {other}"
            ),
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

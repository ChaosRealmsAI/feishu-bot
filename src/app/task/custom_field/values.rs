use super::*;

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

use super::super::*;

pub(in crate::app) fn build_base_view_create_body(args: BaseViewCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "view create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("base view create needs --name or raw body"))?;
    Ok(json!({
        "view_name": name,
        "view_type": args.view_type,
    }))
}

pub(in crate::app) fn build_base_view_update_body(args: BaseViewUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "view update body",
        );
    }
    let mut body = Map::new();
    if let Some(name) = args.name {
        body.insert("view_name".to_string(), Value::String(name));
    }
    let property = build_base_view_property(
        args.property_json,
        args.hidden_field_ids,
        args.filter_conjunction,
        args.filter_conditions,
        args.filter_condition_omitted,
        args.hierarchy_field_id,
    )?;
    if let Some(property) = property {
        body.insert("property".to_string(), property);
    }
    if body.is_empty() {
        bail!("base view update needs --name, typed property flags, --property-json, or raw body");
    }
    Ok(Value::Object(body))
}

fn build_base_view_property(
    property_json: Option<String>,
    hidden_field_ids: Vec<String>,
    filter_conjunction: Option<String>,
    filter_conditions: Vec<String>,
    filter_condition_omitted: Option<bool>,
    hierarchy_field_id: Option<String>,
) -> Result<Option<Value>> {
    let mut property = match property_json {
        Some(property) => match ensure_json_object(
            parse_json_value(&property, "property-json")?,
            "view.property",
        )? {
            Value::Object(map) => map,
            _ => unreachable!("ensure_json_object returned a non-object"),
        },
        None => Map::new(),
    };

    let hidden_field_ids = clean_string_values(hidden_field_ids);
    if !hidden_field_ids.is_empty() {
        let mut hidden_fields = property
            .remove("hidden_fields")
            .map(|value| ensure_json_array(value, "view.property.hidden_fields"))
            .transpose()?
            .and_then(|value| value.as_array().cloned())
            .unwrap_or_default();
        for field_id in hidden_field_ids {
            if hidden_fields
                .iter()
                .any(|value| value.as_str() == Some(&field_id))
            {
                continue;
            }
            hidden_fields.push(Value::String(field_id));
        }
        property.insert("hidden_fields".to_string(), Value::Array(hidden_fields));
    }

    let has_filter = filter_conjunction.is_some()
        || !filter_conditions.is_empty()
        || filter_condition_omitted.is_some();
    if has_filter {
        let mut filter_info = property
            .remove("filter_info")
            .map(|value| ensure_json_object(value, "view.property.filter_info"))
            .transpose()?
            .and_then(|value| value.as_object().cloned())
            .unwrap_or_default();
        if let Some(conjunction) = filter_conjunction.filter(|value| !value.trim().is_empty()) {
            filter_info.insert("conjunction".to_string(), Value::String(conjunction));
        }
        if !filter_conditions.is_empty() {
            filter_info.insert(
                "conditions".to_string(),
                Value::Array(
                    filter_conditions
                        .into_iter()
                        .map(parse_base_view_filter_condition)
                        .collect::<Result<Vec<_>>>()?,
                ),
            );
        }
        if let Some(condition_omitted) = filter_condition_omitted {
            filter_info.insert(
                "condition_omitted".to_string(),
                Value::Bool(condition_omitted),
            );
        }
        property.insert("filter_info".to_string(), Value::Object(filter_info));
    }

    if let Some(field_id) = hierarchy_field_id.filter(|value| !value.trim().is_empty()) {
        let mut hierarchy_config = property
            .remove("hierarchy_config")
            .map(|value| ensure_json_object(value, "view.property.hierarchy_config"))
            .transpose()?
            .and_then(|value| value.as_object().cloned())
            .unwrap_or_default();
        hierarchy_config.insert("field_id".to_string(), Value::String(field_id));
        property.insert(
            "hierarchy_config".to_string(),
            Value::Object(hierarchy_config),
        );
    }

    if property.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Object(property)))
    }
}

fn parse_base_view_filter_condition(value: String) -> Result<Value> {
    let mut parts = value.splitn(4, ':');
    let field_id = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs field_id"))?;
    let field_type = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs field_type"))?;
    let operator = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs operator"))?;
    let raw_filter_value = parts.next().ok_or_else(|| {
        anyhow!("base view --filter-condition must be field_id:field_type:operator:value")
    })?;
    let filter_value = if let Some(json_value) = raw_filter_value.strip_prefix("json:") {
        serde_json::to_string(&parse_json_value(
            json_value,
            "filter condition JSON value",
        )?)?
    } else {
        raw_filter_value.to_string()
    };
    Ok(json!({
        "field_id": field_id,
        "field_type": field_type,
        "operator": operator,
        "value": filter_value,
    }))
}

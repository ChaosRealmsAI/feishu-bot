use super::super::*;

pub(in crate::app) fn build_base_field_list_query(
    args: &BaseFieldListArgs,
) -> Vec<(String, String)> {
    let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    push_query_opt(&mut query, "view_id", args.view_id.clone());
    if args.text_field_as_array {
        query.push(("text_field_as_array".to_string(), "true".to_string()));
    }
    query
}

pub(in crate::app) fn build_base_record_search_body(args: &BaseRecordSearchArgs) -> Result<Value> {
    let has_raw = has_json_input(&args.body_json, &args.file, args.stdin);
    let has_typed = args.view_id.is_some()
        || !args.field_names.is_empty()
        || args.field_names_json.is_some()
        || args.filter_json.is_some()
        || args.sort_json.is_some()
        || args.automatic_fields;
    if has_raw {
        if has_typed {
            bail!("base record search cannot combine raw body input with typed search flags");
        }
        return ensure_json_object(
            read_json_value(args.body_json.clone(), args.file.clone(), args.stdin)?,
            "record search body",
        );
    }

    let mut body = Map::new();
    insert_opt_string(&mut body, "view_id", args.view_id.clone());
    let field_names = base_record_search_field_names(args)?;
    insert_string_array(&mut body, "field_names", field_names);
    if let Some(filter_json) = args.filter_json.as_ref() {
        let filter = ensure_json_object(parse_json_value(filter_json, "filter-json")?, "filter")?;
        body.insert("filter".to_string(), filter);
    }
    if let Some(sort_json) = args.sort_json.as_ref() {
        let sort = ensure_json_array(parse_json_value(sort_json, "sort-json")?, "sort")?;
        body.insert("sort".to_string(), sort);
    }
    if args.automatic_fields {
        body.insert("automatic_fields".to_string(), Value::Bool(true));
    }
    Ok(Value::Object(body))
}

fn base_record_search_field_names(args: &BaseRecordSearchArgs) -> Result<Vec<String>> {
    let mut names = clean_string_values(args.field_names.clone());
    if let Some(field_names_json) = args.field_names_json.as_ref() {
        let value = parse_json_value(field_names_json, "field-names-json")?;
        let field_names = if let Some(field_names) = value.get("field_names") {
            field_names.clone()
        } else {
            value
        };
        let field_names = ensure_json_array(field_names, "field_names")?;
        let Some(items) = field_names.as_array() else {
            unreachable!("ensure_json_array returned a non-array");
        };
        for item in items {
            let name = item
                .as_str()
                .ok_or_else(|| anyhow!("field_names must contain only strings"))?;
            if !name.trim().is_empty() {
                names.push(name.to_string());
            }
        }
    }
    Ok(names)
}

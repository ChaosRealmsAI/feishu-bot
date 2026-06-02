use super::*;

pub(in crate::app) fn build_base_app_update_body(args: BaseAppUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base app update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if let Some(is_advanced) = args.is_advanced {
        body.insert("is_advanced".to_string(), Value::Bool(is_advanced));
    }
    if body.is_empty() {
        bail!("base update needs --name, --is-advanced, or raw body");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_base_copy_body(args: BaseCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base copy body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "folder_token", args.folder_token);
    insert_opt_string(&mut body, "time_zone", args.time_zone);
    if let Some(without_content) = args.without_content {
        body.insert("without_content".to_string(), Value::Bool(without_content));
    }
    if body.is_empty() {
        bail!(
            "base copy needs --name, --folder-token, --without-content, --time-zone, or raw body"
        );
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_base_table_batch_create_body(
    args: BaseTableBatchCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "table batch_create body",
        );
    }
    if let Some(tables_json) = args.tables_json {
        let value = parse_json_value(&tables_json, "tables-json")?;
        if value.get("tables").is_some() {
            return ensure_json_object(value, "table batch_create body");
        }
        return Ok(json!({ "tables": ensure_json_array(value, "tables")? }));
    }
    let tables = args
        .name
        .into_iter()
        .filter(|name| !name.trim().is_empty())
        .map(|name| json!({ "name": name }))
        .collect::<Vec<_>>();
    if tables.is_empty() {
        bail!("table batch-create needs --name, --tables-json, or raw body");
    }
    Ok(json!({ "tables": tables }))
}

pub(in crate::app) fn build_base_table_create_body(args: BaseTableCreateArgs) -> Result<Value> {
    let mut table = Map::new();
    insert_opt_string(&mut table, "name", args.name);
    insert_opt_string(&mut table, "default_view_name", args.default_view_name);

    let mut fields = Vec::new();
    if let Some(value) =
        read_optional_json_value(args.fields_json, args.fields_file, args.fields_stdin)?
    {
        match ensure_json_array(value, "table.fields")? {
            Value::Array(items) => fields.extend(items),
            _ => unreachable!("ensure_json_array only returns arrays"),
        }
    }
    for spec in args.field_specs {
        fields.push(parse_base_table_field_spec(&spec)?);
    }
    if !fields.is_empty() {
        table.insert("fields".to_string(), Value::Array(fields));
    }

    Ok(json!({ "table": Value::Object(table) }))
}

pub(in crate::app) fn build_base_table_update_body(args: BaseTableUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base table update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if body.is_empty() {
        bail!("base table update needs --name or raw body");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_base_dashboard_copy_body(args: BaseDashboardCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "dashboard copy body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("dashboard copy needs --name or raw body"))?;
    Ok(json!({ "name": name }))
}

pub(in crate::app) fn build_base_workflow_update_body(
    args: BaseWorkflowUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "workflow update body",
        );
    }
    let status = args
        .status
        .ok_or_else(|| anyhow!("workflow update needs --status or raw body"))?;
    Ok(json!({ "status": status.as_feishu() }))
}

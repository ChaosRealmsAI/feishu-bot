use super::*;

pub(in crate::app) fn build_sheet_create_body(args: SheetCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "spreadsheet create body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "title", args.title);
    insert_opt_string(&mut body, "folder_token", args.folder_token);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_sheet_add_body(args: SheetAddArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return wrap_sheet_batch_request(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "addSheet",
            "sheet add body",
        );
    }
    let mut properties = Map::new();
    let title = args
        .title
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("sheet add-sheet needs --title or raw body"))?;
    properties.insert("title".to_string(), Value::String(title));
    insert_opt_i64(&mut properties, "index", args.index);
    Ok(json!({ "requests": [{ "addSheet": { "properties": properties } }] }))
}

pub(in crate::app) fn build_sheet_copy_body(args: SheetCopyArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return wrap_sheet_batch_request(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "copySheet",
            "sheet copy body",
        );
    }
    let mut destination = Map::new();
    insert_opt_string(&mut destination, "title", args.title);
    insert_opt_i64(&mut destination, "index", args.index);
    Ok(json!({
        "requests": [{
            "copySheet": {
                "source": { "sheetId": args.sheet_id },
                "destination": destination
            }
        }]
    }))
}

pub(in crate::app) fn build_sheet_delete_body(args: SheetDeleteArgs) -> Value {
    json!({
        "requests": [{
            "deleteSheet": { "sheetId": args.sheet_id }
        }]
    })
}

pub(in crate::app) fn build_sheet_update_body(args: SheetUpdateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return wrap_sheet_batch_request(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "updateSheet",
            "sheet update body",
        );
    }
    let mut properties = Map::new();
    properties.insert("sheetId".to_string(), Value::String(args.sheet_id));
    insert_opt_string(&mut properties, "title", args.title);
    insert_opt_i64(&mut properties, "index", args.index);
    if let Some(hidden) = args.hidden {
        properties.insert("hidden".to_string(), Value::Bool(hidden));
    }
    insert_opt_i64(&mut properties, "frozenRowCount", args.frozen_row_count);
    insert_opt_i64(&mut properties, "frozenColCount", args.frozen_col_count);
    if args.protect_lock.is_some() || args.lock_info.is_some() || !args.protect_users.is_empty() {
        let mut protect = Map::new();
        insert_opt_string(&mut protect, "lock", args.protect_lock);
        insert_opt_string(&mut protect, "lockInfo", args.lock_info);
        if !args.protect_users.is_empty() {
            protect.insert(
                "userIDs".to_string(),
                Value::Array(args.protect_users.into_iter().map(Value::String).collect()),
            );
        }
        properties.insert("protect".to_string(), Value::Object(protect));
    }
    if properties.len() <= 1 {
        bail!("sheet update-sheet needs a property flag or raw body");
    }
    Ok(json!({ "requests": [{ "updateSheet": { "properties": properties } }] }))
}

pub(in crate::app) fn build_sheet_merge_body(args: SheetMergeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "merge_cells body",
        );
    }
    let range = args
        .range
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("sheet merge needs --range or raw body"))?;
    Ok(json!({
        "range": range,
        "mergeType": normalize_sheet_merge_type(&args.merge_type)?,
    }))
}

pub(in crate::app) fn build_sheet_unmerge_body(args: SheetUnmergeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "unmerge_cells body",
        );
    }
    let range = args
        .range
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("sheet unmerge needs --range or raw body"))?;
    Ok(json!({ "range": range }))
}

pub(in crate::app) fn build_sheet_style_body(args: SheetStyleArgs) -> Result<Value> {
    let has_raw = has_json_input(&args.body_json, &args.file, args.stdin);
    let has_typed = !args.ranges.is_empty()
        || args.style_json.is_some()
        || args.bold.is_some()
        || args.italic.is_some()
        || args.font_size.is_some()
        || args.font_clean.is_some()
        || args.text_decoration.is_some()
        || args.formatter.is_some()
        || args.h_align.is_some()
        || args.v_align.is_some()
        || args.fore_color.is_some()
        || args.back_color.is_some()
        || args.border_type.is_some()
        || args.border_color.is_some()
        || args.clean.is_some();
    if has_raw {
        if has_typed {
            bail!("sheet style cannot combine raw body input with typed style flags");
        }
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "styles_batch_update body",
        );
    }
    let ranges = clean_string_values(args.ranges.clone());
    if ranges.is_empty() {
        bail!("sheet style needs at least one --range or raw body");
    }
    let style = build_sheet_style_object(args)?;
    if style.is_empty() {
        bail!("sheet style needs at least one style flag, --style-json, or raw body");
    }
    Ok(json!({
        "data": [{
            "ranges": ranges,
            "style": Value::Object(style),
        }]
    }))
}

pub(in crate::app) fn build_sheet_values_body(args: SheetValuesWriteArgs) -> Result<Value> {
    if args.body_json.is_some() {
        return read_json_value(args.body_json, args.file, args.stdin);
    }
    if args.values_json.is_none() && (args.file.is_some() || args.stdin) {
        let value = read_json_value(None, args.file, args.stdin)?;
        if value.get("valueRange").is_some() {
            return Ok(value);
        }
        return Ok(json!({
            "valueRange": {
                "range": args.range,
                "values": ensure_json_array(value, "values")?,
            }
        }));
    }
    let values = read_json_value(args.values_json, None, false)?;
    Ok(json!({
        "valueRange": {
            "range": args.range,
            "values": ensure_json_array(values, "values")?,
        }
    }))
}

fn normalize_sheet_merge_type(value: &str) -> Result<&'static str> {
    match value.trim().to_ascii_uppercase().replace('-', "_").as_str() {
        "ALL" | "MERGE_ALL" => Ok("MERGE_ALL"),
        "ROWS" | "MERGE_ROWS" => Ok("MERGE_ROWS"),
        "COLUMNS" | "COLS" | "MERGE_COLUMNS" => Ok("MERGE_COLUMNS"),
        _ => bail!("merge type must be MERGE_ALL, MERGE_ROWS, or MERGE_COLUMNS"),
    }
}

fn build_sheet_style_object(args: SheetStyleArgs) -> Result<Map<String, Value>> {
    let mut style = if let Some(style_json) = args.style_json {
        ensure_json_object(parse_json_value(&style_json, "style-json")?, "style")?
            .as_object()
            .cloned()
            .unwrap_or_default()
    } else {
        Map::new()
    };

    let mut font = style
        .remove("font")
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    if let Some(value) = args.bold {
        font.insert("bold".to_string(), Value::Bool(value));
    }
    if let Some(value) = args.italic {
        font.insert("italic".to_string(), Value::Bool(value));
    }
    insert_opt_string(&mut font, "fontSize", args.font_size);
    if let Some(value) = args.font_clean {
        font.insert("clean".to_string(), Value::Bool(value));
    }
    if !font.is_empty() {
        style.insert("font".to_string(), Value::Object(font));
    }

    insert_opt_i64(&mut style, "textDecoration", args.text_decoration);
    insert_opt_string(&mut style, "formatter", args.formatter);
    insert_opt_i64(&mut style, "hAlign", args.h_align);
    insert_opt_i64(&mut style, "vAlign", args.v_align);
    insert_opt_sheet_color(&mut style, "foreColor", args.fore_color)?;
    insert_opt_sheet_color(&mut style, "backColor", args.back_color)?;
    insert_opt_string(
        &mut style,
        "borderType",
        args.border_type
            .map(|value| value.trim().to_ascii_uppercase()),
    );
    insert_opt_sheet_color(&mut style, "borderColor", args.border_color)?;
    if let Some(value) = args.clean {
        style.insert("clean".to_string(), Value::Bool(value));
    }
    Ok(style)
}

fn insert_opt_sheet_color(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<String>,
) -> Result<()> {
    let Some(value) = value else {
        return Ok(());
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let color = if trimmed.starts_with('#') {
        trimmed.to_string()
    } else {
        format!("#{trimmed}")
    };
    if color.len() != 7 || !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("{key} must be a hex color like #ff0000 or ff0000");
    }
    object.insert(key.to_string(), Value::String(color));
    Ok(())
}

fn wrap_sheet_batch_request(value: Value, request_name: &str, label: &str) -> Result<Value> {
    let object = ensure_json_object(value, label)?;
    if object.get("requests").is_some() {
        return Ok(object);
    }
    Ok(json!({ "requests": [{ request_name: object }] }))
}

use super::*;

pub(super) fn base_record_write_query(
    client_token: Option<String>,
    user_id_type: UserIdTypeArg,
    ignore_consistency_check: bool,
) -> Vec<(String, String)> {
    let mut query = vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )];
    push_query_opt(&mut query, "client_token", client_token);
    if ignore_consistency_check {
        query.push(("ignore_consistency_check".to_string(), "true".to_string()));
    }
    query
}

pub(in crate::app) fn read_base_record_fields(
    fields: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let mut output = match read_optional_json_value(text, file, stdin)? {
        Some(value) => match read_base_record_fields_value(value)? {
            Value::Object(map) => map,
            _ => unreachable!("read_base_record_fields_value returned a non-object"),
        },
        None => Map::new(),
    };
    for field in fields {
        let (key, value) = parse_base_record_field_pair(field)?;
        output.insert(key, value);
    }
    if output.is_empty() {
        bail!("base record needs --field, --fields-json, --fields-file, or --fields-stdin");
    }
    Ok(Value::Object(output))
}

fn read_base_record_fields_value(value: Value) -> Result<Value> {
    if let Some(fields) = value.get("fields") {
        return ensure_json_object(fields.clone(), "fields");
    }
    ensure_json_object(value, "fields")
}

fn parse_base_record_field_pair(value: String) -> Result<(String, Value)> {
    let (key, raw_value) = value
        .split_once('=')
        .ok_or_else(|| anyhow!("base record --field must be name=value, got {value}"))?;
    let key = key.trim();
    if key.is_empty() {
        bail!("base record --field key cannot be empty");
    }
    Ok((key.to_string(), parse_base_record_field_value(raw_value)?))
}

fn parse_base_record_field_value(value: &str) -> Result<Value> {
    if let Some(json_value) = value.strip_prefix("json:") {
        return parse_json_value(json_value, "base record field json value");
    }
    if let Some(string_value) = value.strip_prefix("str:") {
        return Ok(Value::String(string_value.to_string()));
    }
    if let Some(date_value) = value.strip_prefix("date:") {
        return Ok(json!(parse_base_record_date_millis(date_value)?));
    }
    if let Some(datetime_value) = value.strip_prefix("datetime:") {
        return Ok(json!(parse_base_record_datetime_millis(datetime_value)?));
    }
    match serde_json::from_str(value) {
        Ok(value) => Ok(value),
        Err(_) => Ok(Value::String(value.to_string())),
    }
}

pub(super) async fn normalize_base_record_write_fields(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
    fields: &mut Value,
) -> Result<()> {
    if !base_fields_contain_date_like_string(fields) {
        return Ok(());
    }

    let Ok(date_fields) = load_base_date_field_names(api, app_token, table_id).await else {
        return Ok(());
    };
    let Some(fields) = fields.as_object_mut() else {
        return Ok(());
    };
    for (field_name, value) in fields {
        if !date_fields.contains(field_name) {
            continue;
        }
        let Some(text) = value.as_str() else {
            continue;
        };
        if let Some(timestamp) = maybe_parse_base_record_date_millis(text)? {
            *value = json!(timestamp);
        }
    }
    Ok(())
}

pub(super) async fn normalize_base_record_write_records(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
    records: &mut Value,
) -> Result<()> {
    if !base_fields_contain_date_like_string(records) {
        return Ok(());
    }

    let Ok(date_fields) = load_base_date_field_names(api, app_token, table_id).await else {
        return Ok(());
    };
    let Some(records) = records.as_array_mut() else {
        return Ok(());
    };
    for record in records {
        let Some(fields) = record.get_mut("fields").and_then(Value::as_object_mut) else {
            continue;
        };
        for (field_name, value) in fields {
            if !date_fields.contains(field_name) {
                continue;
            }
            let Some(text) = value.as_str() else {
                continue;
            };
            if let Some(timestamp) = maybe_parse_base_record_date_millis(text)? {
                *value = json!(timestamp);
            }
        }
    }
    Ok(())
}

async fn load_base_date_field_names(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
) -> Result<Vec<String>> {
    let path = format!("/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
    let mut page_token = None;
    let mut names = Vec::new();

    loop {
        let mut query = vec![("page_size".to_string(), "100".to_string())];
        push_query_opt(&mut query, "page_token", page_token);
        let response = api.get_json(&path, &query).await?;
        let data = response.get("data").unwrap_or(&Value::Null);
        if let Some(items) = data.get("items").and_then(Value::as_array) {
            for item in items {
                if !base_field_item_is_date(item) {
                    continue;
                }
                if let Some(name) = item.get("field_name").and_then(Value::as_str) {
                    names.push(name.to_string());
                }
            }
        }

        if !data
            .get("has_more")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            break;
        }
        let next = data
            .get("page_token")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if next.is_empty() {
            break;
        }
        page_token = Some(next);
    }

    Ok(names)
}

fn base_field_item_is_date(item: &Value) -> bool {
    item.get("type").and_then(Value::as_i64) == Some(5)
        || item
            .get("ui_type")
            .and_then(Value::as_str)
            .is_some_and(|ui_type| ui_type.eq_ignore_ascii_case("DateTime"))
}

fn base_fields_contain_date_like_string(value: &Value) -> bool {
    match value {
        Value::String(text) => maybe_parse_base_record_date_millis(text)
            .ok()
            .flatten()
            .is_some(),
        Value::Array(values) => values.iter().any(base_fields_contain_date_like_string),
        Value::Object(map) => map.values().any(base_fields_contain_date_like_string),
        _ => false,
    }
}

fn maybe_parse_base_record_date_millis(value: &str) -> Result<Option<i64>> {
    let value = value.trim();
    if value.is_empty() || value.chars().all(|char| char.is_ascii_digit()) {
        return Ok(None);
    }
    parse_base_record_datetime_millis(value)
        .map(Some)
        .or_else(|_| {
            parse_base_record_date_millis(value)
                .map(Some)
                .or_else(|_| Ok(None))
        })
}

fn parse_base_record_datetime_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    if value.chars().all(|char| char.is_ascii_digit()) {
        return value
            .parse::<i64>()
            .with_context(|| format!("parse base datetime milliseconds: {value}"));
    }
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.timestamp_millis());
    }
    for format in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base datetime is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base datetime must be milliseconds, RFC3339, or local 'YYYY-MM-DD HH:MM[:SS]': {value}");
}

fn parse_base_record_date_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    for format in ["%Y-%m-%d", "%Y/%m/%d"] {
        if let Ok(date) = NaiveDate::parse_from_str(value, format) {
            let naive = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("invalid base date: {value}"))?;
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base date is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base date must be YYYY-MM-DD or YYYY/MM/DD: {value}");
}

pub(in crate::app) fn read_base_record_batch_records(
    record_fields: Vec<String>,
    record_ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    require_record_ids: bool,
) -> Result<Value> {
    let mut records = match read_optional_json_value(text, file, stdin)? {
        Some(value) => match read_records_value(value)? {
            Value::Array(values) => values,
            _ => unreachable!("read_records_value returned a non-array"),
        },
        None => Vec::new(),
    };

    for (index, record_id) in clean_string_values(record_ids).into_iter().enumerate() {
        ensure_record_object(&mut records, index)?;
        set_record_id(&mut records[index], record_id)?;
    }

    for field in record_fields {
        let (index, key, value) = parse_base_record_indexed_field_pair(field)?;
        ensure_record_object(&mut records, index)?;
        let fields = ensure_record_fields_object(&mut records[index])?;
        fields.insert(key, value);
    }

    if records.is_empty() {
        bail!("base record batch needs --record-field, --records-json, --records-file, or --records-stdin");
    }
    if require_record_ids {
        for (index, record) in records.iter().enumerate() {
            let Some(record_id) = record.get("record_id").and_then(Value::as_str) else {
                bail!("base record batch-update needs --record-id for record index {index}, or records_json entries with record_id");
            };
            if record_id.trim().is_empty() {
                bail!("base record batch-update record_id cannot be empty at index {index}");
            }
        }
    }
    Ok(Value::Array(records))
}

fn read_records_value(value: Value) -> Result<Value> {
    let records = if let Some(records) = value.get("records") {
        records.clone()
    } else {
        value
    };
    let array = records
        .as_array()
        .ok_or_else(|| anyhow!("records must be a JSON array or object with records array"))?;
    let normalized = array
        .iter()
        .map(|record| {
            if record.get("fields").is_some() {
                Ok(record.clone())
            } else {
                Ok(json!({ "fields": ensure_json_object(record.clone(), "record fields")? }))
            }
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Value::Array(normalized))
}

fn parse_base_record_indexed_field_pair(value: String) -> Result<(usize, String, Value)> {
    let (index, pair) = value.split_once(':').ok_or_else(|| {
        anyhow!("base record --record-field must be index:name=value, got {value}")
    })?;
    let index = index
        .trim()
        .parse::<usize>()
        .with_context(|| format!("parse record-field index in {value}"))?;
    let (key, field_value) = parse_base_record_field_pair(pair.to_string())?;
    Ok((index, key, field_value))
}

fn ensure_record_object(records: &mut Vec<Value>, index: usize) -> Result<()> {
    while records.len() <= index {
        records.push(json!({ "fields": {} }));
    }
    if !records[index].is_object() {
        bail!("record index {index} must be an object");
    }
    Ok(())
}

fn ensure_record_fields_object(record: &mut Value) -> Result<&mut Map<String, Value>> {
    let record = record
        .as_object_mut()
        .ok_or_else(|| anyhow!("record must be an object"))?;
    if record.get("fields").is_none() {
        record.insert("fields".to_string(), json!({}));
    }
    record
        .get_mut("fields")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| anyhow!("record.fields must be an object"))
}

fn set_record_id(record: &mut Value, record_id: String) -> Result<()> {
    let record = record
        .as_object_mut()
        .ok_or_else(|| anyhow!("record must be an object"))?;
    record.insert("record_id".to_string(), Value::String(record_id));
    Ok(())
}

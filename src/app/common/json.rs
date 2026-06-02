use super::*;

pub(in crate::app) fn insert_opt_string(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<String>,
) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        object.insert(key.to_string(), Value::String(value));
    }
}

pub(in crate::app) fn insert_opt_i64(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<i64>,
) {
    if let Some(value) = value {
        object.insert(key.to_string(), Value::Number(value.into()));
    }
}

pub(in crate::app) fn insert_opt_u8(object: &mut Map<String, Value>, key: &str, value: Option<u8>) {
    if let Some(value) = value {
        object.insert(key.to_string(), json!(value));
    }
}

pub(in crate::app) fn insert_string_array(
    object: &mut Map<String, Value>,
    key: &str,
    values: Vec<String>,
) {
    let values = values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if !values.is_empty() {
        object.insert(key.to_string(), Value::Array(values));
    }
}

pub(in crate::app) fn read_json_value(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let text = read_content(text, file, stdin)?;
    parse_json_value(&text, "JSON")
}

pub(in crate::app) fn read_optional_json_value(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Option<Value>> {
    read_optional_content(text, file, stdin)?
        .map(|text| parse_json_value(&text, "JSON"))
        .transpose()
}

pub(in crate::app) fn parse_json_value(text: &str, label: &str) -> Result<Value> {
    serde_json::from_str(text).with_context(|| format!("parse {label}"))
}

pub(in crate::app) fn ensure_json_array(value: Value, label: &str) -> Result<Value> {
    if value.is_array() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON array")
    }
}

pub(in crate::app) fn ensure_json_object(value: Value, label: &str) -> Result<Value> {
    if value.is_object() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON object")
    }
}

pub(in crate::app) fn read_record_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let record_ids = if let Some(record_ids) = value.get("record_ids") {
            record_ids.clone()
        } else if let Some(records) = value.get("records") {
            records.clone()
        } else {
            value
        };
        return ensure_json_array(record_ids, "record_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!(
            "provide --record-id at least once, or JSON via --record-ids-json/--records-json/--file/--stdin"
        );
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

pub(in crate::app) fn read_table_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let table_ids = if let Some(table_ids) = value.get("table_ids") {
            table_ids.clone()
        } else if let Some(tables) = value.get("tables") {
            tables.clone()
        } else {
            value
        };
        return ensure_json_array(table_ids, "table_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("provide --table-id at least once, or JSON via --table-ids-json/--file/--stdin");
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

pub(in crate::app) fn collect_json_string_array(
    mut values: Vec<String>,
    text: Option<String>,
    label: &str,
) -> Result<Option<Value>> {
    if let Some(text) = text {
        let value = parse_json_value(&text, label)?;
        let array = if let Some(nested) = value.get(label) {
            nested.clone()
        } else {
            value
        };
        return Ok(Some(ensure_json_array(array, label)?));
    }
    values.retain(|value| !value.trim().is_empty());
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(Value::Array(
        values.into_iter().map(Value::String).collect(),
    )))
}

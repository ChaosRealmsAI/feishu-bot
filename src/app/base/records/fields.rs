use super::*;

use super::dates::{parse_base_record_date_millis, parse_base_record_datetime_millis};

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

pub(super) fn parse_base_record_field_pair(value: String) -> Result<(String, Value)> {
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

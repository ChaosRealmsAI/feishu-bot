use super::*;

use super::fields::parse_base_record_field_pair;

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

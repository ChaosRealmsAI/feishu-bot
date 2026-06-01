use super::*;

pub(in crate::app) fn build_base_media_extra(
    raw_extra: Option<String>,
    table_id: Option<String>,
    field_id: Option<String>,
    record_id: Option<String>,
    file_tokens: &[String],
) -> Result<Option<String>> {
    let raw_extra = raw_extra.filter(|value| !value.trim().is_empty());
    let table_id = table_id.filter(|value| !value.trim().is_empty());
    let field_id = field_id.filter(|value| !value.trim().is_empty());
    let record_id = record_id.filter(|value| !value.trim().is_empty());

    if raw_extra.is_some() && (table_id.is_some() || field_id.is_some() || record_id.is_some()) {
        bail!("use either --extra or --table-id/--field-id/--record-id, not both");
    }
    if let Some(extra) = raw_extra {
        return Ok(Some(extra));
    }
    if table_id.is_none() && field_id.is_none() && record_id.is_none() {
        return Ok(None);
    }

    let table_id = table_id.ok_or_else(|| anyhow!("base media extra needs --table-id"))?;
    let field_id = field_id.ok_or_else(|| anyhow!("base media extra needs --field-id"))?;
    let record_id = record_id.ok_or_else(|| anyhow!("base media extra needs --record-id"))?;
    if file_tokens.is_empty() {
        bail!("base media extra needs at least one --file-token");
    }

    let mut record_tokens = Map::new();
    record_tokens.insert(record_id, json!(file_tokens));
    let mut attachments = Map::new();
    attachments.insert(field_id, Value::Object(record_tokens));
    Ok(Some(
        json!({
            "bitablePerm": {
                "tableId": table_id,
                "attachments": attachments
            }
        })
        .to_string(),
    ))
}

pub(in crate::app) fn build_base_media_field_value(
    file_tokens: Vec<String>,
    field: Option<String>,
) -> Result<Value> {
    let file_tokens = file_tokens
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    if file_tokens.is_empty() {
        bail!("base media field-value needs at least one --file-token");
    }

    let value = Value::Array(
        file_tokens
            .into_iter()
            .map(|token| json!({ "file_token": token }))
            .collect(),
    );
    let mut data = Map::new();
    data.insert("value".to_string(), value.clone());
    data.insert(
        "usage".to_string(),
        Value::String(
            "Use data.value as a Base attachment field value, or data.fields with `base record create/update --fields-json`; for CLI pairs use `--field '附件=json:[...]'`."
                .to_string(),
        ),
    );
    if let Some(field) = field.filter(|value| !value.trim().is_empty()) {
        let mut fields = Map::new();
        fields.insert(field, value);
        data.insert("fields".to_string(), Value::Object(fields));
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": data
    }))
}

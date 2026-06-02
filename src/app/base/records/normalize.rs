use super::*;

use super::dates::{base_fields_contain_date_like_string, maybe_parse_base_record_date_millis};

pub(in crate::app) async fn normalize_base_record_write_fields(
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

pub(in crate::app) async fn normalize_base_record_write_records(
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

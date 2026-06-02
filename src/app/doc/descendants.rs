use super::*;

pub(in crate::app) fn parse_raw_children(text: &str) -> Result<Vec<Value>> {
    let value: Value = serde_json::from_str(text).context("parse raw children JSON")?;
    if let Some(children) = value.as_array() {
        return Ok(children.clone());
    }
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        return Ok(children.clone());
    }
    bail!("raw children JSON must be an array or an object with a children array")
}

pub(in crate::app) fn converted_to_descendant_body(converted: Value) -> Result<Value> {
    let data = converted
        .get("data")
        .ok_or_else(|| anyhow!("convert response missing data: {converted}"))?;
    if let Some(images) = data
        .get("block_id_to_image_urls")
        .and_then(Value::as_array)
        .filter(|items| !items.is_empty())
    {
        bail!(
            "official converter returned image URL mappings that this CLI cannot upload yet: {}",
            serde_json::to_string(images)?
        );
    }
    let children_id = data
        .get("first_level_block_ids")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| anyhow!("convert response missing first_level_block_ids"))?;
    let mut descendants = data
        .get("blocks")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| anyhow!("convert response missing blocks"))?;
    for block in &mut descendants {
        sanitize_descendant_block(block);
    }
    Ok(json!({
        "index": -1,
        "children_id": children_id,
        "descendants": descendants,
    }))
}

pub(in crate::app) fn ensure_descendant_defaults(body: &mut Value) -> Result<()> {
    let object = body
        .as_object_mut()
        .ok_or_else(|| anyhow!("descendant body must be a JSON object"))?;
    object
        .entry("index".to_string())
        .or_insert_with(|| Value::Number((-1).into()));
    let needs_children_id = object
        .get("children_id")
        .and_then(Value::as_array)
        .is_none_or(|children| children.is_empty());
    let descendants = object
        .get_mut("descendants")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("descendant body must contain descendants array"))?;
    let inferred_children_id = if needs_children_id {
        let ids = descendants
            .iter()
            .filter_map(|block| block.get("block_id").and_then(Value::as_str))
            .map(|id| Value::String(id.to_string()))
            .collect::<Vec<_>>();
        Some(ids)
    } else {
        None
    };
    for block in descendants {
        sanitize_descendant_block(block);
    }
    if let Some(ids) = inferred_children_id {
        object.insert("children_id".to_string(), Value::Array(ids));
    }
    Ok(())
}

fn sanitize_descendant_block(block: &mut Value) {
    if let Some(object) = block.as_object_mut() {
        object.remove("parent_id");
        object.remove("comment_ids");
        object
            .entry("children".to_string())
            .or_insert_with(|| Value::Array(Vec::new()));
    }
    remove_unsupported_descendant_fields(block);
}

fn remove_unsupported_descendant_fields(value: &mut Value) {
    match value {
        Value::Object(object) => {
            object.remove("merge_info");
            for child in object.values_mut() {
                remove_unsupported_descendant_fields(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                remove_unsupported_descendant_fields(item);
            }
        }
        _ => {}
    }
}

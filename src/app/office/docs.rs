use super::*;

#[derive(Debug)]
pub(super) struct CreatedDoc {
    pub(super) document_id: String,
    pub(super) node_token: Option<String>,
    pub(super) url: String,
    pub(super) create_response: Value,
    pub(super) append_response: Option<Value>,
}

pub(super) async fn create_wiki_doc(
    api: &mut FeishuClient,
    space_id: &str,
    parent_node_token: Option<&str>,
    title: &str,
    content_type: ContentTypeArg,
    content: &str,
    auth: ApiAuthArg,
) -> Result<CreatedDoc> {
    let path = format!("/wiki/v2/spaces/{}/nodes", encode_path_segment(space_id));
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String("docx".to_string()));
    body.insert("node_type".to_string(), Value::String("origin".to_string()));
    body.insert("title".to_string(), Value::String(title.to_string()));
    if let Some(parent) = parent_node_token.filter(|value| !value.trim().is_empty()) {
        body.insert(
            "parent_node_token".to_string(),
            Value::String(parent.to_string()),
        );
    }
    let create_response = wiki_request_json(
        api,
        Method::POST,
        &path,
        &[],
        Some(Value::Object(body)),
        auth,
    )
    .await?;
    let document_id = get_string(&create_response, &["data", "node", "obj_token"])
        .or_else(|| get_string(&create_response, &["data", "obj_token"]))
        .ok_or_else(|| {
            anyhow!("wiki create-node response missing docx obj_token: {create_response}")
        })?;
    let node_token = get_string(&create_response, &["data", "node", "node_token"])
        .or_else(|| get_string(&create_response, &["data", "node_token"]));
    let append_response = if content.trim().is_empty() {
        None
    } else {
        Some(
            api.append_converted_content_with_auth(
                &document_id,
                &document_id,
                content_type,
                content,
                auth,
            )
            .await?,
        )
    };
    let url = node_token
        .as_deref()
        .map(|token| wiki_url(api, token))
        .unwrap_or_else(|| api.document_url(&document_id));
    Ok(CreatedDoc {
        document_id,
        node_token,
        url,
        create_response,
        append_response,
    })
}

pub(super) async fn create_standalone_report_doc(
    api: &mut FeishuClient,
    title: &str,
    content_type: ContentTypeArg,
    content: &str,
    auth: ApiAuthArg,
) -> Result<Value> {
    let created = api.create_document_with_auth(title, None, auth).await?;
    let document_id = get_string(&created, &["data", "document", "document_id"])
        .or_else(|| get_string(&created, &["data", "document_id"]))
        .ok_or_else(|| anyhow!("create document response missing document_id: {created}"))?;
    let append_response = if content.trim().is_empty() {
        None
    } else {
        Some(
            api.append_converted_content_with_auth(
                &document_id,
                &document_id,
                content_type,
                content,
                auth,
            )
            .await?,
        )
    };
    Ok(json!({
        "route": "docx",
        "document_id": document_id,
        "url": api.document_url(&document_id),
        "create_response": created,
        "append_response": append_response,
    }))
}

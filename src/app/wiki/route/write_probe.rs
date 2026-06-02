use super::*;

use super::checks::read_wiki_node_for_probe;

pub(super) async fn run_wiki_write_probe(
    api: &mut FeishuClient,
    target_space_id: Option<String>,
    target_parent_node_token: Option<String>,
    auth: ApiAuthArg,
    title: Option<String>,
    apply: bool,
) -> Value {
    let Some(space_id) = target_space_id else {
        return json!({
            "ok": false,
            "error": "write probe requires --space-id or FEISHU_WIKI_SPACE_ID"
        });
    };
    let title = title.unwrap_or_else(|| {
        format!(
            "Feishu Bot Wiki write probe {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )
    });
    let content = format!(
        "# {title}\n\nCreated by `feishu-bot wiki route-check --write-probe` at {}.\n\nThis document proves whether future AI reports can be moved into the configured Feishu Wiki space.",
        Local::now().format("%Y-%m-%d %H:%M:%S %:z")
    );

    let mut output = Map::new();
    output.insert("ok".to_string(), Value::Bool(false));
    output.insert("title".to_string(), Value::String(title.clone()));
    output.insert(
        "target_space_id".to_string(),
        Value::String(space_id.clone()),
    );
    if let Some(parent) = target_parent_node_token.as_ref() {
        output.insert(
            "target_parent_node_token".to_string(),
            Value::String(parent.clone()),
        );
    }
    output.insert(
        "auth".to_string(),
        Value::String(format!("{auth:?}").to_lowercase()),
    );

    let doc = match api.create_document(&title, None).await {
        Ok(doc) => doc,
        Err(error) => {
            output.insert(
                "create_error".to_string(),
                Value::String(format!("{error:#}")),
            );
            return Value::Object(output);
        }
    };
    output.insert("create_response".to_string(), doc.clone());
    let Some(document_id) = get_string(&doc, &["data", "document", "document_id"])
        .or_else(|| get_string(&doc, &["data", "document_id"]))
    else {
        output.insert(
            "create_error".to_string(),
            Value::String(format!(
                "create document response did not include document_id: {doc}"
            )),
        );
        return Value::Object(output);
    };
    output.insert(
        "document_id".to_string(),
        Value::String(document_id.clone()),
    );
    output.insert(
        "url".to_string(),
        Value::String(api.document_url(&document_id)),
    );

    match api
        .append_document(&document_id, &document_id, &content)
        .await
    {
        Ok(append_response) => {
            output.insert("append_response".to_string(), append_response);
        }
        Err(error) => {
            output.insert(
                "append_error".to_string(),
                Value::String(format!("{error:#}")),
            );
            return Value::Object(output);
        }
    }

    let path = format!(
        "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
        encode_path_segment(&space_id)
    );
    let body = build_doc_create_wiki_move_body(&document_id, target_parent_node_token, apply);
    match wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await {
        Ok(move_response) => {
            let node_token = first_string_path(
                &move_response,
                &[
                    &["data", "wiki_token"],
                    &["data", "wiki_node_token"],
                    &["data", "node_token"],
                    &["data", "node", "node_token"],
                    &["data", "result", "wiki_token"],
                    &["data", "result", "node_token"],
                ],
            );
            let task_id = first_string_path(
                &move_response,
                &[
                    &["data", "task_id"],
                    &["data", "task", "task_id"],
                    &["data", "result", "task_id"],
                ],
            );
            output.insert("move_response".to_string(), move_response);
            if let Some(node_token) = node_token {
                output.insert(
                    "wiki_node_token".to_string(),
                    Value::String(node_token.clone()),
                );
                match read_wiki_node_for_probe(api, &node_token, auth).await {
                    Ok(read_response) => {
                        output.insert("ok".to_string(), Value::Bool(true));
                        output.insert("node_readback".to_string(), read_response);
                    }
                    Err(error) => {
                        output.insert(
                            "node_readback_error".to_string(),
                            Value::String(format!("{error:#}")),
                        );
                    }
                }
            } else if let Some(task_id) = task_id {
                output.insert("task_id".to_string(), Value::String(task_id));
                output.insert(
                    "pending".to_string(),
                    Value::String(
                        "move_docs_to_wiki returned an async task_id; poll with `feishu-bot wiki task --task-id <task_id>`"
                            .to_string(),
                    ),
                );
            } else {
                output.insert(
                    "move_result_note".to_string(),
                    Value::String(
                        "move_docs_to_wiki succeeded but no wiki node token or task_id was found"
                            .to_string(),
                    ),
                );
            }
        }
        Err(error) => {
            output.insert(
                "move_error".to_string(),
                Value::String(format!("{error:#}")),
            );
        }
    }

    Value::Object(output)
}

fn first_string_path(value: &Value, paths: &[&[&str]]) -> Option<String> {
    paths.iter().find_map(|path| get_string(value, path))
}

use super::*;

pub(super) async fn readback_chat(api: &mut FeishuClient, chat_id: Option<&str>) -> Value {
    let Some(chat_id) = chat_id else {
        return json!({ "status": "skipped_missing_chat_id" });
    };
    let path = format!("/im/v1/chats/{chat_id}");
    probe_value(
        api.get_json(
            &path,
            &[("user_id_type".to_string(), "open_id".to_string())],
        )
        .await,
    )
}

pub(super) async fn readback_wiki_node(
    api: &mut FeishuClient,
    node_token: Option<&str>,
    auth: ApiAuthArg,
) -> Value {
    let Some(node_token) = node_token else {
        return json!({ "status": "skipped_missing_node_token" });
    };
    probe_value(
        wiki_request_json(
            api,
            Method::GET,
            "/wiki/v2/spaces/get_node",
            &[("token".to_string(), node_token.to_string())],
            None,
            auth,
        )
        .await,
    )
}

pub(super) async fn readback_base(
    api: &mut FeishuClient,
    app_token: Option<&str>,
    table_id: Option<&str>,
) -> Value {
    let Some(app_token) = app_token else {
        return json!({ "status": "skipped_missing_app_token" });
    };
    let app_path = format!("/bitable/v1/apps/{app_token}");
    let table_path = format!("/bitable/v1/apps/{app_token}/tables");
    let tables = probe_value(
        api.get_json(&table_path, &[("page_size".to_string(), "100".to_string())])
            .await,
    );
    json!({
        "app": probe_value(api.get_json(&app_path, &[]).await),
        "target_table_id": table_id,
        "tables": tables,
    })
}

pub(super) async fn readback_message(api: &mut FeishuClient, message_id: Option<&str>) -> Value {
    let Some(message_id) = message_id else {
        return json!({ "status": "skipped_missing_message_id" });
    };
    let path = format!("/im/v1/messages/{}", encode_path_segment(message_id));
    probe_value(
        api.get_json(
            &path,
            &[("user_id_type".to_string(), "open_id".to_string())],
        )
        .await,
    )
}

pub(super) fn extract_chat_id(value: &Value) -> Option<String> {
    get_string(value, &["data", "chat_id"])
        .or_else(|| get_string(value, &["data", "chat", "chat_id"]))
}

pub(super) fn extract_message_id(value: &Value) -> Option<String> {
    get_string(value, &["data", "message_id"])
        .or_else(|| get_string(value, &["data", "message", "message_id"]))
}

pub(super) fn extract_table_id(value: &Value) -> Option<String> {
    get_string(value, &["data", "table_id"])
        .or_else(|| get_string(value, &["data", "table", "table_id"]))
}

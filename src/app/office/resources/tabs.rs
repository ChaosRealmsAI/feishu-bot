use super::*;

pub(in crate::app::office) async fn add_office_tabs(
    api: &mut FeishuClient,
    project: &OfficeProject,
) -> Value {
    let Some(chat_id) = project.chat_id.as_deref() else {
        return json!({ "status": "skipped_missing_chat" });
    };
    let mut items = Vec::new();
    if let Some(node_token) = project.wiki_index_node_token.as_deref() {
        items.push(json!({
            "name": "项目主页",
            "result": probe_value(add_chat_url_tab(api, chat_id, "项目主页", &wiki_url(api, node_token)).await),
        }));
    }
    if let Some(app_token) = project.base_app_token.as_deref() {
        items.push(json!({
            "name": "项目日志",
            "result": probe_value(add_chat_url_tab(api, chat_id, "项目日志", &base_url(api, app_token)).await),
        }));
    }
    json!({ "status": "attempted", "items": items })
}

async fn add_chat_url_tab(
    api: &mut FeishuClient,
    chat_id: &str,
    name: &str,
    url: &str,
) -> Result<Value> {
    let path = format!("/im/v1/chats/{chat_id}/chat_tabs");
    api.post_json(
        &path,
        &[],
        json!({
            "chat_tabs": [{
                "tab_name": name,
                "tab_type": "url",
                "tab_content": { "url": url },
                "tab_config": { "is_built_in": true },
            }]
        }),
    )
    .await
}

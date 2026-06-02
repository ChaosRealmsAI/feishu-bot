use super::*;

pub(in crate::app::office) async fn send_office_summary(
    api: &mut FeishuClient,
    project: &mut OfficeProject,
) -> Result<Value> {
    let chat_id = required_project_field(project.chat_id.as_deref(), &project.project, "chat_id")?;
    let mut lines = vec![
        format!("{} 项目空间已初始化", project.name),
        "后续 AI 汇报会按项目独立写入这个群聊。".to_string(),
    ];
    if let Some(node_token) = project.wiki_index_node_token.as_deref() {
        lines.push(format!("Wiki：{}", wiki_url(api, node_token)));
    }
    if let Some(app_token) = project.base_app_token.as_deref() {
        lines.push(format!("Base：{}", base_url(api, app_token)));
    }
    let sent = api
        .send_text(chat_id, "chat_id", &lines.join("\n"), None)
        .await?;
    let message_id = extract_message_id(&sent);
    let pin = pin_message(api, message_id.as_deref()).await;
    project.pinned_summary_message_id = message_id.clone();
    Ok(json!({
        "sent": sent,
        "message_id": message_id,
        "pin": pin,
        "message_get": readback_message(api, message_id.as_deref()).await,
    }))
}

pub(in crate::app::office) async fn pin_message(
    api: &mut FeishuClient,
    message_id: Option<&str>,
) -> Value {
    let Some(message_id) = message_id else {
        return json!({ "ok": false, "error": "missing message_id" });
    };
    probe_value(
        api.post_json("/im/v1/pins", &[], json!({ "message_id": message_id }))
            .await,
    )
}

use super::*;

pub(in crate::app::office) async fn ensure_office_chat(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
) -> Result<Value> {
    if let Some(chat_id) = args
        .chat_id
        .clone()
        .or_else(|| project.chat_id.clone())
        .filter(|value| !value.trim().is_empty())
    {
        project.chat_id = Some(chat_id.clone());
        return Ok(json!({
            "status": "reused",
            "chat_id": chat_id,
            "readback": readback_chat(api, project.chat_id.as_deref()).await,
        }));
    }

    let mut users = args.users.clone();
    if users.is_empty() {
        if let Some(default_user_id) = api.config.default_user_id.clone() {
            users.push(default_user_id);
        }
    }
    if users.is_empty() {
        bail!("office bootstrap needs --user or FEISHU_USER_ID to create a project chat");
    }
    let user_type = args.user_id_type.resolve(users.first().map(String::as_str));
    let mut body = json!({
        "name": project.name,
        "description": format!("feishu-bot office project chat: {}", project.name),
        "chat_mode": "group",
        "chat_type": "private",
        "group_message_type": "chat",
        "user_id_list": users,
    });
    if let Some(path) = args.avatar_file.as_ref() {
        let uploaded = api.upload_im_image(path, "avatar").await?;
        if let Some(image_key) = get_string(&uploaded, &["data", "image_key"]) {
            body["avatar"] = Value::String(image_key);
        }
    }
    let created = api
        .post_json(
            "/im/v1/chats",
            &[("user_id_type".to_string(), user_type.to_string())],
            body,
        )
        .await?;
    let chat_id = extract_chat_id(&created)
        .ok_or_else(|| anyhow!("create chat response missing chat_id: {created}"))?;
    project.chat_id = Some(chat_id.clone());
    Ok(json!({
        "status": "created",
        "chat_id": chat_id,
        "create_response": created,
    }))
}

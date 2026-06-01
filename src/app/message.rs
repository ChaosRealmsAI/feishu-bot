use super::*;

mod poll;
mod voice;

pub(super) use poll::*;
pub(super) use voice::*;

pub(super) async fn run_message_command(
    api: &mut FeishuClient,
    command: MessageCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        MessageCommand::List(args) => {
            let mut query = vec![
                ("container_id".to_string(), args.container_id),
                ("container_id_type".to_string(), args.container_id_type),
                ("sort_type".to_string(), args.sort_type),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "start_time", args.start_time);
            push_query_opt(&mut query, "end_time", args.end_time);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/im/v1/messages", &query).await?
        }
        MessageCommand::Get(args) => {
            let path = format!("/im/v1/messages/{}", args.message_id);
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            if args.user_card_content {
                query.push((
                    "card_msg_content_type".to_string(),
                    "user_card_content".to_string(),
                ));
            }
            api.get_json(&path, &query).await?
        }
        MessageCommand::Send(args) => {
            let text = read_content(args.text, args.file, args.stdin)?;
            let receive_id_type = args.to_type.resolve(&args.to);
            api.send_text(&args.to, receive_id_type, &text, args.uuid.as_deref())
                .await?
        }
        MessageCommand::LoopCheck(args) => run_message_loop_check(api, args).await?,
        MessageCommand::SendJson(args) => {
            let content = read_message_content_json(args.content_json, args.file, args.stdin)?;
            api.send_message_json(
                &args.to,
                args.to_type.resolve(&args.to),
                &args.msg_type,
                content,
                args.uuid.as_deref(),
            )
            .await?
        }
        MessageCommand::UploadImage(args) => {
            api.upload_im_image(&args.file, &args.image_type).await?
        }
        MessageCommand::UploadFile(args) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            api.upload_im_file(&args.file, file_name, &args.file_type, args.duration)
                .await?
        }
        MessageCommand::SendImage(args) => {
            let uploaded = api.upload_im_image(&args.file, &args.image_type).await?;
            let image_key = get_string(&uploaded, &["data", "image_key"])
                .ok_or_else(|| anyhow!("upload image response missing image_key: {uploaded}"))?;
            api.send_message_json(
                &args.to,
                args.to_type.resolve(&args.to),
                "image",
                json!({ "image_key": image_key }),
                args.uuid.as_deref(),
            )
            .await?
        }
        MessageCommand::SendFile(args) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            let msg_type = resolve_upload_message_type(&args.file_type, &args.msg_type)?;
            let uploaded = api
                .upload_im_file(
                    &args.file,
                    file_name.clone(),
                    &args.file_type,
                    args.duration,
                )
                .await?;
            let file_key = get_string(&uploaded, &["data", "file_key"])
                .ok_or_else(|| anyhow!("upload file response missing file_key: {uploaded}"))?;
            let content = build_uploaded_file_message_content(
                &file_key,
                &file_name,
                msg_type,
                args.duration,
                args.cover_image_key,
            );
            api.send_message_json(
                &args.to,
                args.to_type.resolve(&args.to),
                msg_type,
                content,
                args.uuid.as_deref(),
            )
            .await?
        }
        MessageCommand::SendVoice(args) => run_message_send_voice(api, args).await?,
        MessageCommand::DownloadImage(args) => {
            let bytes = api.download_im_image(&args.image_key).await?;
            write_output_file(&args.output, &bytes)?;
            json!({ "code": 0, "msg": "success", "data": { "output": args.output.display().to_string(), "bytes": bytes.len() } })
        }
        MessageCommand::DownloadFile(args) => {
            let bytes = api.download_im_file(&args.file_key).await?;
            write_output_file(&args.output, &bytes)?;
            json!({ "code": 0, "msg": "success", "data": { "output": args.output.display().to_string(), "bytes": bytes.len() } })
        }
        MessageCommand::ReplyJson(args) => {
            let content = read_message_content_json(args.content_json, args.file, args.stdin)?;
            api.reply_message_json(
                &args.message_id,
                &args.msg_type,
                content,
                args.uuid.as_deref(),
            )
            .await?
        }
        MessageCommand::Reply(args) => run_message_reply(api, args).await?,
        MessageCommand::Ack(args) => run_message_ack(api, args).await?,
        MessageCommand::Poll(args) => run_message_poll(api, args).await?,
        MessageCommand::EditJson(args) => {
            let content = read_message_content_json(args.content_json, args.file, args.stdin)?;
            api.edit_message_json(&args.message_id, &args.msg_type, content)
                .await?
        }
        MessageCommand::Delete(args) => api.delete_message(&args.message_id).await?,
        MessageCommand::ReadUsers(args) => {
            let path = format!("/im/v1/messages/{}/read_users", args.message_id);
            let mut query = vec![
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        MessageCommand::Resource(args) => {
            let bytes = api
                .download_message_resource(&args.message_id, &args.file_key, &args.resource_type)
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({ "output": args.output.display().to_string(), "bytes": bytes.len() })
        }
        MessageCommand::Reaction(MessageReactionCommand::List(args)) => {
            let path = format!("/im/v1/messages/{}/reactions", args.message_id);
            let mut query = vec![
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "reaction_type", args.reaction_type);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        MessageCommand::Reaction(MessageReactionCommand::Add(args)) => {
            let path = format!("/im/v1/messages/{}/reactions", args.message_id);
            let body = build_reaction_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        MessageCommand::Reaction(MessageReactionCommand::Delete(args)) => {
            let path = format!(
                "/im/v1/messages/{}/reactions/{}",
                args.message_id, args.reaction_id
            );
            api.delete_json(&path, &[], None).await?
        }
        MessageCommand::Pin(MessagePinCommand::List(args)) => {
            let mut query = vec![
                ("chat_id".to_string(), args.chat_id),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "start_time", args.start_time);
            push_query_opt(&mut query, "end_time", args.end_time);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/im/v1/pins", &query).await?
        }
        MessageCommand::Pin(MessagePinCommand::Add(args)) => {
            api.post_json("/im/v1/pins", &[], json!({ "message_id": args.message_id }))
                .await?
        }
        MessageCommand::Pin(MessagePinCommand::Delete(args)) => {
            let path = format!("/im/v1/pins/{}", args.message_id);
            api.delete_json(&path, &[], None).await?
        }
    };
    print_response(raw_json, "message operation completed", data)
}

async fn run_message_reply(api: &mut FeishuClient, args: MessageReplyArgs) -> Result<Value> {
    let text = read_content(args.text, args.file, args.stdin)?;
    let content = message_text_content(&text);
    let replied = api
        .reply_message_json(&args.message_id, "text", content, args.uuid.as_deref())
        .await?;
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "message_id": args.message_id,
            "text": text,
            "reply": replied,
        }
    }))
}

async fn run_message_ack(api: &mut FeishuClient, args: MessageAckArgs) -> Result<Value> {
    let reaction_path = format!(
        "/im/v1/messages/{}/reactions",
        encode_path_segment(&args.message_id)
    );
    let reaction = api
        .post_json(
            &reaction_path,
            &[],
            json!({ "reaction_type": { "emoji_type": args.emoji_type } }),
        )
        .await?;

    let reply_text = read_optional_content(args.reply_text, args.reply_file, args.reply_stdin)?;
    let reply = if let Some(text) = reply_text.filter(|text| !text.trim().is_empty()) {
        Some(
            api.reply_message_json(
                &args.message_id,
                "text",
                message_text_content(&text),
                args.uuid.as_deref(),
            )
            .await?,
        )
    } else {
        None
    };

    let (message_get, reactions) = if args.readback {
        let message_path = format!("/im/v1/messages/{}", encode_path_segment(&args.message_id));
        let message_get = probe_value(
            api.get_json(
                &message_path,
                &[("user_id_type".to_string(), "open_id".to_string())],
            )
            .await,
        );
        let reactions = probe_value(
            api.get_json(
                &reaction_path,
                &[
                    ("user_id_type".to_string(), "open_id".to_string()),
                    ("page_size".to_string(), "50".to_string()),
                ],
            )
            .await,
        );
        (Some(message_get), Some(reactions))
    } else {
        (None, None)
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "message_id": args.message_id,
            "status": args.status,
            "emoji_type": args.emoji_type,
            "status_semantics": "workflow_status_reaction_not_official_read_receipt",
            "reaction": reaction,
            "reply": reply,
            "message_get": message_get,
            "reactions": reactions,
        }
    }))
}

async fn run_message_loop_check(
    api: &mut FeishuClient,
    args: MessageLoopCheckArgs,
) -> Result<Value> {
    let generated = format!(
        "飞书Bot闭环测试 cli-loop-{}\n时间 {}\n如果你看到这条，说明 message loop-check 到当前账号可见。",
        Local::now().format("%Y%m%d%H%M%S"),
        Local::now().format("%Y-%m-%d %H:%M:%S %:z")
    );
    let text = if args.text.is_none() && args.file.is_none() && !args.stdin {
        generated
    } else {
        read_content(args.text, args.file, args.stdin)?
    };
    let receive_id_type = args.to_type.resolve(&args.to).to_string();
    let sent = api
        .send_text(&args.to, &receive_id_type, &text, args.uuid.as_deref())
        .await?;
    let proof = probe_sent_text_message(api, &args.to, &sent, &text).await?;
    let message_id = get_string(&proof, &["message_id"])
        .ok_or_else(|| anyhow!("loop-check proof missing message_id: {proof}"))?;
    let chat_id = get_string(&proof, &["chat_id"])
        .ok_or_else(|| anyhow!("loop-check proof missing chat_id: {proof}"))?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "receive_id": args.to,
            "receive_id_type": receive_id_type,
            "message_id": message_id,
            "chat_id": chat_id,
            "text": text,
            "closed_loop": proof.get("closed_loop").cloned().unwrap_or(Value::Null),
            "sent": sent,
            "message_get": proof.get("message_get").cloned().unwrap_or(Value::Null),
            "message_list": proof.get("message_list").cloned().unwrap_or(Value::Null),
            "chat_get": proof.get("chat_get").cloned().unwrap_or(Value::Null),
            "chat_members": proof.get("chat_members").cloned().unwrap_or(Value::Null),
            "read_users": proof.get("read_users").cloned().unwrap_or(Value::Null),
        }
    }))
}

pub(super) async fn probe_sent_text_message(
    api: &mut FeishuClient,
    receive_id: &str,
    sent: &Value,
    expected_text: &str,
) -> Result<Value> {
    let message_id = get_string(sent, &["data", "message_id"])
        .ok_or_else(|| anyhow!("send response missing message_id: {sent}"))?;
    let chat_id = get_string(sent, &["data", "chat_id"])
        .ok_or_else(|| anyhow!("send response missing chat_id: {sent}"))?;

    let message_get_path = format!("/im/v1/messages/{}", encode_path_segment(&message_id));
    let message_get = api
        .get_json(
            &message_get_path,
            &[("user_id_type".to_string(), "open_id".to_string())],
        )
        .await;
    let message_list = api
        .get_json(
            "/im/v1/messages",
            &[
                ("container_id".to_string(), chat_id.clone()),
                ("container_id_type".to_string(), "chat".to_string()),
                ("sort_type".to_string(), "ByCreateTimeDesc".to_string()),
                ("page_size".to_string(), "5".to_string()),
            ],
        )
        .await;
    let chat_get_path = format!("/im/v1/chats/{}", encode_path_segment(&chat_id));
    let chat_get = api.get_json(&chat_get_path, &[]).await;
    let chat_members_path = format!("/im/v1/chats/{}/members", encode_path_segment(&chat_id));
    let chat_members = api
        .get_json(
            &chat_members_path,
            &[
                ("member_id_type".to_string(), "open_id".to_string()),
                ("page_size".to_string(), "20".to_string()),
            ],
        )
        .await;
    let read_users_path = format!(
        "/im/v1/messages/{}/read_users",
        encode_path_segment(&message_id)
    );
    let read_users = api
        .get_json(
            &read_users_path,
            &[
                ("user_id_type".to_string(), "open_id".to_string()),
                ("page_size".to_string(), "20".to_string()),
            ],
        )
        .await;

    let message_get = probe_value(message_get);
    let message_list = probe_value(message_list);
    let chat_get = probe_value(chat_get);
    let chat_members = probe_value(chat_members);
    let read_users = probe_value(read_users);
    let message_get_contains_text = response_contains_multiline_text(&message_get, expected_text);
    let message_list_contains_message_id = response_contains(&message_list, &message_id);
    let chat_owner_matches_target = get_string(&chat_get, &["response", "data", "owner_id"])
        .is_some_and(|owner| owner == receive_id);
    let chat_members_contains_target = response_contains(&chat_members, receive_id);
    let read_users_count = read_users
        .pointer("/response/data/items")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);

    Ok(json!({
        "message_id": message_id,
        "chat_id": chat_id,
        "closed_loop": {
            "send_ok": true,
            "message_get_ok": message_get.get("ok").and_then(Value::as_bool).unwrap_or(false),
            "message_get_contains_text": message_get_contains_text,
            "message_list_ok": message_list.get("ok").and_then(Value::as_bool).unwrap_or(false),
            "message_list_contains_message_id": message_list_contains_message_id,
            "chat_get_ok": chat_get.get("ok").and_then(Value::as_bool).unwrap_or(false),
            "chat_owner_matches_receive_id": chat_owner_matches_target,
            "chat_members_ok": chat_members.get("ok").and_then(Value::as_bool).unwrap_or(false),
            "chat_members_contains_receive_id": chat_members_contains_target,
            "read_users_ok": read_users.get("ok").and_then(Value::as_bool).unwrap_or(false),
            "read_users_count": read_users_count,
        },
        "message_get": message_get,
        "message_list": message_list,
        "chat_get": chat_get,
        "chat_members": chat_members,
        "read_users": read_users,
    }))
}

pub(super) fn probe_value(result: Result<Value>) -> Value {
    match result {
        Ok(response) => json!({ "ok": true, "response": response }),
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

pub(super) fn response_contains(value: &Value, needle: &str) -> bool {
    serde_json::to_string(value).is_ok_and(|text| text.contains(needle))
}

pub(super) fn response_contains_multiline_text(value: &Value, needle: &str) -> bool {
    needle
        .lines()
        .filter(|line| !line.trim().is_empty())
        .all(|line| response_contains(value, line))
}

fn read_message_content_json(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    ensure_json_object(read_json_value(text, file, stdin)?, "message content")
}

pub(super) fn message_text_content(text: &str) -> Value {
    json!({ "text": text })
}

pub(super) fn resolve_upload_message_type<'a>(
    file_type: &str,
    msg_type: &'a str,
) -> Result<&'a str> {
    let normalized = msg_type.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "auto" => match file_type.trim().to_ascii_lowercase().as_str() {
            "mp4" => Ok("media"),
            "opus" => Ok("audio"),
            _ => Ok("file"),
        },
        "file" | "media" | "audio" => Ok(match normalized.as_str() {
            "file" => "file",
            "media" => "media",
            "audio" => "audio",
            _ => unreachable!(),
        }),
        _ => bail!("message send-file --msg-type must be auto, file, media, or audio"),
    }
}

pub(super) fn build_uploaded_file_message_content(
    file_key: &str,
    file_name: &str,
    msg_type: &str,
    duration: Option<u64>,
    cover_image_key: Option<String>,
) -> Value {
    match msg_type {
        "media" => {
            let mut body = Map::new();
            body.insert("file_key".to_string(), Value::String(file_key.to_string()));
            insert_opt_string(&mut body, "image_key", cover_image_key);
            Value::Object(body)
        }
        "audio" => {
            let mut body = Map::new();
            body.insert("file_key".to_string(), Value::String(file_key.to_string()));
            if let Some(duration) = duration {
                body.insert("duration".to_string(), Value::Number(duration.into()));
            }
            Value::Object(body)
        }
        _ => json!({
            "file_key": file_key,
            "file_name": file_name
        }),
    }
}

pub(super) fn build_reaction_body(args: MessageReactionAddArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        let value = read_json_value(args.body_json, args.file, args.stdin)?;
        if value.get("reaction_type").is_some() {
            return ensure_json_object(value, "reaction body");
        }
        return Ok(json!({ "reaction_type": ensure_json_object(value, "reaction_type")? }));
    }
    let emoji_type = args
        .emoji_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("reaction add needs --emoji-type or raw JSON"))?;
    Ok(json!({ "reaction_type": { "emoji_type": emoji_type } }))
}

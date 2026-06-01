use super::*;

mod content;
mod poll;
mod probe;
mod voice;

pub(super) use content::*;
pub(super) use poll::*;
pub(super) use probe::*;
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

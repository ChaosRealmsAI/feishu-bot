use super::*;

pub(in crate::app) fn read_message_content_json(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    ensure_json_object(read_json_value(text, file, stdin)?, "message content")
}

pub(in crate::app) fn message_text_content(text: &str) -> Value {
    json!({ "text": text })
}

pub(in crate::app) fn resolve_upload_message_type<'a>(
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

pub(in crate::app) fn build_uploaded_file_message_content(
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

pub(in crate::app) fn build_reaction_body(args: MessageReactionAddArgs) -> Result<Value> {
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

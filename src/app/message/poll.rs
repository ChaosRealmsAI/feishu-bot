use super::*;

pub(in crate::app) async fn run_message_poll(
    api: &mut FeishuClient,
    args: MessagePollArgs,
) -> Result<Value> {
    let state_file = args
        .state_file
        .clone()
        .unwrap_or_else(default_message_state_path);
    let state_key = args
        .state_key
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| args.chat_id.clone());
    let mut state = read_message_state(&state_file)?;
    let state_cursor = state_cursor_position(&state, &state_key);
    let previous_cursor = args.since_position.or(state_cursor);

    let listed = api
        .get_json(
            "/im/v1/messages",
            &[
                ("container_id".to_string(), args.chat_id.clone()),
                ("container_id_type".to_string(), "chat".to_string()),
                ("sort_type".to_string(), "ByCreateTimeDesc".to_string()),
                ("page_size".to_string(), args.page_size.to_string()),
            ],
        )
        .await?;
    let fetched_items = listed
        .pointer("/data/items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let latest_cursor = latest_message_cursor(&fetched_items);

    if previous_cursor.is_none() && args.from_now {
        let saved = if let Some((position, message_id)) = latest_cursor.as_ref() {
            save_message_state_cursor(
                &mut state,
                &state_key,
                &args.chat_id,
                *position,
                message_id.clone(),
            );
            write_message_state(&state_file, &state)?;
            true
        } else {
            false
        };
        return Ok(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "chat_id": args.chat_id,
                "state_file": state_file.display().to_string(),
                "state_key": state_key,
                "previous_cursor": previous_cursor,
                "latest_cursor": cursor_json(latest_cursor),
                "from_now": true,
                "new_count": 0,
                "items": [],
                "actions": [],
                "cursor_saved": saved,
                "list_summary": message_list_summary(&listed, fetched_items.len()),
            }
        }));
    }

    let new_items = filter_poll_items(
        &fetched_items,
        previous_cursor,
        args.include_app_messages,
        args.include_system_messages,
    );
    let mut actions = Vec::new();
    for item in &new_items {
        let Some(message_id) = message_id_of(item) else {
            continue;
        };
        if let Some(emoji_type) = args
            .ack_emoji
            .as_ref()
            .filter(|value| !value.trim().is_empty())
        {
            let path = format!(
                "/im/v1/messages/{}/reactions",
                encode_path_segment(&message_id)
            );
            let response = probe_value(
                api.post_json(
                    &path,
                    &[],
                    json!({ "reaction_type": { "emoji_type": emoji_type } }),
                )
                .await,
            );
            actions.push(json!({
                "message_id": message_id,
                "action": "reaction",
                "emoji_type": emoji_type,
                "result": response,
            }));
        }
        if let Some(reply_text) = args
            .reply_text
            .as_ref()
            .filter(|value| !value.trim().is_empty())
        {
            let response = probe_value(
                api.reply_message_json(&message_id, "text", message_text_content(reply_text), None)
                    .await,
            );
            actions.push(json!({
                "message_id": message_id,
                "action": "reply",
                "result": response,
            }));
        }
    }

    let should_save_cursor = args.mark_seen || !actions.is_empty();
    let cursor_saved = if should_save_cursor {
        if let Some((position, message_id)) = latest_cursor.as_ref() {
            save_message_state_cursor(
                &mut state,
                &state_key,
                &args.chat_id,
                *position,
                message_id.clone(),
            );
            write_message_state(&state_file, &state)?;
            true
        } else {
            false
        }
    } else {
        false
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "chat_id": args.chat_id,
            "state_file": state_file.display().to_string(),
            "state_key": state_key,
            "previous_cursor": previous_cursor,
            "latest_cursor": cursor_json(latest_cursor),
            "new_count": new_items.len(),
            "items": new_items,
            "actions": actions,
            "cursor_saved": cursor_saved,
            "list_summary": message_list_summary(&listed, fetched_items.len()),
        }
    }))
}

fn default_message_state_path() -> PathBuf {
    dirs::config_dir()
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("feishu")
        .join("message-state.json")
}

fn read_message_state(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(json!({ "chats": {} }));
    }
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let value = serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))?;
    Ok(value)
}

fn write_message_state(path: &Path, state: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(state).context("serialize message state")?;
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

fn state_cursor_position(state: &Value, state_key: &str) -> Option<u64> {
    state
        .get("chats")
        .and_then(Value::as_object)
        .and_then(|chats| chats.get(state_key))
        .and_then(|chat| chat.get("last_message_position"))
        .and_then(value_as_u64)
}

fn save_message_state_cursor(
    state: &mut Value,
    state_key: &str,
    chat_id: &str,
    position: u64,
    message_id: String,
) {
    let Some(object) = ensure_object_map(state) else {
        return;
    };
    let chats = object.entry("chats").or_insert_with(|| json!({}));
    let Some(chats) = ensure_object_map(chats) else {
        return;
    };
    chats.insert(
        state_key.to_string(),
        json!({
            "chat_id": chat_id,
            "last_message_position": position,
            "last_message_id": message_id,
            "updated_at": Local::now().to_rfc3339(),
        }),
    );
}

fn ensure_object_map(value: &mut Value) -> Option<&mut Map<String, Value>> {
    if !value.is_object() {
        *value = json!({});
    }
    value.as_object_mut()
}

fn cursor_json(cursor: Option<(u64, String)>) -> Value {
    cursor
        .map(|(position, message_id)| {
            json!({
                "message_position": position,
                "message_id": message_id,
            })
        })
        .unwrap_or(Value::Null)
}

fn message_list_summary(listed: &Value, fetched_count: usize) -> Value {
    json!({
        "fetched_count": fetched_count,
        "has_more": listed.pointer("/data/has_more").and_then(Value::as_bool).unwrap_or(false),
        "page_token": listed.pointer("/data/page_token").and_then(Value::as_str).unwrap_or(""),
    })
}

pub(in crate::app) fn message_position(value: &Value) -> Option<u64> {
    value.get("message_position").and_then(value_as_u64)
}

fn value_as_u64(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse::<u64>().ok()))
}

pub(in crate::app) fn message_id_of(value: &Value) -> Option<String> {
    value
        .get("message_id")
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn message_sender_type(value: &Value) -> Option<String> {
    value
        .get("sender")
        .and_then(|sender| sender.get("sender_type"))
        .and_then(Value::as_str)
        .map(|text| text.to_ascii_lowercase())
}

fn message_msg_type(value: &Value) -> Option<String> {
    value
        .get("msg_type")
        .and_then(Value::as_str)
        .map(|text| text.to_ascii_lowercase())
}

pub(in crate::app) fn latest_message_cursor(items: &[Value]) -> Option<(u64, String)> {
    items
        .iter()
        .filter_map(|item| Some((message_position(item)?, message_id_of(item)?)))
        .max_by_key(|(position, _)| *position)
}

pub(in crate::app) fn filter_poll_items(
    items: &[Value],
    cursor: Option<u64>,
    include_app_messages: bool,
    include_system_messages: bool,
) -> Vec<Value> {
    let mut filtered = items
        .iter()
        .filter(|item| match (message_position(item), cursor) {
            (Some(position), Some(cursor)) => position > cursor,
            (Some(_), None) => true,
            (None, _) => false,
        })
        .filter(|item| {
            include_app_messages
                || !matches!(message_sender_type(item).as_deref(), Some("app" | "bot"))
        })
        .filter(|item| {
            include_system_messages || message_msg_type(item).as_deref() != Some("system")
        })
        .cloned()
        .collect::<Vec<_>>();
    filtered.sort_by_key(message_position);
    filtered
}

use super::*;

pub(super) async fn run_message_loop_check(
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

pub(in crate::app) async fn probe_sent_text_message(
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

pub(in crate::app) fn probe_value(result: Result<Value>) -> Value {
    match result {
        Ok(response) => json!({ "ok": true, "response": response }),
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

pub(in crate::app) fn response_contains(value: &Value, needle: &str) -> bool {
    serde_json::to_string(value).is_ok_and(|text| text.contains(needle))
}

pub(in crate::app) fn response_contains_multiline_text(value: &Value, needle: &str) -> bool {
    needle
        .lines()
        .filter(|line| !line.trim().is_empty())
        .all(|line| response_contains(value, line))
}

use super::*;

pub(super) async fn run_bot_command(
    api: &mut FeishuClient,
    command: BotCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        BotCommand::Info => {
            let response = api.get_json("/bot/v3/info", &[]).await?;
            normalize_bot_info_response(response)
        }
    };
    print_response(raw_json, "bot operation completed", data)
}

pub(super) fn normalize_bot_info_response(response: Value) -> Value {
    let bot = response
        .get("bot")
        .cloned()
        .or_else(|| response.pointer("/data/bot").cloned())
        .unwrap_or(Value::Null);
    let open_id = get_string(&response, &["bot", "open_id"])
        .or_else(|| get_string(&response, &["data", "bot", "open_id"]));

    let mut data = Map::new();
    data.insert("bot".to_string(), bot);
    if let Some(open_id) = open_id {
        data.insert("open_id".to_string(), Value::String(open_id.clone()));
        data.insert(
            "wiki_member_add_example".to_string(),
            Value::String(format!(
                "feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id {open_id} --member-role admin"
            )),
        );
    }

    json!({
        "code": response.get("code").cloned().unwrap_or(Value::Number(0.into())),
        "msg": response.get("msg").cloned().unwrap_or_else(|| Value::String("success".to_string())),
        "data": data,
        "raw": response,
    })
}

use super::*;

pub(in crate::app) fn build_chat_create_body(args: &ChatCreateArgs) -> Result<Value> {
    if let Some(mut body) = read_raw_object(
        args.body_json.clone(),
        args.body_file.clone(),
        args.stdin,
        "chat create body",
    )? {
        insert_opt_string_value(&mut body, "avatar", args.avatar.clone());
        return Ok(body);
    }

    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(args.name.clone()));
    body.insert("chat_mode".to_string(), Value::String("group".to_string()));
    body.insert(
        "chat_type".to_string(),
        Value::String(args.chat_type.clone()),
    );
    body.insert(
        "group_message_type".to_string(),
        Value::String(args.group_message_type.clone()),
    );
    insert_opt_string(&mut body, "description", args.description.clone());
    insert_opt_string(&mut body, "avatar", args.avatar.clone());
    insert_opt_string(&mut body, "owner_id", args.owner_id.clone());
    insert_string_list(&mut body, "user_id_list", &args.users);
    insert_string_list(&mut body, "bot_id_list", &args.bots);
    Ok(Value::Object(body))
}

pub(super) fn build_chat_update_body(args: &ChatUpdateArgs) -> Result<Value> {
    if let Some(mut body) = read_raw_object(
        args.body_json.clone(),
        args.body_file.clone(),
        args.stdin,
        "chat update body",
    )? {
        insert_opt_string_value(&mut body, "avatar", args.avatar.clone());
        return Ok(body);
    }

    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name.clone());
    insert_opt_string(&mut body, "description", args.description.clone());
    insert_opt_string(&mut body, "avatar", args.avatar.clone());
    insert_opt_string(&mut body, "owner_id", args.owner_id.clone());
    insert_opt_string(&mut body, "chat_type", args.chat_type.clone());
    insert_opt_string(
        &mut body,
        "group_message_type",
        args.group_message_type.clone(),
    );
    insert_opt_string(
        &mut body,
        "add_member_permission",
        args.add_member_permission.clone(),
    );
    insert_opt_string(
        &mut body,
        "share_card_permission",
        args.share_card_permission.clone(),
    );
    insert_opt_string(
        &mut body,
        "at_all_permission",
        args.at_all_permission.clone(),
    );
    insert_opt_string(&mut body, "edit_permission", args.edit_permission.clone());
    insert_opt_string(
        &mut body,
        "membership_approval",
        args.membership_approval.clone(),
    );
    insert_opt_string(
        &mut body,
        "join_message_visibility",
        args.join_message_visibility.clone(),
    );
    insert_opt_string(
        &mut body,
        "leave_message_visibility",
        args.leave_message_visibility.clone(),
    );
    Ok(Value::Object(body))
}

pub(super) async fn insert_uploaded_avatar(
    api: &mut FeishuClient,
    file: Option<&PathBuf>,
    body: &mut Value,
) -> Result<()> {
    let Some(file) = file else {
        return Ok(());
    };
    let object = body
        .as_object_mut()
        .ok_or_else(|| anyhow!("chat body must be a JSON object"))?;
    if object.get("avatar").is_some() {
        return Ok(());
    }
    let uploaded = api.upload_im_image(file, "avatar").await?;
    let image_key = get_string(&uploaded, &["data", "image_key"])
        .ok_or_else(|| anyhow!("avatar upload response missing image_key: {uploaded}"))?;
    object.insert("avatar".to_string(), Value::String(image_key));
    Ok(())
}

pub(super) async fn upload_chat_tab_icon(
    api: &mut FeishuClient,
    file: Option<&PathBuf>,
) -> Result<Option<String>> {
    let Some(file) = file else {
        return Ok(None);
    };
    let uploaded = api.upload_im_image(file, "message").await?;
    let image_key = get_string(&uploaded, &["data", "image_key"])
        .ok_or_else(|| anyhow!("tab icon upload response missing image_key: {uploaded}"))?;
    Ok(Some(image_key))
}

pub(in crate::app) fn build_chat_tab_body(
    args: &ChatTabWriteArgs,
    is_update: bool,
    uploaded_icon_key: Option<String>,
) -> Result<Value> {
    if let Some(value) =
        read_optional_json_value(args.body_json.clone(), args.body_file.clone(), args.stdin)?
    {
        return wrap_json_body(value, "chat_tabs", "chat tab body");
    }

    if is_update
        && args
            .tab_id
            .as_ref()
            .is_none_or(|value| value.trim().is_empty())
    {
        bail!("chat tab update needs --tab-id unless raw body JSON is used");
    }

    let tab_type = args.tab_type.trim().to_ascii_lowercase();
    let mut tab = Map::new();
    insert_opt_string(&mut tab, "tab_id", args.tab_id.clone());
    insert_opt_string(&mut tab, "tab_name", args.name.clone());
    tab.insert("tab_type".to_string(), Value::String(tab_type.clone()));

    let mut content = Map::new();
    match tab_type.as_str() {
        "url" => {
            let url = required_nonempty(args.url.as_deref(), "chat tab url needs --url")?;
            content.insert("url".to_string(), Value::String(url.to_string()));
        }
        "doc" => {
            let doc = required_nonempty(args.doc.as_deref(), "chat tab doc needs --doc")?;
            content.insert("doc".to_string(), Value::String(doc.to_string()));
        }
        _ => bail!("chat tab typed builder only supports --tab-type url or doc; use --body-json for official raw tabs"),
    }
    tab.insert("tab_content".to_string(), Value::Object(content));

    let icon_key = uploaded_icon_key.or_else(|| args.icon_key.clone());
    if icon_key.is_some() || args.built_in {
        let mut config = Map::new();
        insert_opt_string(&mut config, "icon_key", icon_key);
        config.insert("is_built_in".to_string(), Value::Bool(args.built_in));
        tab.insert("tab_config".to_string(), Value::Object(config));
    }

    Ok(json!({ "chat_tabs": [Value::Object(tab)] }))
}

pub(super) fn build_chat_menu_add_body(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let value = read_json_value(text, file, stdin)?;
    if value.get("menu_tree").is_some() {
        return ensure_json_object(value, "chat menu add body");
    }
    Ok(json!({ "menu_tree": ensure_json_object(value, "menu_tree")? }))
}

pub(super) fn build_repeated_ids_body(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    key: &str,
    label: &str,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        return wrap_json_body(value, key, label);
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("{label} needs repeated --id/--tab-id values or raw body JSON");
    }
    Ok(json!({ key: ids }))
}

pub(in crate::app) fn build_chat_members_body(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        if value.get("id_list").is_some() {
            return ensure_json_object(value, "chat member body");
        }
        return Ok(json!({ "id_list": ensure_json_array(value, "id_list")? }));
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("chat member add/delete needs --id, --body-json, --file, or --stdin");
    }
    Ok(json!({
        "id_list": ids.into_iter().map(Value::String).collect::<Vec<_>>()
    }))
}

pub(in crate::app) fn chat_member_query(
    member_id_type: ChatMemberIdTypeArg,
    succeed_type: u8,
) -> Vec<(String, String)> {
    vec![
        (
            "member_id_type".to_string(),
            member_id_type.as_api_value().to_string(),
        ),
        ("succeed_type".to_string(), succeed_type.to_string()),
    ]
}

fn read_raw_object(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    label: &str,
) -> Result<Option<Value>> {
    read_optional_json_value(text, file, stdin)?
        .map(|value| ensure_json_object(value, label))
        .transpose()
}

fn wrap_json_body(value: Value, key: &str, label: &str) -> Result<Value> {
    if value.get(key).is_some() {
        return ensure_json_object(value, label);
    }
    if key == "chat_tabs" && value.is_object() {
        return Ok(json!({ key: [value] }));
    }
    if value.is_array() {
        return Ok(json!({ key: ensure_json_array(value, key)? }));
    }
    Ok(json!({ key: ensure_json_array(value, key)? }))
}

fn insert_opt_string_value(body: &mut Value, key: &str, value: Option<String>) {
    if let (Some(object), Some(value)) = (body.as_object_mut(), value) {
        if !value.trim().is_empty() && object.get(key).is_none() {
            object.insert(key.to_string(), Value::String(value));
        }
    }
}

fn insert_string_list(body: &mut Map<String, Value>, key: &str, values: &[String]) {
    let values = values
        .iter()
        .filter(|value| !value.trim().is_empty())
        .map(|value| Value::String(value.clone()))
        .collect::<Vec<_>>();
    if !values.is_empty() {
        body.insert(key.to_string(), Value::Array(values));
    }
}

fn required_nonempty<'a>(value: Option<&'a str>, message: &str) -> Result<&'a str> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!(message.to_string()))
}

impl ChatMemberIdTypeArg {
    fn as_api_value(self) -> &'static str {
        match self {
            ChatMemberIdTypeArg::OpenId => "open_id",
            ChatMemberIdTypeArg::UnionId => "union_id",
            ChatMemberIdTypeArg::UserId => "user_id",
            ChatMemberIdTypeArg::AppId => "app_id",
        }
    }
}

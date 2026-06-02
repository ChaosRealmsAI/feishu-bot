use super::*;

const PROJECT_LOG_TABLE_NAME: &str = "项目日志";

pub(super) async fn ensure_office_chat(
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

pub(super) async fn ensure_office_wiki_index(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    let Some(space_id) = args
        .space_id
        .clone()
        .or_else(|| project.wiki_space_id.clone())
        .or_else(|| api.config.default_wiki_space_id.clone())
        .filter(|value| !value.trim().is_empty())
    else {
        next_actions.push(
            "Wiki index skipped because no --space-id or FEISHU_WIKI_SPACE_ID is configured."
                .to_string(),
        );
        return Ok(json!({ "status": "skipped_missing_space" }));
    };
    project.wiki_space_id = Some(space_id.clone());
    project.wiki_parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| project.wiki_parent_node_token.clone())
        .or_else(|| api.config.default_wiki_parent_node_token.clone());

    if project.wiki_index_obj_token.is_some() && project.wiki_index_node_token.is_some() {
        return Ok(json!({
            "status": "reused",
            "space_id": space_id,
            "node_token": project.wiki_index_node_token,
            "document_id": project.wiki_index_obj_token,
            "url": project.wiki_index_node_token.as_deref().map(|token| wiki_url(api, token)),
        }));
    }

    let created = match create_wiki_doc(
        api,
        &space_id,
        project.wiki_parent_node_token.as_deref(),
        &format!("{} 项目主页", project.name),
        ContentTypeArg::Markdown,
        &office_index_markdown(project),
        args.auth,
    )
    .await
    {
        Ok(created) => created,
        Err(error) => {
            next_actions.push(format!(
                "Wiki index creation failed but the project chat can still be used. Grant the app/bot edit permission in the Wiki space, or rerun bootstrap with --auth user/--skip-wiki: {error:#}"
            ));
            return Ok(json!({
                "status": "error",
                "space_id": space_id,
                "error": format!("{error:#}"),
            }));
        }
    };
    project.wiki_index_node_token = created.node_token.clone();
    project.wiki_index_obj_token = Some(created.document_id.clone());
    Ok(json!({
        "status": "created",
        "space_id": space_id,
        "node_token": created.node_token,
        "document_id": created.document_id,
        "url": created.url,
        "create_response": created.create_response,
        "append_response": created.append_response,
    }))
}

pub(super) async fn ensure_office_base(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    if project.base_app_token.is_some() && project.base_table_id.is_some() {
        return Ok(json!({
            "status": "reused",
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
        }));
    }

    let mut create_route = "base";
    let mut base_node_token = None;
    let app_token = if let Some(space_id) = project.wiki_space_id.as_deref() {
        let path = format!("/wiki/v2/spaces/{}/nodes", encode_path_segment(space_id));
        let body = json!({
            "obj_type": "bitable",
            "node_type": "origin",
            "title": format!("{} 项目多维表格", project.name),
        });
        match wiki_request_json(api, Method::POST, &path, &[], Some(body), args.auth).await {
            Ok(value) => {
                create_route = "wiki_bitable";
                base_node_token = get_string(&value, &["data", "node", "node_token"]);
                get_string(&value, &["data", "node", "obj_token"])
                    .or_else(|| get_string(&value, &["data", "obj_token"]))
                    .ok_or_else(|| anyhow!("wiki bitable response missing obj_token: {value}"))?
            }
            Err(error) => {
                next_actions.push(format!(
                    "Wiki bitable creation failed; fell back to plain Base create: {error:#}"
                ));
                create_plain_base(api, &project.name).await?
            }
        }
    } else {
        create_plain_base(api, &project.name).await?
    };

    let table = create_project_log_table(api, &app_token).await?;
    let table_id = extract_table_id(&table)
        .ok_or_else(|| anyhow!("project log table create response missing table_id: {table}"))?;
    project.base_node_token = base_node_token;
    project.base_app_token = Some(app_token.clone());
    project.base_table_id = Some(table_id.clone());
    Ok(json!({
        "status": "created",
        "route": create_route,
        "app_token": app_token,
        "node_token": project.base_node_token,
        "table_id": table_id,
        "table_response": table,
    }))
}

pub(super) async fn add_office_tabs(api: &mut FeishuClient, project: &OfficeProject) -> Value {
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

pub(super) async fn send_office_summary(
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

async fn create_plain_base(api: &mut FeishuClient, project_name: &str) -> Result<String> {
    let created = api
        .post_json(
            "/bitable/v1/apps",
            &[],
            json!({ "name": format!("{project_name} 项目多维表格") }),
        )
        .await?;
    get_string(&created, &["data", "app", "app_token"])
        .or_else(|| get_string(&created, &["data", "app_token"]))
        .ok_or_else(|| anyhow!("create Base response missing app_token: {created}"))
}

async fn create_project_log_table(api: &mut FeishuClient, app_token: &str) -> Result<Value> {
    let path = format!("/bitable/v1/apps/{app_token}/tables");
    let fields = ["类型", "标题", "状态", "链接", "摘要", "创建时间"]
        .into_iter()
        .map(|field_name| json!({ "field_name": field_name, "type": 1 }))
        .collect::<Vec<_>>();
    api.post_json(
        &path,
        &[],
        json!({
            "table": {
                "name": PROJECT_LOG_TABLE_NAME,
                "fields": fields,
            }
        }),
    )
    .await
}

pub(super) async fn append_project_base_record(
    api: &mut FeishuClient,
    project: &OfficeProject,
    kind: &str,
    title: &str,
    status: &str,
    url: &str,
    summary: &str,
) -> Value {
    let (Some(app_token), Some(table_id)) = (
        project.base_app_token.as_deref(),
        project.base_table_id.as_deref(),
    ) else {
        return json!({ "ok": false, "error": "project has no base_app_token/base_table_id" });
    };
    let path = format!("/bitable/v1/apps/{app_token}/tables/{table_id}/records");
    probe_value(
        api.post_json(
            &path,
            &[],
            json!({
                "fields": {
                    "类型": kind,
                    "标题": title,
                    "状态": status,
                    "链接": url,
                    "摘要": summary,
                    "创建时间": office_now(),
                }
            }),
        )
        .await,
    )
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

pub(super) async fn pin_message(api: &mut FeishuClient, message_id: Option<&str>) -> Value {
    let Some(message_id) = message_id else {
        return json!({ "ok": false, "error": "missing message_id" });
    };
    probe_value(
        api.post_json("/im/v1/pins", &[], json!({ "message_id": message_id }))
            .await,
    )
}

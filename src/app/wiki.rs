use super::*;

pub(super) async fn run_wiki_command(
    api: &mut FeishuClient,
    command: WikiCommand,
    raw_json: bool,
) -> Result<()> {
    let command = match command {
        WikiCommand::RouteCheck(args) => {
            let strict = args.strict;
            let data = run_wiki_route_check(api, args).await?;
            let route_ready = data
                .get("data")
                .and_then(|data| data.get("route_ready"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
            print_response(raw_json, "wiki operation completed", data.clone())?;
            if strict && !route_ready {
                bail!("{}", wiki_route_check_strict_error(&data));
            }
            return Ok(());
        }
        other => other,
    };

    let data = match command {
        WikiCommand::RouteCheck(_) => unreachable!("route-check is handled before dispatch"),
        WikiCommand::CreateSpace(args) => {
            let body = build_wiki_create_space_body(args)?;
            wiki_request_json(
                api,
                Method::POST,
                "/wiki/v2/spaces",
                &[],
                Some(body),
                ApiAuthArg::User,
            )
            .await?
        }
        WikiCommand::Spaces(args) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            wiki_request_json(api, Method::GET, "/wiki/v2/spaces", &query, None, args.auth).await?
        }
        WikiCommand::Space(args) => {
            let path = format!("/wiki/v2/spaces/{}", encode_path_segment(&args.space_id));
            let mut query = Vec::new();
            push_query_opt(&mut query, "lang", args.lang);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Nodes(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes",
                encode_path_segment(&args.space_id)
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "parent_node_token", args.parent_node_token);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Node(args) => {
            let mut query = vec![("token".to_string(), args.token)];
            push_query_opt(&mut query, "obj_type", args.obj_type);
            wiki_request_json(
                api,
                Method::GET,
                "/wiki/v2/spaces/get_node",
                &query,
                None,
                args.auth,
            )
            .await?
        }
        WikiCommand::CreateNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_create_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::MoveNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/move",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_move_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::CopyNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/copy",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_copy_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::UpdateTitle(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/update_title",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_update_title_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::MoveDocsToWiki(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_move_docs_to_wiki_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::List(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members",
                encode_path_segment(&args.space_id)
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::Add(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let mut query = Vec::new();
            if let Some(value) = args.need_notification {
                query.push(("need_notification".to_string(), value.to_string()));
            }
            let body = build_wiki_member_add_body(args)?;
            wiki_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::Delete(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members/{}",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.member_id)
            );
            let auth = args.auth;
            let body = build_wiki_member_delete_body(args)?;
            wiki_request_json(api, Method::DELETE, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Setting(WikiSettingCommand::Update(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/setting",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_setting_update_body(args)?;
            wiki_request_json(api, Method::PUT, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Task(args) => {
            let path = format!("/wiki/v2/tasks/{}", encode_path_segment(&args.task_id));
            let query = vec![("task_type".to_string(), args.task_type)];
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Search(args) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token.clone());
            let body = build_wiki_search_body(args)?;
            wiki_request_json(
                api,
                Method::POST,
                "/wiki/v2/nodes/search",
                &query,
                Some(body),
                ApiAuthArg::User,
            )
            .await?
        }
    };
    print_response(raw_json, "wiki operation completed", data)
}

async fn run_wiki_route_check(api: &mut FeishuClient, args: WikiRouteCheckArgs) -> Result<Value> {
    if args.page_size == 0 || args.page_size > 50 {
        bail!("wiki route-check page_size must be between 1 and 50");
    }

    let target_space_id = args
        .space_id
        .clone()
        .or_else(|| api.config.default_wiki_space_id.clone());
    let target_parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| api.config.default_wiki_parent_node_token.clone());

    let mut checks = Vec::new();
    let page_size = args.page_size.to_string();
    checks.push(
        wiki_route_check_call(
            api,
            "list_spaces",
            Method::GET,
            "/wiki/v2/spaces".to_string(),
            vec![("page_size".to_string(), page_size.clone())],
            args.auth,
            &["wiki:wiki", "wiki:wiki:readonly", "wiki:space:retrieve"],
        )
        .await,
    );

    if let Some(space_id) = target_space_id.as_deref() {
        let encoded_space_id = encode_path_segment(space_id);
        checks.push(
            wiki_route_check_call(
                api,
                "get_target_space",
                Method::GET,
                format!("/wiki/v2/spaces/{encoded_space_id}"),
                Vec::new(),
                args.auth,
                &["wiki:wiki", "wiki:wiki:readonly", "wiki:space:read"],
            )
            .await,
        );
        checks.push(
            wiki_route_check_call(
                api,
                "list_target_nodes",
                Method::GET,
                format!("/wiki/v2/spaces/{encoded_space_id}/nodes"),
                vec![("page_size".to_string(), page_size)],
                args.auth,
                &["wiki:wiki", "wiki:wiki:readonly", "wiki:node:retrieve"],
            )
            .await,
        );
    }

    let all_api_checks_ok = checks
        .iter()
        .all(|check| check.get("ok").and_then(Value::as_bool).unwrap_or(false));
    let read_route_ready =
        api.config.default_doc_create_wiki && target_space_id.is_some() && all_api_checks_ok;
    let write_probe = if args.write_probe {
        Some(
            run_wiki_write_probe(
                api,
                target_space_id.clone(),
                target_parent_node_token.clone(),
                args.auth,
                args.write_probe_title.clone(),
                args.write_probe_apply,
            )
            .await,
        )
    } else {
        None
    };
    let write_probe_ok = write_probe
        .as_ref()
        .and_then(|probe| probe.get("ok"))
        .and_then(Value::as_bool);
    let route_ready = read_route_ready && (!args.write_probe || write_probe_ok == Some(true));
    let recommendation = wiki_route_recommendation(
        api.config.default_doc_create_wiki,
        target_space_id.is_some(),
        all_api_checks_ok,
        args.write_probe,
        write_probe_ok,
    );

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "route_ready": route_ready,
            "read_route_ready": read_route_ready,
            "write_probe_ran": args.write_probe,
            "default_doc_create_wiki": api.config.default_doc_create_wiki,
            "target_space_id": target_space_id,
            "target_parent_node_token": target_parent_node_token,
            "auth": format!("{:?}", args.auth).to_lowercase(),
            "has_user_access_token": api.config.user_access_token.as_ref().is_some_and(|token| !token.trim().is_empty()),
            "checks": checks,
            "write_probe": write_probe,
            "recommendation": recommendation
        }
    }))
}

async fn run_wiki_write_probe(
    api: &mut FeishuClient,
    target_space_id: Option<String>,
    target_parent_node_token: Option<String>,
    auth: ApiAuthArg,
    title: Option<String>,
    apply: bool,
) -> Value {
    let Some(space_id) = target_space_id else {
        return json!({
            "ok": false,
            "error": "write probe requires --space-id or FEISHU_WIKI_SPACE_ID"
        });
    };
    let title = title.unwrap_or_else(|| {
        format!(
            "Feishu Bot Wiki write probe {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )
    });
    let content = format!(
        "# {title}\n\nCreated by `feishu-bot wiki route-check --write-probe` at {}.\n\nThis document proves whether future AI reports can be moved into the configured Feishu Wiki space.",
        Local::now().format("%Y-%m-%d %H:%M:%S %:z")
    );

    let mut output = Map::new();
    output.insert("ok".to_string(), Value::Bool(false));
    output.insert("title".to_string(), Value::String(title.clone()));
    output.insert(
        "target_space_id".to_string(),
        Value::String(space_id.clone()),
    );
    if let Some(parent) = target_parent_node_token.as_ref() {
        output.insert(
            "target_parent_node_token".to_string(),
            Value::String(parent.clone()),
        );
    }
    output.insert(
        "auth".to_string(),
        Value::String(format!("{auth:?}").to_lowercase()),
    );

    let doc = match api.create_document(&title, None).await {
        Ok(doc) => doc,
        Err(error) => {
            output.insert(
                "create_error".to_string(),
                Value::String(format!("{error:#}")),
            );
            return Value::Object(output);
        }
    };
    output.insert("create_response".to_string(), doc.clone());
    let Some(document_id) = get_string(&doc, &["data", "document", "document_id"])
        .or_else(|| get_string(&doc, &["data", "document_id"]))
    else {
        output.insert(
            "create_error".to_string(),
            Value::String(format!(
                "create document response did not include document_id: {doc}"
            )),
        );
        return Value::Object(output);
    };
    output.insert(
        "document_id".to_string(),
        Value::String(document_id.clone()),
    );
    output.insert(
        "url".to_string(),
        Value::String(api.document_url(&document_id)),
    );

    match api
        .append_document(&document_id, &document_id, &content)
        .await
    {
        Ok(append_response) => {
            output.insert("append_response".to_string(), append_response);
        }
        Err(error) => {
            output.insert(
                "append_error".to_string(),
                Value::String(format!("{error:#}")),
            );
            return Value::Object(output);
        }
    }

    let path = format!(
        "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
        encode_path_segment(&space_id)
    );
    let body = build_doc_create_wiki_move_body(&document_id, target_parent_node_token, apply);
    match wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await {
        Ok(move_response) => {
            let node_token = first_string_path(
                &move_response,
                &[
                    &["data", "wiki_token"],
                    &["data", "wiki_node_token"],
                    &["data", "node_token"],
                    &["data", "node", "node_token"],
                    &["data", "result", "wiki_token"],
                    &["data", "result", "node_token"],
                ],
            );
            let task_id = first_string_path(
                &move_response,
                &[
                    &["data", "task_id"],
                    &["data", "task", "task_id"],
                    &["data", "result", "task_id"],
                ],
            );
            output.insert("move_response".to_string(), move_response);
            if let Some(node_token) = node_token {
                output.insert(
                    "wiki_node_token".to_string(),
                    Value::String(node_token.clone()),
                );
                match read_wiki_node_for_probe(api, &node_token, auth).await {
                    Ok(read_response) => {
                        output.insert("ok".to_string(), Value::Bool(true));
                        output.insert("node_readback".to_string(), read_response);
                    }
                    Err(error) => {
                        output.insert(
                            "node_readback_error".to_string(),
                            Value::String(format!("{error:#}")),
                        );
                    }
                }
            } else if let Some(task_id) = task_id {
                output.insert("task_id".to_string(), Value::String(task_id));
                output.insert(
                    "pending".to_string(),
                    Value::String(
                        "move_docs_to_wiki returned an async task_id; poll with `feishu-bot wiki task --task-id <task_id>`"
                            .to_string(),
                    ),
                );
            } else {
                output.insert(
                    "move_result_note".to_string(),
                    Value::String(
                        "move_docs_to_wiki succeeded but no wiki node token or task_id was found"
                            .to_string(),
                    ),
                );
            }
        }
        Err(error) => {
            output.insert(
                "move_error".to_string(),
                Value::String(format!("{error:#}")),
            );
        }
    }

    Value::Object(output)
}

fn first_string_path(value: &Value, paths: &[&[&str]]) -> Option<String> {
    paths.iter().find_map(|path| get_string(value, path))
}

async fn read_wiki_node_for_probe(
    api: &mut FeishuClient,
    node_token: &str,
    auth: ApiAuthArg,
) -> Result<Value> {
    wiki_request_json(
        api,
        Method::GET,
        "/wiki/v2/spaces/get_node",
        &[("token".to_string(), node_token.to_string())],
        None,
        auth,
    )
    .await
}

async fn wiki_route_check_call(
    api: &mut FeishuClient,
    name: &str,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    auth: ApiAuthArg,
    required_scopes_hint: &[&str],
) -> Value {
    let method_label = method.as_str().to_string();
    let auth_label = format!("{auth:?}").to_lowercase();
    match wiki_request_json(api, method, &path, &query, None, auth).await {
        Ok(response) => json!({
            "name": name,
            "ok": true,
            "method": method_label,
            "path": path,
            "auth": auth_label,
            "required_scopes_hint": required_scopes_hint,
            "response": response
        }),
        Err(error) => json!({
            "name": name,
            "ok": false,
            "method": method_label,
            "path": path,
            "auth": auth_label,
            "required_scopes_hint": required_scopes_hint,
            "error": format!("{error:#}")
        }),
    }
}

pub(super) fn wiki_route_check_strict_error(data: &Value) -> String {
    let recommendation = get_string(data, &["data", "recommendation"])
        .unwrap_or_else(|| "inspect the route-check JSON for details".to_string());
    let first_failed = data
        .get("data")
        .and_then(|data| data.get("checks"))
        .and_then(Value::as_array)
        .and_then(|checks| {
            checks
                .iter()
                .find(|check| check.get("ok").and_then(Value::as_bool) != Some(true))
        });
    let failed_name = first_failed
        .and_then(|check| get_string(check, &["name"]))
        .unwrap_or_else(|| "write_probe".to_string());
    let failed_error = first_failed
        .and_then(|check| get_string(check, &["error"]))
        .or_else(|| get_string(data, &["data", "write_probe", "move_error"]))
        .unwrap_or_default();
    if failed_error.is_empty() {
        format!("wiki route is not ready after {failed_name}: {recommendation}")
    } else {
        format!("wiki route is not ready after {failed_name}: {recommendation}; {failed_error}")
    }
}

pub(super) fn wiki_route_recommendation(
    default_doc_create_wiki: bool,
    has_target_space: bool,
    all_api_checks_ok: bool,
    write_probe_requested: bool,
    write_probe_ok: Option<bool>,
) -> &'static str {
    if !has_target_space {
        "Set FEISHU_WIKI_SPACE_ID or pass --space-id before using Wiki as the default route."
    } else if !default_doc_create_wiki {
        "Set FEISHU_DOC_CREATE_WIKI_DEFAULT=true so plain `feishu-bot doc create` attempts Wiki publishing."
    } else if !all_api_checks_ok {
        "Wiki route is configured, but OpenAPI checks failed. Grant Wiki scopes and add the app or bot to the target Wiki space, then rerun route-check."
    } else if write_probe_requested && write_probe_ok != Some(true) {
        "Wiki read route is configured, but the write probe did not prove publishing. Fix the write_probe error before claiming future reports can all go through Wiki."
    } else if write_probe_requested {
        "Wiki write route is ready. Future AI reports can use plain `feishu-bot doc create --wiki-fallback-ok` and verify wiki_move or wiki node readback."
    } else {
        "Wiki read route is ready. Run `feishu-bot wiki route-check --write-probe` once before claiming future reports can all go through Wiki."
    }
}

pub(super) async fn wiki_request_json(
    api: &mut FeishuClient,
    method: Method,
    path: &str,
    query: &[(String, String)],
    body: Option<Value>,
    auth: ApiAuthArg,
) -> Result<Value> {
    api.request_json_with_auth(method, path, query, body, auth, &[])
        .await
}

pub(super) fn build_doc_create_wiki_move_body(
    document_id: &str,
    parent_wiki_token: Option<String>,
    apply: bool,
) -> Value {
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String("docx".to_string()));
    body.insert(
        "obj_token".to_string(),
        Value::String(document_id.to_string()),
    );
    insert_opt_string(&mut body, "parent_wiki_token", parent_wiki_token);
    if apply {
        body.insert("apply".to_string(), Value::Bool(true));
    }
    Value::Object(body)
}

fn build_wiki_create_space_body(args: WikiCreateSpaceArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki create-space body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "description", args.description);
    insert_opt_string(&mut body, "open_sharing", args.open_sharing);
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_create_node_body(args: WikiCreateNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki create-node body",
        );
    }
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String(args.obj_type));
    body.insert("node_type".to_string(), Value::String(args.node_type));
    insert_opt_string(&mut body, "parent_node_token", args.parent_node_token);
    insert_opt_string(&mut body, "origin_node_token", args.origin_node_token);
    insert_opt_string(&mut body, "title", args.title);
    Ok(Value::Object(body))
}

fn build_wiki_move_node_body(args: WikiMoveNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki move-node body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "target_parent_token", args.target_parent_token);
    insert_opt_string(&mut body, "target_space_id", args.target_space_id);
    if body.is_empty() {
        bail!("wiki move-node requires --target-parent-token, --target-space-id, or raw JSON body");
    }
    Ok(Value::Object(body))
}

fn build_wiki_copy_node_body(args: WikiCopyNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki copy-node body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "target_parent_token", args.target_parent_token);
    insert_opt_string(&mut body, "target_space_id", args.target_space_id);
    insert_opt_string(&mut body, "title", args.title);
    if !body.contains_key("target_parent_token") && !body.contains_key("target_space_id") {
        bail!("wiki copy-node requires --target-parent-token, --target-space-id, or raw JSON body");
    }
    Ok(Value::Object(body))
}

fn build_wiki_update_title_body(args: WikiUpdateTitleArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki update-title body",
        );
    }
    let title = args
        .title
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki update-title requires --title unless --body-json/--file/--stdin is used")
        })?;
    Ok(json!({ "title": title }))
}

pub(super) fn build_wiki_move_docs_to_wiki_body(args: WikiMoveDocsToWikiArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki move-docs-to-wiki body",
        );
    }
    let obj_type = args
        .obj_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki move-docs-to-wiki requires --obj-type unless raw JSON body is used")
        })?;
    let obj_token = args
        .obj_token
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki move-docs-to-wiki requires --obj-token unless raw JSON body is used")
        })?;
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String(obj_type));
    body.insert("obj_token".to_string(), Value::String(obj_token));
    insert_opt_string(&mut body, "parent_wiki_token", args.parent_wiki_token);
    if args.apply {
        body.insert("apply".to_string(), Value::Bool(true));
    }
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_member_add_body(args: WikiMemberAddArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki member add body",
        );
    }
    let member_type = args
        .member_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member add requires --member-type unless raw JSON body is used")
        })?;
    let member_id = args
        .member_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member add requires --member-id unless raw JSON body is used")
        })?;
    Ok(json!({
        "member_type": member_type,
        "member_id": member_id,
        "member_role": args.member_role
    }))
}

fn build_wiki_member_delete_body(args: WikiMemberDeleteArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki member delete body",
        );
    }
    let member_type = args
        .member_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member delete requires --member-type unless raw JSON body is used")
        })?;
    Ok(json!({
        "member_type": member_type,
        "member_role": args.member_role
    }))
}

fn build_wiki_setting_update_body(args: WikiSettingUpdateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki setting update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "create_setting", args.create_setting);
    insert_opt_string(&mut body, "security_setting", args.security_setting);
    insert_opt_string(&mut body, "comment_setting", args.comment_setting);
    if body.is_empty() {
        bail!("wiki setting update requires at least one setting flag or raw JSON body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_search_body(args: WikiSearchArgs) -> Result<Value> {
    if args.page_size == 0 || args.page_size > 50 {
        bail!("wiki search page_size must be between 1 and 50");
    }
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki search body",
        );
    }
    let query = args
        .query
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki search requires --query unless --body-json/--file/--stdin is used")
        })?;
    if args.node_id.is_some() && args.space_id.is_none() {
        bail!("wiki search --node-id requires --space-id");
    }
    let mut body = Map::new();
    body.insert("query".to_string(), Value::String(query));
    insert_opt_string(&mut body, "space_id", args.space_id);
    insert_opt_string(&mut body, "node_id", args.node_id);
    Ok(Value::Object(body))
}

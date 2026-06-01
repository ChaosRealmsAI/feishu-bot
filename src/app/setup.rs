use super::*;

pub(super) async fn run_setup_command(
    command: SetupCommand,
    use_lark: bool,
    base_url_override: Option<String>,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        SetupCommand::Plan(args) => run_setup_plan(args)?,
        SetupCommand::OpenScopes(args) => run_setup_open_scopes(args)?,
        SetupCommand::WikiBot(args) => {
            let config = Config::load(use_lark, base_url_override)?;
            let mut api = FeishuClient::new(config);
            run_setup_wiki_bot(
                &mut api,
                args.space_id,
                args.member_role,
                args.need_notification,
                args.auth,
            )
            .await?
        }
        SetupCommand::Quickstart(args) => {
            run_setup_quickstart(args, use_lark, base_url_override).await?
        }
        SetupCommand::Auto(args) => run_setup_auto(args, use_lark, base_url_override).await?,
    };
    print_response(raw_json, "setup operation completed", data)
}

fn run_setup_plan(args: SetupPlanArgs) -> Result<Value> {
    let values = load_env_values().unwrap_or_default();
    let grant = build_setup_grant(&values, &args.groups, args.token_type);
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "env": setup_env_status(&values),
            "scope_grant": grant.unwrap_or_else(|error| json!({ "ok": false, "error": format!("{error:#}") })),
            "oauth": setup_oauth_plan(&values),
            "wiki_bot": setup_wiki_bot_plan(&values),
            "browser": setup_browser_plan(),
            "recommended_auto": "feishu-bot setup quickstart --open-browser",
            "next_actions": setup_next_actions(&values),
        }
    }))
}

fn run_setup_open_scopes(args: SetupOpenScopesArgs) -> Result<Value> {
    let values = load_env_values().unwrap_or_default();
    let grant = build_setup_grant(&values, &args.groups, args.token_type)?;
    let url = grant
        .get("grant_url")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("setup grant response missing grant_url"))?
        .to_string();
    let browser_open = if args.browser {
        probe_value(run_setup_browser_open(&url).map(|_| json!({ "opened": true })))
    } else {
        json!({ "status": "skipped" })
    };
    let system_open = if args.system_browser {
        probe_value(run_system_browser_open(&url).map(|_| json!({ "opened": true })))
    } else {
        json!({ "status": "skipped" })
    };
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "scope_grant": grant,
            "browser_open": browser_open,
            "system_open": system_open,
            "next_actions": [
                "Approve the scopes in Feishu Open Platform if the browser opened successfully.",
                "Then run `feishu-bot setup quickstart` or `feishu-bot dogfood verify --include-response`."
            ],
        }
    }))
}

async fn run_setup_auto(
    args: SetupAutoArgs,
    use_lark: bool,
    base_url_override: Option<String>,
) -> Result<Value> {
    let values = load_env_values().unwrap_or_default();
    let grant = build_setup_grant(&values, &args.groups, args.token_type);
    let browser_open = setup_grant_open_probe(&grant, args.open_browser, run_setup_browser_open);

    let config = Config::load(use_lark, base_url_override.clone());
    let doctor = match config.as_ref() {
        Ok(config) => setup_doctor_probe(config).await,
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    };

    let wiki_bot = setup_wiki_bot_probe(&config, args.no_wiki_bot, args.space_id).await;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "env": setup_env_status(&values),
            "scope_grant": grant.unwrap_or_else(|error| json!({ "ok": false, "error": format!("{error:#}") })),
            "browser_open": browser_open,
            "doctor": doctor,
            "wiki_bot": wiki_bot,
            "oauth": setup_oauth_plan(&values),
            "browser": setup_browser_plan(),
            "next_actions": setup_next_actions(&values),
        }
    }))
}

async fn run_setup_quickstart(
    args: SetupQuickstartArgs,
    use_lark: bool,
    base_url_override: Option<String>,
) -> Result<Value> {
    let values = load_env_values().unwrap_or_default();
    let grant = build_setup_grant(&values, &args.groups, args.token_type);
    let browser_open = setup_grant_open_probe(&grant, args.open_browser, run_setup_browser_open);
    let system_open = setup_grant_open_probe(&grant, args.system_browser, run_system_browser_open);

    let config = Config::load(use_lark, base_url_override);
    let doctor = match config.as_ref() {
        Ok(config) => setup_doctor_probe(config).await,
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    };
    let wiki_bot = setup_wiki_bot_probe(&config, args.no_wiki_bot, args.space_id.clone()).await;
    let selected_groups = setup_group_names(&args.groups);
    let quickstart = setup_quickstart_plan(&values, &args.project, &selected_groups);

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "profile": "office",
            "env": setup_env_status(&values),
            "scope_grant": grant.unwrap_or_else(|error| json!({ "ok": false, "error": format!("{error:#}") })),
            "browser_open": browser_open,
            "system_open": system_open,
            "doctor": doctor,
            "wiki_bot": wiki_bot,
            "oauth": setup_oauth_plan(&values),
            "browser": setup_browser_plan(),
            "quickstart": quickstart,
            "next_actions": setup_next_actions(&values),
        }
    }))
}

fn setup_grant_open_probe(
    grant: &Result<Value>,
    should_open: bool,
    open: fn(&str) -> Result<()>,
) -> Value {
    if !should_open {
        return json!({ "status": "skipped" });
    }
    match grant.as_ref() {
        Ok(value) => match value.get("grant_url").and_then(Value::as_str) {
            Some(url) => probe_value(open(url).map(|_| json!({ "opened": true }))),
            None => json!({ "ok": false, "error": "setup grant response missing grant_url" }),
        },
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

async fn setup_wiki_bot_probe(
    config: &Result<Config>,
    should_skip: bool,
    space_id: Option<String>,
) -> Value {
    if should_skip {
        return json!({ "status": "skipped" });
    }
    match config {
        Ok(config) => {
            let mut api = FeishuClient::new(config.clone());
            probe_value(
                run_setup_wiki_bot(
                    &mut api,
                    space_id,
                    "admin".to_string(),
                    Some(false),
                    ApiAuthArg::User,
                )
                .await,
            )
        }
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

async fn setup_doctor_probe(config: &Config) -> Value {
    let mut api = FeishuClient::new(config.clone());
    match api.tenant_token().await {
        Ok(_) => json!({
            "ok": true,
            "tenant": {
                "tenant_access_token_configured": true,
                "base_url": config.base_url.clone(),
                "app_id": mask_app_id(&config.app_id),
                "default_user_id_configured": config.default_user_id.is_some(),
                "user_access_token_configured": config.user_access_token.is_some(),
                "wiki_space_id_configured": config.default_wiki_space_id.is_some(),
            }
        }),
        Err(error) => json!({
            "ok": false,
            "tenant": {
                "tenant_access_token_configured": false,
                "base_url": config.base_url.clone(),
                "app_id": mask_app_id(&config.app_id),
                "default_user_id_configured": config.default_user_id.is_some(),
                "user_access_token_configured": config.user_access_token.is_some(),
                "wiki_space_id_configured": config.default_wiki_space_id.is_some(),
                "error": format!("{error:#}"),
            }
        }),
    }
}

async fn run_setup_wiki_bot(
    api: &mut FeishuClient,
    space_id: Option<String>,
    member_role: String,
    need_notification: Option<bool>,
    auth: ApiAuthArg,
) -> Result<Value> {
    let space_id = space_id
        .or_else(|| api.config.default_wiki_space_id.clone())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("setup wiki-bot needs --space-id or FEISHU_WIKI_SPACE_ID"))?;
    let bot = normalize_bot_info_response(api.get_json("/bot/v3/info", &[]).await?);
    let open_id = get_string(&bot, &["data", "open_id"])
        .ok_or_else(|| anyhow!("bot info response missing open_id: {bot}"))?;
    let path = format!("/wiki/v2/spaces/{}/members", encode_path_segment(&space_id));
    let initial_member_list = probe_value(
        wiki_request_json(
            api,
            Method::GET,
            &path,
            &[("page_size".to_string(), "50".to_string())],
            None,
            auth,
        )
        .await,
    );
    if wiki_member_has_role(&initial_member_list, &open_id, &member_role) {
        return Ok(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "status": "reused",
                "space_id": space_id,
                "member_type": "openid",
                "member_id": open_id,
                "member_role": member_role,
                "auth": format!("{auth:?}").to_lowercase(),
                "bot": bot,
                "member_add": { "status": "skipped_existing_member" },
                "member_list": initial_member_list,
                "next_actions": [
                    "Rerun `feishu-bot office bootstrap --project <project> --send-summary` without --skip-wiki.",
                    "Rerun `feishu-bot wiki route-check --write-probe --strict` to prove Wiki write readiness."
                ],
            }
        }));
    }
    let mut query = Vec::new();
    if let Some(value) = need_notification {
        query.push(("need_notification".to_string(), value.to_string()));
    }
    let body = build_wiki_member_add_body(WikiMemberAddArgs {
        space_id: space_id.clone(),
        member_type: Some("openid".to_string()),
        member_id: Some(open_id.clone()),
        member_role: member_role.clone(),
        need_notification,
        auth,
        body_json: None,
        file: None,
        stdin: false,
    })?;
    let member_add = wiki_request_json(api, Method::POST, &path, &query, Some(body), auth).await?;
    let member_list = probe_value(
        wiki_request_json(
            api,
            Method::GET,
            &path,
            &[("page_size".to_string(), "50".to_string())],
            None,
            auth,
        )
        .await,
    );
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "space_id": space_id,
            "member_type": "openid",
            "member_id": open_id,
            "member_role": member_role,
            "auth": format!("{auth:?}").to_lowercase(),
            "bot": bot,
            "member_add": member_add,
            "member_list": member_list,
            "next_actions": [
                "Rerun `feishu-bot office bootstrap --project <project> --send-summary` without --skip-wiki.",
                "Rerun `feishu-bot wiki route-check --write-probe --strict` to prove Wiki write readiness."
            ],
        }
    }))
}

fn wiki_member_has_role(member_list: &Value, open_id: &str, role: &str) -> bool {
    member_list
        .pointer("/response/data/members")
        .and_then(Value::as_array)
        .is_some_and(|members| {
            members.iter().any(|member| {
                member.get("member_id").and_then(Value::as_str) == Some(open_id)
                    && member.get("member_type").and_then(Value::as_str) == Some("openid")
                    && (member.get("member_role").and_then(Value::as_str) == Some(role)
                        || member.get("member_perm").and_then(Value::as_str) == Some(role))
            })
        })
}

fn build_setup_grant(
    values: &HashMap<String, String>,
    groups: &[String],
    token_type: ApiAuthArg,
) -> Result<Value> {
    let app_id = get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).ok_or_else(|| {
        anyhow!("missing FEISHU_APP_ID or LARK_APP_ID; set it before opening scope grants")
    })?;
    let selected = setup_group_names(groups);
    let scopes = collect_setup_scopes(&selected)?;
    let token_type_text = scope_token_type(token_type);
    let url = format!(
        "https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot-setup&token_type={}",
        app_id,
        scopes.join(","),
        token_type_text
    );
    Ok(json!({
        "ok": true,
        "groups": selected,
        "scope_count": scopes.len(),
        "scopes": scopes,
        "token_type": token_type_text,
        "grant_url": url,
        "browser_command": format!("feishu-bot browser open --url \"{url}\""),
    }))
}

fn setup_group_names(groups: &[String]) -> Vec<String> {
    let raw = if groups.is_empty() {
        vec!["office".to_string()]
    } else {
        groups.to_vec()
    };
    let mut out = Vec::new();
    for group in raw {
        let group = group.trim().to_ascii_lowercase();
        if group.is_empty() {
            continue;
        }
        let expanded: Vec<&str> = match group.as_str() {
            "office" => OFFICE_SCOPE_GROUPS.to_vec(),
            other => vec![other],
        };
        for item in expanded {
            if !out.iter().any(|existing| existing == item) {
                out.push(item.to_string());
            }
        }
    }
    out
}

fn collect_setup_scopes(groups: &[String]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for group in groups {
        for (_name, scopes) in scope_groups(group)? {
            for scope in scopes {
                if !out.iter().any(|existing| existing == scope) {
                    out.push(scope.to_string());
                }
            }
        }
    }
    Ok(out)
}

fn setup_env_status(values: &HashMap<String, String>) -> Value {
    json!({
        "app_id": get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).as_deref().map(mask_app_id),
        "app_secret_configured": get_any(values, &["FEISHU_APP_SECRET", "LARK_APP_SECRET"]).is_some(),
        "default_user_id_configured": get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_some(),
        "user_access_token_configured": get_any(values, &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"]).is_some(),
        "refresh_token_configured": get_any(values, &["FEISHU_REFRESH_TOKEN", "LARK_REFRESH_TOKEN"]).is_some(),
        "wiki_space_id": get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some(),
        "office_state_file": get_any(values, &["FEISHU_OFFICE_STATE_FILE", "LARK_OFFICE_STATE_FILE"]),
    })
}

fn setup_oauth_plan(values: &HashMap<String, String>) -> Value {
    let configured = get_any(
        values,
        &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
    )
    .is_some();
    json!({
        "user_access_token_configured": configured,
        "url_command": "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope search:docs:read --scope search:message --scope wiki:wiki --scope wiki:node:create",
        "token_command": "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
        "refresh_command": "feishu-bot oauth refresh --save-env",
    })
}

fn setup_wiki_bot_plan(values: &HashMap<String, String>) -> Value {
    json!({
        "space_configured": get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some(),
        "user_token_configured": get_any(values, &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"]).is_some(),
        "command": "feishu-bot setup wiki-bot --auth user",
        "manual_equivalent": "feishu-bot bot info && feishu-bot wiki member add --auth user --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin",
    })
}

fn setup_quickstart_plan(
    values: &HashMap<String, String>,
    project: &str,
    selected_groups: &[String],
) -> Value {
    let project = if project.trim().is_empty() {
        "AI Project"
    } else {
        project.trim()
    };
    let project_arg = shell_quote(project);
    let group_args = if selected_groups.is_empty() {
        "--group office".to_string()
    } else {
        selected_groups
            .iter()
            .map(|group| format!("--group {}", shell_quote(group)))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let user_arg = if get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_some() {
        "--user \"$FEISHU_USER_ID\""
    } else {
        "--user <open_id>"
    };
    let space_arg = if get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_some() {
        "--space-id \"$FEISHU_WIKI_SPACE_ID\""
    } else {
        "--space-id <space_id>"
    };
    json!({
        "purpose": "Bring a new Feishu app/account to the common AI office workflow: group chat, Wiki docs, Base project log, message polling, and search.",
        "script": format!("scripts/feishu-bot-setup.sh --project {project_arg}"),
        "commands": {
            "inspect_env": "feishu-bot doctor",
            "plan": "feishu-bot setup plan --json",
            "grant_permissions": format!("feishu-bot setup open-scopes {group_args} --browser --json"),
            "oauth_url": "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope search:docs:read --scope search:message --scope wiki:wiki --scope wiki:node:create",
            "save_oauth_code": "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
            "refresh_oauth": "feishu-bot oauth refresh --save-env",
            "add_wiki_bot": "feishu-bot setup wiki-bot --auth user --json",
            "bootstrap_dry_run": format!("feishu-bot office bootstrap --project {project_arg} --dry-run --json"),
            "bootstrap": format!("feishu-bot office bootstrap --project {project_arg} {user_arg} {space_arg} --send-summary --json"),
            "status_check": format!("feishu-bot office status --project {project_arg} --check --json"),
            "progress_update": format!("feishu-bot office progress --project {project_arg} --title \"进度更新\" --status doing --summary \"当前进展\" --json"),
            "inbox_from_now": format!("feishu-bot office inbox --project {project_arg} --from-now --json"),
            "inbox_process": format!("feishu-bot office inbox --project {project_arg} --reply-text \"收到，我来处理\" --json"),
            "search": format!("feishu-bot office search --project {project_arg} --query \"关键词\" --json")
        },
        "common_order": [
            "doctor",
            "setup quickstart/open scope grant",
            "OAuth user token when user-token APIs are needed",
            "setup wiki-bot when using a Wiki space",
            "office bootstrap",
            "office report/progress/voice-report",
            "office inbox/search/status"
        ]
    })
}

fn setup_browser_plan() -> Value {
    json!({
        "ensure_command": "feishu-bot browser ensure",
        "open_scope_command": "feishu-bot setup open-scopes --browser",
        "account_safety": "For multi-account Chrome profiles, bind and verify the intended Playwright MCP profile before approving account-sensitive grants.",
    })
}

fn setup_next_actions(values: &HashMap<String, String>) -> Vec<String> {
    let mut actions = Vec::new();
    if get_any(values, &["FEISHU_APP_ID", "LARK_APP_ID"]).is_none() {
        actions.push("Set FEISHU_APP_ID or LARK_APP_ID.".to_string());
    }
    if get_any(values, &["FEISHU_APP_SECRET", "LARK_APP_SECRET"]).is_none() {
        actions.push("Set FEISHU_APP_SECRET or LARK_APP_SECRET.".to_string());
    }
    if get_any(values, &["FEISHU_USER_ID", "LARK_USER_ID"]).is_none() {
        actions
            .push("Set FEISHU_USER_ID so setup can create one-person project groups.".to_string());
    }
    if get_any(
        values,
        &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
    )
    .is_none()
    {
        actions.push("Run the OAuth URL/token flow to enable user-token APIs such as Wiki member writes and search.".to_string());
    }
    if get_any(values, &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"]).is_none() {
        actions.push("Set FEISHU_WIKI_SPACE_ID if reports should default to Wiki.".to_string());
    }
    if actions.is_empty() {
        actions.push("Run `feishu-bot setup quickstart --open-browser`, approve scopes if needed, then run `feishu-bot dogfood verify --include-response`.".to_string());
    }
    actions
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn run_setup_browser_open(url: &str) -> Result<()> {
    run_browser_command(BrowserCommand::Open(BrowserOpenArgs {
        url: url.to_string(),
    }))
}

fn run_system_browser_open(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        run_status(ProcessCommand::new("open").arg(url))
    }
    #[cfg(target_os = "windows")]
    {
        run_status(ProcessCommand::new("cmd").args(["/C", "start", "", url]))
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        run_status(ProcessCommand::new("xdg-open").arg(url))
    }
}

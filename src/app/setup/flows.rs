use super::browser::{run_setup_browser_open, run_system_browser_open, setup_grant_open_probe};
use super::plan::*;
use super::probes::{setup_doctor_probe, setup_wiki_bot_probe};
use super::*;

pub(super) fn run_setup_plan(args: SetupPlanArgs) -> Result<Value> {
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

pub(super) fn run_setup_open_scopes(args: SetupOpenScopesArgs) -> Result<Value> {
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

pub(super) async fn run_setup_auto(
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

pub(super) async fn run_setup_quickstart(
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

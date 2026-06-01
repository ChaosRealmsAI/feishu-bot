use super::*;

mod modules;

use modules::manifest_modules;
pub(super) fn print_manifest(args: &ManifestArgs) -> Result<()> {
    let mut manifest = build_manifest()?;
    if let Some(filter) = args.module.as_deref() {
        let modules = manifest
            .get_mut("modules")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| anyhow!("manifest modules missing"))?;
        retain_manifest_modules(modules, filter);
        if modules.is_empty() {
            bail!("no manifest module matched {filter}");
        }
    }
    if args.compact {
        println!("{}", serde_json::to_string(&manifest)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&manifest)?);
    }
    Ok(())
}

pub(super) fn retain_manifest_modules(modules: &mut Vec<Value>, filter: &str) {
    let needle = filter.trim().to_lowercase();
    let has_exact = modules
        .iter()
        .any(|module| manifest_module_exact_matches(module, &needle));
    if has_exact {
        modules.retain(|module| manifest_module_exact_matches(module, &needle));
    } else {
        modules.retain(|module| manifest_module_matches(module, &needle));
    }
}

pub(super) fn manifest_module_exact_matches(module: &Value, needle: &str) -> bool {
    ["name", "command"]
        .iter()
        .filter_map(|key| module.get(*key).and_then(Value::as_str))
        .any(|value| {
            let normalized = value.to_lowercase();
            normalized == needle
                || normalized
                    .split_whitespace()
                    .last()
                    .is_some_and(|last| last == needle)
        })
}

pub(super) fn manifest_module_matches(module: &Value, needle: &str) -> bool {
    ["name", "command", "scope_group"]
        .iter()
        .filter_map(|key| module.get(*key).and_then(Value::as_str))
        .any(|value| value.to_lowercase().contains(needle))
}

pub(super) fn build_manifest() -> Result<Value> {
    let values = load_env_values().unwrap_or_default();
    let app_id =
        get_any(&values, &["FEISHU_APP_ID", "LARK_APP_ID"]).unwrap_or_else(|| "<app_id>".into());
    let scope_values: Vec<Value> = scope_groups("all")?
        .into_iter()
        .map(|(name, scopes)| {
            json!({
                "group": name,
                "scopes": scopes,
                "grant_url": format!(
                    "https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot&token_type=tenant",
                    app_id,
                    scopes.join(",")
                ),
            })
        })
        .collect();

    Ok(json!({
        "schema_version": 1,
        "package": "feishu-bot",
        "display_name": "飞书Bot",
        "binary": "feishu-bot",
        "aliases": ["feishuBot", "feishu"],
        "version": env!("CARGO_PKG_VERSION"),
        "purpose": "AI-ready local Feishu Bot automation for Feishu/Lark office workflows.",
        "layers": {
            "workflow_modules": ["office", "dogfood"],
            "setup_modules": ["setup", "oauth", "scopes", "browser"],
            "atomic_modules": ["message", "chat", "doc", "wiki", "base", "task", "drive", "calendar", "search", "sheet", "approval", "board", "contact", "directory", "vc", "minutes", "okr", "attendance", "mail", "corehr", "helpdesk", "hire", "api"],
            "guidance": "Use workflow modules for normal AI office loops; drop to atomic modules for exact OpenAPI operations, troubleshooting, or unsupported workflow edges."
        },
        "first_commands": [
            "feishu-bot manifest",
            "feishu-bot ai",
            "feishu-bot --help",
            "feishu-bot bot info",
            "feishu-bot setup plan",
            "feishu-bot setup quickstart --open-browser",
            "feishu-bot office --help",
            "feishu-bot office list",
            "feishu-bot office bootstrap --project \"AI Project\" --dry-run",
            "feishu-bot dogfood verify",
            "feishu-bot office bootstrap --project \"AI Project\" --user \"$FEISHU_USER_ID\" --space-id \"$FEISHU_WIKI_SPACE_ID\" --send-summary",
            "feishu-bot office progress --project \"AI Project\" --title \"Progress\" --summary \"Current status\"",
            "feishu-bot office report --project \"AI Project\" --title \"Capability Demo\" --file demo.md --base-record --pin",
            "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope task:task:read",
            "feishu-bot dogfood publish --title \"Capability Demo\" --file demo.md",
            "feishu-bot message loop-check --to \"$FEISHU_USER_ID\" --to-type open-id",
            "feishu-bot scopes --group all",
            "feishu-bot doctor"
        ],
        "environment": {
            "required": ["FEISHU_APP_ID", "FEISHU_APP_SECRET"],
            "common": ["FEISHU_USER_ID", "FEISHU_USER_ACCESS_TOKEN", "FEISHU_WIKI_SPACE_ID", "FEISHU_WIKI_PARENT_NODE_TOKEN", "FEISHU_DOC_CREATE_WIKI_DEFAULT", "FEISHU_HELPDESK_ID", "FEISHU_HELPDESK_TOKEN"],
            "env_files": {
                "project": "./.env",
                "cwd": "$PWD/.env",
                "override": "FEISHU_ENV_FILE or LARK_ENV_FILE"
            },
            "secret_policy": "Do not print real app secrets, tenant tokens, user tokens, or helpdesk tokens in final answers."
        },
        "modules": manifest_modules(),
        "scopes": scope_values,
        "raw_openapi_fallback": {
            "command": "feishu-bot api",
            "path_rule": "Use paths under /open-apis, for example /im/v1/chats.",
            "body_rule": "Pass raw JSON with --body-json, --file, or --stdin.",
            "auth_rule": "Use --auth tenant by default or --auth user when official docs require user_access_token.",
            "binary_rule": "Use feishu-bot api download --output for binary GET responses.",
            "multipart_rule": "Use feishu-bot api multipart with --field key=value and --file part_name=./path for upload endpoints.",
            "when_to_use": "Use when official Feishu exposes an API that does not yet have a typed wrapper."
        },
        "completion_rules_for_ai": [
            "Prefer typed commands when present.",
            "Run the relevant --help command before using an unfamiliar module.",
            "Use --json for machine parsing of API responses.",
            "If Feishu returns 99991672, inspect permission_violations and open the grant_url from feishu-bot scopes.",
            "Do not claim a module works until a help command or a harmless read/list command has been verified."
        ]
    }))
}

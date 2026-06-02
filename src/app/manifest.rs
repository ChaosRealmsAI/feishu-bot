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
    manifest_field_matches(module, &["name", "command", "aliases"], |value| {
        let normalized = value.to_lowercase();
        normalized == needle
            || normalized
                .split_whitespace()
                .last()
                .is_some_and(|last| last == needle)
    })
}

pub(super) fn manifest_module_matches(module: &Value, needle: &str) -> bool {
    manifest_field_matches(
        module,
        &["name", "command", "scope_group", "aliases", "tags"],
        |value| value.to_lowercase().contains(needle),
    )
}

fn manifest_field_matches(module: &Value, keys: &[&str], predicate: impl Fn(&str) -> bool) -> bool {
    keys.iter().any(|key| match module.get(*key) {
        Some(Value::String(value)) => predicate(value),
        Some(Value::Array(values)) => values.iter().any(|value| {
            value.as_str().is_some_and(|value| {
                let normalized = value.to_lowercase();
                predicate(&normalized)
            })
        }),
        _ => false,
    })
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
            "setup_modules": ["setup", "oauth", "bot", "scopes", "browser"],
            "atomic_modules": ["message", "chat", "doc", "wiki", "base", "task", "drive", "calendar", "search", "sheet", "approval", "board", "contact", "directory", "vc", "minutes", "okr", "attendance", "mail", "corehr", "helpdesk", "hire", "notify", "api"],
            "guidance": "Use workflow modules for normal AI office loops; drop to atomic modules for exact OpenAPI operations, troubleshooting, or unsupported workflow edges."
        },
        "workflow_layer": {
            "default_command": "feishu-bot office",
            "verification_command": "feishu-bot dogfood verify",
            "preferred_commands": [
                "feishu-bot office list --json",
                "feishu-bot office bootstrap --project \"AI Project\" --dry-run --json",
                "feishu-bot office bootstrap --project \"AI Project\" --user \"$FEISHU_USER_ID\" --space-id \"$FEISHU_WIKI_SPACE_ID\" --send-summary --json",
                "feishu-bot office progress --project \"AI Project\" --title \"Progress\" --summary \"Current status\" --json",
                "feishu-bot office report --project \"AI Project\" --title \"Capability Demo\" --file demo.md --base-record --pin --json",
                "feishu-bot office inbox --project \"AI Project\" --from-now --json",
                "feishu-bot office search --project \"AI Project\" --query \"decision\" --json",
                "feishu-bot dogfood verify --module message --json",
                "feishu-bot dogfood verify --module task --module search --auto-refresh-user-token --strict --json"
            ],
            "local_safe_commands": [
                "feishu-bot office list --json",
                "feishu-bot office bootstrap --project \"AI Project\" --dry-run --json",
                "feishu-bot office report --project \"AI Project\" --title \"Capability Demo\" --file demo.md --dry-run --json",
                "feishu-bot office status --project \"AI Project\" --json"
            ],
            "proof_rule": "For write workflows, create the Feishu object, read it back, and inspect returned readback fields before reporting success.",
            "state_file": "~/.config/feishu/office-projects.json",
            "state_override_env": "FEISHU_OFFICE_STATE_FILE"
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
            "A help command only proves discoverability; it does not prove the Feishu capability works.",
            "For read capability, run dogfood verify or a harmless read/list command and inspect JSON status.",
            "For write capability, create the real Feishu object, read it back, and inspect returned readback fields before claiming success."
        ]
    }))
}

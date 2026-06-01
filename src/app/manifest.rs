use super::*;
pub(super) fn print_scope_groups(group: &str, token_type: ApiAuthArg) -> Result<()> {
    let values = load_env_values().unwrap_or_default();
    let app_id =
        get_any(&values, &["FEISHU_APP_ID", "LARK_APP_ID"]).unwrap_or_else(|| "<app_id>".into());
    let groups = scope_groups(group)?;
    let token_type = scope_token_type(token_type);
    for (name, scopes) in groups {
        println!("[{name}]");
        for scope in &scopes {
            println!("- {scope}");
        }
        println!(
            "grant_url=https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot&token_type={}",
            app_id,
            scopes.join(","),
            token_type
        );
        println!();
    }
    Ok(())
}

pub(super) fn scope_token_type(token_type: ApiAuthArg) -> &'static str {
    match token_type {
        ApiAuthArg::Tenant => "tenant",
        ApiAuthArg::User => "user",
    }
}

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
        "modules": [
            {
                "name": "oauth",
                "command": "feishu-bot oauth",
                "scope_group": "user-token",
                "status": "OAuth v2 helpers",
                "ai_use": "Generate Feishu OAuth authorization URLs, exchange authorization codes for user_access_token, refresh user tokens, and verify user token identity. Use this when dogfood verify reports missing_user_token for my tasks, mail, search, minutes, wiki create-space/search, or meeting host operations.",
                "help": ["feishu-bot oauth --help", "feishu-bot oauth url --help", "feishu-bot oauth token --help", "feishu-bot oauth refresh --help", "feishu-bot oauth user-info --help"],
                "examples": [
                    "feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope task:task:read",
                    "feishu-bot browser open --url \"<authorization_url>\"",
                    "feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env",
                    "feishu-bot oauth refresh --save-env",
                    "feishu-bot oauth user-info"
                ],
                "known_permission_edges": [
                    "The redirect URI must be registered in Open Platform security settings.",
                    "oauth url defaults to FEISHU_OAUTH_REDIRECT_URI, LARK_OAUTH_REDIRECT_URI, or http://localhost:8080/callback.",
                    "Tokens are masked by default; --raw and --print-env intentionally expose secrets.",
                    "Use --save-env to persist FEISHU_USER_ACCESS_TOKEN and FEISHU_REFRESH_TOKEN into the selected env file."
                ]
            },
            {
                "name": "bot",
                "command": "feishu-bot bot",
                "scope_group": "im",
                "status": "typed wrapper",
                "ai_use": "Read current app bot identity, including bot open_id, so AI can grant Wiki/doc permissions to the right app bot.",
                "help": ["feishu-bot bot --help", "feishu-bot bot info --help"],
                "examples": [
                    "feishu-bot bot info",
                    "feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin"
                ],
                "known_permission_edges": [
                    "bot info calls /bot/v3/info with tenant_access_token and expects the app bot capability to be enabled."
                ]
            },
            {
                "name": "setup",
                "command": "feishu-bot setup",
                "layer": "setup",
                "scope_group": "im,doc,wiki,base,search,user-token",
                "status": "first-run automation and permission remediation",
                "ai_use": "Prepare a Feishu app/account for AI operation: inspect env shape, build or open Open Platform scope grant URLs, guide OAuth user-token setup, open URLs through the Playwright MCP browser bridge, and add the current app bot to a Wiki space with user-token auth.",
                "help": ["feishu-bot setup --help", "feishu-bot setup plan --help", "feishu-bot setup quickstart --help", "feishu-bot setup open-scopes --help", "feishu-bot setup wiki-bot --help", "feishu-bot setup auto --help"],
                "examples": [
                    "feishu-bot setup plan",
                    "feishu-bot setup quickstart --open-browser",
                    "scripts/feishu-bot-setup.sh --project \"AI Project\"",
                    "feishu-bot setup open-scopes --group office --browser",
                    "feishu-bot setup wiki-bot --auth user",
                    "feishu-bot setup auto --open-browser"
                ],
                "known_permission_edges": [
                    "Opening a grant URL is automated; approving permissions still happens in the signed-in human browser account.",
                    "wiki-bot needs FEISHU_USER_ACCESS_TOKEN and a user allowed to manage the target Wiki space.",
                    "For multi-account Chrome, verify the intended Playwright MCP profile before approving account-sensitive grants.",
                    "setup never prints app_secret or raw tokens; doctor/setup mask configured secrets."
                ]
            },
            {
                "name": "dogfood",
                "command": "feishu-bot dogfood",
                "scope_group": "doc",
                "status": "closed-loop high-level workflow",
                "ai_use": "Verify current-account module readiness with real OpenAPI probes; publish one standalone capability demo doc, read it back, attempt Wiki when configured, send it to the configured receiver, and verify the exact link message through message/chat/member probes.",
                "help": ["feishu-bot dogfood --help", "feishu-bot dogfood verify --help", "feishu-bot dogfood publish --help"],
                "examples": [
                    "feishu-bot dogfood verify",
                    "feishu-bot dogfood verify --module calendar --module task --include-response",
                    "feishu-bot dogfood verify --write --module doc --module base --module task",
                    "feishu-bot dogfood verify --write --module board --include-response",
                    "feishu-bot dogfood publish --title \"Base role v2 demo\" --file dogfood/demo.md",
                    "feishu-bot dogfood publish --title \"HTML demo\" --content-type html --file demo.html",
                    "feishu-bot dogfood publish --title \"Non-Wiki draft\" --file demo.md --no-wiki"
                ],
                "known_permission_edges": [
                    "dogfood verify exits successfully even when probes fail; inspect data.summary and per-probe status.",
                    "Failed dogfood verify probes include remediation JSON with grant URLs, browser commands, env vars, and rerun commands.",
                    "dogfood verify defaults to read-only probes. --write and --send-loop-check intentionally create Feishu data.",
                    "Defaults to FEISHU_USER_ID for delivery and returns an error if no receiver is configured.",
                    "Wiki move is attempted when FEISHU_DOC_CREATE_WIKI_DEFAULT or FEISHU_WIKI_SPACE_ID is configured; failures are returned as wiki_move_error while the fallback docx is still sent.",
                    "send_loop_check.closed_loop is the required proof before claiming the demo was sent."
                ]
            },
            {
                "name": "office",
                "command": "feishu-bot office",
                "layer": "workflow",
                "scope_group": "im,wiki,doc,base,search",
                "status": "AI-first high-level workflows over atomic commands",
                "ai_use": "Default daily interface for one-human-plus-AI work. Bootstrap an isolated project group, Wiki index, Base log, and tabs; write one independent report doc per demo; send lightweight progress updates into chat/Base; send concise chat notifications and voice reports; poll the project inbox with cursor state and ack/reply defaults; search project messages/docs; inspect readback status; and clean local project state safely.",
                "state": {
                    "default_path": "~/.config/feishu/office-projects.json",
                    "override_env": "FEISHU_OFFICE_STATE_FILE",
                    "schema": ["project", "name", "chat_id", "wiki_space_id", "wiki_index_node_token", "wiki_index_obj_token", "base_node_token", "base_app_token", "base_table_id", "pinned_summary_message_id", "created_at", "updated_at"]
                },
                "help": ["feishu-bot office --help", "feishu-bot office list --help", "feishu-bot office bootstrap --help", "feishu-bot office report --help", "feishu-bot office progress --help", "feishu-bot office voice-report --help", "feishu-bot office inbox --help", "feishu-bot office poll --help", "feishu-bot office status --help", "feishu-bot office search --help", "feishu-bot office cleanup --help"],
                "examples": [
                    "feishu-bot office list",
                    "feishu-bot office bootstrap --project \"AI项目\" --dry-run",
                    "feishu-bot office bootstrap --project \"AI项目\" --user \"$FEISHU_USER_ID\" --space-id \"$FEISHU_WIKI_SPACE_ID\" --send-summary",
                    "feishu-bot office report --project \"AI项目\" --title \"功能演示\" --file ./demo.md --dry-run",
                    "feishu-bot office report --project \"AI项目\" --title \"功能演示\" --file ./demo.md --base-record --pin",
                    "feishu-bot office report --project \"AI项目\" --title \"HTML演示\" --content-type html --file ./demo.html",
                    "feishu-bot office progress --project \"AI项目\" --title \"进度更新\" --status doing --summary \"当前进展\"",
                    "feishu-bot office progress --project \"AI项目\" --title \"阶段总结\" --file ./summary.md --wiki-report --pin",
                    "feishu-bot office voice-report --project \"AI项目\" --text \"语音汇报内容\"",
                    "feishu-bot office inbox --project \"AI项目\" --from-now",
                    "feishu-bot office inbox --project \"AI项目\" --reply-text \"收到，我来处理\"",
                    "feishu-bot office poll --project \"AI项目\" --ack-emoji OK --reply-text \"收到，我来处理\" --mark-seen",
                    "feishu-bot office search --project \"AI项目\" --query \"需求\"",
                    "feishu-bot office status --project \"AI项目\" --check",
                    "feishu-bot office cleanup --project \"AI项目\" --dry-run"
                ],
                "known_permission_edges": [
                    "list, bootstrap --dry-run, report --dry-run, and status without --check do not call Feishu OpenAPI and can run before credentials are configured.",
                    "bootstrap writes real Feishu resources unless --dry-run or --skip-wiki/--skip-base/--skip-tabs are used.",
                    "report defaults to project Wiki when wiki_space_id is available; --no-wiki creates a standalone docx.",
                    "progress is the default lightweight update path: chat message plus Base row, with optional Wiki/docx detail report.",
                    "inbox is a safer wrapper over poll for daily use: it defaults to ack emoji OK and mark-seen cursor saving.",
                    "poll uses the same local cursor mechanics as message poll and stores state separately with state_key office:<project>.",
                    "search uses user-token Feishu search APIs and needs FEISHU_USER_ACCESS_TOKEN.",
                    "cleanup does not delete Wiki/Base/chat resources by default; it only deletes known messages when --confirm --delete-messages is set and removes local state when confirmed."
                ],
                "atomic_fallbacks": ["feishu-bot message --help", "feishu-bot chat --help", "feishu-bot wiki --help", "feishu-bot doc --help", "feishu-bot base --help", "feishu-bot search --help"]
            },
            {
                "name": "message",
                "command": "feishu-bot message",
                "scope_group": "im",
                "status": "typed wrappers plus native JSON payloads",
                "ai_use": "Send/read/reply/edit/delete messages; upload and send images/files/videos/audio; download resources; poll project chats with a local cursor; ack user messages with reaction status markers; list read users, reactions, and pins.",
                "help": ["feishu-bot message --help", "feishu-bot message reply --help", "feishu-bot message ack --help", "feishu-bot message poll --help", "feishu-bot message upload-image --help", "feishu-bot message upload-file --help", "feishu-bot message list --help", "feishu-bot message reaction --help", "feishu-bot message pin --help"],
                "examples": [
                    "feishu-bot message send --to \"$FEISHU_USER_ID\" --text \"hello\"",
                    "feishu-bot message loop-check --to \"$FEISHU_USER_ID\" --to-type open-id",
                    "feishu-bot message send-image --to \"$FEISHU_USER_ID\" --file ./image.png",
                    "feishu-bot message send-file --to \"$FEISHU_USER_ID\" --file ./demo.mp4 --file-type mp4",
                    "feishu-bot message send-file --to \"$FEISHU_USER_ID\" --file ./voice.opus --file-type opus --duration 3000",
                    "feishu-bot message send-voice --to \"$FEISHU_USER_ID\" --file ./voice.mp3 --readback",
                    "feishu-bot message send-voice --to \"$FEISHU_USER_ID\" --text \"语音播报内容\" --readback",
                    "feishu-bot message reply --message-id om_xxx --text \"收到，我来处理\"",
                    "feishu-bot message ack --message-id om_xxx --emoji-type OK --reply-text \"已读，开始处理\" --readback",
                    "feishu-bot message poll --chat-id oc_xxx --from-now --mark-seen",
                    "feishu-bot message poll --chat-id oc_xxx --ack-emoji OK --reply-text \"收到\" --mark-seen",
                    "feishu-bot message list --container-id oc_xxx --container-id-type chat --page-size 20",
                    "feishu-bot message resource --message-id om_xxx --file-key file_xxx --type file --output ./download.bin"
                ],
                "known_permission_edges": [
                    "reaction list needs im:message.reactions:read",
                    "message ack uses reactions as workflow status markers; it is not an official Feishu read receipt",
                    "message read-users only reports Feishu read-user data for bot-sent messages within Feishu's sender/read-user limits",
                    "message poll stores a local cursor under ~/.config/feishu/message-state.json by default and ignores app/bot/system messages unless explicitly included",
                    "message image/file upload and resource download need im:resource or im:resource:upload",
                    "image upload is limited to 10 MB; file/video upload is limited to 30 MB",
                    "send-file --msg-type auto maps mp4 to media, opus to audio, and other files to file",
                    "send-voice needs ffmpeg/ffprobe for non-OPUS files and vox when synthesizing from text",
                    "Use message loop-check for dogfood; it proves send/get/list/chat/member/read-users through CLI before reporting that the human-visible send loop works."
                ]
            },
            {
                "name": "chat",
                "command": "feishu-bot chat",
                "scope_group": "im",
                "status": "typed wrappers plus raw JSON escape hatches",
                "ai_use": "Discover chats, inspect metadata, create/update/delete project groups, manage members, set avatars, and operate chat tabs/menus.",
                "help": ["feishu-bot chat --help", "feishu-bot chat member --help", "feishu-bot chat tab --help", "feishu-bot chat menu --help"],
                "examples": [
                    "feishu-bot chat list --page-size 20",
                    "feishu-bot chat create --name \"AI 项目群\" --user \"$FEISHU_USER_ID\" --avatar-file ./avatar.png",
                    "feishu-bot chat update --chat-id oc_xxx --name \"AI 项目群 v2\" --avatar-file ./avatar.png",
                    "feishu-bot chat tab add --chat-id oc_xxx --name \"项目页\" --tab-type url --url https://example.com",
                    "feishu-bot chat menu add --chat-id oc_xxx --body-file ./menu-tree.json",
                    "feishu-bot chat member list --chat-id oc_xxx"
                ],
                "known_permission_edges": [
                    "Create/update chat and member management need group/chat permissions and bot ability.",
                    "Group avatars use image upload with image_type=avatar.",
                    "Chat tabs only support typed doc/url create/update/delete; other tab types are client-only or read-only through OpenAPI.",
                    "Chat menus require the bot/user to be in the group and may require group tab/menu/widget management permission.",
                    "chat delete dissolves the group for everyone; it is not a client-side hide/remove-left-sidebar operation.",
                    "Personal left-sidebar labels/folders in the Feishu client are not exposed by the group OpenAPI; use project groups, avatars, tabs, menus, pins, search, and feed-card APIs instead."
                ]
            },
            {
                "name": "contact",
                "command": "feishu-bot contact",
                "scope_group": "contact",
                "status": "typed wrappers",
                "ai_use": "Resolve users/departments before sending or sharing.",
                "help": ["feishu-bot contact --help"],
                "examples": ["feishu-bot contact user get --user-id \"$FEISHU_USER_ID\""]
            },
            {
                "name": "directory",
                "command": "feishu-bot directory",
                "scope_group": "directory",
                "status": "typed wrappers with tenant/user-token reads and raw JSON filter escape hatch",
                "ai_use": "Search employees by keyword, batch-get employee fields, and filter employees by email, mobile, department/status, or job number.",
                "help": [
                    "feishu-bot directory --help",
                    "feishu-bot directory employee --help",
                    "feishu-bot directory employee search --help",
                    "feishu-bot directory employee mget --help",
                    "feishu-bot directory employee filter --help"
                ],
                "examples": [
                    "feishu-bot directory employee search --query \"张三\" --page-size 10",
                    "feishu-bot directory employee mget --employee-id <open_id> --field base_info.name",
                    "feishu-bot directory employee filter --condition 'base_info.email=eq=\"user@example.com\"'"
                ],
                "known_permission_edges": [
                    "Tenant-token reads follow the app contact range.",
                    "User-token reads follow the admin range of FEISHU_USER_ACCESS_TOKEN.",
                    "Fields must be requested explicitly and each sensitive field has its own directory:* field scope."
                ]
            },
            {
                "name": "doc",
                "command": "feishu-bot doc",
                "scope_group": "doc",
                "status": "typed markdown writer, official converter, media insertion, and raw block escape hatch",
                "ai_use": "Create/write/read docx docs, insert image/file media, preview block output, print templates, append raw blocks, send links with delivery proof, and optionally/default move newly created docs into Wiki.",
                "help": ["feishu-bot doc capabilities", "feishu-bot doc create --help", "feishu-bot doc insert-media --help", "feishu-bot doc template --kind all", "feishu-bot doc preview --file notes.md"],
                "examples": [
                    "feishu-bot doc create --writer official --title \"Report\" --file report.md",
                    "feishu-bot doc append --auth user --document-id <wiki_obj_token> --writer official --file report.md",
                    "feishu-bot doc raw --auth user --document-id <wiki_obj_token>",
                    "feishu-bot doc create --writer official --title \"Report\" --file report.md --send-to \"$FEISHU_USER_ID\" --send-loop-check",
                    "feishu-bot doc send-link --document-id docx_xxx --to \"$FEISHU_USER_ID\" --send-loop-check",
                    "feishu-bot doc insert-media --document-id docx_xxx --kind image --file ./image.png --width 640 --align 2",
                    "feishu-bot doc insert-media --document-id docx_xxx --kind file --file ./attachment.pdf --view-type 1",
                    "FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --writer official --title \"Report\" --file report.md",
                    "feishu-bot doc create --writer official --title \"Report\" --file report.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok",
                    "feishu-bot doc append-json --document-id docx_xxx --block-id docx_xxx --file blocks.json"
                ],
                "format_notes": [
                    "Mermaid fenced code is preserved as source in docx code blocks.",
                    "Use doc insert-media for normal images and file attachments; it creates the block, uploads media, and patches the token.",
                    "Renderable Mermaid/PlantUML should use feishu-bot board import.",
                    "Public docx OpenAPI cannot create every UI-only block, such as writable mindnote blocks."
                ],
                "known_permission_edges": [
                    "Use --send-loop-check with --send-to during dogfood; it verifies the exact doc link message through message get/list, chat metadata, chat members, and read-users."
                ]
            },
            {
                "name": "board",
                "command": "feishu-bot board",
                "scope_group": "board",
                "status": "typed wrappers and raw node escape hatch",
                "ai_use": "Create/read whiteboards and import Mermaid/PlantUML source as board nodes.",
                "help": ["feishu-bot board --help"],
                "examples": ["feishu-bot board import --syntax mermaid --file graph.mmd"]
            },
            {
                "name": "base",
                "command": "feishu-bot base",
                "scope_group": "base",
                "status": "typed wrappers",
                "ai_use": "Parse Base links; create/copy Base apps; manage tables, typed fields, views, records, attachment media, dashboards, workflows, forms, advanced permission roles including advanced permissions 2.0 base_rule, and role members.",
                "help": [
                    "feishu-bot base --help",
                    "feishu-bot base parse-url --help",
                    "feishu-bot base table --help",
                    "feishu-bot base field --help",
                    "feishu-bot base record --help",
                    "feishu-bot base media --help",
                    "feishu-bot base dashboard --help",
                    "feishu-bot base workflow --help",
                    "feishu-bot base form --help",
                    "feishu-bot base role --help",
                    "feishu-bot base member --help"
                ],
                "examples": [
                    "feishu-bot base parse-url 'https://example.feishu.cn/base/appxxx?table=tblxxx&view=vewxxx'",
                    "feishu-bot base create --name \"AI Tasks\"",
                    "feishu-bot base table create --app-token app_xxx --name \"Requests\" --default-view-name \"Default\" --field \"Title:text\" --field \"Status:single-select:Open:0|Done:1\" --field \"Amount:currency:0.00|CNY\"",
                    "feishu-bot base field create --app-token app_xxx --table-id tbl_xxx --name \"Status\" --kind single-select --option \"Open:0\" --option \"Done:1\"",
                    "feishu-bot base field create --app-token app_xxx --table-id tbl_xxx --name \"Amount\" --kind currency --formatter \"0.00\" --currency-code CNY",
                    "feishu-bot base field update --app-token app_xxx --table-id tbl_xxx --field-id fld_xxx --name \"Stage\" --kind multi-select --option \"Doing:2\" --option \"Blocked:3\"",
                    "feishu-bot base field list --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --text-field-as-array",
                    "feishu-bot base view update --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --hidden-field-id fld_internal --filter-conjunction and --filter-condition 'fld_status:3:is:json:[\"opt_done\"]' --hierarchy-field-id fld_parent",
                    "feishu-bot base record create --app-token app_xxx --table-id tbl_xxx --field \"Name=demo\" --field \"Score=12.5\" --field \"Done=true\"",
                    "feishu-bot base record create --app-token app_xxx --table-id tbl_xxx --field \"Due=date:2026-06-02\" --field \"ReviewAt=datetime:2026-06-02T10:30:00+08:00\"",
                    "feishu-bot base record update --app-token app_xxx --table-id tbl_xxx --record-id rec_xxx --field \"Status=done\" --field \"Clear=null\"",
                    "feishu-bot base record search --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --field-name \"Name\" --automatic-fields",
                    "feishu-bot base record search --app-token app_xxx --table-id tbl_xxx --filter-json '{\"conjunction\":\"and\",\"conditions\":[]}' --sort-json '[]'",
                    "feishu-bot base record batch-create --app-token app_xxx --table-id tbl_xxx --record-field \"0:Name=A\" --record-field \"1:Name=B\"",
                    "feishu-bot base record batch-update --app-token app_xxx --table-id tbl_xxx --record-id rec_a --record-id rec_b --record-field \"0:Status=done\" --record-field \"1:Clear=null\"",
                    "feishu-bot base record batch-create --app-token app_xxx --table-id tbl_xxx --records-json '[{\"fields\":{\"Name\":\"demo\"}}]'",
                    "feishu-bot base media upload --app-token app_xxx --kind file --file ./demo.mp4",
                    "feishu-bot base media field-value --file-token <file_token> --field \"附件\"",
                    "feishu-bot base workflow block-list --app-token app_xxx",
                    "feishu-bot base workflow update --app-token app_xxx --workflow-id wfl_xxx --status disable",
                    "feishu-bot base role list --app-token app_xxx --api-version v2",
                    "feishu-bot base role create --app-token app_xxx --api-version v2 --name \"Readonly\" --table-roles-json '[{\"table_id\":\"tbl_xxx\",\"table_perm\":1}]' --allow-base-complex-edit false --allow-copy false",
                    "feishu-bot base member batch-add --app-token app_xxx --role-id rol_xxx --member open_id:ou_xxx"
                ],
                "known_permission_edges": [
                    "Existing user-owned Bases may also require adding the app as a collaborator inside the Base.",
                    "base table create supports repeated --field name:kind[:config] for common table.fields; use --fields-json for native Feishu payloads.",
                    "Base media upload returns a file_token that still has to be written into an attachment field through base record create/update.",
                    "Base record date fields accept date:YYYY-MM-DD and datetime:<RFC3339/local time>; when field metadata is readable, plain YYYY-MM-DD or YYYY/MM/DD strings are converted automatically for date fields.",
                    "View property flags cover common hidden_fields, filter_info, and hierarchy_config edits; use --property-json for newer Feishu view capabilities.",
                    "Advanced permission role/member commands require advanced permissions enabled and manageable permission on the Base.",
                    "For advanced permissions 2.0 custom roles, prefer base role list/create --api-version v2; v2 supports base_rule.base_complex_edit and base_rule.copy."
                ]
            },
            {
                "name": "task",
                "command": "feishu-bot task",
                "scope_group": "task",
                "status": "typed wrappers and raw JSON bodies",
                "ai_use": "Create/update/read/delete tasks with typed due/start RFC3339/local/date/millisecond times, repeat_rule, custom_complete, origin, extra, mode, milestones, reminders, and custom_fields; complete/reopen tasks; manage task members, reminders, dependencies, task-tasklist links, tasklists, tasklist collaborators, custom sections, custom fields/options/values, attachments, full CRUD comments, and subtasks.",
                "help": ["feishu-bot task --help", "feishu-bot task tasklist --help", "feishu-bot task section --help", "feishu-bot task custom-field --help", "feishu-bot task attachment --help", "feishu-bot task member --help", "feishu-bot task reminder --help", "feishu-bot task dependency --help", "feishu-bot task comment --help", "feishu-bot task subtask --help"],
                "examples": [
                    "feishu-bot task create --summary \"Follow up\"",
                    "feishu-bot task create --summary \"Review proposal\" --due-at 2026-06-02T15:00:00+08:00 --start-date 2026-06-02",
                    "feishu-bot task create --summary \"Submit proposal\" --due-at \"2026-06-03 18:00\" --reminder-minute 30",
                    "feishu-bot task create --summary \"All-day milestone\" --due-date 2026-06-05 --mode 1 --is-milestone true",
                    "feishu-bot task create --summary \"Weekly sync\" --due-ms 1780000000000 --due-all-day --repeat-rule \"FREQ=WEEKLY;INTERVAL=1\"",
                    "feishu-bot task create --summary \"External ticket\" --origin-json '{\"platform_i18n_name\":{\"en_us\":\"AI System\"},\"href\":{\"url\":\"https://example.com/t/1\"}}' --custom-complete-json '{\"pc\":{\"tip\":{\"en_us\":\"Finish in the source system\"}}}' --extra eyJzb3VyY2UiOiJhaSJ9",
                    "feishu-bot task list --completed false --type my_tasks",
                    "feishu-bot task update --guid <guid> --due-at 2026-06-03T18:00:00+08:00 --mode 1 --is-milestone true",
                    "feishu-bot task update --guid <guid> --clear-start --clear-repeat-rule --clear-custom-complete",
                    "feishu-bot task member add --task-guid <guid> --assignee ou_xxx",
                    "feishu-bot task add-tasklist --task-guid <guid> --tasklist-guid <tasklist_guid> --section-guid <section_guid>",
                    "feishu-bot task section create --resource-type tasklist --resource-id <tasklist_guid> --name \"In progress\"",
                    "feishu-bot task custom-field create --resource-id <tasklist_guid> --name \"Priority\" --type single_select --option High --option Medium --option Low",
                    "feishu-bot task custom-field set-value --task-guid <guid> --custom-field-guid <field_guid> --type single-select --option-guid <option_guid>",
                    "feishu-bot task attachment upload --resource-id <task_guid> --file ./brief.pdf",
                    "feishu-bot task reminder add --task-guid <guid> --reminder-minute 30",
                    "feishu-bot task dependency add --task-guid <guid> --dependency-task-guid <next_guid>",
                    "feishu-bot task tasklist add-member --tasklist-guid <tasklist_guid> --editor ou_xxx",
                    "feishu-bot task comment create --task-guid <guid> --content \"done\"",
                    "feishu-bot task comment update --comment-id <comment_id> --content \"updated\"",
                    "feishu-bot task comment delete --comment-id <comment_id>"
                ],
                "known_permission_edges": [
                    "feishu-bot task list defaults to --auth user because Feishu's task list API requires user_access_token and returns the caller's my-tasks view; use --completed true|false to filter done/undone tasks.",
                    "Core task/tasklist/member/reminder/subtask commands plus section/custom-field/attachment/dependency/comment wrappers support --auth tenant|user.",
                    "Use --due-at/--start-at for RFC3339 or local timestamps, --due-date/--start-date for all-day dates, and --due-ms/--start-ms only when millisecond values are already available.",
                    "Task reminders are relative to due time; use --reminder-minute and change existing reminders by remove then add because Feishu currently supports one reminder per task.",
                    "Tenant-token task calls operate on app-owned task visibility; user-token calls require FEISHU_USER_ACCESS_TOKEN and match that user's Feishu Task Center visibility.",
                    "Task dependency add/remove also requires edit permission on the involved tasks."
                ]
            },
            {
                "name": "drive",
                "command": "feishu-bot drive",
                "scope_group": "drive",
                "status": "typed wrappers",
                "ai_use": "Upload/download Drive files, including multipart large Drive uploads; upload/download doc/sheet/Base media assets; import local files into online docs; export cloud docs to local files; manage folders, permissions, comments, versions, subscriptions, and view records.",
                "help": ["feishu-bot drive --help", "feishu-bot drive media --help", "feishu-bot drive import --help", "feishu-bot drive export --help", "feishu-bot drive comment --help", "feishu-bot drive version --help", "feishu-bot drive subscription --help", "feishu-bot drive view-record --help", "feishu-bot drive folder --help", "feishu-bot drive permission --help"],
                "examples": [
                    "feishu-bot drive upload --file ./report.pdf --folder-token <folder_token>",
                    "feishu-bot drive upload-large --file ./large-video.mp4 --folder-token <folder_token>",
                    "feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png",
                    "feishu-bot drive import file --file ./page.html --type docx --folder-token \"\" --title \"HTML Preview\"",
                    "feishu-bot drive export file --token <docx_token> --type docx --file-extension pdf --output ./doc.pdf",
                    "feishu-bot drive comment create --file-token <docx_token> --file-type docx --text \"需要复核\"",
                    "feishu-bot drive version create --file-token <docx_token> --obj-type docx --name \"AI 修订版\"",
                    "feishu-bot drive permission member-list --token <docx_token> --file-type docx",
                    "feishu-bot drive permission member-add --token <docx_token> --file-type docx --member-id \"$FEISHU_USER_ID\" --perm view"
                ],
                "known_permission_edges": [
                    "drive upload uses drive/v1/files/upload_all for Drive files up to 20 MB.",
                    "drive upload-large uses drive/v1/files/upload_prepare, upload_part, and upload_finish for larger Drive files.",
                    "drive media upload uses drive/v1/medias/upload_all for docx/sheet/bitable/import assets up to 20 MB.",
                    "drive export supports doc/docx to pdf/docx and sheet/bitable to xlsx/csv; exported files are temporary.",
                    "drive comment wrappers cover global comments and replies; local comments are readable through list/batch-get but not created by public OpenAPI.",
                    "drive subscription create/get/update require FEISHU_USER_ACCESS_TOKEN.",
                    "drive view-record requires document management permission and drive:file:view_record:readonly.",
                    "HTML online preview should be created as native docx through doc writer or drive import, not treated as raw HTML hosting."
                ]
            },
            {
                "name": "calendar",
                "command": "feishu-bot calendar",
                "scope_group": "calendar",
                "status": "typed wrappers",
                "ai_use": "List/create calendars; create/list/update/delete events; query one or many users/rooms free-busy; add/list/delete event attendees and list chat-attendee members.",
                "help": ["feishu-bot calendar --help", "feishu-bot calendar event --help", "feishu-bot calendar freebusy --help", "feishu-bot calendar attendee --help"],
                "examples": [
                    "feishu-bot calendar freebusy list --user-id \"$FEISHU_USER_ID\" --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00",
                    "feishu-bot calendar freebusy batch --user-id ou_xxx --user-id ou_yyy --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00",
                    "feishu-bot calendar event create --calendar-id primary --summary \"Sync\" --start-ts 1780202400 --end-ts 1780204200",
                    "feishu-bot calendar attendee add --calendar-id <calendar_id> --event-id <event_id> --user \"$FEISHU_USER_ID\"",
                    "feishu-bot calendar attendee list --calendar-id <calendar_id> --event-id <event_id>"
                ]
            },
            {
                "name": "vc",
                "command": "feishu-bot vc",
                "scope_group": "vc",
                "status": "typed readable and meeting-operation wrappers",
                "ai_use": "Reserve/update/delete video meetings, read active meetings/details/history/recordings/reports/rooms, invite participants, set hosts, end meetings, and start/stop/share recordings.",
                "help": ["feishu-bot vc --help", "feishu-bot vc reserve --help", "feishu-bot vc meeting --help", "feishu-bot vc recording --help", "feishu-bot vc report --help", "feishu-bot vc room --help"],
                "examples": [
                    "feishu-bot vc reserve apply --end-time <sec> --owner-id <open_id> --topic \"AI sync\"",
                    "feishu-bot vc reserve active-meeting --reserve-id <reserve_id> --with-participants",
                    "feishu-bot vc meeting get --meeting-id <meeting_id>",
                    "feishu-bot vc meeting invite --meeting-id <meeting_id> --user <open_id>",
                    "feishu-bot vc meeting set-host --meeting-id <meeting_id> --user-id <open_id>",
                    "feishu-bot vc recording start --meeting-id <meeting_id> --timezone 8",
                    "feishu-bot vc recording set-permission --meeting-id <meeting_id> --user <open_id>",
                    "feishu-bot vc report daily --start-time <sec> --end-time <sec>",
                    "feishu-bot vc room list --page-size 20"
                ],
                "known_permission_edges": [
                    "Tenant-token reserve apply requires --owner-id.",
                    "Meeting detail reads may require vc:meeting:readonly or vc:meeting.meetingevent:read.",
                    "Room reads may require vc:room, vc:room:readonly, or vc:rooms.room.basicinfo:read.",
                    "Report reads may require vc:report:readonly.",
                    "Set-host can require both vc:meeting and vc:meeting.participant:write.",
                    "Invite/end/recording start/stop/permission usually require FEISHU_USER_ACCESS_TOKEN and the operator must be in the meeting or host.",
                    "Reserve-created meetings do not create Calendar events; use calendar event commands if a calendar event is required."
                ]
            },
            {
                "name": "minutes",
                "command": "feishu-bot minutes",
                "scope_group": "minutes",
                "status": "typed wrappers plus transcript binary export",
                "ai_use": "Search Feishu Minutes, read metadata, fetch AI artifacts, get media download URLs, and export transcripts.",
                "help": ["feishu-bot minutes --help", "feishu-bot minutes search --help", "feishu-bot minutes transcript --help"],
                "examples": [
                    "feishu-bot minutes search --query \"周会\" --page-size 20",
                    "feishu-bot minutes get --minute-token <minute_token_or_url>",
                    "feishu-bot minutes transcript --minute-token <minute_token_or_url> --need-speaker --need-timestamp --file-format txt --output ./minute.txt"
                ],
                "known_permission_edges": [
                    "minutes search requires user_access_token via FEISHU_USER_ACCESS_TOKEN.",
                    "Metadata reads may require minutes:minutes, minutes:minutes:readonly, or minutes:minutes.basic:read.",
                    "AI artifact reads require minutes:minutes.artifacts:read.",
                    "Media download URLs may require minutes:minute:download or minutes:minutes.media:export.",
                    "transcript/media export also depends on the Minute file export settings and app data access range."
                ]
            },
            {
                "name": "search",
                "command": "feishu-bot search",
                "scope_group": "search",
                "status": "typed wrappers for docs/message search and custom search connector indexing",
                "ai_use": "Search visible Feishu docs/wiki/messages and manage custom search data sources, schemas, and indexed items.",
                "help": ["feishu-bot search --help", "feishu-bot search docs --help", "feishu-bot search message --help", "feishu-bot search source --help", "feishu-bot search item --help"],
                "examples": [
                    "feishu-bot search docs --query \"飞书Bot\" --page-size 10",
                    "feishu-bot search message --query \"上线\" --chat-id oc_xxx --page-size 20",
                    "feishu-bot search item create --data-source-id <id> --id item_1 --title \"标题\" --url \"https://example.com\" --text \"全文\""
                ],
                "known_permission_edges": [
                    "docs and message search require FEISHU_USER_ACCESS_TOKEN.",
                    "custom search connector APIs may require an eligible Feishu plan in addition to search:data_source scopes."
                ]
            },
            {
                "name": "okr",
                "command": "feishu-bot okr",
                "scope_group": "okr",
                "status": "tenant-token readable wrappers",
                "ai_use": "Read OKR periods, period rules, one user's OKR list, and batch fetch OKR details.",
                "help": ["feishu-bot okr --help", "feishu-bot okr period --help", "feishu-bot okr user-okrs --help", "feishu-bot okr batch-get --help"],
                "examples": [
                    "feishu-bot okr period list --page-size 20",
                    "feishu-bot okr period-rule list",
                    "feishu-bot okr user-okrs --user-id \"$FEISHU_USER_ID\" --offset 0 --limit 5",
                    "feishu-bot okr batch-get --okr-id <okr_id>"
                ],
                "known_permission_edges": [
                    "OKR APIs require scopes such as okr:okr.period:readonly, okr:okr:readonly, or okr:okr.",
                    "User OKR list reads may require okr:okr.content:readonly.",
                    "Some tenants require Feishu OKR enterprise edition before period-rule or OKR reads are available."
                ]
            },
            {
                "name": "attendance",
                "command": "feishu-bot attendance",
                "scope_group": "attendance",
                "status": "tenant-token wrappers with raw JSON write escape hatches",
                "ai_use": "Read attendance groups, shifts, user schedules, task results, flow records, and statistics; import/delete flow records with explicit raw JSON.",
                "help": [
                    "feishu-bot attendance --help",
                    "feishu-bot attendance group --help",
                    "feishu-bot attendance shift --help",
                    "feishu-bot attendance schedule query --help",
                    "feishu-bot attendance task query --help",
                    "feishu-bot attendance flow --help",
                    "feishu-bot attendance stats query --help"
                ],
                "examples": [
                    "feishu-bot attendance group list --page-size 20",
                    "feishu-bot attendance shift list --page-size 20",
                    "feishu-bot attendance schedule query --user-id <employee_id> --from 20260501 --to 20260531",
                    "feishu-bot attendance task query --user-id <employee_id> --from 20260501 --to 20260531 --ignore-invalid-users",
                    "feishu-bot attendance flow query --user-id <employee_id> --from-ts 1760000000 --to-ts 1760086400"
                ],
                "known_permission_edges": [
                    "Attendance group and shift reads require attendance:rule or attendance:rule:readonly.",
                    "Schedules, task results, flow records, and stats require attendance:task or attendance:task:readonly.",
                    "Attendance APIs also depend on Feishu People/Attendance edition and attendance management data range.",
                    "flow delete accepts at most 10 imported record IDs per request."
                ]
            },
            {
                "name": "mail",
                "command": "feishu-bot mail",
                "scope_group": "mail",
                "status": "typed wrappers with user-token send and tenant/user-token reads",
                "ai_use": "List/read/send Mail messages and inspect folders, contacts, aliases, sendable addresses, accessible mailboxes, rules, and labels.",
                "help": [
                    "feishu-bot mail --help",
                    "feishu-bot mail message --help",
                    "feishu-bot mail message send --help",
                    "feishu-bot mail folder --help",
                    "feishu-bot mail contact --help",
                    "feishu-bot mail settings --help"
                ],
                "examples": [
                    "feishu-bot mail message list --mailbox me --page-size 10",
                    "feishu-bot mail message get --mailbox me --message-id <message_id> --format metadata",
                    "feishu-bot mail folder list --mailbox me",
                    "feishu-bot mail settings send-as --mailbox me",
                    "feishu-bot mail message send --mailbox me --to user@example.com --subject \"hello\" --text \"body\""
                ],
                "known_permission_edges": [
                    "mailbox=me and message send require FEISHU_USER_ACCESS_TOKEN.",
                    "Tenant-token reads of explicit mailboxes require Mail data resource permissions.",
                    "Full message bodies, subjects, addresses, and contact fields need separate Mail field scopes."
                ]
            },
            {
                "name": "corehr",
                "command": "feishu-bot corehr",
                "scope_group": "corehr",
                "status": "tenant-token readable wrappers with raw JSON query escape hatches",
                "ai_use": "Search/batch-get CoreHR departments, list/get/batch-get jobs, query/get employee job data, get personal information, and list/get process instances.",
                "help": [
                    "feishu-bot corehr --help",
                    "feishu-bot corehr department --help",
                    "feishu-bot corehr job --help",
                    "feishu-bot corehr job-data --help",
                    "feishu-bot corehr process --help"
                ],
                "examples": [
                    "feishu-bot corehr department search --page-size 20 --field department_name",
                    "feishu-bot corehr job list --page-size 20",
                    "feishu-bot corehr job-data query --employment-id <id> --page-size 20",
                    "feishu-bot corehr process list --modify-time-from <ms> --modify-time-to <ms>"
                ],
                "known_permission_edges": [
                    "CoreHR APIs require both Open Platform scopes and Feishu People data-range grants.",
                    "Sensitive fields such as department manager/custom fields, job levels, job data fields, and user_id need separate field scopes."
                ]
            },
            {
                "name": "helpdesk",
                "command": "feishu-bot helpdesk",
                "scope_group": "helpdesk",
                "status": "typed wrappers with service-desk token header and raw JSON bodies",
                "ai_use": "List/get Helpdesk tickets, list ticket messages, start service conversations, send helpdesk bot messages, and read FAQ categories/articles.",
                "help": [
                    "feishu-bot helpdesk --help",
                    "feishu-bot helpdesk ticket --help",
                    "feishu-bot helpdesk service --help",
                    "feishu-bot helpdesk message --help",
                    "feishu-bot helpdesk faq --help"
                ],
                "examples": [
                    "feishu-bot helpdesk ticket list --page-size 20",
                    "feishu-bot helpdesk ticket get --ticket-id <ticket_id>",
                    "feishu-bot helpdesk ticket messages --ticket-id <ticket_id>",
                    "feishu-bot helpdesk service start --open-id <open_id> --human-service",
                    "feishu-bot helpdesk message send --receiver-id <open_id> --text \"hello\"",
                    "feishu-bot helpdesk faq list --search \"登录\" --page-size 20"
                ],
                "known_permission_edges": [
                    "Helpdesk APIs require FEISHU_HELPDESK_ID and FEISHU_HELPDESK_TOKEN from the Helpdesk admin API credential page.",
                    "The CLI sends X-Lark-Helpdesk-Authorization as base64(helpdesk_id:helpdesk_token).",
                    "Ticket and FAQ reads need helpdesk:all:readonly; service start needs helpdesk:helpdesk:access; bot message send needs helpdesk:all."
                ]
            },
            {
                "name": "hire",
                "command": "feishu-bot hire",
                "scope_group": "hire",
                "status": "typed wrappers for core recruiting reads plus explicit raw JSON writes",
                "ai_use": "List/read Hire jobs, job schemas, talents, applications, application details, interviews, processes, requirement schemas, metadata, locations, and attachments; create talents and reopen jobs when explicitly requested.",
                "help": [
                    "feishu-bot hire --help",
                    "feishu-bot hire job --help",
                    "feishu-bot hire talent --help",
                    "feishu-bot hire application --help",
                    "feishu-bot hire interview --help",
                    "feishu-bot hire metadata --help",
                    "feishu-bot hire location --help"
                ],
                "examples": [
                    "feishu-bot hire job list --page-size 20",
                    "feishu-bot hire job detail --job-id <job_id>",
                    "feishu-bot hire talent list --keyword \"张三\" --page-size 10",
                    "feishu-bot hire application detail --application-id <application_id> --option with_job --option with_talent",
                    "feishu-bot hire interview by-talent --talent-id <talent_id>",
                    "feishu-bot hire process list --page-size 50",
                    "feishu-bot hire metadata resume-sources --page-size 20"
                ],
                "known_permission_edges": [
                    "Hire APIs require Feishu Hire product availability and Hire data-range grants in addition to Open Platform scopes.",
                    "Sensitive user_id fields require contact:user.employee_id:readonly.",
                    "Application detail options such as with_offer, with_agency, with_referral, and with_portal require their corresponding hire:* readonly scopes.",
                    "Tenant-specific custom fields and schema-bound writes should use --body-json/--file/--stdin copied from official OpenAPI explorer."
                ]
            },
            {
                "name": "wiki",
                "command": "feishu-bot wiki",
                "scope_group": "wiki",
                "status": "typed wrappers",
                "ai_use": "Diagnose the default Wiki publishing route; create/list wiki spaces, list/resolve/create/move/copy/rename nodes, move docs into Wiki, manage members/settings, search visible wiki nodes, and poll wiki tasks.",
                "help": ["feishu-bot wiki --help", "feishu-bot wiki route-check --help", "feishu-bot wiki member --help", "feishu-bot wiki setting --help"],
                "examples": [
                    "feishu-bot wiki route-check",
                    "feishu-bot wiki route-check --write-probe",
                    "feishu-bot wiki route-check --write-probe --strict",
                    "feishu-bot wiki spaces",
                    "feishu-bot wiki create-node --space-id <space_id> --title \"AI 演示\" --obj-type docx",
                    "feishu-bot wiki move-docs-to-wiki --space-id <space_id> --obj-type docx --obj-token <document_id>",
                    "feishu-bot wiki member list --space-id <space_id>",
                    "feishu-bot wiki search --query \"关键字\""
                ],
                "known_permission_edges": [
                    "create-space and search require FEISHU_USER_ACCESS_TOKEN because the official APIs require user_access_token.",
                    "tenant-token calls only see/edit wiki spaces where the app or bot is already a space member or admin.",
                    "move-docs-to-wiki also requires management permission on the source document and edit permission on the destination wiki parent.",
                    "Use route-check --write-probe --strict before claiming future AI reports can all go through Wiki; read checks alone do not prove publishing."
                ]
            },
            {
                "name": "sheet",
                "command": "feishu-bot sheet",
                "scope_group": "sheet",
                "status": "typed wrappers",
                "ai_use": "Create spreadsheets, inspect and manage sheet tabs, read/write/append/prepend values, merge/unmerge ranges, and apply cell styles.",
                "help": ["feishu-bot sheet --help", "feishu-bot sheet values --help"],
                "examples": [
                    "feishu-bot sheet get-sheet --spreadsheet-token sht_xxx --sheet-id <sheet_id>",
                    "feishu-bot sheet add-sheet --spreadsheet-token sht_xxx --title \"数据\"",
                    "feishu-bot sheet update-sheet --spreadsheet-token sht_xxx --sheet-id <sheet_id> --title \"新标题\"",
                    "feishu-bot sheet values update --spreadsheet-token sht_xxx --range Sheet1!A1:B2 --values-json '[[\"a\",\"b\"]]'",
                    "feishu-bot sheet values prepend --spreadsheet-token sht_xxx --range Sheet1!A:B --values-json '[[\"top\",\"row\"]]'",
                    "feishu-bot sheet merge --spreadsheet-token sht_xxx --range Sheet1!A1:C1 --merge-type MERGE_ALL",
                    "feishu-bot sheet style --spreadsheet-token sht_xxx --range Sheet1!A1:C1 --bold true --back-color fff2cc --border-type FULL_BORDER"
                ],
                "known_permission_edges": [
                    "Sheet metadata reads may require sheets:spreadsheet, sheets:spreadsheet:readonly, drive:drive, drive:drive:readonly, or sheets:spreadsheet.meta:read.",
                    "Cell value reads/writes and style/merge updates require spreadsheet file permission in addition to Sheets scopes.",
                    "Wiki-hosted Sheets use the wiki node obj_token as spreadsheet_token after resolving with `feishu-bot wiki node`."
                ]
            },
            {
                "name": "approval",
                "command": "feishu-bot approval",
                "scope_group": "approval",
                "status": "typed native approval and third-party connector wrappers",
                "ai_use": "Get/create/subscribe approval definitions, list/query/create/get/cancel instances, search/approve/reject/transfer/add-sign/rollback tasks, and sync/check third-party approval connector instances.",
                "help": ["feishu-bot approval --help", "feishu-bot approval definition --help", "feishu-bot approval instance --help", "feishu-bot approval task --help", "feishu-bot approval external --help"],
                "examples": [
                    "feishu-bot approval definition get --approval-code <code>",
                    "feishu-bot approval instance query --approval-code <code> --instance-status PENDING",
                    "feishu-bot approval task search --approval-code <code> --task-status PENDING",
                    "feishu-bot approval task approve --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment OK",
                    "feishu-bot approval external definition-get --approval-code <code>",
                    "feishu-bot approval external instance-sync --file external-instance.json"
                ],
                "known_permission_edges": [
                    "Definition reads may require approval:approval:readonly, approval:approval, or approval:definition.",
                    "Task search may require approval:approval.list:readonly or approval:approval:readonly.",
                    "Approval forms and external connector payloads are schema-specific; use definition get and official JSON files.",
                    "Task operations require the operator user ID and task_id from instance task_list.",
                    "Rollback uses task_def_key_list from instance timeline node_key values."
                ]
            },
            {
                "name": "notify",
                "command": "feishu-bot notify",
                "scope_group": "im",
                "status": "opinionated AI task card",
                "ai_use": "Send status cards to a user or project chat.",
                "help": ["feishu-bot notify --help"],
                "examples": ["feishu-bot notify --to \"$FEISHU_USER_ID\" --status done --task smoke --summary ok"]
            },
            {
                "name": "api",
                "command": "feishu-bot api",
                "scope_group": "any",
                "status": "universal OpenAPI escape hatch: tenant/user auth, JSON, binary download, multipart upload",
                "ai_use": "Call any official Feishu OpenAPI path not yet wrapped by typed commands.",
                "help": ["feishu-bot api --help", "feishu-bot api download --help", "feishu-bot api multipart --help"],
                "examples": [
                    "feishu-bot api get --path /im/v1/chats --query page_size=10",
                    "feishu-bot api get --auth user --path /search/v2/data_sources",
                    "feishu-bot api multipart --path /im/v1/images --field image_type=message --file image=./image.png"
                ]
            },
            {
                "name": "browser",
                "command": "feishu-bot browser",
                "scope_group": "local",
                "status": "local Playwright MCP helper",
                "ai_use": "Verify browser bridge status and inspect the current logged-in Feishu/Open Platform page.",
                "help": ["feishu-bot browser --help"],
                "examples": ["feishu-bot browser tabs"]
            }
        ],
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

pub(super) fn scope_groups(group: &str) -> Result<Vec<(&'static str, Vec<&'static str>)>> {
    let all = vec![
        ("user-token", vec!["offline_access", "auth:user.id:read"]),
        (
            "im",
            vec![
                "im:message",
                "im:message:readonly",
                "im:message.history:readonly",
                "im:message:send_as_bot",
                "im:message:update",
                "im:message:recall",
                "im:message.group_at_msg:readonly",
                "im:message.group_msg",
                "im:message.p2p_msg:readonly",
                "im:message.reactions:read",
                "im:message.reactions:write_only",
                "im:message.pins:read",
                "im:message.pins:write_only",
                "im:resource",
                "im:resource:upload",
                "im:chat",
                "im:chat:create",
                "im:chat:operate_as_owner",
                "im:chat.group_info:readonly",
            ],
        ),
        (
            "contact",
            vec![
                "contact:contact.base:readonly",
                "contact:department.organize:readonly",
                "contact:contact:access_as_app",
                "contact:contact:readonly",
                "contact:contact:readonly_as_app",
            ],
        ),
        (
            "directory",
            vec![
                "directory:employee:search",
                "directory:employee:read",
                "directory:employee:list",
                "directory:employee.base.base:read",
                "directory:employee.base.name.name:read",
                "directory:employee.base.name.another_name:read",
                "directory:employee.base.mobile:read",
                "directory:employee.base.email:read",
                "directory:employee.base.enterprise_email:read",
                "directory:employee.base.enterprise_email_alias:read",
                "directory:employee.base.avatar:read",
                "directory:employee.base.background_image:read",
                "directory:employee.base.description:read",
                "directory:employee.base.department:read",
                "directory:employee.base.department_path:read",
                "directory:employee.base.dept_order:read",
                "directory:employee.base.dotted_line_leaders:read",
                "directory:employee.base.external_id:read",
                "directory:employee.base.gender:read",
                "directory:employee.base.geo:read",
                "directory:employee.base.is_admin:read",
                "directory:employee.base.is_primary_admin:read",
                "directory:employee.base.is_resigned:read",
                "directory:employee.base.leader:read",
                "directory:employee.base.leader_id:read",
                "directory:employee.base.resign_time:read",
                "directory:employee.base.role:read",
                "directory:employee.base.status:read",
                "directory:employee.base.active_status:read",
                "directory:employee.base.subscription_ids:read",
                "directory:employee.base.custom_field:read",
                "directory:employee.base.data_source:read",
                "directory:employee.work.base_work:read",
                "directory:employee.work.employment:read",
                "directory:employee.work.employment_type:read",
                "directory:employee.work.extension_number:read",
                "directory:employee.work.job_number:read",
                "directory:employee.work.job_title:read",
                "directory:employee.work.join_date:read",
                "directory:employee.work.resign_date:read",
                "directory:employee.work.resign_reason:read",
                "directory:employee.work.resign_remark:read",
                "directory:employee.work.resign_type:read",
                "directory:employee.work.staff_status:read",
                "directory:employee.work.work_country_or_region:read",
                "directory:employee.work.work_place:read",
                "directory:employee.work.work_station:read",
                "directory:employee.work.job_level:read",
                "directory:employee.work.job_family:read",
                "directory:department.base:read",
                "directory:department.count:read",
                "directory:department.custom_field:read",
                "directory:department.data_source:read",
                "directory:department.department_path:read",
                "directory:department.external_id:read",
                "directory:department.has_child:read",
                "directory:department.leader:read",
                "directory:department.name:read",
                "directory:department.order_weight:read",
                "directory:department.organization:read",
                "directory:department.parent_id:read",
                "directory:department.status:read",
                "directory:job_title.base:read",
                "directory:job_title.status:read",
                "directory:job_family.base:read",
                "directory:job_family.path:read",
                "directory:job_family.status:read",
                "directory:job_level.base:read",
                "directory:job_level.order:read",
                "directory:job_level.status:read",
                "directory:place.base:read",
                "directory:place.status:read",
            ],
        ),
        (
            "doc",
            vec![
                "docx:document",
                "docx:document:readonly",
                "docx:document:write_only",
                "docx:document:create",
                "docx:document.block:convert",
            ],
        ),
        (
            "board",
            vec!["board:whiteboard:node:create", "board:whiteboard:node:read"],
        ),
        (
            "base",
            vec![
                "bitable:app",
                "bitable:app:readonly",
                "base:app:create",
                "base:app:read",
                "base:app:update",
                "base:table:create",
                "base:table:read",
                "base:table:update",
                "base:table:delete",
                "base:field:create",
                "base:field:read",
                "base:field:update",
                "base:field:delete",
                "base:view:read",
                "base:view:write_only",
                "base:record:create",
                "base:record:retrieve",
                "base:record:update",
                "base:record:delete",
                "base:dashboard:read",
                "base:dashboard:copy",
                "base:workflow:read",
                "base:workflow:write",
                "base:form:read",
                "base:form:update",
                "base:role:read",
                "base:role:create",
                "base:role:update",
                "base:role:delete",
                "base:collaborator:read",
                "base:collaborator:create",
                "base:collaborator:delete",
                "docs:document.media:upload",
                "docs:document.media:download",
            ],
        ),
        (
            "task",
            vec![
                "task:task:write",
                "task:task:writeonly",
                "task:task:read",
                "task:task:readonly",
                "task:personnel:writeonly",
                "task:tasklist:read",
                "task:tasklist:write",
                "task:tasklist:writeonly",
                "task:section:read",
                "task:section:write",
                "task:section:writeonly",
                "task:custom_field:read",
                "task:custom_field:write",
                "task:custom_field:writeonly",
                "task:attachment:read",
                "task:attachment:write",
                "task:attachment:upload",
                "task:attachment:delete",
                "task:comment:read",
                "task:comment:write",
                "task:comment:writeonly",
                "task:comment:delete",
            ],
        ),
        (
            "drive",
            vec![
                "drive:drive",
                "drive:drive:readonly",
                "drive:file",
                "drive:file:readonly",
                "drive:file:upload",
                "drive:file:download",
                "docs:doc",
                "docs:document.media:upload",
                "docs:document.media:download",
                "docs:document:import",
                "docs:document:export",
                "drive:export:readonly",
                "docs:document.comment:read",
                "docs:document.comment:create",
                "docs:document.comment:update",
                "docs:document.comment:delete",
                "docs:document.comment:write_only",
                "docs:document.subscription",
                "docs:document.subscription:read",
                "drive:drive:version",
                "drive:drive:version:readonly",
                "drive:file:view_record:readonly",
                "contact:user.base:readonly",
                "contact:user.employee_id:readonly",
                "space:document:retrieve",
            ],
        ),
        (
            "permission",
            vec![
                "docs:permission.member",
                "docs:permission.member:read",
                "docs:permission.member:readonly",
                "docs:permission.member:retrieve",
                "docs:permission.member:create",
                "docs:permission.member:update",
                "docs:permission.member:delete",
                "docs:permission.member:auth",
                "docs:permission.setting",
                "docs:permission.setting:read",
                "docs:permission.setting:readonly",
                "docs:permission.setting:write_only",
            ],
        ),
        (
            "calendar",
            vec![
                "calendar:calendar",
                "calendar:calendar:readonly",
                "calendar:calendar:read",
                "calendar:calendar.calendar:readonly",
                "calendar:calendar.free_busy:read",
                "calendar:calendar.event:read",
                "calendar:calendar.event:create",
                "calendar:calendar.event:update",
                "calendar:calendar.event:writeonly",
                "calendar:calendar.event:delete",
            ],
        ),
        (
            "vc",
            vec![
                "vc:meeting",
                "vc:meeting:readonly",
                "vc:meeting.all_meeting:readonly",
                "vc:meeting.meetingevent:read",
                "vc:meeting.participant:write",
                "vc:report:readonly",
                "vc:record",
                "vc:record:readonly",
                "vc:reserve",
                "vc:reserve:readonly",
                "vc:room",
                "vc:room:readonly",
                "vc:rooms.room.basicinfo:read",
                "vc:rooms.roomlevel:read",
            ],
        ),
        (
            "minutes",
            vec![
                "minutes:minutes",
                "minutes:minutes:readonly",
                "minutes:minutes.basic:read",
                "minutes:minutes.search:read",
                "minutes:minutes.artifacts:read",
                "minutes:minute:download",
                "minutes:minutes.media:export",
                "minutes:minutes.transcript:export",
            ],
        ),
        (
            "search",
            vec![
                "search:docs:read",
                "search:message",
                "search:data_source",
                "search:data_source:readonly",
            ],
        ),
        (
            "okr",
            vec![
                "okr:okr.period:readonly",
                "okr:okr:readonly",
                "okr:okr.content:readonly",
                "okr:okr",
            ],
        ),
        (
            "attendance",
            vec![
                "attendance:rule",
                "attendance:rule:readonly",
                "attendance:task",
                "attendance:task:readonly",
            ],
        ),
        (
            "mail",
            vec![
                "mail:user_mailbox",
                "mail:user_mailbox:readonly",
                "mail:user_mailbox.message:readonly",
                "mail:user_mailbox.message:send",
                "mail:user_mailbox.message:modify",
                "mail:user_mailbox.message.subject:read",
                "mail:user_mailbox.message.address:read",
                "mail:user_mailbox.message.body:read",
                "mail:user_mailbox.folder:read",
                "mail:user_mailbox.folder:write",
                "mail:user_mailbox.mail_contact:read",
                "mail:user_mailbox.mail_contact:write",
                "mail:user_mailbox.mail_contact.mail_address:read",
                "mail:user_mailbox.mail_contact.phone:read",
                "mail:user_mailbox.rule:read",
                "mail:user_mailbox.rule:write",
                "contact:user.employee_id:readonly",
            ],
        ),
        (
            "corehr",
            vec![
                "corehr:corehr:readonly",
                "corehr:corehr",
                "corehr:department:read",
                "corehr:department:write",
                "corehr:department.manager:read",
                "corehr:department.organize:read",
                "corehr:department.custom_fields:read",
                "corehr:department.cost_center_id:read",
                "corehr:job.only:read",
                "corehr:job:read",
                "corehr:job:write",
                "corehr:job.job_level:read",
                "corehr:employee.job_data:read",
                "corehr:job_data:read",
                "corehr:job_data:write",
                "corehr:employment.job:read",
                "corehr:employment.job_level:read",
                "corehr:employment.job_level:write",
                "corehr:employment.pathway:read",
                "corehr:employment.pathway:write",
                "corehr:employment.position:read",
                "corehr:employment.position:write",
                "corehr:employment.job_grade:read",
                "corehr:employment.job_grade:write",
                "corehr:job_data.assignment_start_reason:read",
                "corehr:job_data.compensation_type:read",
                "corehr:job_data.job_data_reason:read",
                "corehr:job_data.service_company:read",
                "corehr:job_data.work_shift:read",
                "corehr:person:read",
                "corehr:person:write",
                "corehr:process:read",
                "contact:user.employee_id:readonly",
            ],
        ),
        (
            "helpdesk",
            vec![
                "helpdesk:all:readonly",
                "helpdesk:all",
                "helpdesk:helpdesk:access",
                "contact:user.employee_id:readonly",
            ],
        ),
        (
            "hire",
            vec![
                "hire:talent:readonly",
                "hire:talent",
                "hire:people_cli",
                "hire:job:readonly",
                "hire:job.composite_info:readonly",
                "hire:job",
                "hire:application:readonly",
                "hire:application",
                "hire:interview:readonly",
                "hire:interview",
                "hire:job_process:readonly",
                "hire:job_requirement:readonly",
                "hire:job_requirement",
                "hire:attachment:readonly",
                "hire:attachment",
                "hire:location:readonly",
                "hire:subject:readonly",
                "hire:site:readonly",
                "hire:site",
                "hire:employee:readonly",
                "hire:evaluation:readonly",
                "hire:offer:readonly",
                "hire:offer_salary:readonly",
                "hire:agency:readonly",
                "hire:agency_salary:readonly",
                "hire:referral:readonly",
                "contact:user.employee_id:readonly",
            ],
        ),
        (
            "wiki",
            vec![
                "wiki:wiki",
                "wiki:wiki:readonly",
                "wiki:space:retrieve",
                "wiki:space:read",
                "wiki:space:write_only",
                "wiki:node:retrieve",
                "wiki:node:read",
                "wiki:node:create",
                "wiki:node:move",
                "wiki:node:copy",
                "wiki:node:update",
                "wiki:member:retrieve",
                "wiki:member:create",
                "wiki:member:update",
                "wiki:setting:write_only",
            ],
        ),
        (
            "sheet",
            vec![
                "sheets:spreadsheet",
                "sheets:spreadsheet:readonly",
                "sheets:spreadsheet:create",
                "sheets:spreadsheet:read",
                "sheets:spreadsheet:write_only",
                "sheets:spreadsheet.meta:read",
                "sheets:spreadsheet.meta:write_only",
                "drive:drive",
                "drive:drive:readonly",
            ],
        ),
        (
            "approval",
            vec![
                "approval:approval",
                "approval:approval:readonly",
                "approval:approval.list:readonly",
                "approval:definition",
                "approval:instance",
                "approval:instance:readonly",
                "approval:task",
                "approval:external_approval",
                "approval:external_instance",
                "approval:external_task",
            ],
        ),
    ];
    if group == "all" {
        return Ok(all);
    }
    let selected = all
        .into_iter()
        .filter(|(name, _)| *name == group)
        .collect::<Vec<_>>();
    if selected.is_empty() {
        bail!("unknown scope group: {group}");
    }
    Ok(selected)
}

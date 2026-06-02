use serde_json::{json, Value};

pub(in crate::app) fn workflow_manifest_modules() -> Vec<Value> {
    vec![
        json!({
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
        }),
        json!({
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
        }),
        json!({
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
                "feishu-bot setup auto --open-browser --json",
                "scripts/feishu-bot-setup.sh --project \"AI Project\" --open-browser",
                "feishu-bot setup open-scopes --group office --browser",
                "feishu-bot setup wiki-bot --auth user"
            ],
            "known_permission_edges": [
                "Opening a grant URL is automated; approving permissions still happens in the signed-in human browser account.",
                "wiki-bot needs FEISHU_USER_ACCESS_TOKEN and a user allowed to manage the target Wiki space.",
                "For multi-account Chrome, verify the intended Playwright MCP profile before approving account-sensitive grants.",
                "scripts/feishu-bot-setup.sh opens browser URLs only when --open-browser or FEISHU_BOT_SETUP_OPEN_BROWSER=1 is set.",
                "setup never prints app_secret or raw tokens; doctor/setup mask configured secrets."
            ]
        }),
        json!({
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
        }),
        json!({
            "name": "office",
            "command": "feishu-bot office",
            "layer": "workflow",
            "scope_group": "im,wiki,doc,base,permission,search",
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
        }),
    ]
}

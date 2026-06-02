pub(in crate::app) const ROOT_AFTER_HELP: &str = r#"AI handoff:
  feishu-bot ai                         Print the operator playbook for future AI agents.
  feishu-bot manifest                   Print machine-readable command/scope manifest.
  feishu-bot doctor                     Verify env, app credentials, and tenant token.
  feishu-bot oauth url                  Build a user-token authorization URL.
  feishu-bot oauth token                Exchange OAuth code for user_access_token.
  feishu-bot scopes --group all         Print scope checklist and grant URLs.
  feishu-bot scopes --group doc --token-type user Print user-token docx grant URL.
  feishu-bot bot info                   Print app bot open_id for Wiki/doc permissions.
  feishu-bot setup quickstart           Automate common first-run env/scope/OAuth/browser/Wiki setup checks.
  feishu-bot dogfood publish            Publish one closed-loop capability demo doc.
  feishu-bot dogfood verify             Run real current-account capability probes.
  feishu-bot office --help              High-level project workflows for chat/Wiki/Base/report/poll.
  feishu-bot doc preview --file notes.md Show local native docx block mapping.
  feishu-bot doc template --kind all   Print raw docx block templates for AI agents.
  feishu-bot doc convert --file notes.md Use Feishu official Markdown/HTML converter.
  feishu-bot base --help                Base/Bitable app, table, field, record commands.
  feishu-bot task --help                Feishu Task v2 commands.
  feishu-bot board --help               Whiteboard Mermaid/PlantUML and node commands.
  feishu-bot contact --help             Contact user and department lookup.
  feishu-bot directory --help           Directory employee search, filter, and batch reads.
  feishu-bot chat --help                Chat discovery, metadata, and members.
  feishu-bot message --help             Message text/image/file/resource/reaction/pin commands.
  feishu-bot drive --help               Drive files, folders, media, import/export, comments, versions.
  feishu-bot calendar --help            Calendar and event commands.
  feishu-bot vc --help                  Video meeting, report, recording, and room commands.
  feishu-bot minutes --help             Feishu Minutes search, metadata, AI artifacts, and transcript export.
  feishu-bot search --help              Search docs/messages and manage search connector indexes.
  feishu-bot okr --help                 OKR periods, period rules, and user OKR reads.
  feishu-bot attendance --help          Attendance groups, shifts, schedules, flows, and stats.
  feishu-bot mail --help                Mail messages, folders, contacts, aliases, and send.
  feishu-bot corehr --help              CoreHR departments, jobs, job data, persons, and processes.
  feishu-bot helpdesk --help            Helpdesk tickets, ticket messages, bot pushes, and FAQ reads.
  feishu-bot hire --help                Hire jobs, talents, applications, interviews, and processes.
  feishu-bot wiki --help                Wiki space and node commands.
  feishu-bot sheet --help               Spreadsheet sheet and value commands.
  feishu-bot approval --help            Approval instance commands.
  feishu-bot api --help                 Raw tenant-token OpenAPI escape hatch.

Read-only diagnostics:
  feishu-bot doctor
  feishu-bot setup plan
  feishu-bot office list
  feishu-bot office bootstrap --project "demo" --dry-run
  feishu-bot office report --project "demo" --title "dry run" --content "hello" --dry-run
  feishu-bot --json manifest --module base

Write smoke sequence; creates real Feishu data:
  feishu-bot office progress --project "demo" --title "status" --summary "ok"
  feishu-bot message send --to "$FEISHU_USER_ID" --text "hello from feishu-bot"
  feishu-bot doc create --title "Bot smoke" --writer official --content $'# Smoke\n\n- docx ok'
  feishu-bot base create --name "Bot Base smoke"
  feishu-bot notify --to "$FEISHU_USER_ID" --status done --task "smoke" --summary "ok"
"#;

pub(in crate::app) const OAUTH_AFTER_HELP: &str = r#"AI-safe user token workflow:
  feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope task:task:read
  feishu-bot oauth url --scope "offline_access auth:user.id:read docx:document:readonly docx:document:write_only wiki:wiki wiki:space:write_only wiki:node:create"
  feishu-bot browser open --url "<authorization_url>"
  feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env --env-file private/local.env
  feishu-bot oauth refresh --save-env --env-file private/local.env
  feishu-bot oauth user-info
  feishu-bot --json dogfood verify --module task --include-response

Use this when a probe reports missing_user_token or expired_user_token. The
redirect URI must be registered in the app's Open Platform security settings.
By default the CLI uses FEISHU_OAUTH_REDIRECT_URI, LARK_OAUTH_REDIRECT_URI, or
http://localhost:8080/callback. Tokens are masked by default; --raw and
--print-env intentionally expose secrets.
"#;

pub(in crate::app) const BOT_AFTER_HELP: &str = r#"AI-safe bot identity workflow:
  feishu-bot bot info
  feishu-bot --json bot info
  feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin

`bot info` calls /bot/v3/info with tenant_access_token. Feishu does not require
an OpenAPI scope for this endpoint, but the app must have bot capability enabled.
Use the returned open_id when granting the app/bot Wiki space membership or
document permissions.
"#;

pub(in crate::app) const SETUP_AFTER_HELP: &str = r#"AI-safe setup automation:
  feishu-bot setup plan
  feishu-bot setup quickstart --open-browser
  feishu-bot setup auto --open-browser --json
  feishu-bot setup open-scopes --group office --browser
  feishu-bot setup wiki-bot --auth user

`setup` is the preferred first-run helper. It does not store secrets by itself.
It checks env shape, builds Feishu Open Platform scope grant URLs, can open them
through the existing Playwright MCP browser bridge, and can add the current app
bot to FEISHU_WIKI_SPACE_ID using a user token. OAuth authorization still
requires the signed-in human account to approve in the browser and paste the
redirect code into `feishu-bot oauth token`.

Use `setup quickstart` or `scripts/feishu-bot-setup.sh` for the normal
one-human-plus-AI office profile. It returns the exact next commands for
permission grant, OAuth token saving, Wiki bot membership, project bootstrap,
progress updates, inbox polling, and search. Use `setup auto` when an AI agent
should run the safe setup sequence and return machine-readable next actions in
one command; it may open browser URLs when `--open-browser` is passed.
"#;

pub(in crate::app) const DOGFOOD_AFTER_HELP: &str = r##"AI-safe dogfood workflow:
  feishu-bot dogfood publish --title "能力演示" --file ./demo.md
  feishu-bot dogfood publish --title "能力演示" --content "# Demo"
  feishu-bot dogfood publish --title "HTML 演示" --content-type html --file ./demo.html
  feishu-bot dogfood publish --title "非 Wiki 草稿" --file ./demo.md --no-wiki
  feishu-bot dogfood verify
  feishu-bot dogfood verify --module calendar --module task --include-response
  feishu-bot dogfood verify --write --module doc --module base --module task
  feishu-bot dogfood verify --write --module board --include-response
  feishu-bot dogfood verify --send-loop-check --to "$FEISHU_USER_ID"
  feishu-bot dogfood verify --module task --module search --auto-refresh-user-token --strict

This is the preferred final step after adding a new CLI capability. It creates
one standalone docx, writes the content, reads the doc back, attempts Wiki when
configured, sends the exact link message to --to or FEISHU_USER_ID, and returns
send_loop_check proof for message get/list, chat metadata, chat members, and
read-users.

`verify` is the preferred first step before claiming a module works. It runs
real OpenAPI probes against the current app/account and classifies each result
as ok, no_data, missing_scope, missing_user_token, expired_user_token,
missing_helpdesk_config, upstream_api_error, or api_error.
Default probes are read-only; --write and --send-loop-check intentionally create
real Feishu data. --auto-refresh-user-token intentionally refreshes and saves
the user token before retrying expired user-token probes. --strict exits non-zero
when any probe is still not usable after retries. Failed probes include
remediation JSON with grant URLs, browser commands, required env vars, and rerun
commands for the next AI step.
"##;

pub(in crate::app) const OFFICE_AFTER_HELP: &str = r#"AI office workflow layer:
  feishu-bot office list
  feishu-bot office bootstrap --project "AI项目" --dry-run
  feishu-bot office bootstrap --project "AI项目" --user "$FEISHU_USER_ID" --space-id "$FEISHU_WIKI_SPACE_ID" --send-summary
  feishu-bot office report --project "AI项目" --title "功能演示" --file ./demo.md --dry-run
  feishu-bot office report --project "AI项目" --title "功能演示" --file ./demo.md --base-record --pin
  feishu-bot office progress --project "AI项目" --title "进度更新" --status doing --summary "当前进展"
  feishu-bot office progress --project "AI项目" --title "阶段总结" --file ./summary.md --wiki-report --pin
  feishu-bot office voice-report --project "AI项目" --text "这是一条语音汇报"
  feishu-bot office inbox --project "AI项目" --from-now
  feishu-bot office inbox --project "AI项目" --reply-text "收到，我来处理"
  feishu-bot office poll --project "AI项目" --from-now --mark-seen
  feishu-bot office poll --project "AI项目" --ack-emoji OK --reply-text "收到，我来处理" --mark-seen
  feishu-bot office search --project "AI项目" --query "需求"
  feishu-bot office status --project "AI项目" --check
  feishu-bot office cleanup --project "AI项目" --dry-run

Use office commands first for daily AI work. They keep a local project registry at
~/.config/feishu/office-projects.json by default, write each demo/report as an
independent Wiki/docx document, notify the project group, optionally log to Base,
and return readback probes. Use atomic commands such as message, wiki, base,
chat, task, drive, search, and calendar when the workflow layer is too coarse or
when you need one exact OpenAPI operation.

Safety:
  list, bootstrap --dry-run, report --dry-run, and status without --check do not
  call Feishu OpenAPI and can be used before credentials are configured.
"#;

pub(in crate::app) const API_AFTER_HELP: &str = r#"Raw Feishu OpenAPI escape hatch:
  feishu-bot api get --path /bitable/v1/apps/<app_token>
  feishu-bot api post --path /task/v2/tasks --body-json '{"summary":"raw task"}'
  feishu-bot api post --path /bitable/v1/apps/<app_token>/tables/<table_id>/records --body-json '{"fields":{"标题":"hello"}}'
  feishu-bot api get --auth user --path /search/v2/data_sources
  feishu-bot api download --path /drive/v1/files/<file_token>/download --output ./file.bin
  feishu-bot api multipart --path /im/v1/images --field image_type=message --file image=./image.png
  feishu-bot api multipart --path /drive/v1/medias/upload_all --field file_name=demo.png --field parent_type=docx_image --field parent_node=<block_id> --field size=123 --file file=./demo.png

Rules:
  --path is relative to /open-apis and must start with /.
  --query can be repeated as key=value.
  --header can be repeated as key=value for product-specific headers.
  --auth tenant uses tenant_access_token; --auth user uses FEISHU_USER_ACCESS_TOKEN.
  --body-json, --file, or --stdin must contain JSON for write methods.
  api multipart sends multipart/form-data with text --field key=value and
  file parts as --file part_name=./path. It covers official upload APIs before a
  typed wrapper exists.

Prefer typed commands when available. Use this raw API layer for every official
Feishu endpoint that has not yet been given a dedicated CLI subcommand.
"#;

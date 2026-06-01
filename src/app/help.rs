pub(super) const AI_USAGE: &str = include_str!("../../docs/AI-USAGE.md");
pub(super) const ROOT_AFTER_HELP: &str = r#"AI handoff:
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
  feishu-bot office progress --project "demo" --title "status" --summary "ok"
  feishu-bot --json manifest --module base

Write smoke sequence; creates real Feishu data:
  feishu-bot message send --to "$FEISHU_USER_ID" --text "hello from feishu-bot"
  feishu-bot doc create --title "Bot smoke" --writer official --content $'# Smoke\n\n- docx ok'
  feishu-bot base create --name "Bot Base smoke"
  feishu-bot notify --to "$FEISHU_USER_ID" --status done --task "smoke" --summary "ok"
"#;
pub(super) const OAUTH_AFTER_HELP: &str = r#"AI-safe user token workflow:
  feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope task:task:read
  feishu-bot oauth url --scope "offline_access auth:user.id:read docx:document:readonly docx:document:write_only wiki:wiki wiki:space:write_only wiki:node:create"
  feishu-bot browser open --url "<authorization_url>"
  feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env
  feishu-bot oauth refresh --save-env
  feishu-bot oauth user-info
  feishu-bot --json dogfood verify --module task --include-response

Use this when a probe reports missing_user_token. The redirect URI must be
registered in the app's Open Platform security settings. By default the CLI uses
FEISHU_OAUTH_REDIRECT_URI, LARK_OAUTH_REDIRECT_URI, or
http://localhost:8080/callback. Tokens are masked by default; --raw and
--print-env intentionally expose secrets.
"#;
pub(super) const BOT_AFTER_HELP: &str = r#"AI-safe bot identity workflow:
  feishu-bot bot info
  feishu-bot --json bot info
  feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin

`bot info` calls /bot/v3/info with tenant_access_token. Feishu does not require
an OpenAPI scope for this endpoint, but the app must have bot capability enabled.
Use the returned open_id when granting the app/bot Wiki space membership or
document permissions.
"#;
pub(super) const SETUP_AFTER_HELP: &str = r#"AI-safe setup automation:
  feishu-bot setup plan
  feishu-bot setup quickstart --open-browser
  feishu-bot setup open-scopes --group office --browser
  feishu-bot setup wiki-bot --auth user
  feishu-bot setup auto --open-browser

`setup` is the preferred first-run helper. It does not store secrets by itself.
It checks env shape, builds Feishu Open Platform scope grant URLs, can open them
through the existing Playwright MCP browser bridge, and can add the current app
bot to FEISHU_WIKI_SPACE_ID using a user token. OAuth authorization still
requires the signed-in human account to approve in the browser and paste the
redirect code into `feishu-bot oauth token`.

Use `setup quickstart` or `scripts/feishu-bot-setup.sh` for the normal
one-human-plus-AI office profile. It returns the exact next commands for
permission grant, OAuth token saving, Wiki bot membership, project bootstrap,
progress updates, inbox polling, and search.
"#;
pub(super) const DOGFOOD_AFTER_HELP: &str = r##"AI-safe dogfood workflow:
  feishu-bot dogfood publish --title "能力演示" --file ./demo.md
  feishu-bot dogfood publish --title "能力演示" --content "# Demo"
  feishu-bot dogfood publish --title "HTML 演示" --content-type html --file ./demo.html
  feishu-bot dogfood publish --title "非 Wiki 草稿" --file ./demo.md --no-wiki
  feishu-bot dogfood verify
  feishu-bot dogfood verify --module calendar --module task --include-response
  feishu-bot dogfood verify --write --module doc --module base --module task
  feishu-bot dogfood verify --write --module board --include-response
  feishu-bot dogfood verify --send-loop-check --to "$FEISHU_USER_ID"

This is the preferred final step after adding a new CLI capability. It creates
one standalone docx, writes the content, reads the doc back, attempts Wiki when
configured, sends the exact link message to --to or FEISHU_USER_ID, and returns
send_loop_check proof for message get/list, chat metadata, chat members, and
read-users.

`verify` is the preferred first step before claiming a module works. It runs
real OpenAPI probes against the current app/account and classifies each result
as ok, missing_scope, missing_user_token, missing_helpdesk_config, or api_error.
Default probes are read-only; --write and --send-loop-check intentionally create
real Feishu data. Failed probes include remediation JSON with grant URLs,
browser commands, required env vars, and rerun commands for the next AI step.
"##;
pub(super) const OFFICE_AFTER_HELP: &str = r#"AI office workflow layer:
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
pub(super) const MESSAGE_SEND_AFTER_HELP: &str = r#"Examples:
  feishu-bot message send --to "$FEISHU_USER_ID" --text "hello"
  feishu-bot message loop-check --to "$FEISHU_USER_ID" --to-type open-id
  printf 'multi-line\nmessage\n' | feishu-bot message send --to "$FEISHU_USER_ID" --stdin
  feishu-bot message send --to oc_xxx --to-type chat-id --file ./message.txt
  feishu-bot message send-json --to "$FEISHU_USER_ID" --msg-type interactive --content-json '{"config":{"wide_screen_mode":true},"elements":[]}'
  feishu-bot message upload-image --file ./image.png
  feishu-bot message send-image --to "$FEISHU_USER_ID" --file ./image.png
  feishu-bot message upload-file --file ./demo.mp4 --file-type mp4 --duration 3000
  feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./demo.mp4 --file-type mp4
  feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./voice.opus --file-type opus --duration 3000
  feishu-bot message send-voice --to "$FEISHU_USER_ID" --file ./voice.mp3 --readback
  feishu-bot message send-voice --to "$FEISHU_USER_ID" --text "语音播报内容" --readback
  feishu-bot message reply-json --message-id om_xxx --msg-type text --content-json '{"text":"reply"}'
  feishu-bot message reply --message-id om_xxx --text "收到，我来处理"
  feishu-bot message ack --message-id om_xxx --emoji-type OK --reply-text "已读，开始处理"
  feishu-bot message poll --chat-id oc_xxx --from-now --mark-seen
  feishu-bot message poll --chat-id oc_xxx --ack-emoji OK --reply-text "收到" --mark-seen
  feishu-bot message edit-json --message-id om_xxx --msg-type text --content-json '{"text":"edited"}'
  feishu-bot message delete --message-id om_xxx
  feishu-bot message list --container-id oc_xxx --container-id-type chat --page-size 20
  feishu-bot message get --message-id om_xxx
  feishu-bot message read-users --message-id om_xxx
  feishu-bot message resource --message-id om_xxx --file-key file_xxx --type file --output ./download.bin
  feishu-bot message download-image --image-key img_v2_xxx --output ./image.png
  feishu-bot message download-file --file-key file_xxx --output ./download.bin
  feishu-bot message reaction list --message-id om_xxx
  feishu-bot message reaction add --message-id om_xxx --emoji-type SMILE
  feishu-bot message reaction delete --message-id om_xxx --reaction-id <reaction_id>
  feishu-bot message pin list --chat-id oc_xxx
  feishu-bot message pin add --message-id om_xxx
  feishu-bot message pin delete --message-id om_xxx

Receiver type inference:
  ou_... -> open_id
  oc_... -> chat_id
  on_... -> union_id
  contains @ -> email
  otherwise -> user_id

send-file message type:
  --msg-type auto maps --file-type mp4 to media/video, opus to audio, otherwise file.
  Use --cover-image-key <image_key> to set a video cover image when sending media.

send-voice:
  Use --file for MP3/WAV/M4A/OPUS input. Non-OPUS files are converted with ffmpeg
  and duration is detected with ffprobe. Use --text/--text-file/--stdin to call
  vox first, then send the generated voice as a Feishu audio message.

Use `message loop-check` for dogfood. It sends one text message, then reads back
the message by message_id, lists the target chat, reads chat metadata, lists chat
members, and checks read-users. Do not claim a human-visible send loop is proven
unless message_get/list/chat/member probes all pass and the target member is the
expected Feishu account.

Use `message poll --from-now --mark-seen` once per project chat to establish a
local cursor, then run `message poll --ack-emoji OK --reply-text "收到"` to pick
up user messages, add a reaction status, optionally reply, and move the cursor.
`message ack` uses Feishu reactions as workflow status markers; it is not an
official read receipt. Use `message read-users` only for Feishu read-user data on
messages sent by the bot.
"#;
pub(super) const CONTACT_AFTER_HELP: &str = r#"AI-safe contact workflow:
  feishu-bot contact user get --user-id "$FEISHU_USER_ID"
  feishu-bot contact user list --department-id 0 --page-size 10
  feishu-bot contact department children --department-id 0 --page-size 10
  feishu-bot contact department get --department-id 0
  feishu-bot contact department search --query "研发"

Tenant-token access is limited by the app's contact scope and visible
department range. Use `feishu-bot scopes --group contact` when permissions are
missing.
"#;
pub(super) const DIRECTORY_AFTER_HELP: &str = r#"AI-safe Directory workflow:
  feishu-bot directory employee search --query "张三" --page-size 10
  feishu-bot directory employee search --query user@example.com --field base_info.employee_id --field base_info.email
  feishu-bot directory employee mget --employee-id <open_id> --field base_info.name --field work_info.job_title
  feishu-bot directory employee filter --condition 'base_info.email=eq="user@example.com"'
  feishu-bot directory employee filter --condition 'work_info.job_number=eq="E12345"' --field base_info.name

Directory v1 is the newer admin org-directory API. It supports tenant and user
tokens; tenant mode follows the app contact range, user mode follows the admin
range of FEISHU_USER_ACCESS_TOKEN. Pass --body-json/--file/--stdin for full
official filter bodies.
"#;
pub(super) const NOTIFY_AFTER_HELP: &str = r#"Examples:
  feishu-bot notify --to "$FEISHU_USER_ID" --status done --task "build" --summary "passed"
  feishu-bot notify --project my-project --status error --summary "tests failed" --details "cargo test failed|see logs"
  feishu-bot notify --project my-project --link "https://example.com/report" --text "full report"

Without --to:
  The CLI creates/reuses a private project chat, stores the mapping in
  ~/.config/feishu/projects.json.
"#;
pub(super) const CHAT_AFTER_HELP: &str = r#"AI-safe chat workflow:
  feishu-bot chat list
  feishu-bot chat search --query "项目"
  feishu-bot chat get --chat-id oc_xxx
  feishu-bot chat create --name "AI 项目群" --user "$FEISHU_USER_ID" --avatar-file ./avatar.png
  feishu-bot chat update --chat-id oc_xxx --name "AI 项目群 v2" --avatar-file ./avatar.png
  feishu-bot chat member list --chat-id oc_xxx
  feishu-bot chat member add --chat-id oc_xxx --id "$FEISHU_USER_ID"
  feishu-bot chat member is-in-chat --chat-id oc_xxx
  feishu-bot chat member delete --chat-id oc_xxx --id <open_id>
  feishu-bot chat tab list --chat-id oc_xxx
  feishu-bot chat tab add --chat-id oc_xxx --name "项目页" --tab-type url --url https://example.com
  feishu-bot chat tab add --chat-id oc_xxx --name "知识库" --tab-type doc --doc https://my.feishu.cn/wiki/xxx
  feishu-bot chat menu get --chat-id oc_xxx
  feishu-bot chat menu add --chat-id oc_xxx --body-file ./menu-tree.json
  feishu-bot chat delete --chat-id oc_xxx

Use `chat list` or `chat search` to discover oc_ chat IDs before sending group
messages. Member add/delete defaults to open_id; use --member-id-type app-id
when adding a bot by App ID. For AI project isolation, prefer one group per
project/topic, set a recognizable avatar, add doc/url tabs for durable context,
pin important messages, and use group menus for common links/actions. Feishu's
personal left-sidebar labels are client-side and are not exposed by the group
OpenAPI; use naming prefixes, avatars, tabs, menus, pins, and optional feed-card
APIs as the automatable substitute. `chat delete` dissolves the group for
everyone; it is not a client-side "hide/remove this conversation from my left
sidebar" operation.
"#;
pub(super) const DOC_AFTER_HELP: &str = r#"AI-safe workflow:
  1. feishu-bot doc preview --file ./doc.md
  2. feishu-bot doc create --title "Title" --file ./doc.md
  3. feishu-bot doc blocks --document-id <id>
  4. feishu-bot doc raw --document-id <id>
  5. feishu-bot doc send-link --document-id <id> --to "$FEISHU_USER_ID" --send-loop-check

For docx objects created by `feishu-bot wiki create-node --auth user`, keep using
the user token for writes and reads:
  feishu-bot doc append --auth user --document-id <obj_token> --file ./doc.md
  feishu-bot doc raw --auth user --document-id <obj_token>

Supported Markdown-ish input -> native Feishu docx blocks:
  #..######### headings -> heading1..heading9
  plain text           -> text
  - item               -> bullet
  1. item              -> ordered
  ```rust code fences  -> code with Feishu CodeLanguage when known
  ```mermaid fences    -> code/plain-text source, not rendered Mermaid plugin
  > quote              -> quote
  - [ ] / - [x]        -> todo
  ---                  -> divider

For rare/new Feishu blocks and subtype fields, use:
  feishu-bot doc template --kind all
  feishu-bot doc template --kind table-descendant
  feishu-bot doc insert-media --document-id <id> --kind image --file ./image.png
  feishu-bot doc insert-media --document-id <id> --kind file --file ./report.pdf
  feishu-bot doc append-json --document-id <id> --file ./children.json
  feishu-bot doc append-descendant --document-id <id> --file ./descendant-body.json

Run `feishu-bot doc capabilities` for the full AI writing boundary. Use
--send-loop-check whenever a document link is sent as dogfood; it proves the
exact link message through message get/list, chat metadata, chat members, and
read-users probes.
"#;
pub(super) const DOC_MEDIA_AFTER_HELP: &str = r#"AI-safe docx media workflow:
  feishu-bot doc insert-media --document-id <id> --kind image --file ./image.png --width 640 --align 2
  feishu-bot doc insert-media --document-id <id> --kind file --file ./report.pdf --view-type 1
  feishu-bot doc blocks --document-id <id>
  feishu-bot doc raw --document-id <id>

insert-media automates Feishu's required docx media sequence:
  1. append an image/file placeholder block under --block-id or the document root
  2. upload the local asset with drive/v1/medias/upload_all using docx_image/docx_file
  3. patch the block with replace_image or replace_file

The command needs docx block write scopes plus docs:document.media:upload. It is
for files up to 20 MB, matching Feishu's upload_all media endpoint.
"#;
pub(super) const DOC_CAPABILITIES: &str = r#"Feishu docx AI writing capabilities

Recommended writer choice:
  --writer official   Feishu Markdown/HTML converter; best for normal AI docs,
                      tables, links, inline styles, lists, headings, and code.
  --writer local      Predictable direct block creation; no converter scope needed.
  append-json         Raw child blocks under one parent block.
  append-descendant   Raw nested descendant request body with explicit block IDs.
  insert-media        One-shot image/file block insertion with Drive media upload.

Local Markdown-ish writer:
  #..######### headings -> heading1..heading9
  plain text           -> text
  - item               -> bullet
  1. item              -> ordered
  > quote              -> quote
  - [ ] / - [x]        -> todo
  ---                  -> divider
  ```rust fences       -> code with CodeLanguage when Feishu has that enum
  ```mermaid fences    -> code block with PlainText language; source is preserved

Mermaid boundary:
  Feishu's public docx OpenAPI exposes diagram blocks with diagram_type
  1=flowchart and 2=UML, but does not expose a Mermaid source field. The
  official Markdown converter also maps ```mermaid to a normal code block.
  Therefore this CLI preserves Mermaid source as code in docx. For rendered
  Mermaid/PlantUML, create or locate a board block and use:
    feishu-bot board import --whiteboard-id <id> --syntax mermaid --file diagram.mmd

Raw subtype coverage:
  For block types or subtype fields not modeled by the local writer, generate
  Feishu's native JSON and call append-json or append-descendant. This is how AI
  should write table/table_cell descendants, grid/grid_column, iframe with
  iframe.component.type/url, file/image tokens, bitable, sheet, callout,
  isv/add_ons, board, agenda, link_preview, sub_page_list, and future writable
  block types.

Image/file media:
  Use `feishu-bot doc insert-media` for normal images and attachments. It creates
  the target block, uploads with drive media, then patches the block token.

Known non-writable public docx blocks:
  diagram/rendered Mermaid, mindnote, task blocks, synced blocks, and AI
  template blocks are not writable through the public docx OpenAPI today. Do not
  invent JSON for them.

Known BlockType labels:
  1 page, 2 text, 3..11 heading1..heading9, 12 bullet, 13 ordered, 14 code,
  15 quote, 17 todo, 18 bitable, 19 callout, 20 chat_card, 21 diagram,
  22 divider, 23 file, 24 grid, 25 grid_column, 26 iframe, 27 image, 28 isv,
  29 mindnote, 30 sheet, 31 table, 32 table_cell, 33 view, 34 quote_container,
  35 task, 36 okr, 37 okr_objective, 38 okr_key_result, 39 okr_progress,
  40 add_ons, 41 jira_issue, 42 wiki_catalog, 43 board, 44 agenda,
  45 agenda_item, 46 agenda_item_title, 47 agenda_item_content,
  48 link_preview, 49 source_synced, 50 reference_synced, 51 sub_page_list,
  52 ai_template, 999 undefined.
"#;
pub(super) const DOC_TEMPLATE_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc template --kind all
  feishu-bot doc template --kind support-matrix
  feishu-bot doc template --kind mermaid-code-child > mermaid.json
  feishu-bot doc append-json --document-id <id> --file mermaid.json
  feishu-bot doc template --kind image-child > image.json
  feishu-bot doc template --kind link-preview-child > link.json
  feishu-bot doc template --kind table-descendant > table.json
  feishu-bot doc append-descendant --document-id <id> --file table.json

Template classes:
  *-child        Request body for doc append-json / children API.
  *-descendant  Request body for doc append-descendant / descendant API.
  support-matrix Machine-readable write strategy for common docx block types.

Mermaid note:
  docx Mermaid is stored as source code. For rendered Mermaid, create or locate a
  board block, get its whiteboard_id from `feishu-bot doc blocks`, then run
  `feishu-bot board import --syntax mermaid --whiteboard-id <id>`.
"#;
pub(super) const DOC_PREVIEW_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc preview --file ./guide.md
  feishu-bot --json doc preview --file ./guide.md
  printf '# Title\n\n- item\n' | feishu-bot doc preview --stdin

This command does not call Feishu and does not need FEISHU_APP_ID/SECRET.
"#;
pub(super) const DOC_CREATE_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc create --title "Runbook" --file ./runbook.md
  feishu-bot doc create --title "Runbook" --writer official --content-type markdown --file ./runbook.md
  feishu-bot doc create --title "HTML import" --writer official --content-type html --file ./page.html
  feishu-bot doc create --title "Runbook" --stdin < ./runbook.md
  feishu-bot doc create --title "Runbook" --file ./runbook.md --send-to "$FEISHU_USER_ID" --send-loop-check
  feishu-bot doc create --title "Dogfood" --writer official --file ./demo.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok
  feishu-bot wiki create-node --auth user --space-id <space_id> --title "AI 演示" --obj-type docx
  feishu-bot doc append --auth user --document-id <wiki_obj_token> --writer official --file ./demo.md
  FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Dogfood" --file ./demo.md --wiki
  FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Dogfood" --file ./demo.md
  FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --title "Strict Wiki" --file ./demo.md --wiki-strict
  feishu-bot doc create --title "Private draft" --file ./draft.md --no-wiki

The root page block_id equals document_id, so appended content is inserted under
the document root by default.

Wiki publishing creates the docx first, then calls Wiki move_docs_to_wiki. Use
FEISHU_DOC_CREATE_WIKI_DEFAULT=true plus FEISHU_WIKI_SPACE_ID and optional
FEISHU_WIKI_PARENT_NODE_TOKEN to make this the default dogfood route. Use
--no-wiki for one-off local docs. When FEISHU_DOC_CREATE_WIKI_DEFAULT=true,
Wiki move failures keep and return the fallback docx unless --wiki-strict is
passed. Use --wiki-fallback-ok for explicit one-off --wiki commands that must
also return and send the fallback docx when Wiki permissions are not ready.
Use --send-loop-check whenever --send-to is part of dogfood; it proves the exact
doc link message with message get/list, chat metadata, chat members, and
read-users probes.
"#;
pub(super) const DOC_CONVERT_AFTER_HELP: &str = r#"Examples:
  feishu-bot doc convert --file ./guide.md
  feishu-bot doc convert --content-type html --file ./page.html
  feishu-bot --json doc convert --file ./guide.md

This calls Feishu's official Markdown/HTML -> docx blocks converter and needs
the docx:document.block:convert app scope.
"#;
pub(super) const DOC_RAW_BLOCK_AFTER_HELP: &str = r#"Advanced AI escape hatch:
  feishu-bot doc append-json --document-id <id> --file ./children.json
  feishu-bot doc append-json --document-id <id> --raw-json '[{"block_type":2,...}]'
  feishu-bot doc append-descendant --document-id <id> --file ./descendant-body.json

append-json accepts either:
  [{...block...}, {...block...}]
  {"children":[{...block...}]}

append-descendant accepts the full Feishu descendant request body, for example:
  {"index":-1,"children_id":["block_a"],"descendants":[{"block_id":"block_a",...}]}

Use this when the AI needs a newer/rarer Feishu block that the local writer does
not model yet.
"#;
pub(super) const BOARD_AFTER_HELP: &str = r#"AI-safe Board workflow:
  feishu-bot doc template --kind board-child > board.json
  feishu-bot doc append-json --document-id <doc_id> --file board.json
  feishu-bot doc blocks --document-id <doc_id>
  feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file ./diagram.mmd
  feishu-bot board import --whiteboard-id <whiteboard_id> --syntax plantuml --file ./diagram.puml
  feishu-bot board node-create --whiteboard-id <whiteboard_id> --file ./nodes.json

The docx `diagram` block is not writable through the public docx OpenAPI. The
Board API is the supported rendered Mermaid/PlantUML path when a whiteboard
block exists in the document.
"#;
pub(super) const BASE_AFTER_HELP: &str = r#"AI-safe Base workflow:
  feishu-bot base parse-url 'https://example.feishu.cn/base/<app_token>?table=<table_id>&view=<view_id>'
  feishu-bot base create --name "AI 工作台"
  feishu-bot base update --app-token <app_token> --name "AI 工作台 v2"
  feishu-bot base copy --app-token <app_token> --name "AI 工作台副本" --folder-token <folder_token>
  feishu-bot base table list --app-token <app_token>
  feishu-bot base table create --app-token <app_token> --name "需求"
  feishu-bot base table create --app-token <app_token> --name "需求" --default-view-name "默认视图" --field "标题:text" --field "状态:single-select:待处理:0|完成:1" --field "金额:currency:0.00|CNY" --field "截止日期:date:yyyy/MM/dd"
  feishu-bot base table batch-create --app-token <app_token> --name "需求" --name "实验"
  feishu-bot base table update --app-token <app_token> --table-id <table_id> --name "需求池"
  feishu-bot base field list --app-token <app_token> --table-id <table_id> --view-id <view_id> --text-field-as-array
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "状态" --kind single-select --option "待处理:0" --option "完成:1"
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "金额" --kind currency --formatter "0.00" --currency-code CNY
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "截止日期" --kind date --date-formatter "yyyy/MM/dd" --auto-fill false
  feishu-bot base field update --app-token <app_token> --table-id <table_id> --field-id <field_id> --name "阶段" --kind multi-select --option "进行中:2" --option "阻塞:3"
  feishu-bot base view list --app-token <app_token> --table-id <table_id>
  feishu-bot base view create --app-token <app_token> --table-id <table_id> --name "看板" --view-type kanban
  feishu-bot base view update --app-token <app_token> --table-id <table_id> --view-id <view_id> --hidden-field-id fld_internal --filter-conjunction and --filter-condition 'fld_status:3:is:json:["opt_done"]' --hierarchy-field-id fld_parent
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 标题=hello --field 分数=12.5 --field 完成=true
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 截止日期=date:2026-06-02 --field 会议时间=datetime:2026-06-02T10:30:00+08:00
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field '附件=json:[{"file_token":"<file_token>"}]'
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --fields-json '{"标题":"hello"}'
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --view-id <view_id> --field-name "标题" --automatic-fields
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --filter-json '{"conjunction":"and","conditions":[]}' --sort-json '[]'
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --body-json '{}'
  feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --records-json '[{"record_id":"rec...","fields":{"状态":"done"}}]'
  feishu-bot base record batch-create --app-token <app_token> --table-id <table_id> --record-field 0:标题=A --record-field 0:状态=open --record-field 1:标题=B
  feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --record-id rec_a --record-id rec_b --record-field 0:状态=done --record-field 1:清空=null
  feishu-bot base media upload --app-token <app_token> --kind file --file ./demo.mp4
  feishu-bot base media field-value --file-token <file_token> --field "附件"
  feishu-bot base record update --app-token <app_token> --table-id <table_id> --record-id <record_id> --field 状态=done --field 清空=null
  feishu-bot base record update --app-token <app_token> --table-id <table_id> --record-id <record_id> --fields-json '{"附件":[{"file_token":"<file_token>"}]}'
  feishu-bot base media tmp-url --file-token <file_token> --table-id <table_id> --field-id <field_id> --record-id <record_id>
  feishu-bot base media download --file-token <file_token> --output ./asset.bin --table-id <table_id> --field-id <field_id> --record-id <record_id>
  feishu-bot base dashboard list --app-token <app_token>
  feishu-bot base dashboard copy --app-token <app_token> --block-id <block_id> --name "指标副本"
  feishu-bot base workflow list --app-token <app_token>
  feishu-bot base workflow block-list --app-token <app_token>
  feishu-bot base workflow update --app-token <app_token> --workflow-id <workflow_id> --status disable
  feishu-bot base form get --app-token <app_token> --table-id <table_id> --form-id <form_id>
  feishu-bot base form update --app-token <app_token> --table-id <table_id> --form-id <form_id> --body-json '{...}'
  feishu-bot base update --app-token <app_token> --is-advanced true
  feishu-bot base role list --app-token <app_token> --api-version v2
  feishu-bot base role create --app-token <app_token> --api-version v2 --name "只读成员" --table-roles-json '[...]' --allow-base-complex-edit false --allow-copy false
  feishu-bot base role create --app-token <app_token> --api-version v2 --body-json '{"role_name":"只读成员","table_roles":[...],"base_rule":{"base_complex_edit":0,"copy":0}}'
  feishu-bot base member list --app-token <app_token> --role-id <role_id>
  feishu-bot base member add --app-token <app_token> --role-id <role_id> --member-id "$FEISHU_USER_ID" --member-id-type open_id
  feishu-bot base member batch-add --app-token <app_token> --role-id <role_id> --member open_id:ou_xxx --member chat_id:oc_xxx
  feishu-bot base field delete --app-token <app_token> --table-id <table_id> --field-id <field_id>
  feishu-bot base table batch-delete --app-token <app_token> --table-id <table_id>
  feishu-bot base table delete --app-token <app_token> --table-id <table_id>

Important:
  app_token is the token after /base/ or /app/ in the Base URL. table_id is
  usually in the table= query parameter, or from `base table list`. Use
  `base parse-url` when the user pastes a Base URL. If the URL starts with /wiki/, parse-url
  returns the wiki_node_token; run `feishu-bot wiki node --token <wiki_node_token>`
  and use obj_token as app_token when obj_type is bitable.

Tenant-token access only sees Bases that the app can access. For existing user
owned Bases, add the app as a document/Base collaborator in Feishu, or create
the Base through this CLI.

Base attachments are two-step: `base media upload` returns a file_token scoped
to the Base; write that token into an attachment field with `base record
create/update`. Use `base media field-value` to generate the attachment JSON.
For fields, prefer `base field create/update --kind ...` for common typed
fields such as text, number, currency, single-select, multi-select, date,
checkbox, user, phone, url, attachment, link, formula, location, group, and
auto-number. Use `--type`, `--ui-type`, and `--property-json` as the native
escape hatch when Feishu adds a new field capability.
For new tables, prefer `base table create --field "name:kind[:config]"` when
the AI needs fields at creation time. Config examples: select options split by
`|`, currency `formatter|CURRENCY`, date formatter, formula expression, linked
table_id, user/group `multiple=true`, or `json:{...}` for raw field.property.
For record writes, `--field name=value` parses JSON literals by default. Use
`str:` to force text, `json:` for native objects/arrays, `date:YYYY-MM-DD` for
local all-day Base date fields, and `datetime:` for RFC3339 or local
`YYYY-MM-DD HH:MM[:SS]` values. When field metadata can be read, plain
`YYYY-MM-DD`/`YYYY/MM/DD` strings are also converted automatically for Base
date fields.
For views, use typed update flags for hidden fields, filter_info, and
hierarchy_config; use --property-json for view capabilities not yet typed.
For Bases with advanced permissions, pass table/field/record IDs when
downloading so the tool can build the official bitablePerm extra.

Advanced permission role/member commands require the Base to have advanced
permissions enabled and the caller to have manageable permission on the Base.
For advanced permissions 2.0 custom roles, prefer `base role list/create
--api-version v2`; v2 adds the official `base_rule` permission points for
Base copy/download/print (`base_complex_edit`) and content copy (`copy`).
"#;
pub(super) const TASK_AFTER_HELP: &str = r#"AI-safe task workflow:
  feishu-bot task tasklist create --name "AI 项目清单"
  feishu-bot task tasklist list
  feishu-bot task tasklist tasks --tasklist-guid <tasklist_guid>
  feishu-bot task list --completed false --type my_tasks
  feishu-bot task tasklist add-member --tasklist-guid <tasklist_guid> --editor "$FEISHU_USER_ID"
  feishu-bot task tasklist remove-member --tasklist-guid <tasklist_guid> --viewer "$FEISHU_USER_ID"
  feishu-bot task create --summary "写周报" --description "整理本周进展" --assignee "$FEISHU_USER_ID"
  feishu-bot task create --summary "明天下午复核" --due-at 2026-06-02T15:00:00+08:00 --start-date 2026-06-02
  feishu-bot task create --summary "提交方案" --due-at "2026-06-03 18:00" --reminder-minute 30
  feishu-bot task create --summary "全天里程碑" --due-date 2026-06-05 --due-all-day --mode 1 --is-milestone true
  feishu-bot task create --summary "每周同步" --due-ms 1780000000000 --due-all-day --repeat-rule "FREQ=WEEKLY;INTERVAL=1"
  feishu-bot task create --summary "外部工单" --origin-json '{"platform_i18n_name":{"zh_cn":"AI系统"},"href":{"url":"https://example.com/t/1"}}' --custom-complete-json '{"pc":{"tip":{"zh_cn":"请去外部系统完成"}}}' --extra "eyJzb3VyY2UiOiJhaSJ9"
  feishu-bot task create --summary "里程碑" --due-date 2026-06-30 --mode 1 --is-milestone true --reminder-minute 30
  feishu-bot task get --guid <task_guid>
  feishu-bot task update --guid <task_guid> --summary "新标题" --due-at 2026-06-03T18:00:00+08:00
  feishu-bot task update --guid <task_guid> --clear-start --clear-repeat-rule --extra "e30="
  feishu-bot task member add --task-guid <task_guid> --assignee "$FEISHU_USER_ID"
  feishu-bot task member remove --task-guid <task_guid> --follower "$FEISHU_USER_ID"
  feishu-bot task tasklists --task-guid <task_guid>
  feishu-bot task add-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid> --section-guid <section_guid>
  feishu-bot task remove-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid>
  feishu-bot task section list --resource-type tasklist --resource-id <tasklist_guid>
  feishu-bot task section create --resource-type tasklist --resource-id <tasklist_guid> --name "进行中"
  feishu-bot task section tasks --section-guid <section_guid>
  feishu-bot task custom-field list --resource-type tasklist --resource-id <tasklist_guid>
  feishu-bot task custom-field create --resource-id <tasklist_guid> --name "优先级" --type single_select --option 高 --option 中 --option 低
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type single-select --option-guid <option_guid>
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type text --value "复核通过"
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type multi-select --option-guid <option_a> --option-guid <option_b>
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type member --member "$FEISHU_USER_ID"
  feishu-bot task custom-field option update --custom-field-guid <field_guid> --option-guid <option_guid> --is-hidden true
  feishu-bot task attachment list --resource-id <task_guid>
  feishu-bot task attachment upload --resource-id <task_guid> --file ./image.png --file ./brief.pdf
  feishu-bot task attachment delete --attachment-guid <attachment_guid>
  feishu-bot task reminder add --task-guid <task_guid> --reminder-minute 30
  feishu-bot task reminder remove --task-guid <task_guid> --reminder-id <reminder_id>
  feishu-bot task dependency add --task-guid <task_guid> --dependency-task-guid <next_task_guid>
  feishu-bot task dependency remove --task-guid <task_guid> --dependency-task-guid <next_task_guid>
  feishu-bot task comment create --task-guid <task_guid> --content "进展说明"
  feishu-bot task comment list --task-guid <task_guid>
  feishu-bot task comment get --comment-id <comment_id>
  feishu-bot task comment update --comment-id <comment_id> --content "更新后的说明"
  feishu-bot task comment delete --comment-id <comment_id>
  feishu-bot task complete --guid <task_guid>
  feishu-bot task subtask create --task-guid <task_guid> --summary "子任务"

Task create/update exposes official typed fields for due/start all-day,
completed_at, repeat_rule, custom_complete, origin, extra, reminders, mode,
is_milestone, and custom_fields. For future fields not yet typed, pass Feishu's
native task JSON:
  feishu-bot task create --body-json '{"summary":"任务","members":[...]}'
  feishu-bot task update --guid <task_guid> --body-json '{"task":{...},"update_fields":[...]}'
  feishu-bot task custom-field create --body-json '{"name":"价格","type":"number","resource_type":"tasklist","resource_id":"<tasklist_guid>","number_setting":{"format":"cny","decimal_count":2,"separator":"thousand"}}'

`feishu-bot task list` defaults to `--auth user` because Feishu's official task-list
API is user-access-token only and lists the caller's "my tasks" view. Use
`--completed true|false` to filter that view; `--type` defaults to `my_tasks`.
Set FEISHU_USER_ACCESS_TOKEN before using that command. Core task/tasklist/member/
reminder/subtask commands plus section/custom-field/attachment/dependency/
comment wrappers accept `--auth tenant|user`; use tenant auth for app-owned task
data and user auth when matching the logged-in user's Feishu Task Center
visibility. App scopes, tasklist permissions, and resource visibility still
matter. Custom field values are typed through `custom-field set-value`; use
`--clear` to set text, number, datetime, or single-select to an empty string,
and member/multi-select to an empty array.
Use --due-at/--start-at for RFC3339 timestamps or local "YYYY-MM-DD HH:MM[:SS]"
values; use --due-date/--start-date for all-day dates. The old --due-ms and
--start-ms remain available when the AI already has Feishu millisecond values.
Task reminders are relative to the task due time in Feishu. Use
`--reminder-minute` during task/subtask creation or `task reminder add`; existing
reminders should be changed by `reminder remove` then `reminder add`. Feishu
currently supports one reminder per task.
"#;
pub(super) const API_AFTER_HELP: &str = r#"Raw Feishu OpenAPI escape hatch:
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
pub(super) const DRIVE_AFTER_HELP: &str = r#"AI-safe Drive workflow:
  feishu-bot drive list --folder-token <folder_token>
  feishu-bot drive folder create --name "AI 输出" --folder-token ""
  feishu-bot drive upload --folder-token <folder_token> --file ./report.pdf
  feishu-bot drive upload-large --folder-token <folder_token> --file ./large-video.mp4
  feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png
  feishu-bot drive media upload --parent-type bitable_file --parent-node <app_token> --drive-route-token <app_token> --file ./video.mp4
  feishu-bot drive media download --file-token <media_token> --output ./asset.bin
  feishu-bot drive import file --file ./page.html --type docx --folder-token "" --title "HTML Preview"
  feishu-bot drive import get --ticket <ticket>
  feishu-bot drive export file --token <docx_token> --type docx --file-extension pdf --output ./doc.pdf
  feishu-bot drive export create --token <sheet_token> --type sheet --file-extension xlsx
  feishu-bot drive comment create --file-token <docx_token> --file-type docx --text "需要复核"
  feishu-bot drive comment list --file-token <docx_token> --file-type docx --is-whole
  feishu-bot drive version create --file-token <docx_token> --obj-type docx --name "AI 修订版"
  feishu-bot drive view-record --file-token <docx_token> --file-type docx
  feishu-bot drive download --file-token <file_token> --output ./report.pdf
  feishu-bot drive permission public-get --token <docx_token> --file-type docx
  feishu-bot drive permission member-list --token <docx_token> --file-type docx
  feishu-bot drive permission member-add --token <docx_token> --file-type docx --member-id "$FEISHU_USER_ID" --perm edit
  feishu-bot drive stats --file-token <token> --file-type docx
  feishu-bot drive copy --file-token <token> --file-type docx --folder-token <folder_token>

Folder token "" means the root folder for create-folder/import. Existing
user-owned folders still need the app to have document/folder access. `upload`
uses drive/v1/files/upload_all for Drive files. `media upload` uses
drive/v1/medias/upload_all for doc/sheet/Base assets and HTML/Markdown import
staging. Both single-call upload paths support non-empty files up to 20 MB; use
`upload-large` for Drive files that need the official multipart
upload_prepare/upload_part/upload_finish flow.
`export` creates/polls/downloads asynchronous docx/sheet/Base export tasks.
`comment` manages global comments and replies; use raw JSON for complex comment
elements. `subscription` uses user_access_token because Feishu's subscription
API is user-token only.
`permission member-list` is the readback step after sharing a docx/sheet/Base
with a user or chat; verify collaborators before claiming the recipient can
access the artifact.
For media assets embedded in docs/sheets/Base, use the matching media endpoint
and Feishu block/parent token semantics.
"#;
pub(super) const CALENDAR_AFTER_HELP: &str = r#"AI-safe calendar workflow:
  feishu-bot calendar primary
  feishu-bot calendar list
  feishu-bot calendar create --summary "AI 日历"
  feishu-bot calendar event create --calendar-id <id> --summary "同步会" --start-ts 1760000000 --end-ts 1760003600
  feishu-bot calendar event list --calendar-id <id>
  feishu-bot calendar freebusy list --user-id "$FEISHU_USER_ID" --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
  feishu-bot calendar freebusy batch --user-id ou_xxx --user-id ou_yyy --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
  feishu-bot calendar attendee add --calendar-id <id> --event-id <event_id> --user "$FEISHU_USER_ID"
  feishu-bot calendar attendee list --calendar-id <id> --event-id <event_id>
  feishu-bot calendar attendee delete --calendar-id <id> --event-id <event_id> --attendee-id <attendee_id>
  feishu-bot calendar attendee chat-members --calendar-id <id> --event-id <event_id> --attendee-id <chat_attendee_id>

For rooms, recurrence, reminders, conferencing, and complex attendee fields,
pass native Feishu JSON with --body-json/--file/--stdin.
"#;
pub(super) const WIKI_AFTER_HELP: &str = r#"AI-safe wiki workflow:
  feishu-bot wiki route-check
  feishu-bot wiki route-check --write-probe
  feishu-bot wiki route-check --write-probe --strict
  feishu-bot wiki spaces
  feishu-bot wiki nodes --space-id <space_id>
  feishu-bot wiki create-node --space-id <space_id> --title "AI 演示" --obj-type docx
  feishu-bot wiki move-docs-to-wiki --space-id <space_id> --obj-type docx --obj-token <document_id>
  feishu-bot wiki node --token <wiki_node_token>
  feishu-bot wiki task --task-id <task_id>

Admin workflows:
  feishu-bot wiki member list --space-id <space_id>
  feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id <open_id> --member-role admin
  feishu-bot wiki setting update --space-id <space_id> --create-setting admin_and_member

User-token workflows:
  feishu-bot wiki create-space --name "AI 知识库"
  feishu-bot wiki create-node --auth user --space-id <space_id> --title "AI 文档" --obj-type docx
  feishu-bot doc append --auth user --document-id <obj_token> --file ./doc.md
  feishu-bot wiki search --query "关键字"

Wiki nodes reference underlying doc/sheet/bitable/file tokens. Use the matching
typed command to edit the underlying object after locating it. For dogfood,
publish one standalone docx, move it into Wiki, then read both the wiki node and
underlying docx back before reporting success.
Run route-check first when the AI must decide whether future reports can go
through Wiki by default; the normal check verifies config plus read access, and
`--write-probe` creates a proof docx and attempts the real Wiki move. Add
`--strict` in automation so the command exits non-zero unless route_ready is
true.
"#;
pub(super) const SHEET_AFTER_HELP: &str = r#"AI-safe sheets workflow:
  feishu-bot sheet create --title "AI 数据表" --folder-token <folder_token>
  feishu-bot sheet get --spreadsheet-token <token>
  feishu-bot sheet sheets --spreadsheet-token <token>
  feishu-bot sheet get-sheet --spreadsheet-token <token> --sheet-id <sheet_id>
  feishu-bot sheet add-sheet --spreadsheet-token <token> --title "数据" --index 1
  feishu-bot sheet update-sheet --spreadsheet-token <token> --sheet-id <sheet_id> --title "新标题" --frozen-row-count 1
  feishu-bot sheet copy-sheet --spreadsheet-token <token> --sheet-id <sheet_id> --title "副本"
  feishu-bot sheet delete-sheet --spreadsheet-token <token> --sheet-id <sheet_id>
  feishu-bot sheet values get --spreadsheet-token <token> --range Sheet1!A1:C10
  feishu-bot sheet values update --spreadsheet-token <token> --range Sheet1!A1:B2 --values-json '[[1,2],[3,4]]'
  feishu-bot sheet values append --spreadsheet-token <token> --range Sheet1!A:B --values-json '[["new","row"]]'
  feishu-bot sheet values prepend --spreadsheet-token <token> --range Sheet1!A:B --values-json '[["top","row"]]'
  feishu-bot sheet merge --spreadsheet-token <token> --range Sheet1!A1:C1 --merge-type MERGE_ALL
  feishu-bot sheet unmerge --spreadsheet-token <token> --range Sheet1!A1:C1
  feishu-bot sheet style --spreadsheet-token <token> --range Sheet1!A1:C1 --bold true --back-color fff2cc --border-type FULL_BORDER

Use `sheet create` to start from zero, then manage tabs with add/update/copy/delete
and write cells with values update/append/prepend. Use merge/unmerge/style to
make AI-generated tables readable before sending them. Use --body-json for
complex Sheets v2/v3 native payloads.
"#;
pub(super) const APPROVAL_AFTER_HELP: &str = r#"AI-safe approval workflow:
  feishu-bot approval definition get --approval-code <code>
  feishu-bot approval definition subscribe --approval-code <code>
  feishu-bot approval instance list --approval-code <code> --start-time <ms> --end-time <ms>
  feishu-bot approval instance query --approval-code <code> --instance-status PENDING
  feishu-bot approval instance get --instance-code <code>
  feishu-bot approval instance create --body-json '{...}'
  feishu-bot approval instance cancel --approval-code <code> --instance-code <code> --user-id <open_id>
  feishu-bot approval task search --approval-code <code> --task-status PENDING
  feishu-bot approval task approve --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment OK
  feishu-bot approval task reject --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment "needs changes"
  feishu-bot approval task transfer --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --transfer-user-id <open_id>
  feishu-bot approval task add-sign --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --add-user-id <open_id> --add-sign-type 3
  feishu-bot approval task rollback --task-id <task_id> --user-id <open_id> --task-def-key START --reason "revise"
  feishu-bot approval external definition-get --approval-code <code>
  feishu-bot approval external definition-create --file external-definition.json
  feishu-bot approval external instance-sync --file external-instance.json
  feishu-bot approval external instance-check --file external-check.json

Approval forms are schema-specific. Prefer --body-json copied from the approval
definition or OpenAPI explorer. Use `definition get` before creating an
instance so the AI can inspect form widget IDs, node keys, and task IDs.
"#;
pub(super) const VC_AFTER_HELP: &str = r#"AI-safe video meeting workflow:
  feishu-bot vc reserve apply --end-time <sec> --owner-id <open_id> --topic "AI sync"
  feishu-bot vc reserve get --reserve-id <reserve_id>
  feishu-bot vc reserve active-meeting --reserve-id <reserve_id> --with-participants
  feishu-bot vc reserve update --reserve-id <reserve_id> --end-time <sec>
  feishu-bot vc reserve delete --reserve-id <reserve_id>
  feishu-bot vc meeting get --meeting-id <meeting_id>
  feishu-bot vc meeting list-by-no --meeting-no 123456789 --start-time <sec> --end-time <sec>
  feishu-bot vc meeting invite --meeting-id <meeting_id> --user <open_id>
  feishu-bot vc meeting set-host --meeting-id <meeting_id> --user-id <open_id>
  feishu-bot vc meeting end --meeting-id <meeting_id>
  feishu-bot vc recording get --meeting-id <meeting_id>
  feishu-bot vc recording start --meeting-id <meeting_id> --timezone 8
  feishu-bot vc recording stop --meeting-id <meeting_id>
  feishu-bot vc recording set-permission --meeting-id <meeting_id> --user <open_id>
  feishu-bot vc report daily --start-time <sec> --end-time <sec>
  feishu-bot vc report top-user --start-time <sec> --end-time <sec> --limit 10 --order-by 1
  feishu-bot vc room list --page-size 20
  feishu-bot vc room get --room-id <room_id>
  feishu-bot vc room mget --room-id <room_id>
  feishu-bot vc room-level list --page-size 20

Reserve APIs can use tenant or user auth. In-meeting invite/end and recording
start/stop/permission APIs usually require user_access_token and meeting host or
participant permission. Use --auth tenant only for endpoints that Feishu allows
to run as the app, such as set-host and reserve operations. Set-host may require
both vc:meeting and vc:meeting.participant:write.
"#;
pub(super) const MINUTES_AFTER_HELP: &str = r#"AI-safe Minutes workflow:
  feishu-bot minutes search --query "周会" --page-size 20
  feishu-bot minutes get --minute-token <minute_token_or_url>
  feishu-bot minutes artifacts --minute-token <minute_token_or_url>
  feishu-bot minutes media --minute-token <minute_token_or_url>
  feishu-bot minutes transcript --minute-token <minute_token_or_url> --need-speaker --need-timestamp --file-format txt --output ./minute.txt

Minute tokens can be passed directly or as full Feishu/Lark minutes URLs. Search
supports --filter-json and --body-json for native Feishu filter payloads.
"#;
pub(super) const SEARCH_AFTER_HELP: &str = r#"AI-safe Search workflow:
  feishu-bot search docs --query "飞书Bot" --page-size 10
  feishu-bot search message --query "上线" --chat-id oc_xxx --page-size 20
  feishu-bot search source list --page-size 20
  feishu-bot search schema create --file ./schema.json
  feishu-bot search source create --name "AI 索引" --schema-id ai_schema --state 0
  feishu-bot search item create --data-source-id <id> --id item_1 --title "标题" --url "https://example.com" --text "全文"

Docs/message search requires FEISHU_USER_ACCESS_TOKEN. Search connector
source/schema/item commands use tenant_access_token and need search:data_source
scopes.
"#;
pub(super) const OKR_AFTER_HELP: &str = r#"AI-safe OKR workflow:
  feishu-bot okr period list --page-size 20
  feishu-bot okr period-rule list
  feishu-bot okr user-okrs --user-id "$FEISHU_USER_ID" --offset 0 --limit 5
  feishu-bot okr batch-get --okr-id <okr_id> --lang zh_cn

OKR commands use tenant_access_token by default and require OKR scopes such as
okr:okr.period:readonly, okr:okr:readonly, or okr:okr. Some tenants also require
Feishu OKR enterprise edition.
"#;
pub(super) const ATTENDANCE_AFTER_HELP: &str = r#"AI-safe Attendance workflow:
  feishu-bot attendance group list --page-size 20
  feishu-bot attendance shift list --page-size 20
  feishu-bot attendance shift query --shift-name "早班"
  feishu-bot attendance schedule query --user-id <employee_id> --from 20260501 --to 20260531
  feishu-bot attendance task query --user-id <employee_id> --from 20260501 --to 20260531 --ignore-invalid-users
  feishu-bot attendance flow query --user-id <employee_id> --from-ts 1760000000 --to-ts 1760086400
  feishu-bot attendance stats query --user-id <employee_id> --operator-user-id <employee_id> --from 20260501 --to 20260531

Attendance commands use tenant_access_token and require attendance scopes:
attendance:rule/attendance:rule:readonly for groups and shifts, and
attendance:task/attendance:task:readonly for schedules, tasks, flows, and stats.
Employee IDs default to employee_id; use --employee-type employee-no for work
numbers. flow delete accepts at most 10 record IDs per request.
"#;
pub(super) const MAIL_AFTER_HELP: &str = r#"AI-safe Mail workflow:
  feishu-bot mail message list --mailbox me --page-size 10
  feishu-bot mail message get --mailbox me --message-id <message_id> --format metadata
  feishu-bot mail folder list --mailbox me
  feishu-bot mail settings send-as --mailbox me
  feishu-bot mail settings accessible --mailbox me
  feishu-bot mail contact list --mailbox me --page-size 20
  feishu-bot mail message send --mailbox me --to user@example.com --subject "hello" --text "body"

Mail commands use user_access_token when --mailbox me or --auth user is used.
Sending mail always requires FEISHU_USER_ACCESS_TOKEN and
mail:user_mailbox.message:send. Tenant-token reads of explicit mailboxes also
require Mail data resource permissions in the Feishu Open Platform.
"#;
pub(super) const COREHR_AFTER_HELP: &str = r#"AI-safe CoreHR workflow:
  feishu-bot corehr department search --page-size 20 --field department_name --field code
  feishu-bot corehr department get --department-id <id> --field department_name
  feishu-bot corehr job list --page-size 20
  feishu-bot corehr job get --job-id <id>
  feishu-bot corehr job batch-get --job-id <id> --field job_name
  feishu-bot corehr job-data query --employment-id <id> --page-size 20
  feishu-bot corehr job-data get --job-data-id <id>
  feishu-bot corehr person get --person-id <id>
  feishu-bot corehr process list --modify-time-from <ms> --modify-time-to <ms> --page-size 20
  feishu-bot corehr process get --process-id <id>

CoreHR commands use tenant_access_token and require CoreHR scopes plus Feishu
People data-range grants. Use --body-json/--file/--stdin for full official
CoreHR filters that are not exposed as typed flags.
"#;

pub(super) const HELPDESK_AFTER_HELP: &str = r#"AI-safe Helpdesk workflow:
  feishu-bot helpdesk ticket list --page-size 20
  feishu-bot helpdesk ticket get --ticket-id <ticket_id>
  feishu-bot helpdesk ticket messages --ticket-id <ticket_id> --page-size 20
  feishu-bot helpdesk service start --open-id <open_id> --human-service
  feishu-bot helpdesk message send --receiver-id <open_id> --text "hello"
  feishu-bot helpdesk faq categories --lang zh_cn
  feishu-bot helpdesk faq list --search "登录" --page-size 20

Helpdesk APIs require tenant_access_token plus FEISHU_HELPDESK_ID and
FEISHU_HELPDESK_TOKEN. The CLI sends X-Lark-Helpdesk-Authorization as
base64(helpdesk_id:helpdesk_token). Use --body-json/--file/--stdin for full
official bodies when typed flags are not enough.
"#;

pub(super) const HIRE_AFTER_HELP: &str = r#"AI-safe Hire workflow:
  feishu-bot hire job list --page-size 20
  feishu-bot hire job detail --job-id <job_id>
  feishu-bot hire job schemas --scenario 1
  feishu-bot hire process list --page-size 50
  feishu-bot hire talent list --keyword "张三" --page-size 10
  feishu-bot hire talent get --talent-id <talent_id>
  feishu-bot hire application list --talent-id <talent_id> --page-size 20
  feishu-bot hire application get --application-id <application_id>
  feishu-bot hire interview by-talent --talent-id <talent_id>
  feishu-bot hire requirement schemas --page-size 20

Write operations are explicit:
  feishu-bot hire talent create --name "张三" --email zhangsan@example.com
  feishu-bot hire job open --job-id <job_id> --is-never-expired true
  feishu-bot hire talent create --body-json '{...official combined_create body...}'

Hire APIs use tenant_access_token and Feishu Hire data ranges. Sensitive fields
such as user_id require contact:user.employee_id:readonly. Use
--body-json/--file/--stdin when the official Hire payload has custom fields or a
tenant-specific schema.
"#;

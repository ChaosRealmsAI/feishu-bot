# 飞书Bot (`feishu-bot`)

飞书Bot is a Rust project for using a Feishu/Lark custom app as a local bot.
The Cargo package is `feishu-bot`; the installed command is `feishu-bot`.

It reuses these environment variables:

```bash
FEISHU_APP_ID=cli_xxx
FEISHU_APP_SECRET=xxx
FEISHU_USER_ID=ou_xxx
# Optional, only for APIs that Feishu requires to run as a user, such as task
# list, docs/message search, minutes search, and mail send.
FEISHU_USER_ACCESS_TOKEN=u_xxx
FEISHU_REFRESH_TOKEN=r_xxx
FEISHU_OAUTH_REDIRECT_URI=http://localhost:8080/callback
# Optional, makes `feishu-bot doc create` publish new docs into Wiki by default.
FEISHU_WIKI_SPACE_ID=wiki_space_id
FEISHU_WIKI_PARENT_NODE_TOKEN=wik_xxx
FEISHU_DOC_CREATE_WIKI_DEFAULT=true
# Optional, overrides the local high-level office workflow state file.
FEISHU_OFFICE_STATE_FILE=private/office-projects.json
# With FEISHU_DOC_CREATE_WIKI_DEFAULT=true, doc create keeps the fallback docx
# and returns wiki_move_error if Wiki moving is blocked. Use --wiki-strict to fail.
# Optional, only for Helpdesk APIs.
FEISHU_HELPDESK_ID=123456
FEISHU_HELPDESK_TOKEN=ht_xxx
```

The CLI loads environment variables from the process, project `.env`,
current-directory `.env`, and optional `FEISHU_ENV_FILE` or `LARK_ENV_FILE`.
Examples that pass `"$FEISHU_USER_ID"` still need that variable exported in the
shell, or you can pass the concrete user ID directly.

When `dogfood verify` reports `missing_user_token`, start the OAuth v2 flow.
When it reports `expired_user_token`, try `oauth refresh` first and rerun the
probe; if refresh fails, rerun the OAuth URL/token flow. For automated checks,
`dogfood verify --auto-refresh-user-token` can refresh, save, and retry expired
user-token probes in one run.

```bash
feishu-bot oauth url --scope offline_access --scope auth:user.id:read --scope task:task:read
feishu-bot browser open --url "<authorization_url>"
feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env --env-file private/local.env
feishu-bot oauth refresh --save-env --env-file private/local.env
feishu-bot oauth user-info
feishu-bot dogfood verify --profile office --auto-refresh-user-token --strict --json
```

## Code layout

The bot is split by module instead of keeping all logic in one file:

- `src/lib.rs` owns the shared application entrypoint and JSON error handling.
- `src/bin/feishu-bot.rs` is the primary command entrypoint.
- `src/bin/feishuBot.rs` and `src/bin/feishu.rs` are compatibility aliases.
- `src/app/mod.rs` wires the app modules and handles only the outer
  no-config/no-API command dispatch. `src/app/dispatch.rs` routes configured
  API commands, `src/app/doctor.rs` prints masked connectivity diagnostics, and
  `src/app/response.rs` centralizes Feishu JSON/binary response parsing.
- `src/app/cli/` contains Clap command and argument definitions, split into
  generated parts plus focused workflow submodules such as the office project
  and interaction argument groups.
- `src/app/office.rs` contains the AI-first dispatcher plus bootstrap workflow;
  `src/app/office/` contains report/progress workflows, interaction/status
  workflows, local dry-run/list helpers, document writers, resource writers,
  formatting, links, readback, and state helpers. `src/app/office/resources/`
  splits bootstrap resources by chat, Wiki, Base, chat tabs, and summary/pin
  messages. `src/app/office/local/` splits API-free list, dry-run, status,
  cleanup, and local dispatch helpers.
- `src/app/client.rs` contains the Feishu HTTP client type; `src/app/client/`
  contains token/auth/request execution plus document, IM/chat, Board, Drive,
  and Minutes convenience methods. `src/app/client/request/` splits token/auth,
  common JSON wrappers, raw JSON execution, Helpdesk, binary, and multipart
  request helpers.
- `src/app/common.rs` re-exports shared helpers; `src/app/common/` contains
  query/path helpers, content and key-value input readers, JSON body helpers,
  and ID/content-type argument resolvers.
- `src/app/config.rs` contains `.env` loading, base URL selection, and masking helpers.
- `src/app/approval.rs` contains Approval execution; `src/app/approval/`
  contains Approval body builders and query helper builders.
- `src/app/attendance.rs` contains Attendance execution, group/shift/schedule/task/flow/stats operations, and attendance body builders.
- `src/app/base.rs` contains Base/Bitable command execution; `src/app/base/`
  contains schema, field schema, record, and permission execution, media,
  reference, and helper builders split by query, view, and permission domains.
  `src/app/base/field_schema/` separates AI-friendly table field specs from
  OpenAPI field body construction. `src/app/base/records/` separates record
  field input parsing, batch record input parsing, and date-field normalization.
- `src/app/board.rs` contains Board/whiteboard execution, Mermaid/PlantUML import, raw node creation, and Board node JSON normalization.
- `src/app/calendar.rs` contains Calendar command execution; `src/app/calendar/`
  contains Calendar event, attendee, and free/busy body builders.
- `src/app/chat.rs` contains Chat execution; `src/app/chat/` contains chat
  create/update, tab/menu, and member body/query builders.
- `src/app/doc.rs` contains docx command execution; `src/app/doc/` contains
  Markdown/local block mapping helpers, docx media insertion helpers, raw
  descendant body normalization, and AI-ready raw block templates split by
  support matrix, child blocks, and descendant blocks.
- `src/app/dogfood.rs` contains dogfood command dispatch plus write/message
  loop probes; `src/app/dogfood/` contains publish execution, read-probe verify
  orchestration, read-probe specs, probe classification, summary aggregation,
  and AI remediation helpers. `src/app/dogfood/probes/` splits probe output,
  error classification, remediation, and module-filter helpers.
- `src/app/drive.rs` contains the Drive command dispatcher; `src/app/drive/`
  contains Drive execution modules for file, media, comment, version,
  subscription, permission, and import/export operations. Query/body builders
  stay in focused helper modules such as `src/app/drive/comment.rs`,
  `src/app/drive/permissions.rs`, `src/app/drive/transfer.rs`, and
  `src/app/drive/helpers/`.
- `src/app/helpdesk.rs` contains Helpdesk execution, ticket/message/FAQ operations, service-start body builders, and helpdesk message body builders.
- `src/app/oauth.rs` contains OAuth command dispatch; `src/app/oauth/` contains
  authorization URL/PKCE helpers, token exchange/refresh/user-info requests,
  token masking/env persistence, and human output formatting.
- `src/app/raw_api.rs` contains raw Feishu API passthrough execution.
- `src/app/search.rs` contains Search execution, doc/message search, and custom search connector body builders.
- `src/app/setup.rs` contains setup command dispatch; `src/app/setup/`
  contains first-run plan/scope builders, quickstart/auto/open-scopes flows,
  doctor/Wiki probes, Wiki bot membership automation, and browser open helpers.
- `src/app/sheet.rs` contains Sheets execution; `src/app/sheet/` contains sheet
  tab/style/value body builders.
- `src/app/task.rs` contains Task command execution; `src/app/task/` contains
  tasklist and structure execution, Task body/query helpers,
  input/time/reminder normalization helpers, and tasklist/comment/member
  collaboration builders plus section and reminder/dependency relation builders.
  `src/app/task/helpers/` splits task create/update bodies, shared query/request
  helpers, member builders, input normalization, collaboration, and relations.
  `src/app/task/custom_field/` splits custom-field metadata, option, setting,
  and value body builders.
- `src/app/vc.rs` contains VC execution and meeting/reserve/recording/report/room
  routing; `src/app/vc/` contains VC request body builders.
- `src/app/wiki.rs` contains Wiki/knowledge-space command execution;
  `src/app/wiki/` contains typed Wiki request body builders, and
  `src/app/wiki/route/` splits route-check calls, write-probe publishing,
  recommendation text, and the shared Wiki request wrapper.
- `src/app/help.rs` exposes AI help text; `src/app/help/` splits long
  AI-facing help sections by workflow area. `src/app/manifest.rs` builds the
  machine-readable AI manifest; `src/app/manifest/` splits module metadata by
  workflow area. `src/app/scopes.rs` prints and filters scope grants, with the
  static scope registry split by domain in `src/app/scopes/groups/`;
  `src/app/output.rs` re-exports output helpers, while `src/app/output/`
  contains generic response summaries, document block/code output, and Feishu
  block/code label maps.
- `src/app/mail.rs` contains Mail execution, mailbox auth selection, message/folder/contact/settings/rule/label operations, and mail send body builders.
- `src/app/message.rs` contains IM message execution, image/file/video helpers,
  reactions, and pins; `src/app/message/` contains voice synthesis/conversion
  send entrypoints plus voice prepare/audio/synth helpers, message
  content/reaction body builders, polling cursor/state, and loop-check/readback
  probe helpers.
- `src/app/minutes.rs` contains Minutes execution, metadata/artifacts/media/transcript operations, token parsing, and search body builders.
- `src/app/okr.rs` contains OKR execution, OKR query helpers, and ID validation.
- `src/app/people.rs` contains Contact, Directory, and CoreHR execution and
  body builders; `src/app/people/` contains Directory, CoreHR, and Hire
  execution and helpers. Hire-specific query/body/id-type helpers live under
  `src/app/people/hire/`.
- `src/app/tests.rs` contains unit tests.

See `docs/ARCHITECTURE.md` before adding new Feishu APIs.
See `docs/DOGFOOD.md` before claiming a capability works.
See `docs/QUALITY_STATUS.md` before continuing broad refactors.
See `skills/feishu-bot-office/SKILL.md` for the repeatable AI office workflow.

## Quick checks

```bash
scripts/check-commit.sh
scripts/ci-local.sh
feishu-bot ai
feishu-bot manifest
feishu-bot --help
feishu-bot setup --help
feishu-bot setup plan
feishu-bot dogfood verify --profile office --auto-refresh-user-token --strict --json
feishu-bot oauth --help
feishu-bot scopes --group all
feishu-bot doc capabilities
feishu-bot doc template --kind all
feishu-bot doc template --kind support-matrix
feishu-bot board --help
feishu-bot contact --help
feishu-bot directory --help
feishu-bot chat --help
feishu-bot message --help
feishu-bot message list --help
feishu-bot message reaction --help
feishu-bot message pin --help
feishu-bot base --help
feishu-bot base media --help
feishu-bot task --help
feishu-bot drive --help
feishu-bot calendar --help
feishu-bot vc --help
feishu-bot minutes --help
feishu-bot search --help
feishu-bot okr --help
feishu-bot attendance --help
feishu-bot mail --help
feishu-bot corehr --help
feishu-bot helpdesk --help
feishu-bot hire --help
feishu-bot wiki --help
feishu-bot sheet --help
feishu-bot approval --help
feishu-bot api --help
feishu-bot doctor
feishu-bot office --help
feishu-bot office list
feishu-bot office bootstrap --project "demo" --dry-run
feishu-bot office report --project "demo" --title "dry run" --content "hello" --dry-run
feishu-bot office status --project "demo"
feishu-bot dogfood verify --profile office --auto-refresh-user-token --strict --json
feishu-bot --json manifest --module base
feishu-bot doc preview --file ./docs/feishu-bot-guide.md
```

The commands above are read-only or local-only except `dogfood verify`, which is
read-only by default but may call real Feishu probes. The commands below create
real Feishu data and should only be used when you intentionally want a smoke
object/message:

```bash
feishu-bot office progress --project "demo" --title "status" --summary "ok"
feishu-bot dogfood publish --title "Bot smoke" --file ./demo.md
```

First-run setup that may operate the signed-in browser or add the app bot to a
Wiki space is intentionally separate from quick checks:

```bash
feishu-bot setup quickstart --no-wiki-bot
feishu-bot setup quickstart --open-browser
```

Install it as a normal CLI:

```bash
cargo install --path .
feishu-bot doctor
```

Required app scopes for the full workflow:

- `im:message`, `im:message:readonly`, `im:message.history:readonly`, `im:message:send_as_bot`, `im:message:update`, `im:message:recall`
- `im:message.group_at_msg:readonly`, `im:message.group_msg`, `im:message.p2p_msg:readonly`
- `im:message.reactions:read`, `im:message.reactions:write_only`
- `im:message.pins:read`, `im:message.pins:write_only`
- `im:resource`
- `im:chat`, `im:chat:create`, `im:chat:operate_as_owner`, `im:chat.group_info:readonly`
- `contact:contact.base:readonly`, `contact:contact:access_as_app`, `contact:contact:readonly`, `contact:contact:readonly_as_app`
- Directory scopes from `feishu-bot scopes --group directory`, including
  `directory:employee:search`, `directory:employee:read`,
  `directory:employee:list`, and per-field employee/department/job/work-place
  read scopes.
- `docx:document`, `docx:document:create`
- `docx:document.block:convert`
- `board:whiteboard:node:create`, `board:whiteboard:node:read`
- `bitable:app`, `bitable:app:readonly`
- Base scopes from `feishu-bot scopes --group base`, including app/table/field/view/record, dashboard, workflow, form, role/member, and media scopes
- `task:task:write` or `task:task:writeonly`; `task:task:read` / `task:task:readonly` for reads
- Tasklist/comment scopes from `feishu-bot scopes --group task`
- Drive scopes from `feishu-bot scopes --group drive`, including `drive:file`,
  `drive:file:readonly`, `drive:file:upload`, and `drive:file:download`
- VC scopes from `feishu-bot scopes --group vc`, including meeting/reserve write
  scopes, `vc:meeting.participant:write`, report, recording, meeting room, and
  room-level read scopes
- Minutes scopes from `feishu-bot scopes --group minutes`, including basic read,
  search, AI artifacts, media export, and transcript export. `minutes search`
  also requires `FEISHU_USER_ACCESS_TOKEN`.
- Search scopes from `feishu-bot scopes --group search`, including
  `search:docs:read`, `search:message`, `search:data_source`, and
  `search:data_source:readonly`. Docs/message search requires
  `FEISHU_USER_ACCESS_TOKEN`.
- OKR scopes from `feishu-bot scopes --group okr`, including
  `okr:okr.period:readonly`, `okr:okr:readonly`, and `okr:okr`.
- Attendance scopes from `feishu-bot scopes --group attendance`, including
  `attendance:rule`, `attendance:rule:readonly`, `attendance:task`, and
  `attendance:task:readonly`.
- Mail scopes from `feishu-bot scopes --group mail`, including message read/send,
  mailbox, folder, contact, rule, and sensitive field scopes. `--mailbox me`
  and sending mail require `FEISHU_USER_ACCESS_TOKEN`.
- CoreHR scopes from `feishu-bot scopes --group corehr`, including department,
  job, job-data, person, process, and sensitive field scopes. CoreHR also needs
  Feishu People data-range grants in the Open Platform.
- Helpdesk scopes from `feishu-bot scopes --group helpdesk`, including
  `helpdesk:all:readonly`, `helpdesk:all`, and
  `helpdesk:helpdesk:access`. Helpdesk APIs also require
  `FEISHU_HELPDESK_ID` and `FEISHU_HELPDESK_TOKEN`.
- Hire scopes from `feishu-bot scopes --group hire`, including job, talent,
  `hire:people_cli`, application, interview, process, requirement, attachment,
  location, subject, site, and related sensitive entity scopes. Hire also needs Feishu Hire
  product/data-range grants in the Open Platform.
- Cloud document permission scopes from `feishu-bot scopes --group permission`,
  including `docs:permission.member:*` and `docs:permission.setting:*`
- Sheet scopes from `feishu-bot scopes --group sheet`, including create, metadata read,
  and values write scopes
- Calendar, Wiki, and Approval scopes prompted by their API errors

## AI handoff

Future AI agents should start with:

```bash
feishu-bot ai
feishu-bot manifest
feishu-bot scopes --group all
feishu-bot scopes --group directory
feishu-bot scopes --group helpdesk
feishu-bot scopes --group hire
feishu-bot doctor
```

`feishu-bot ai` is embedded in the binary and does not need credentials. `feishu-bot
manifest` prints a machine-readable module/scope/command manifest for future AI
agents; use `feishu-bot manifest --module directory` or
`feishu-bot manifest --module helpdesk` or `feishu-bot manifest --module hire` for
compact product-specific handoffs.
`feishu-bot scopes` prints scope groups and Open Platform grant URLs. The operator
playbook covers environment, scopes, receiver IDs, smoke tests, document-writing
workflow, browser bridge, and troubleshooting.

## Messages

```bash
feishu-bot message send --to "$FEISHU_USER_ID" --text "hello from Feishu Bot"
feishu-bot message loop-check --to "$FEISHU_USER_ID" --to-type open-id
feishu-bot message send-json --to "$FEISHU_USER_ID" --msg-type text --content-json '{"text":"native JSON message"}'
feishu-bot message upload-image --file ./image.png
feishu-bot message send-image --to "$FEISHU_USER_ID" --file ./image.png
feishu-bot message upload-file --file ./demo.mp4 --file-type mp4 --duration 3000
feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./demo.mp4 --file-type mp4
feishu-bot message send-file --to "$FEISHU_USER_ID" --file ./voice.opus --file-type opus --duration 3000
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
```

Use `message upload-image` when the AI needs an `image_key`; Feishu supports
JPG/JPEG/PNG/WEBP/GIF/BMP/ICO/TIFF/HEIC up to 10 MB. Use
`message upload-file` for files, audio, and videos up to 30 MB; `--file-type
mp4` is the native video path and `stream` is the fallback for other file
types. `send-image` and `send-file` do the upload and message send in one step.
For `send-file`, `--msg-type auto` sends `mp4` as native `media`, `opus` as
native `audio`, and all other file types as `file`; use `--cover-image-key` for
video cover images when the AI already has an uploaded image key.

Use `message loop-check` for dogfood: it sends one message and immediately reads
back the message, recent chat messages, chat metadata, chat members, and
read-users. Use `message send-json`, `reply-json`, `edit-json`, and `delete` for
native Feishu message payloads. Use `message list/get/resource/read-users` for
message inspection and downloads, and `message reaction/pin` for message
interactions.

Receiver ID type is inferred:

- `ou_...` -> `open_id`
- `oc_...` -> `chat_id`
- `on_...` -> `union_id`
- `name@example.com` -> `email`

## Chats

```bash
feishu-bot chat list
feishu-bot chat search --query "项目"
feishu-bot chat get --chat-id <chat_id>
feishu-bot chat create --name "AI 项目群" --user "$FEISHU_USER_ID"
feishu-bot chat member list --chat-id <chat_id>
feishu-bot chat member add --chat-id <chat_id> --id "$FEISHU_USER_ID"
feishu-bot chat member is-in-chat --chat-id <chat_id>
feishu-bot chat member delete --chat-id <chat_id> --id <open_id>
```

## Directory

Use `feishu-bot directory` for the newer organization directory employee APIs when
contact lookup is too shallow but full CoreHR is not needed.

```bash
feishu-bot directory employee search --query "张三" --page-size 10
feishu-bot directory employee search --query user@example.com --field base_info.employee_id --field base_info.email
feishu-bot directory employee mget --employee-id <open_id> --field base_info.name --field work_info.job_title
feishu-bot directory employee filter --condition 'base_info.email=eq="user@example.com"'
feishu-bot directory employee filter --condition 'work_info.job_number=eq="E12345"' --field base_info.name
```

Directory supports tenant and user tokens with `--auth tenant|user`.
`employee_id_type` defaults to `open_id`; use `--employee-id-type
employee-id|union-id` when the upstream IDs differ. Each returned field needs a
matching `directory:*` field scope, so run `feishu-bot scopes --group directory`
when a call returns `99991672`.

## Notifications

```bash
feishu-bot notify \
  --project feishu-bot \
  --status done \
  --task "Bot dogfood" \
  --summary "Rust CLI sent this card" \
  --details "token ok|message ok|doc API ready" \
  --text "This card was sent by the new Rust CLI."
```

Without `--to`, `notify` creates or reuses a project chat and stores the mapping
in `~/.config/feishu/projects.json`. It also reads old mappings from
`~/.config/feishu/projects.json`.

## Documents

Create a new docx document and append content. There are two writers:

- `--writer local`: no conversion API; maps a small Markdown-ish syntax to native docx blocks.
- `--writer official`: calls Feishu's official Markdown/HTML -> docx block converter. Use this first for richer Markdown/HTML such as tables, inline styles, and links.

The local writer supports headings, paragraphs, unordered lists, ordered lists,
code fences, quotes, todos, and dividers.

Preview the generated native blocks before writing a larger document:

```bash
feishu-bot doc preview --file ./guide.md
feishu-bot --json doc preview --file ./guide.md
```

```bash
feishu-bot doc create \
  --title "Feishu Bot dogfood" \
  --content $'# Hello\n\n- created from CLI\n- written through docx block API'
```

Use official conversion:

```bash
feishu-bot doc convert --file ./guide.md
feishu-bot dogfood publish --title "Guide dogfood" --file ./guide.md
feishu-bot doc create --writer official --title "Guide" --file ./guide.md
feishu-bot doc create --writer official --title "Guide" --file ./guide.md --send-to "$FEISHU_USER_ID" --send-loop-check
feishu-bot doc create --writer official --content-type html --title "HTML Guide" --file ./guide.html
feishu-bot doc create --writer official --title "Dogfood" --file ./demo.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok
FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --writer official --title "Dogfood" --file ./demo.md
FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --writer official --title "Strict Wiki" --file ./demo.md --wiki-strict
feishu-bot drive import file --file ./guide.html --type docx --folder-token "" --title "HTML Guide"
```

HTML online preview is not exposed as “host this arbitrary HTML page” in the
public Drive/docx OpenAPI. The AI-safe solution is to convert or import HTML
into a native Feishu `docx`: use `doc create --writer official --content-type
html` for HTML strings/files, or `drive import file --type docx` when the AI has
a real `.html` file and wants Feishu's import pipeline to return a previewable
online document URL.

Append to an existing document:

```bash
feishu-bot doc append --document-id doxcn... --file ./notes.md
```

Read plain text:

```bash
feishu-bot doc raw --document-id doxcn...
```

Inspect native docx block types:

```bash
feishu-bot doc blocks --document-id doxcn...
```

Print the AI writing boundary, including Mermaid and advanced block subtypes:

```bash
feishu-bot doc capabilities
feishu-bot doc template --kind all
feishu-bot doc template --kind support-matrix
```

Advanced AI raw block writing:

```bash
feishu-bot doc insert-media --document-id doxcn... --kind image --file ./image.png --width 640 --align 2
feishu-bot doc insert-media --document-id doxcn... --kind file --file ./attachment.pdf --view-type 1
feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png
feishu-bot drive media upload --parent-type docx_file --parent-node <file_block_id> --drive-route-token <document_id> --file ./attachment.pdf
feishu-bot doc template --kind image-child > image.json
feishu-bot doc template --kind file-child > file.json
feishu-bot doc template --kind iframe-child > iframe.json
feishu-bot doc template --kind link-preview-child > link.json
feishu-bot doc template --kind table-descendant > table.json
feishu-bot doc template --kind grid-descendant > grid.json
feishu-bot doc template --kind callout-descendant > callout.json
feishu-bot doc template --kind quote-container-descendant > quote-container.json
feishu-bot doc template --kind agenda-descendant > agenda.json
feishu-bot doc append-json --document-id doxcn... --file ./children.json
feishu-bot doc append-descendant --document-id doxcn... --file ./descendant-body.json
```

These commands are the escape hatch for rarer/newer Feishu blocks that are not
modeled by the local writer yet, such as table, table_cell, grid, grid_column,
iframe, image, file, sheet, bitable, callout, quote_container, add_ons, board,
agenda, link_preview, wiki catalog blocks, or custom app blocks, as long as the
JSON follows Feishu's docx block API and includes any required subtype fields.
For normal docx image/file blocks, prefer `doc insert-media`: it creates the
target media block, uploads the local asset with `drive media upload`, then
patches the block with `replace_image` or `replace_file`. Use the lower-level
`drive media upload` plus `feishu-bot api` route only for unusual block updates not
covered by `insert-media`. For example, iframe uses `iframe.component.type/url`,
tables use `table.property`, and callouts use `callout.background_color`.

Mermaid is preserved as source in a code block. Feishu's public docx OpenAPI
does not expose a Mermaid source field for a rendered plugin block; `diagram`
only exposes `diagram_type` for flowchart/UML and is not writable through the
public docx API. For rendered Mermaid, create or locate a document board block
and import the Mermaid into the whiteboard:

```bash
feishu-bot doc template --kind board-child > board.json
feishu-bot doc append-json --document-id doxcn... --file board.json
feishu-bot doc blocks --document-id doxcn...
feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file ./diagram.mmd
```

Mindnote is also not currently writable through public docx OpenAPI. Keep it as
an existing/read block or use Feishu UI until Feishu exposes a write shape.
Task blocks, source/reference synced blocks, and AI template blocks are also
read-only from public docx write APIs; create tasks through `feishu-bot task`.

Send a document link:

```bash
feishu-bot doc send-link --document-id doxcn... --to "$FEISHU_USER_ID" --send-loop-check
```

Set `FEISHU_DOC_BASE_URL` if your tenant document URL is not
`https://my.feishu.cn/docx/{document_id}`. It may either be a prefix or a
template containing `{document_id}`.

## Board / Mermaid

```bash
feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file ./diagram.mmd
feishu-bot board import --whiteboard-id <whiteboard_id> --syntax plantuml --file ./diagram.puml
feishu-bot board node-create --whiteboard-id <whiteboard_id> --file ./nodes.json
```

`whiteboard_id` is the `board.token` from a docx block with `block_type=43`.
Use `feishu-bot doc blocks --document-id <id>` to inspect it after creating or
locating a board block; the normal output prints `board_token[block_id]=...`
when a board block exists.

## Base / Bitable

```bash
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
feishu-bot base view create --app-token <app_token> --table-id <table_id> --name "AI 看板" --view-type kanban
feishu-bot base view update --app-token <app_token> --table-id <table_id> --view-id <view_id> --hidden-field-id fld_internal --filter-conjunction and --filter-condition 'fld_status:3:is:json:["opt_done"]' --hierarchy-field-id fld_parent
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 标题=hello --field 分数=12.5 --field 完成=true
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field '附件=json:[{"file_token":"<file_token>"}]'
feishu-bot base record create --app-token <app_token> --table-id <table_id> --fields-json '{"标题":"hello"}'
feishu-bot base record search --app-token <app_token> --table-id <table_id> --view-id <view_id> --field-name "标题" --automatic-fields
feishu-bot base record search --app-token <app_token> --table-id <table_id> --filter-json '{"conjunction":"and","conditions":[]}' --sort-json '[]'
feishu-bot base record search --app-token <app_token> --table-id <table_id> --body-json '{}'
feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --records-json '[{"record_id":"rec...","fields":{"状态":"done"}}]'
feishu-bot base record batch-create --app-token <app_token> --table-id <table_id> --record-field 0:标题=A --record-field 0:状态=open --record-field 1:标题=B
feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --record-id rec_a --record-id rec_b --record-field 0:状态=done --record-field 1:清空=null
feishu-bot base record batch-delete --app-token <app_token> --table-id <table_id> --record-id rec_xxx
feishu-bot base media upload --app-token <app_token> --kind image --file ./screenshot.png
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
```

For existing Bases, grant both Open Platform scopes and document/Base access to
the app itself. Advanced permission role/member commands also require advanced
permissions enabled and manageable permission on the Base.
For advanced permissions 2.0 custom roles, prefer `base role list/create
--api-version v2`. The v2 path uses `/base/v2/apps/:app_token/roles` and
supports `base_rule` permission points: `--allow-base-complex-edit false`
controls Base copy/download/print, and `--allow-copy false` controls content
copy. Use `--base-rule-json` for future v2 permission points.
For `base record create/update --field`, values are JSON-parsed when possible:
`12.5`, `true`, and `null` become native Base values. Use `json:[...]` for
arrays/objects such as attachments, and `str:true` when the desired value is a
literal string.
For batch create/update, use `--record-field index:name=value` to group fields
by record index. Batch update also needs repeated `--record-id` values in the
same index order, unless each raw records JSON entry already has `record_id`.
When the user pastes a Base link, run `feishu-bot base parse-url` first. It accepts
both `/base/<app_token>` and `/app/<app_token>` Base application URLs. If it is a
Wiki URL, use the returned `wiki_node_token` with `feishu-bot wiki node`; when
`obj_type` is `bitable`, the returned `obj_token` is the Base `app_token`.
Use `base field list --view-id` after parsing a Base URL so field metadata
matches the user's current view. Add `--text-field-as-array` when an AI needs
structured field descriptions instead of plain description strings.
Use `base view update` typed flags for common view property edits: hide fields
with `--hidden-field-id`, configure filters with `--filter-conjunction` and
`--filter-condition field_id:field_type:operator:value`, and set hierarchy or
sub-record structure with `--hierarchy-field-id`. Prefix a condition value with
`json:` when Feishu expects the filter value to be a compact JSON string.
For fields, prefer `--kind` instead of numeric `--type`. `--kind` covers common
types: text, barcode, email, number, progress, currency, rating, single-select,
multi-select, date, checkbox, user, phone, url, attachment, link, formula,
duplex-link, location, group, and auto-number. `--property-json` remains the
native escape hatch.
When creating a new table, prefer repeated `--field "name:kind[:config]"` so
the AI can create common fields in one command. Config supports select options
split by `|`, currency `formatter|CURRENCY`, date formatters, formulas, linked
table IDs, user/group `multiple=true`, location input type, and `json:{...}` as
the raw `field.property` escape hatch. `--fields-json` remains available for
full official table.fields payloads.
For record reads, prefer `base record search` with typed flags: `--view-id`,
repeated `--field-name`, `--filter-json`, `--sort-json`, and
`--automatic-fields`. Feishu ignores `view_id` when `filter` or `sort` is set,
because that becomes a full-table conditional query. Use `--body-json` only
when the AI needs the exact native request body.
Base image/file/video fields are two-step: upload with `base media upload`,
then write the returned `file_token` into an attachment field with `base record
create/update`. `base media field-value` prints the exact field JSON. For Base
advanced permissions, pass table/field/record IDs to `base media tmp-url` or
`base media download` so the tool builds the official `bitablePerm` extra.

## Tasks

```bash
feishu-bot task tasklist create --name "AI 项目清单"
feishu-bot task tasklist list
feishu-bot task tasklist tasks --tasklist-guid <tasklist_guid>
feishu-bot task list --completed false --type my_tasks
feishu-bot task tasklist add-member --tasklist-guid <tasklist_guid> --editor "$FEISHU_USER_ID"
feishu-bot task tasklist remove-member --tasklist-guid <tasklist_guid> --viewer "$FEISHU_USER_ID"
feishu-bot task create --summary "follow up" --description "created by feishu-bot"
feishu-bot task create --summary "review proposal" --due-at 2026-06-02T15:00:00+08:00 --start-date 2026-06-02
feishu-bot task create --summary "submit proposal" --due-at "2026-06-03 18:00" --reminder-minute 30
feishu-bot task create --summary "all-day milestone" --due-date 2026-06-05 --mode 1 --is-milestone true
feishu-bot task create --summary "weekly sync" --due-ms 1780000000000 --due-all-day --repeat-rule "FREQ=WEEKLY;INTERVAL=1"
feishu-bot task create --summary "external ticket" --origin-json '{"platform_i18n_name":{"en_us":"AI System"},"href":{"url":"https://example.com/t/1"}}' --custom-complete-json '{"pc":{"tip":{"en_us":"Finish in source system"}}}' --extra eyJzb3VyY2UiOiJhaSJ9
feishu-bot task create --summary "milestone" --due-date 2026-06-30 --mode 1 --is-milestone true --reminder-minute 30
feishu-bot task get --guid <task_guid>
feishu-bot task update --guid <task_guid> --due-at 2026-06-03T18:00:00+08:00 --mode 1 --is-milestone true
feishu-bot task update --guid <task_guid> --clear-start --clear-repeat-rule --clear-custom-complete
feishu-bot task member add --task-guid <task_guid> --assignee "$FEISHU_USER_ID"
feishu-bot task member remove --task-guid <task_guid> --follower "$FEISHU_USER_ID"
feishu-bot task tasklists --task-guid <task_guid>
feishu-bot task add-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid> --section-guid <section_guid>
feishu-bot task remove-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid>
feishu-bot task section list --resource-type tasklist --resource-id <tasklist_guid>
feishu-bot task section create --resource-type tasklist --resource-id <tasklist_guid> --name "In progress"
feishu-bot task section tasks --section-guid <section_guid>
feishu-bot task custom-field list --resource-type tasklist --resource-id <tasklist_guid>
feishu-bot task custom-field create --resource-id <tasklist_guid> --name "Priority" --type single_select --option High --option Medium --option Low
feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type single-select --option-guid <option_guid>
feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type text --value "reviewed"
feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type member --member "$FEISHU_USER_ID"
feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type multi-select --option-guid <option_a> --option-guid <option_b>
feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type text --clear
feishu-bot task custom-field option update --custom-field-guid <field_guid> --option-guid <option_guid> --is-hidden true
feishu-bot task attachment list --resource-id <task_guid>
feishu-bot task attachment upload --resource-id <task_guid> --file ./image.png --file ./brief.pdf
feishu-bot task attachment delete --attachment-guid <attachment_guid>
feishu-bot task reminder add --task-guid <task_guid> --reminder-minute 30
feishu-bot task reminder remove --task-guid <task_guid> --reminder-id <reminder_id>
feishu-bot task dependency add --task-guid <task_guid> --dependency-task-guid <next_task_guid>
feishu-bot task dependency remove --task-guid <task_guid> --dependency-task-guid <next_task_guid>
feishu-bot task comment create --task-guid <task_guid> --content "progress note"
feishu-bot task comment list --task-guid <task_guid>
feishu-bot task comment get --comment-id <comment_id>
feishu-bot task comment update --comment-id <comment_id> --content "updated note"
feishu-bot task comment delete --comment-id <comment_id>
feishu-bot task complete --guid <task_guid>
```

Complex task, tasklist, section, dependency, custom-field, and comment payloads
can be passed as Feishu native JSON with `--body-json`. Custom field values are
typed through `task custom-field set-value`; use `--clear` for the official
empty value.
For task times, prefer `--due-at`/`--start-at` with RFC3339 values such as
`2026-06-02T15:00:00+08:00`, or local values such as `2026-06-02 15:00`.
Use `--due-date`/`--start-date` for all-day dates. `--due-ms` and `--start-ms`
remain available when the AI already has Feishu millisecond timestamps.
Task reminders are relative to the due time. Use `--reminder-minute` when
creating a task/subtask or adding a reminder to an existing task. Feishu
currently supports one reminder per task, so change a reminder by removing the
old one and adding the new one.
`feishu-bot task list` defaults to `--auth user` because Feishu's official task-list
API requires `user_access_token` and returns the caller's "my tasks" view. Use
`--completed true|false` to filter that view; `--type` defaults to `my_tasks`.
Core
task/tasklist/member/reminder/subtask commands plus
section/custom-field/attachment/dependency/comment wrappers support
`--auth tenant|user`. Use tenant auth for app-owned task data and user auth when
the AI must match a Feishu user's Task Center visibility. Task attachment upload
accepts one to five local files per request, each up to 50 MB.

## Drive

```bash
feishu-bot drive list --folder-token <folder_token>
feishu-bot drive folder create --name "AI 输出" --folder-token ""
feishu-bot drive upload --folder-token <folder_token> --file ./report.pdf
feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png
feishu-bot drive media upload --parent-type bitable_file --parent-node <app_token> --drive-route-token <app_token> --file ./video.mp4
feishu-bot drive media tmp-url --file-token <media_token>
feishu-bot drive media download --file-token <media_token> --output ./asset.bin
feishu-bot drive import file --file ./page.html --type docx --folder-token "" --title "HTML Preview"
feishu-bot drive import get --ticket <ticket>
feishu-bot drive export file --token <docx_token> --type docx --file-extension pdf --output ./doc.pdf
feishu-bot drive export create --token <sheet_token> --type sheet --file-extension xlsx
feishu-bot drive upload-large --file ./large-video.mp4 --folder-token <folder_token>
feishu-bot drive download --file-token <file_token> --output ./report.pdf
feishu-bot drive permission public-get --token <docx_token> --file-type docx
feishu-bot drive permission public-update --token <docx_token> --file-type docx --link-share-entity tenant_readable
feishu-bot drive permission member-list --token <docx_token> --file-type docx
feishu-bot drive permission member-add --token <docx_token> --file-type docx --member-id "$FEISHU_USER_ID" --perm edit
feishu-bot drive stats --file-token <token> --file-type docx
feishu-bot drive copy --file-token <token> --file-type docx --folder-token <folder_token>
```

`drive upload` uses Feishu `drive/v1/files/upload_all`, so it is for Drive
files. `drive media upload` uses `drive/v1/medias/upload_all`, so it is for
images/files/videos attached to docx, Sheets, Base, or import staging. Both
single-call upload flows are for non-empty files up to 20 MB. Use
`feishu-bot drive upload-large` for larger Drive files; it wraps Feishu's official
`upload_prepare` / `upload_part` / `upload_finish` multipart flow.

Use `drive import file` to turn local HTML/Markdown/TXT/Office files into native
online Feishu docs/sheets/Base files. For HTML preview, prefer importing or
converting to `docx`; raw uploaded `.html` files are just Drive files and should
not be treated as a guaranteed rendered web page.

Use `drive export file` to create, poll, and download a cloud document export in
one command. Feishu exports doc/docx to PDF or Word, and Sheets/Base to XLSX or
CSV; export files must be downloaded promptly after the task finishes.

`drive permission` wraps the Feishu cloud document permission APIs. Use it after
creating a docx/Base/sheet/file when the AI needs to share access with a user,
chat, or other collaborator. Use `permission member-list` as the readback step
after add/update/delete before claiming a recipient can access the artifact.

## Calendar

```bash
feishu-bot calendar primary
feishu-bot calendar list
feishu-bot calendar freebusy list --user-id "$FEISHU_USER_ID" --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
feishu-bot calendar freebusy batch --user-id ou_xxx --user-id ou_yyy --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
feishu-bot calendar event create --calendar-id <calendar_id> --summary "sync" --start-ts <unix_seconds> --end-ts <unix_seconds>
feishu-bot calendar attendee add --calendar-id <calendar_id> --event-id <event_id> --user "$FEISHU_USER_ID"
feishu-bot calendar attendee list --calendar-id <calendar_id> --event-id <event_id>
feishu-bot calendar attendee delete --calendar-id <calendar_id> --event-id <event_id> --attendee-id <attendee_id>
```

Use `--body-json` for rooms, recurrence, reminders, conferencing, and complex
attendee/resource payloads.

## Video Meetings

```bash
feishu-bot vc reserve apply --end-time <unix_seconds> --owner-id <open_id> --topic "AI sync"
feishu-bot vc reserve get --reserve-id <reserve_id>
feishu-bot vc reserve active-meeting --reserve-id <reserve_id> --with-participants
feishu-bot vc reserve update --reserve-id <reserve_id> --end-time <unix_seconds>
feishu-bot vc reserve delete --reserve-id <reserve_id>
feishu-bot vc meeting get --meeting-id <meeting_id>
feishu-bot vc meeting list-by-no --meeting-no 123456789 --start-time <unix_seconds> --end-time <unix_seconds>
feishu-bot vc meeting invite --meeting-id <meeting_id> --user <open_id>
feishu-bot vc meeting set-host --meeting-id <meeting_id> --user-id <open_id>
feishu-bot vc meeting end --meeting-id <meeting_id>
feishu-bot vc recording get --meeting-id <meeting_id>
feishu-bot vc recording start --meeting-id <meeting_id> --timezone 8
feishu-bot vc recording stop --meeting-id <meeting_id>
feishu-bot vc recording set-permission --meeting-id <meeting_id> --user <open_id>
feishu-bot vc report daily --start-time <unix_seconds> --end-time <unix_seconds>
feishu-bot vc report top-user --start-time <unix_seconds> --end-time <unix_seconds> --limit 10 --order-by 1
feishu-bot vc room list --page-size 20
feishu-bot vc room get --room-id <room_id>
feishu-bot vc room mget --room-id <room_id>
feishu-bot vc room-level list --page-size 20
```

Reserve APIs can use tenant or user auth; tenant-token reserve creation needs
`--owner-id`. Invite/end/recording start/stop/permission usually require
`FEISHU_USER_ACCESS_TOKEN` and host or participant permission. Setting a host can
require both `vc:meeting` and `vc:meeting.participant:write`.

## Minutes

```bash
feishu-bot minutes search --query "周会" --page-size 20
feishu-bot minutes get --minute-token <minute_token_or_url>
feishu-bot minutes artifacts --minute-token <minute_token_or_url>
feishu-bot minutes media --minute-token <minute_token_or_url>
feishu-bot minutes transcript --minute-token <minute_token_or_url> --need-speaker --need-timestamp --file-format txt --output ./minute.txt
```

`minute_token` can be passed directly or copied as a full Feishu/Lark Minutes
URL. `search` uses Feishu's user-token API, so set `FEISHU_USER_ACCESS_TOKEN`.
Metadata, AI artifacts, media download URL, and transcript export use the
normal app token path when the app has the required Minutes scopes and data
access.

## Search

```bash
feishu-bot search docs --query "飞书Bot" --page-size 10
feishu-bot search docs --query "方案" --doc-type DOCX --doc-type BITABLE --only-title
feishu-bot search message --query "上线" --chat-id oc_xxx --page-size 20
feishu-bot search source list --page-size 20
feishu-bot search schema create --file ./schema.json
feishu-bot search source create --name "AI 索引" --schema-id ai_schema --state 0
feishu-bot search item create --data-source-id <id> --id item_1 --title "标题" --url "https://example.com" --text "全文"
```

`docs` and `message` search visible content for the current user and therefore
need `FEISHU_USER_ACCESS_TOKEN`. `source/schema/item` manage Feishu Open Search
connector indexes with `tenant_access_token`; these are useful when an AI needs
to push external project data into Feishu's unified search.

## OKR

```bash
feishu-bot okr period list --page-size 20
feishu-bot okr period-rule list
feishu-bot okr user-okrs --user-id "$FEISHU_USER_ID" --offset 0 --limit 5
feishu-bot okr user-okrs --user-id "$FEISHU_USER_ID" --period-id <period_id> --offset 0 --limit 5
feishu-bot okr batch-get --okr-id <okr_id> --lang zh_cn
```

OKR commands use `tenant_access_token` and need scopes such as
`okr:okr.period:readonly`, `okr:okr:readonly`, or `okr:okr`. `period-rule` and
user OKR reads may also depend on the tenant's OKR edition and data visibility.

## Attendance

```bash
feishu-bot attendance group list --page-size 20
feishu-bot attendance group get --group-id <group_id>
feishu-bot attendance shift list --page-size 20
feishu-bot attendance shift query --shift-name "早班"
feishu-bot attendance schedule query --user-id <employee_id> --from 20260501 --to 20260531
feishu-bot attendance task query --user-id <employee_id> --from 20260501 --to 20260531 --ignore-invalid-users
feishu-bot attendance flow query --user-id <employee_id> --from-ts 1760000000 --to-ts 1760086400
feishu-bot attendance stats query --user-id <employee_id> --operator-user-id <employee_id> --from 20260501 --to 20260531
```

Attendance commands use `tenant_access_token`. Group and shift reads need
`attendance:rule` or `attendance:rule:readonly`; schedules, task results, flow
records, and stats need `attendance:task` or `attendance:task:readonly`.
Employee IDs default to `employee_id`; use `--employee-type employee-no` for
work numbers. `flow import` and `flow delete` accept raw Feishu JSON for explicit
write operations; `flow delete` accepts at most 10 record IDs.

## Mail

```bash
feishu-bot mail message list --mailbox me --page-size 10
feishu-bot mail message get --mailbox me --message-id <message_id> --format metadata
feishu-bot mail folder list --mailbox me
feishu-bot mail settings send-as --mailbox me
feishu-bot mail settings accessible --mailbox me
feishu-bot mail contact list --mailbox me --page-size 20
feishu-bot mail message send --mailbox me --to user@example.com --subject "hello" --text "body"
```

`--mailbox me` uses `FEISHU_USER_ACCESS_TOKEN`. Explicit mailbox addresses can
use tenant-token reads with `--auth tenant`, but the app must be granted Mail
data resource permissions. Full mail bodies, subjects, addresses, and contact
fields require separate Mail field scopes.

## CoreHR

```bash
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
```

CoreHR commands use `tenant_access_token`. They require both Open Platform
CoreHR scopes and Feishu People data-range grants. For complex filters, pass the
official request body with `--body-json`, `--file`, or `--stdin`.

## Helpdesk

```bash
feishu-bot helpdesk ticket list --page-size 20
feishu-bot helpdesk ticket get --ticket-id <ticket_id>
feishu-bot helpdesk ticket messages --ticket-id <ticket_id> --page-size 20
feishu-bot helpdesk service start --open-id <open_id> --human-service
feishu-bot helpdesk message send --receiver-id <open_id> --text "hello"
feishu-bot helpdesk faq categories --lang zh_cn
feishu-bot helpdesk faq list --search "登录" --page-size 20
```

Helpdesk commands use `tenant_access_token` plus the service-desk credential
header `X-Lark-Helpdesk-Authorization`. Set `FEISHU_HELPDESK_ID` and
`FEISHU_HELPDESK_TOKEN` from Helpdesk admin API credentials. Reads need
`helpdesk:all:readonly`, service start needs `helpdesk:helpdesk:access`, and bot
pushes need `helpdesk:all`. For future official payloads, pass native JSON with
`--body-json`, `--file`, or `--stdin`.

## Hire

```bash
feishu-bot hire job list --page-size 20
feishu-bot hire job detail --job-id <job_id>
feishu-bot hire job schemas --scenario 1
feishu-bot hire process list --page-size 50
feishu-bot hire talent list --keyword "张三" --page-size 10
feishu-bot hire talent get --talent-id <talent_id>
feishu-bot hire application list --talent-id <talent_id> --page-size 20
feishu-bot hire application detail --application-id <application_id> --option with_job --option with_talent
feishu-bot hire interview by-talent --talent-id <talent_id>
feishu-bot hire requirement schemas --page-size 20
feishu-bot hire metadata resume-sources --page-size 20
feishu-bot hire location query --location-type 1 --code CN_1
```

Hire commands use `tenant_access_token` and Feishu Hire data ranges. Start with
read-only commands to discover real jobs, talents, applications, processes, and
schema IDs. Writes are explicit: `talent create` and `job open` require typed
flags or official JSON via `--body-json`, `--file`, or `--stdin`.

## Wiki

```bash
feishu-bot wiki route-check
feishu-bot wiki route-check --write-probe
feishu-bot wiki route-check --write-probe --strict
feishu-bot wiki create-space --name "AI 知识库"
feishu-bot wiki spaces
feishu-bot wiki space --space-id <space_id>
feishu-bot wiki nodes --space-id <space_id>
feishu-bot doc create --writer official --title "AI 演示" --file ./demo.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok
FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --writer official --title "AI 演示" --file ./demo.md
feishu-bot wiki create-node --space-id <space_id> --title "AI 演示" --obj-type docx
feishu-bot wiki move-docs-to-wiki --space-id <space_id> --obj-type docx --obj-token <document_id>
feishu-bot wiki move-node --space-id <space_id> --node-token <node_token> --target-parent-token <parent_node_token>
feishu-bot wiki copy-node --space-id <space_id> --node-token <node_token> --target-space-id <space_id> --title "副本"
feishu-bot wiki update-title --space-id <space_id> --node-token <node_token> --title "新标题"
feishu-bot wiki node --token <node_token>
feishu-bot wiki task --task-id <task_id>
feishu-bot wiki member list --space-id <space_id>
feishu-bot wiki setting update --space-id <space_id> --create-setting admin_and_member
feishu-bot wiki search --query "关键字"
```

`create-space` and `search` use `FEISHU_USER_ACCESS_TOKEN` because Feishu's
official APIs require user token there. Tenant-token Wiki calls only operate on
spaces where the app or bot is already a member/admin.
Set `FEISHU_DOC_CREATE_WIKI_DEFAULT=true`, `FEISHU_WIKI_SPACE_ID`, and optional
`FEISHU_WIKI_PARENT_NODE_TOKEN` when AI dogfood documents should go to Wiki by
default via `feishu-bot doc create`; use `--no-wiki` for an explicit one-off draft.
Default Wiki publishing keeps the fallback docx and returns `wiki_move_error`
when the move is blocked; pass `--wiki-strict` if the command should fail on a
Wiki move error. Explicit one-off `--wiki` commands without the default route
can pass `--wiki-fallback-ok` for the same fallback behavior.
Run `feishu-bot wiki route-check --json` before claiming the default route is ready;
it verifies the configured space plus read access for spaces, the target space,
and child nodes without creating data.
Run `feishu-bot wiki route-check --write-probe --json` before claiming future AI
reports can all go through Wiki; it creates a small proof docx and attempts the
same `move_docs_to_wiki` publish path used by `doc create`.
Use `--strict` in automation when the run should exit non-zero unless
`route_ready` is true.
Only count Wiki publishing as successful when the command returns `wiki_move` or
`feishu-bot wiki node --token <wiki_node_token>` can read the moved node. A created
docx URL without `wiki_move` is only the fallback document.
For automated dogfood using explicit `--wiki`, add `--wiki-fallback-ok` so the
command still returns and sends the fallback docx while Wiki permissions are
being granted.

## Sheets

Create a spreadsheet from zero, then inspect sheets and write values:

```bash
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
feishu-bot sheet style --spreadsheet-token <token> --range Sheet1!D2:D20 --formatter "0.00%" --h-align 1
```

Use `merge`, `unmerge`, and `style` when the AI needs to produce readable
spreadsheet reports rather than raw cell values. `style` wraps Feishu
`styles_batch_update` with typed flags for font, color, alignment, formatter,
border, and clean operations while still accepting raw JSON for new fields.

## Approval

```bash
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
```

Approval forms are schema-specific. Use `definition get` or OpenAPI Explorer to
copy official form JSON and task IDs before creating instances or operating
tasks.

## Raw OpenAPI

Use `feishu-bot api` for official endpoints that do not have typed wrappers yet. It
supports tenant/user token selection, JSON requests, binary downloads, and
multipart uploads:

```bash
feishu-bot api get --path /bitable/v1/apps/<app_token>
feishu-bot api get --auth user --path /search/v2/data_sources
feishu-bot api post --path /task/v2/tasks --body-json '{"summary":"raw task"}'
feishu-bot api download --path /drive/v1/files/<file_token>/download --output ./file.bin
feishu-bot api multipart --path /im/v1/images --field image_type=message --file image=./image.png
```

## Browser bridge

The browser commands call the local Playwright MCP extension bridge used by
Codex:

```bash
feishu-bot browser ensure
feishu-bot browser tabs
feishu-bot browser drive
```

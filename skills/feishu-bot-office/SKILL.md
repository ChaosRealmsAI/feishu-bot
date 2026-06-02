---
name: feishu-bot-office
description: Use when an AI agent needs to operate Feishu/Lark for office reporting: create or reuse project chats, send and verify messages, publish Docx/Wiki documents, write Markdown or HTML, send voice/images/files, maintain Wiki/Base project records, poll and acknowledge user replies, and prove every action by readback through feishu-bot. This skill assumes the repository's `feishu-bot` command is available and must never hard-code tenant-specific IDs, tokens, Wiki spaces, chats, or local paths.
---

# Feishu Bot Office

Use this skill to turn AI work into visible Feishu office artifacts. The standard
is closed-loop operation: create/send/write, then read back before reporting
success.

## Ground Rules

- Never hard-code personal IDs, chat IDs, Wiki spaces, document URLs, tokens, or
  local absolute paths.
- Use placeholders or environment variables: `FEISHU_USER_ID`,
  `FEISHU_WIKI_SPACE_ID`, `FEISHU_ENV_FILE`, `<chat_id>`, `<message_id>`,
  `<document_id>`, `<node_token>`, `<obj_token>`.
- Prefer `feishu-bot office` workflow commands for normal project work. Use atomic
  typed commands when the workflow layer is too coarse. Use `feishu-bot api` only
  when no typed command exists.
- Use `--json` when another AI or script needs to parse the result.
- Do not treat exit code 0 from `dogfood verify` as success; inspect probe
  statuses.

## Startup

```bash
feishu-bot --help
feishu-bot ai
feishu-bot manifest
feishu-bot doctor
feishu-bot setup plan
feishu-bot setup quickstart --open-browser
feishu-bot office --help
feishu-bot office list --json
feishu-bot dogfood verify --json
```

If credentials live outside the repo:

```bash
FEISHU_ENV_FILE=private/local.env feishu-bot doctor
```

For first-run permission setup:

```bash
feishu-bot setup plan --json
feishu-bot setup quickstart --open-browser --json
scripts/feishu-bot-setup.sh --project "<project_name>" --open-browser
feishu-bot setup open-scopes --group office --browser --json
feishu-bot setup wiki-bot --auth user --json
feishu-bot setup auto --open-browser --json
```

`setup` can open Feishu grant pages through the browser bridge and can add the
current app bot to the configured Wiki space. Human approval and OAuth redirect
code handoff are still explicit browser steps.
The setup shell script opens browser URLs only when `--open-browser` or
`FEISHU_BOT_SETUP_OPEN_BROWSER=1` is set; confirm the intended Chrome/Feishu
account before enabling it on multi-account machines.

## Project Workspace

Default to the high-level workflow:

```bash
feishu-bot office bootstrap --project "<project_name>" --dry-run --json
feishu-bot office bootstrap --project "<project_name>" --user "$FEISHU_USER_ID" --space-id "$FEISHU_WIKI_SPACE_ID" --send-summary --json
feishu-bot office status --project "<project_name>" --check --json
```

The workflow state defaults to `~/.config/feishu/office-projects.json`. Override
it with `FEISHU_OFFICE_STATE_FILE` when a task needs isolated local state.

Atomic fallback to create or locate a project chat:

```bash
feishu-bot chat create --name "<project_name>" --user "$FEISHU_USER_ID"
feishu-bot chat get --chat-id <chat_id>
feishu-bot chat member list --chat-id <chat_id>
```

Improve chat usability when relevant:

```bash
feishu-bot chat update --chat-id <chat_id> --name "<project_name>" --avatar-file ./avatar.png
feishu-bot chat tab add --chat-id <chat_id> --name "Project Wiki" --tab-type doc --doc <wiki_or_doc_url>
feishu-bot chat menu add --chat-id <chat_id> --body-file ./menu-tree.json
```

## Reporting

Default report flow:

```bash
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --dry-run --json
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --base-record --pin --json
feishu-bot office report --project "<project_name>" --title "<html_title>" --content-type html --file ./page.html --json
```

Use `progress` for normal lightweight status updates. It sends a chat message
and logs to the project Base by default; `--wiki-report` creates a detail doc:

```bash
feishu-bot office progress --project "<project_name>" --title "<title>" --status doing --summary "<short status>" --json
feishu-bot office progress --project "<project_name>" --title "<title>" --file ./summary.md --wiki-report --pin --json
```

Atomic fallback to send status text and prove it is visible:

```bash
feishu-bot message send --to <chat_id> --to-type chat-id --text "<status>"
feishu-bot message get --message-id <message_id>
feishu-bot message list --container-id <chat_id> --container-id-type chat --page-size 20
```

Send media:

```bash
feishu-bot message send-image --to <chat_id> --to-type chat-id --file ./image.png
feishu-bot message send-file --to <chat_id> --to-type chat-id --file ./report.pdf --file-type pdf
feishu-bot office voice-report --project "<project_name>" --text "<spoken update>" --json
feishu-bot message send-voice --to <chat_id> --to-type chat-id --text "<spoken update>" --readback
```

Pin important summaries:

```bash
feishu-bot message pin add --message-id <message_id>
feishu-bot message pin list --chat-id <chat_id>
```

## Wiki And Documents

Prefer Wiki for durable project records:

```bash
feishu-bot wiki route-check --json
feishu-bot wiki create-node --auth user --space-id <space_id> --title "<title>" --obj-type docx
feishu-bot doc append --auth user --document-id <obj_token> --writer official --file ./notes.md
feishu-bot doc append --auth user --document-id <obj_token> --writer official --content-type html --file ./page.html
feishu-bot wiki node --auth user --token <node_token>
feishu-bot doc raw --auth user --document-id <obj_token>
```

For standalone Docx:

```bash
feishu-bot doc create --writer official --title "<title>" --file ./notes.md
feishu-bot doc raw --document-id <document_id>
feishu-bot doc send-link --document-id <document_id> --to <chat_id> --to-type chat-id --send-loop-check
```

Mermaid in Docx should be preserved as source/code. Use Board when a rendered
diagram is required:

```bash
feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file ./diagram.mmd
```

## Project Records In Base

Create a Wiki-hosted Base and maintain project status:

```bash
feishu-bot wiki create-node --auth user --space-id <space_id> --title "<project> Records" --obj-type bitable
feishu-bot base table update --app-token <app_token> --table-id <table_id> --name "Project Records"
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 'Name=<item>' --field 'Status=Done'
feishu-bot base record get --app-token <app_token> --table-id <table_id> --record-id <record_id>
```

If tenant-token Base writes fail on a Wiki-created Base, add the app bot to the
Wiki space first:

```bash
feishu-bot bot info
feishu-bot wiki member add --auth user --space-id <space_id> --member-type openid --member-id <bot_open_id> --member-role admin
```

## User Replies

Establish a cursor, then process later messages:

```bash
feishu-bot office poll --project "<project_name>" --from-now --mark-seen --json
feishu-bot office poll --project "<project_name>" --ack-emoji OK --reply-text "Received, processing." --mark-seen --json
feishu-bot office inbox --project "<project_name>" --from-now --json
feishu-bot office inbox --project "<project_name>" --reply-text "Received, processing." --json
feishu-bot message poll --chat-id <chat_id> --from-now --mark-seen
feishu-bot message poll --chat-id <chat_id> --ack-emoji OK --reply-text "Received, processing." --mark-seen
```

Use reactions as workflow status markers, not as official read receipts.

## Completion Checklist

Before final reporting, confirm:

- The command returned the target Feishu object ID or URL.
- A readback command succeeded.
- The project chat contains the sent message or link.
- Any Wiki/Base record is readable through the CLI.
- Any blocker includes exact remediation from `dogfood verify` or the Feishu API
  error.

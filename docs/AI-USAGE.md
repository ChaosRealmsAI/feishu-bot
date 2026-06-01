# AI Usage Guide

This project is designed for AI agents operating Feishu/Lark through a local
CLI. The core rule is simple: do not claim a capability works until the tool has
made a real call and read back the result.

## First Commands

```bash
feishu-bot --help
feishu-bot ai
feishu-bot manifest
feishu-bot doctor
feishu-bot setup plan
feishu-bot setup quickstart --open-browser
feishu-bot office --help
feishu-bot dogfood verify
```

Use `--json` for machine parsing:

```bash
feishu-bot --json manifest
feishu-bot --json dogfood verify --module message --include-response
```

## Environment

The CLI reads configuration from:

1. process environment
2. project `.env`
3. current-directory `.env`
4. optional `FEISHU_ENV_FILE` or `LARK_ENV_FILE`

Use `.env.example` as the public template. Put real local credentials in an
ignored file, for example `private/local.env`, then run:

```bash
FEISHU_ENV_FILE=private/local.env feishu-bot doctor
```

Never print real `FEISHU_APP_SECRET`, tenant tokens, user access tokens, refresh
tokens, Helpdesk tokens, chat IDs, document URLs, or tenant-specific proof data
in public docs.

## Setup Automation

Use setup before office/dogfood on a new account:

```bash
feishu-bot setup plan --json
feishu-bot setup quickstart --open-browser --json
scripts/feishu-bot-setup.sh --project "AI Project"
feishu-bot setup open-scopes --group office --browser --json
feishu-bot setup wiki-bot --auth user --json
```

`setup` automates everything the tool can safely automate: env shape checks,
scope grant URL construction, Playwright MCP browser opening, bot identity
lookup, and Wiki-space bot membership. Human approval is still required inside
Feishu's authorization pages, and OAuth still needs the redirect `code` to be
passed to `feishu-bot oauth token --save-env`.

`setup quickstart` is the recommended first-run entry. It returns the exact
grant, OAuth, Wiki-bot, bootstrap, progress, inbox, and search commands for the
common one-human-plus-AI workflow.

## Capability Layers

Use two layers deliberately:

- Workflow layer: `feishu-bot office ...` and `feishu-bot dogfood ...`. Use this first
  for daily one-human-plus-AI work because it creates project isolation, writes
  independent report docs, sends concise chat updates, polls replies, and returns
  readback probes.
- Atomic layer: `feishu-bot message`, `chat`, `wiki`, `doc`, `base`, `task`,
  `drive`, `search`, and the other module commands. Use these for exact OpenAPI
  operations, troubleshooting, or unsupported workflow edges.

## AI Operating Loop

1. Run the relevant help command before using an unfamiliar module.
2. Start with read-only/local-only probes: `feishu-bot setup plan --json`,
   `feishu-bot office list --json`, `feishu-bot office bootstrap --dry-run`, and
   `feishu-bot office report --dry-run`.
3. Prefer `feishu-bot office ...` for normal project work.
4. Run `feishu-bot dogfood verify --module <module> --json` before claiming a new
   atomic capability works.
5. Inspect probe status. Exit code 0 does not mean every probe succeeded.
6. If blocked, follow the `remediation` fields in the JSON result.
7. For write paths, create a real object and read it back.
8. Send the result to the target chat or user.
9. Read back the message with `message get`, `message list`, or the readback
   fields returned by `office`.

## Common Office Workflows

Recommended high-level project setup:

```bash
feishu-bot office list --json
feishu-bot office bootstrap --project "AI Project" --dry-run --json
feishu-bot office bootstrap --project "AI Project" --user "$FEISHU_USER_ID" --space-id "$FEISHU_WIKI_SPACE_ID" --send-summary --json
feishu-bot office status --project "AI Project" --check --json
```

Write one independent report document and notify the project group:

```bash
feishu-bot office report --project "AI Project" --title "Capability Demo" --file ./demo.md --dry-run --json
feishu-bot office report --project "AI Project" --title "Capability Demo" --file ./demo.md --base-record --pin --json
feishu-bot office report --project "AI Project" --title "HTML Demo" --content-type html --file ./demo.html --json
```

Send lightweight progress, voice, and handle replies:

```bash
feishu-bot office progress --project "AI Project" --title "Progress" --status doing --summary "Current status" --json
feishu-bot office progress --project "AI Project" --title "Milestone" --file ./summary.md --wiki-report --pin --json
```

```bash
feishu-bot office voice-report --project "AI Project" --text "Project update is ready." --json
feishu-bot office inbox --project "AI Project" --from-now --json
feishu-bot office inbox --project "AI Project" --reply-text "Received, processing." --json
feishu-bot office poll --project "AI Project" --from-now --mark-seen --json
feishu-bot office poll --project "AI Project" --ack-emoji OK --reply-text "Received, processing." --mark-seen --json
```

Search project history:

```bash
feishu-bot office search --project "AI Project" --query "decision" --json
```

Atomic fallback: create or reuse a project chat:

```bash
feishu-bot chat create --name "AI Project" --user "$FEISHU_USER_ID"
feishu-bot chat get --chat-id <chat_id>
feishu-bot chat member list --chat-id <chat_id>
```

Send and verify a message:

```bash
feishu-bot message send --to <chat_id> --to-type chat-id --text "status update"
feishu-bot message get --message-id <message_id>
feishu-bot message list --container-id <chat_id> --container-id-type chat --page-size 20
```

Publish a document:

```bash
feishu-bot doc create --writer official --title "Project Update" --file ./demo.md
feishu-bot doc raw --document-id <document_id>
feishu-bot doc send-link --document-id <document_id> --to <chat_id> --to-type chat-id
```

Use Wiki as the long-term record:

```bash
feishu-bot wiki route-check
feishu-bot wiki create-node --auth user --space-id <space_id> --title "Project Log" --obj-type docx
feishu-bot doc append --auth user --document-id <obj_token> --writer official --file ./demo.md
feishu-bot wiki node --auth user --token <node_token>
```

Maintain project status in Base:

```bash
feishu-bot wiki create-node --auth user --space-id <space_id> --title "Project Base" --obj-type bitable
feishu-bot base table update --app-token <app_token> --table-id <table_id> --name "Project Records"
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 'Name=Kickoff' --field 'Status=Done'
feishu-bot base record get --app-token <app_token> --table-id <table_id> --record-id <record_id>
```

Send voice:

```bash
feishu-bot message send-voice --to <chat_id> --to-type chat-id --text "Project update is ready." --readback
feishu-bot message send-voice --to <chat_id> --to-type chat-id --file ./voice.mp3 --readback
```

Handle user replies:

```bash
feishu-bot message poll --chat-id <chat_id> --from-now --mark-seen
feishu-bot message poll --chat-id <chat_id> --ack-emoji OK --reply-text "Received, processing." --mark-seen
```

## Capability Boundaries

- Wiki creation/search and user-owned document writes require user access token
  scopes.
- Tenant-token Wiki writes require the app or bot to be a member/admin of the
  target space.
- Feishu Docx can preserve Mermaid as code; rendered Mermaid diagrams should use
  Board import.
- Client-side UI features, such as hiding personal conversations in the desktop
  client, may not have public OpenAPI support.

For the repeatable best-practice flow, use the repository skill at
`skills/feishu-bot-office/SKILL.md`.

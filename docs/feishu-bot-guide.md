# Feishu Bot Guide

`feishu-bot` installs the `feishu-bot` binary. It is an AI-oriented command-line tool for
operating Feishu/Lark messages, chats, Docx, Wiki, Base, Drive, tasks, calendar,
search, mail, approvals, and raw OpenAPI endpoints.

## Setup

Copy `.env.example` to a private ignored file and point the tool to it:

```bash
cp .env.example private/local.env
FEISHU_ENV_FILE=private/local.env feishu-bot doctor
```

Required variables:

```bash
FEISHU_APP_ID=cli_xxx
FEISHU_APP_SECRET=replace_me
```

Common optional variables:

```bash
FEISHU_USER_ID=ou_xxx
FEISHU_USER_ACCESS_TOKEN=u_xxx
FEISHU_REFRESH_TOKEN=r_xxx
FEISHU_WIKI_SPACE_ID=wiki_space_id
FEISHU_WIKI_PARENT_NODE_TOKEN=wiki_parent_node_token
FEISHU_DOC_CREATE_WIKI_DEFAULT=false
```

The CLI reads process environment, project `.env`, current-directory `.env`,
and optional `FEISHU_ENV_FILE` or `LARK_ENV_FILE`.

## AI Handoff

Start with:

```bash
feishu-bot --help
feishu-bot ai
feishu-bot manifest
feishu-bot doctor
feishu-bot setup quickstart --open-browser
feishu-bot dogfood verify
```

For any new workflow:

1. Run module help.
2. Run `dogfood verify`.
3. Make the real write/read call.
4. Send the result to Feishu.
5. Read back the message or object before reporting success.

## Core Workflows

Project chat:

```bash
feishu-bot chat create --name "AI Project" --user "$FEISHU_USER_ID"
feishu-bot chat member list --chat-id <chat_id>
feishu-bot chat tab add --chat-id <chat_id> --name "Project Wiki" --tab-type doc --doc <wiki_or_doc_url>
```

Messages:

```bash
feishu-bot message send --to <chat_id> --to-type chat-id --text "hello"
feishu-bot message send-image --to <chat_id> --to-type chat-id --file ./image.png
feishu-bot message send-file --to <chat_id> --to-type chat-id --file ./demo.pdf --file-type pdf
feishu-bot message send-voice --to <chat_id> --to-type chat-id --text "Voice update" --readback
feishu-bot message pin add --message-id <message_id>
feishu-bot message ack --message-id <message_id> --emoji-type OK --reply-text "Received" --readback
```

AI office wrapper:

```bash
feishu-bot setup quickstart --open-browser --json
scripts/feishu-bot-setup.sh --project "AI Project" --open-browser
feishu-bot office bootstrap --project "AI Project" --user "$FEISHU_USER_ID" --space-id "$FEISHU_WIKI_SPACE_ID" --send-summary --json
feishu-bot office progress --project "AI Project" --title "Progress" --status doing --summary "Current status" --json
feishu-bot office report --project "AI Project" --title "Demo" --file ./demo.md --base-record --pin --json
feishu-bot office inbox --project "AI Project" --reply-text "Received, processing." --json
```

Docs and Wiki:

```bash
feishu-bot doc create --writer official --title "Project Update" --file ./demo.md
feishu-bot doc append --document-id <document_id> --writer official --file ./more.md
feishu-bot doc raw --document-id <document_id>
feishu-bot wiki create-node --auth user --space-id <space_id> --title "Project Log" --obj-type docx
feishu-bot doc append --auth user --document-id <obj_token> --writer official --file ./demo.md
```

Base:

```bash
feishu-bot wiki create-node --auth user --space-id <space_id> --title "Project Records" --obj-type bitable
feishu-bot base table update --app-token <app_token> --table-id <table_id> --name "Records"
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 'Name=Kickoff' --field 'Status=Done'
feishu-bot base record search --app-token <app_token> --table-id <table_id> --page-size 20
```

Search and raw API:

```bash
feishu-bot search docs --query "project" --page-size 10
feishu-bot search message --query "project" --chat-id <chat_id> --page-size 20
feishu-bot api get --path /open-apis/bot/v3/info
```

## Maintenance

- Keep `src/app/help.rs` and `src/app/manifest.rs` aligned with new commands.
- Keep `.env.example` public and placeholder-only.
- Keep real dogfood outputs under ignored directories.
- Run `cargo fmt --check`, `cargo test --all-targets`, and help/manifest smoke
  commands before release.

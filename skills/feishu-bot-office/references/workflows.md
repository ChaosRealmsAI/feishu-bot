# Feishu AI Office Workflows

Load this reference only when a task needs concrete command templates.

## Closed-Loop Document Publish

Preferred workflow:

```bash
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --dry-run --json
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --base-record --pin --json
```

Atomic fallback:

```bash
feishu-bot doc create --writer official --title "<title>" --file ./demo.md --json
feishu-bot doc raw --document-id <document_id> --json
feishu-bot doc send-link --document-id <document_id> --to <chat_id> --to-type chat-id --send-loop-check --json
```

## Closed-Loop Wiki Publish

Preferred workflow:

```bash
feishu-bot office bootstrap --project "<project_name>" --dry-run --json
feishu-bot office bootstrap --project "<project_name>" --user "$FEISHU_USER_ID" --space-id <space_id> --send-summary --json
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --json
feishu-bot office status --project "<project_name>" --check --json
```

Atomic fallback:

```bash
feishu-bot wiki route-check --json
feishu-bot wiki create-node --auth user --space-id <space_id> --title "<title>" --obj-type docx --json
feishu-bot doc append --auth user --document-id <obj_token> --writer official --file ./demo.md --json
feishu-bot wiki node --auth user --token <node_token> --json
feishu-bot message send --to <chat_id> --to-type chat-id --text "<wiki_url>" --json
```

## Voice Update

```bash
feishu-bot office voice-report --project "<project_name>" --text "<voice text>" --json
feishu-bot message send-voice --to <chat_id> --to-type chat-id --text "<voice text>" --readback --json
```

## Project Base Record

```bash
feishu-bot office progress --project "<project_name>" --title "<title>" --status doing --summary "<short status>" --json
feishu-bot office progress --project "<project_name>" --title "<title>" --file ./summary.md --wiki-report --pin --json
feishu-bot office report --project "<project_name>" --title "<title>" --file ./demo.md --base-record --json
feishu-bot wiki create-node --auth user --space-id <space_id> --title "<project> Base" --obj-type bitable --json
feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 'Name=<name>' --field 'Status=Done' --json
feishu-bot base record get --app-token <app_token> --table-id <table_id> --record-id <record_id> --json
```

## Human Inbox

```bash
feishu-bot office inbox --project "<project_name>" --from-now --json
feishu-bot office inbox --project "<project_name>" --reply-text "Received, processing." --json
feishu-bot office poll --project "<project_name>" --ack-emoji OK --reply-text "Received, processing." --mark-seen --json
```

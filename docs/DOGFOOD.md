# Dogfood Publishing

Every new Feishu Bot capability must be demonstrated with a real Feishu call.
Do not claim a command works just because the wrapper exists.

## Required Loop

1. Run the relevant `feishu-bot ... --help`.
2. Run `feishu-bot dogfood verify --module <module> --json`.
3. Inspect `data.summary` and each probe `status`.
4. For write paths, create a real object and read it back.
5. Send the result to the configured user or chat.
6. Read back the exact message with `message get` or `message list`.
7. Record blockers as missing scopes, missing user token, or API errors.

`dogfood verify` may exit 0 even when probes are blocked. Automation must read
the JSON result instead of treating exit code 0 as success.

For normal project reporting, prefer the workflow layer after the relevant
module probes pass:

```bash
feishu-bot office bootstrap --project "<project>" --user "$FEISHU_USER_ID" --space-id "$FEISHU_WIKI_SPACE_ID" --send-summary --json
feishu-bot office report --project "<project>" --title "<capability demo>" --file demo.md --base-record --pin --json
feishu-bot office status --project "<project>" --check --json
```

## Status Meanings

- `ok`: the current app/account completed the real Feishu call.
- `missing_scope`: open the grant URL returned by the probe.
- `missing_user_token`: set `FEISHU_USER_ACCESS_TOKEN` and rerun.
- `missing_helpdesk_config`: set `FEISHU_HELPDESK_ID` and
  `FEISHU_HELPDESK_TOKEN`.
- `api_error`: inspect the returned Feishu error and `log_id`.

## Wiki Destination

Use Wiki as the long-term destination when permissions are ready:

```bash
feishu-bot wiki route-check
feishu-bot wiki route-check --write-probe --strict
feishu-bot wiki spaces
feishu-bot wiki nodes --space-id <space_id>
feishu-bot wiki create-node --auth user --space-id <space_id> --title "<capability demo>" --obj-type docx
feishu-bot doc append --auth user --document-id <obj_token> --writer official --file demo.md
feishu-bot wiki node --auth user --token <node_token>
```

For default Docx-to-Wiki publishing:

```bash
FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> \
  feishu-bot doc create --writer official --title "<capability demo>" --file demo.md
```

If a Wiki move is blocked, the CLI may still return a fallback Docx URL and a
`wiki_move_error`. Treat that URL as fallback only; Wiki publishing is proven
only when the response contains `wiki_move` or a follow-up `wiki node` read
succeeds.

## Public Repo Rule

Keep real dogfood artifacts out of the open-source tree. Store them under
ignored directories such as `dogfood/`, `dogfood-artifacts/`, `tmp/`, or
`private/`. Public examples must use placeholders such as `<chat_id>`,
`<message_id>`, `<document_id>`, and `<space_id>`.

# Contributing

This project is built for AI agents and humans who need a reliable Feishu/Lark
CLI. Keep new commands easy to discover from `--help`, `feishu-bot ai`, and
`feishu-bot manifest`.

Before sending changes:

```bash
scripts/check-commit.sh
scripts/ci-local.sh
```

`scripts/check-commit.sh` is the fast local gate. Set
`FEISHU_BOT_FULL_COMMIT_CHECK=1` when you want the same script to run the full
CI-local gate.

When adding a Feishu capability:

- Prefer a typed command over raw string assembly.
- Add examples to help text and the manifest.
- Add tests for argument parsing or request-body construction.
- Dogfood real writes locally, but keep evidence under ignored paths such as
  `dogfood/`, `dogfood-artifacts/`, or `tmp/`.
- Never commit real credentials, tenant IDs, chat IDs, document links, or
  account-specific screenshots.

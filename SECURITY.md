# Security

Do not commit real Feishu/Lark app secrets, user tokens, tenant tokens, refresh
tokens, Helpdesk tokens, Playwright MCP tokens, chat IDs, document URLs, or
tenant-specific dogfood evidence.

Use `.env.example` and `private/.env.example` as templates. Put real local
credentials in ignored files such as `private/local.env`, then run commands with:

```bash
FEISHU_ENV_FILE=private/local.env feishu-bot doctor
```

Before opening a pull request, run:

```bash
scripts/open-source-preflight.sh
cargo fmt -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

If you discover a vulnerability or an accidental secret exposure, open a private
security advisory or contact the maintainer before publishing details.

# Engineering Quality Status

Last reviewed: 2026-06-02

This project is not just a Feishu OpenAPI wrapper. It is intended to be an
AI-operated office bot. The quality goal is therefore stricter than "the code
compiles": future AI agents must understand what to call, avoid leaking local
credentials, create real Feishu artifacts when needed, and read them back before
claiming success.

## Problem Being Solved

The current quality work addresses three concrete problems:

- The project had large mixed-responsibility files that made future AI edits
  risky. Dispatch, request helpers, office workflows, dogfood probes, Drive, and
  Task helpers have been split into smaller responsibility-based modules.
- The product had many atomic commands, but the operating loop needed stronger
  closed-loop proof. `office` and `dogfood` are the workflow layer that creates
  or sends real Feishu data and returns readback evidence.
- The repository is public, so every change must prove that credentials, local
  state, real dogfood artifacts, and media outputs are not included in the
  package or git diff.

## Current Direction

Prefer product and dogfood closure over more refactoring unless a refactor
directly reduces risk for a planned feature.

Use this order:

1. If a user-facing Feishu workflow is unproven, dogfood it first.
2. If a command is hard for AI to discover or call, improve help/manifest/docs.
3. If a module is too large and blocks safe feature work, split it by business
   responsibility.
4. If all of the above are stable, avoid churn.

## Workflow Layer

Daily AI work should prefer:

```bash
feishu-bot office ...
feishu-bot dogfood ...
```

These commands provide the intended high-level behavior:

- create or reuse isolated project chats;
- write one independent document per demo/report;
- use Wiki and Base when configured;
- send project progress, voice, and report messages;
- poll or search project messages;
- return message/Wiki/Base readback probes.

Atomic modules such as `message`, `chat`, `doc`, `wiki`, `base`, `task`,
`drive`, `search`, and `api` remain available for exact OpenAPI operations and
debugging.

## Verification Contract

Before claiming a capability works, gather evidence at the matching scope:

- Local/API-free commands: run the CLI command and inspect JSON output.
- Read-only OpenAPI capability: run `dogfood verify --module <module> --json`
  and inspect each probe status, not just the exit code.
- Write capability: create a real Feishu object, read it back, then send or log
  the result through the target workflow.
- Project workflow: use `office progress`, `office report`, or `office status
  --check` and verify returned readback fields.
- Open-source readiness: run the local gates below and confirm no secrets or
  private artifacts are present.

Recommended gates:

```bash
cargo fmt --check
cargo check --all-targets
cargo test --all-targets
scripts/ci-local.sh
scripts/open-source-preflight.sh
git diff --check
```

Recommended real-account readiness gate for AI office usage:

```bash
feishu-bot dogfood verify --profile office --auto-refresh-user-token --strict --json
```

For AI command discovery, `feishu-bot --json manifest` must expose
`workflow_layer.default_command`, `workflow_layer.verification_command`, and
`workflow_layer.preferred_commands` in addition to the module list.

For package and secret checks:

```bash
cargo package --allow-dirty --list
scripts/open-source-preflight.sh
```

## Refactor Stop Rules

Do not keep splitting files just because a line-count report is available. A
refactor is worth doing only when it meets at least one of these conditions:

- it isolates a workflow boundary that future AI agents must understand;
- it removes mixed API/request/body/output responsibilities from one module;
- it reduces blast radius before adding or verifying a real Feishu capability;
- it updates docs so the new structure is discoverable.

After any refactor, run the same CLI help/manifest checks for the affected
module and, when possible, one local-only or read-only dogfood command.

## Known Remaining Work

These are product-quality priorities, not automatic refactor targets:

- Keep validating `office` as the default one-human-plus-AI operating mode:
  project chat, Wiki/docx report, Base log, inbox/poll/search, and readback.
- Continue using `dogfood verify` for modules whose real-account permissions may
  differ between Feishu tenants.
- Prefer improving AI-facing help and manifest entries when a command is
  correct but hard for an agent to choose.
- Refactor remaining large files only when touching them for a feature, a bug,
  or a verification gap.

## Public Repository Rule

The public repo must contain source, examples, docs, and placeholders only.
Real credentials and proof artifacts belong in ignored paths such as
`private/`, `dogfood/`, `dogfood-artifacts/`, or `tmp/`.

Never commit:

- `.env` with real values;
- Feishu app secrets, tenant tokens, refresh tokens, user access tokens, or
  Playwright MCP extension tokens;
- real chat IDs, private document URLs, or tenant-specific proof data unless
  explicitly scrubbed;
- generated images, audio, video, exported files, or local dogfood outputs.

# Feishu Bot Architecture

Feishu Bot is organized as thin command binaries plus an `app` module tree.

```text
src/lib.rs           Shared application entrypoint and JSON error handling.
src/bin/feishu-bot.rs Primary command entrypoint.
src/bin/feishuBot.rs CamelCase compatibility alias.
src/bin/feishu.rs    Legacy compatibility alias.
src/app/mod.rs       Application dispatcher and module wiring.
src/app/common.rs    Shared query, JSON, content, ID inference, and CLI enum helpers.
src/app/approval.rs  Approval command runner, native approval, task actions, and external connector body builders.
src/app/attendance.rs Attendance command runner, group/shift/schedule/task/flow/stats operations, and attendance body builders.
src/app/base.rs      Base/Bitable command runner.
src/app/base/        Base schema, field schema, record, and permission execution, media, reference, field/list, search, view, role, and member helper builders.
src/app/base/field_schema.rs Base field type parsing, table field spec parsing, and field body builders.
src/app/base/permission_exec.rs Role and member command execution flows.
src/app/base/record_exec.rs Record command execution flows.
src/app/base/schema_exec.rs Table, field, and view command execution flows.
src/app/board.rs     Board/whiteboard command runner, Mermaid/PlantUML import, raw node creation, and Board node JSON normalization.
src/app/calendar.rs  Calendar command runner and Calendar-specific body builders.
src/app/chat.rs      Chat command runner.
src/app/chat/        Chat create/update, tab/menu, and member body/query builders.
src/app/cli/         Clap command and argument definitions split into small parts.
src/app/client.rs    Feishu HTTP client, token handling, and generic request helpers.
src/app/client/      Docx, IM/chat, Board, Drive, and Minutes convenience methods.
src/app/config.rs    Environment loading, base URLs, and secret masking helpers.
src/app/doc.rs       Docx command runner, media insertion, and raw descendant body normalization.
src/app/doc/         Markdown/local block mapping helpers, Docx media insertion helpers, and raw block templates.
src/app/dogfood.rs   Dogfood publish runner plus write/message loop probes.
src/app/dogfood/     Read-probe verify orchestration, probe classification, summaries, and AI remediation helpers.
src/app/drive.rs     Drive command runner.
src/app/drive/       Drive upload, import/export, comment, version, subscription, and permission helpers.
src/app/drive/transfer.rs Drive import/export execution flows.
src/app/drive/comment.rs Typed Drive comment query/body helpers.
src/app/drive/permissions.rs Typed Drive permission query/body helpers.
src/app/help.rs      AI help entrypoint and re-exports.
src/app/help/        Long AI-facing help sections split by workflow area.
src/app/helpdesk.rs  Helpdesk command runner, ticket/message/FAQ operations, service-start body builders, and helpdesk message body builders.
src/app/manifest.rs  Machine-readable manifest entrypoint and filtering.
src/app/manifest/    AI manifest module metadata split by workflow area.
src/app/mail.rs      Mail command runner, mailbox auth selection, message/folder/contact/settings/rule/label operations, and mail body builders.
src/app/message.rs   IM message command runner, image/file/video send helpers, reactions, and pins.
src/app/message/     Voice synthesis/conversion send helpers, message content/reaction body builders, polling cursor/state, and loop-check/readback probe helpers.
src/app/minutes.rs   Minutes command runner, metadata/artifacts/media/transcript operations, token parsing, and search body builders.
src/app/okr.rs       OKR command runner, OKR query helpers, and ID validation.
src/app/office.rs    AI-first project dispatcher plus bootstrap, report, and progress workflows.
src/app/office/      Office interaction/status workflows, local dry-run/list helpers, resource writers, formatting, links, readback, and state helpers.
src/app/output.rs    Human-readable output formatting.
src/app/output/      Feishu block and code-language label maps for output summaries.
src/app/people.rs    Contact, Directory, and CoreHR command runners and body builders.
src/app/people/      Directory, CoreHR, and Hire command runners, query builders, body builders, and ID type helpers.
src/app/raw_api.rs   Raw Feishu API passthrough runner.
src/app/scopes.rs    Open Platform scope filtering and grant URL printing.
src/app/scopes/      Open Platform scope group registry.
src/app/search.rs    Search command runner, doc/message search, and custom search connector body builders.
src/app/setup.rs     Setup automation command runner, doctor probes, Wiki bot grant, and browser open helpers.
src/app/setup/       First-run plan, scope grant, quickstart, and next-action builders.
src/app/sheet.rs     Sheets command runner.
src/app/sheet/       Sheet tab/style/value body builders.
src/app/task.rs      Task command runner.
src/app/task/        Tasklist and structure execution, Task body/query helpers, input/time/reminder normalization helpers, tasklist/comment/member collaboration builders, and section/custom-field plus reminder/dependency relation builders.
src/app/task/tasklist.rs Tasklist command execution flows.
src/app/task/structure_exec.rs Section, custom-field, and attachment command execution flows.
src/app/vc.rs        VC command runner, meeting/reserve/recording/report/room operations, and VC body builders.
src/app/wiki.rs      Wiki/knowledge-space command runner and route checks.
src/app/wiki/        Typed Wiki request body builders and route-check/write-probe helpers.
src/app/tests.rs     Shared unit-test imports and cross-module tests.
src/app/tests/       Capability-specific unit-test modules.
```

Module rules:

- Keep `src/bin/*` thin. They should only start Tokio and call
  `feishu_bot::run_main()`.
- Add new typed Feishu APIs to a business module, not directly to `app/mod.rs`.
- If a module already exists, put command execution and helper builders there.
- Keep `app/cli/` for Clap structs/enums only. Do not put API logic there.
- Keep `app/help.rs` and `app/manifest.rs` in sync when adding AI-facing commands.
- Put new capability-specific tests under `src/app/tests/` instead of growing
  the shared `src/app/tests.rs` file.
- Prefer `pub(super)` for cross-module items inside `app`; avoid `pub` unless it is part of a real library API.
- Every split should preserve `cargo fmt --check`, `cargo check --all-targets`, and `cargo test --all-targets`.

Modules still in `app/mod.rs`:

- Top-level command dispatch, doctor output, and response readers.

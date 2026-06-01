# Feishu Bot Architecture

Feishu Bot is organized as thin command binaries plus an `app` module tree.

```text
src/lib.rs           Shared application entrypoint and JSON error handling.
src/bin/feishu-bot.rs Primary command entrypoint.
src/bin/feishuBot.rs CamelCase compatibility alias.
src/bin/feishu.rs    Legacy compatibility alias.
src/app/mod.rs       Application dispatcher and modules not yet split.
src/app/approval.rs  Approval command runner, native approval, task actions, and external connector body builders.
src/app/attendance.rs Attendance command runner, group/shift/schedule/task/flow/stats operations, and attendance body builders.
src/app/base.rs      Base/Bitable command runner.
src/app/base/        Base media, record, reference, schema, field/list, search, view, role, and member helper builders.
src/app/board.rs     Board/whiteboard command runner, Mermaid/PlantUML import, raw node creation, and Board node JSON normalization.
src/app/calendar.rs  Calendar command runner and Calendar-specific body builders.
src/app/chat.rs      Chat command runner, chat create/list/search/get/delete, member operations, and chat member body builders.
src/app/cli/         Clap command and argument definitions split into small parts.
src/app/client.rs    Feishu HTTP client and token handling.
src/app/client/      Docx document create/append/read helper methods plus Drive/IM/Minutes upload/download helpers.
src/app/config.rs    Environment loading, base URLs, and secret masking helpers.
src/app/doc.rs       Docx command runner, media insertion, and raw descendant body normalization.
src/app/doc/         Markdown/local block mapping helpers and raw block templates.
src/app/dogfood.rs   Dogfood publish runner plus write/message loop probes.
src/app/dogfood/     Read-probe verify orchestration, probe classification, summaries, and AI remediation helpers.
src/app/drive.rs     Drive command runner.
src/app/drive/       Drive upload, import/export, comment, version, subscription, and permission helpers.
src/app/help.rs      AI help entrypoint and re-exports.
src/app/help/        Long AI-facing help sections split by workflow area.
src/app/helpdesk.rs  Helpdesk command runner, ticket/message/FAQ operations, service-start body builders, and helpdesk message body builders.
src/app/manifest.rs  Machine-readable manifest entrypoint and filtering.
src/app/manifest/    AI manifest module metadata split by workflow area.
src/app/mail.rs      Mail command runner, mailbox auth selection, message/folder/contact/settings/rule/label operations, and mail body builders.
src/app/message.rs   IM message command runner, image/file/video send helpers, reactions, and pins.
src/app/message/     Voice synthesis/conversion send helpers plus message polling cursor/state helpers.
src/app/minutes.rs   Minutes command runner, metadata/artifacts/media/transcript operations, token parsing, and search body builders.
src/app/okr.rs       OKR command runner, OKR query helpers, and ID validation.
src/app/office.rs    AI-first project dispatcher plus bootstrap, report, and progress workflows.
src/app/office/      Office interaction/status workflows, local dry-run/list helpers, resource writers, formatting, links, readback, and state helpers.
src/app/output.rs    Human-readable output formatting.
src/app/people.rs    Contact, Directory, and CoreHR command runners and body builders.
src/app/people/      Hire command runner, query builders, body builders, and ID type helpers.
src/app/raw_api.rs   Raw Feishu API passthrough runner.
src/app/scopes.rs    Open Platform scope groups and grant URL printing.
src/app/search.rs    Search command runner, doc/message search, and custom search connector body builders.
src/app/sheet.rs     Sheets command runner, sheet tab operations, and cell value body builders.
src/app/task.rs      Task command runner.
src/app/task/        Task body/query helpers, input/time/reminder normalization helpers, and section/custom-field structure builders.
src/app/vc.rs        VC command runner, meeting/reserve/recording/report/room operations, and VC body builders.
src/app/wiki.rs      Wiki/knowledge-space command runner and route checks.
src/app/wiki/        Typed Wiki request body builders.
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

- Notify/project-chat helpers, common JSON/query/input utilities, and the top-level command dispatcher.

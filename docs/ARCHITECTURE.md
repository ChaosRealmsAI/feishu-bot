# Feishu Bot Architecture

Feishu Bot is organized as thin command binaries plus an `app` module tree.

```text
src/lib.rs           Shared application entrypoint and JSON error handling.
src/bin/feishu-bot.rs Primary command entrypoint.
src/bin/feishuBot.rs CamelCase compatibility alias.
src/bin/feishu.rs    Legacy compatibility alias.
src/app/mod.rs       Application dispatcher and module wiring.
src/app/common.rs    Shared helper re-exports.
src/app/common/      Query/path helpers, content and key-value input readers, JSON body helpers, and ID/content-type argument resolvers.
src/app/approval.rs  Approval command runner.
src/app/approval/    Approval body builders and query helper builders.
src/app/attendance.rs Attendance command runner, group/shift/schedule/task/flow/stats operations, and attendance body builders.
src/app/base.rs      Base/Bitable command runner.
src/app/base/        Base schema, field schema, record, and permission execution, media, reference, field/list, search, view, role, and member helper builders.
src/app/base/field_schema.rs Base field schema entrypoint and re-exports.
src/app/base/field_schema/ Base field body builders, AI-friendly table field spec parser, and shared field build input.
src/app/base/records.rs Base record helper entrypoint and write-query builder.
src/app/base/records/ Base record field input parsing, batch record input parsing, date parsing, and date-field normalization.
src/app/base/permission_exec.rs Role and member command execution flows.
src/app/base/record_exec.rs Record command execution flows.
src/app/base/schema_exec.rs Table, field, and view command execution flows.
src/app/base/helpers/ Base query, view, and permission body/query helpers.
src/app/board.rs     Board/whiteboard command runner, Mermaid/PlantUML import, raw node creation, and Board node JSON normalization.
src/app/calendar.rs  Calendar command runner.
src/app/calendar/    Calendar event, attendee, and free/busy body builders.
src/app/chat.rs      Chat command runner.
src/app/chat/        Chat create/update, tab/menu, and member body/query builders.
src/app/cli/         Clap command and argument definitions split into generated parts and focused workflow submodules.
src/app/cli/office.rs Office workflow command enum.
src/app/cli/office/ Office project/report/voice and interaction/search/cleanup argument definitions.
src/app/client.rs    Feishu HTTP client type and constructor.
src/app/client/      Token/auth/request execution plus Docx, IM/chat, Board, Drive, and Minutes convenience methods.
src/app/client/request.rs Request helper entrypoint.
src/app/client/request/ Token/auth, common JSON wrappers, raw JSON execution, Helpdesk, binary, and multipart request helpers.
src/app/config.rs    Environment loading, base URLs, and secret masking helpers.
src/app/doc.rs       Docx command runner.
src/app/doc/         Markdown/local block mapping helpers, Docx media insertion helpers, and raw block template entrypoints.
src/app/doc/descendants.rs Raw child parsing, converted block cleanup, and descendant body defaults.
src/app/doc/templates/ AI-ready raw block templates split by support matrix, child blocks, and descendant blocks.
src/app/dogfood.rs   Dogfood command dispatch plus write/message loop probes.
src/app/dogfood/     Publish execution, read-probe verify orchestration, read-probe specs, probe classification, summary aggregation, and AI remediation helpers.
src/app/dogfood/publish.rs Dogfood publish workflow and publish input helpers.
src/app/dogfood/specs.rs Dogfood read-probe registry and auth/request metadata.
src/app/dogfood/summary.rs Dogfood probe count, usability, blocked-module, and next-action summaries.
src/app/drive.rs     Drive command dispatcher.
src/app/drive/       Drive file/media/comment/version/subscription/permission execution modules and helpers.
src/app/drive/helpers/ Drive upload, media-extra, import/export task, version/subscription, view-record, and output-file helpers.
src/app/drive/file_ops.rs Drive file list/upload/download/stats/copy/move/delete execution flows.
src/app/drive/media_exec.rs Drive media upload/download/tmp-url execution flows.
src/app/drive/comment_exec.rs Drive comment command execution flows.
src/app/drive/version_exec.rs Drive version and subscription command execution flows.
src/app/drive/permission_exec.rs Drive permission command execution flows.
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
src/app/message/     Voice send entrypoint plus prepare/audio/synth helpers, message content/reaction body builders, polling cursor/state, and loop-check/readback probe helpers.
src/app/minutes.rs   Minutes command runner, metadata/artifacts/media/transcript operations, token parsing, and search body builders.
src/app/okr.rs       OKR command runner, OKR query helpers, and ID validation.
src/app/oauth.rs     OAuth command dispatcher.
src/app/oauth/       OAuth authorization URL/PKCE helpers, token exchange/refresh/user-info requests, token masking/env persistence, and human output formatting.
src/app/office.rs    AI-first project dispatcher plus bootstrap workflow.
src/app/office/      Office report/progress workflows, interaction/status workflows, local dry-run/list helpers, document writers, resource writers, formatting, links, readback, and state helpers.
src/app/office/docs.rs Office Wiki/docx document creation helpers.
src/app/office/report.rs Office report and progress workflow execution.
src/app/output.rs    Human-readable output helper re-exports.
src/app/output/      Generic response summaries, document block/code output printers, and Feishu block/code label maps.
src/app/output/summary.rs Generic human response field/count summaries.
src/app/output/blocks.rs Doc block, conversion, generated-block, and code-language output summaries.
src/app/people.rs    Contact, Directory, and CoreHR command runners and body builders.
src/app/people/      Directory, CoreHR, and Hire command runners, query builders, body builders, and ID type helpers.
src/app/people/hire/ Hire query builders, body builders, and ID type API value mappings.
src/app/raw_api.rs   Raw Feishu API passthrough runner.
src/app/scopes.rs    Open Platform scope filtering and grant URL printing.
src/app/scopes/      Static Open Platform scope group registry.
src/app/scopes/groups.rs Scope group registry entrypoint and stable all-groups ordering.
src/app/scopes/groups/ Static scope groups split by identity, content, and work domains.
src/app/search.rs    Search command runner, doc/message search, and custom search connector body builders.
src/app/setup.rs     Setup automation command dispatcher.
src/app/setup/       First-run plan/scope builders, quickstart/auto/open-scopes flows, doctor/Wiki probes, Wiki bot grant, and browser open helpers.
src/app/sheet.rs     Sheets command runner.
src/app/sheet/       Sheet tab/style/value body builders.
src/app/task.rs      Task command runner.
src/app/task/        Tasklist and structure execution, Task body/query helpers, input/time/reminder normalization helpers, tasklist/comment/member collaboration builders, section plus reminder/dependency relation builders, and custom-field entrypoint.
src/app/task/custom_field/ Task custom-field metadata, value, option, and setting body builders.
src/app/task/tasklist.rs Tasklist command execution flows.
src/app/task/structure_exec.rs Section, custom-field, and attachment command execution flows.
src/app/vc.rs        VC command runner and meeting/reserve/recording/report/room operations.
src/app/vc/          VC request body builders.
src/app/wiki.rs      Wiki/knowledge-space command runner.
src/app/wiki/        Typed Wiki request body builders and route-check entrypoint.
src/app/wiki/route/  Wiki route-check calls, write-probe publishing, recommendation text, and request wrapper.
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

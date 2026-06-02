use serde_json::{json, Value};

pub(in crate::app) fn productivity_manifest_modules() -> Vec<Value> {
    vec![
        json!({
            "name": "task",
            "command": "feishu-bot task",
            "scope_group": "task",
            "status": "typed wrappers and raw JSON bodies",
            "ai_use": "Create/update/read/delete tasks with typed due/start RFC3339/local/date/millisecond times, repeat_rule, custom_complete, origin, extra, mode, milestones, reminders, and custom_fields; complete/reopen tasks; manage task members, reminders, dependencies, task-tasklist links, tasklists, tasklist collaborators, custom sections, custom fields/options/values, attachments, full CRUD comments, and subtasks.",
            "help": ["feishu-bot task --help", "feishu-bot task tasklist --help", "feishu-bot task section --help", "feishu-bot task custom-field --help", "feishu-bot task attachment --help", "feishu-bot task member --help", "feishu-bot task reminder --help", "feishu-bot task dependency --help", "feishu-bot task comment --help", "feishu-bot task subtask --help"],
            "examples": [
                "feishu-bot task create --summary \"Follow up\"",
                "feishu-bot task create --summary \"Review proposal\" --due-at 2026-06-02T15:00:00+08:00 --start-date 2026-06-02",
                "feishu-bot task create --summary \"Submit proposal\" --due-at \"2026-06-03 18:00\" --reminder-minute 30",
                "feishu-bot task create --summary \"All-day milestone\" --due-date 2026-06-05 --mode 1 --is-milestone true",
                "feishu-bot task create --summary \"Weekly sync\" --due-ms 1780000000000 --due-all-day --repeat-rule \"FREQ=WEEKLY;INTERVAL=1\"",
                "feishu-bot task create --summary \"External ticket\" --origin-json '{\"platform_i18n_name\":{\"en_us\":\"AI System\"},\"href\":{\"url\":\"https://example.com/t/1\"}}' --custom-complete-json '{\"pc\":{\"tip\":{\"en_us\":\"Finish in the source system\"}}}' --extra eyJzb3VyY2UiOiJhaSJ9",
                "feishu-bot task list --completed false --type my_tasks",
                "feishu-bot task update --guid <guid> --due-at 2026-06-03T18:00:00+08:00 --mode 1 --is-milestone true",
                "feishu-bot task update --guid <guid> --clear-start --clear-repeat-rule --clear-custom-complete",
                "feishu-bot task member add --task-guid <guid> --assignee ou_xxx",
                "feishu-bot task add-tasklist --task-guid <guid> --tasklist-guid <tasklist_guid> --section-guid <section_guid>",
                "feishu-bot task section create --resource-type tasklist --resource-id <tasklist_guid> --name \"In progress\"",
                "feishu-bot task custom-field create --resource-id <tasklist_guid> --name \"Priority\" --type single_select --option High --option Medium --option Low",
                "feishu-bot task custom-field set-value --task-guid <guid> --custom-field-guid <field_guid> --type single-select --option-guid <option_guid>",
                "feishu-bot task attachment upload --resource-id <task_guid> --file ./brief.pdf",
                "feishu-bot task reminder add --task-guid <guid> --reminder-minute 30",
                "feishu-bot task dependency add --task-guid <guid> --dependency-task-guid <next_guid>",
                "feishu-bot task tasklist add-member --tasklist-guid <tasklist_guid> --editor ou_xxx",
                "feishu-bot task comment create --task-guid <guid> --content \"done\"",
                "feishu-bot task comment update --comment-id <comment_id> --content \"updated\"",
                "feishu-bot task comment delete --comment-id <comment_id>"
            ],
            "known_permission_edges": [
                "feishu-bot task list defaults to --auth user because Feishu's task list API requires user_access_token and returns the caller's my-tasks view; use --completed true|false to filter done/undone tasks.",
                "Core task/tasklist/member/reminder/subtask commands plus section/custom-field/attachment/dependency/comment wrappers support --auth tenant|user.",
                "Use --due-at/--start-at for RFC3339 or local timestamps, --due-date/--start-date for all-day dates, and --due-ms/--start-ms only when millisecond values are already available.",
                "Task reminders are relative to due time; use --reminder-minute and change existing reminders by remove then add because Feishu currently supports one reminder per task.",
                "Tenant-token task calls operate on app-owned task visibility; user-token calls require FEISHU_USER_ACCESS_TOKEN and match that user's Feishu Task Center visibility.",
                "Task dependency add/remove also requires edit permission on the involved tasks."
            ]
        }),
        json!({
            "name": "drive",
            "command": "feishu-bot drive",
            "aliases": ["permission", "drive permission", "drive permissions"],
            "scope_group": "drive",
            "status": "typed wrappers",
            "ai_use": "Upload/download Drive files, including multipart large Drive uploads; upload/download doc/sheet/Base media assets; import local files into online docs; export cloud docs to local files; manage folders, permissions, comments, versions, subscriptions, and view records.",
            "help": ["feishu-bot drive --help", "feishu-bot drive media --help", "feishu-bot drive import --help", "feishu-bot drive export --help", "feishu-bot drive comment --help", "feishu-bot drive version --help", "feishu-bot drive subscription --help", "feishu-bot drive view-record --help", "feishu-bot drive folder --help", "feishu-bot drive permission --help"],
            "examples": [
                "feishu-bot drive upload --file ./report.pdf --folder-token <folder_token>",
                "feishu-bot drive upload-large --file ./large-video.mp4 --folder-token <folder_token>",
                "feishu-bot drive media upload --parent-type docx_image --parent-node <image_block_id> --drive-route-token <document_id> --file ./image.png",
                "feishu-bot drive import file --file ./page.html --type docx --folder-token \"\" --title \"HTML Preview\"",
                "feishu-bot drive export file --token <docx_token> --type docx --file-extension pdf --output ./doc.pdf",
                "feishu-bot drive comment create --file-token <docx_token> --file-type docx --text \"需要复核\"",
                "feishu-bot drive version create --file-token <docx_token> --obj-type docx --name \"AI 修订版\"",
                "feishu-bot drive permission member-list --token <docx_token> --file-type docx",
                "feishu-bot drive permission member-add --token <docx_token> --file-type docx --member-id \"$FEISHU_USER_ID\" --perm view"
            ],
            "known_permission_edges": [
                "drive upload uses drive/v1/files/upload_all for Drive files up to 20 MB.",
                "drive upload-large uses drive/v1/files/upload_prepare, upload_part, and upload_finish for larger Drive files.",
                "drive media upload uses drive/v1/medias/upload_all for docx/sheet/bitable/import assets up to 20 MB.",
                "drive export supports doc/docx to pdf/docx and sheet/bitable to xlsx/csv; exported files are temporary.",
                "drive comment wrappers cover global comments and replies; local comments are readable through list/batch-get but not created by public OpenAPI.",
                "drive subscription create/get/update require FEISHU_USER_ACCESS_TOKEN.",
                "drive view-record requires document management permission and drive:file:view_record:readonly.",
                "HTML online preview should be created as native docx through doc writer or drive import, not treated as raw HTML hosting."
            ]
        }),
        json!({
            "name": "calendar",
            "command": "feishu-bot calendar",
            "scope_group": "calendar",
            "status": "typed wrappers",
            "ai_use": "List/create calendars; create/list/update/delete events; query one or many users/rooms free-busy; add/list/delete event attendees and list chat-attendee members.",
            "help": ["feishu-bot calendar --help", "feishu-bot calendar event --help", "feishu-bot calendar freebusy --help", "feishu-bot calendar attendee --help"],
            "examples": [
                "feishu-bot calendar freebusy list --user-id \"$FEISHU_USER_ID\" --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00",
                "feishu-bot calendar freebusy batch --user-id ou_xxx --user-id ou_yyy --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00",
                "feishu-bot calendar event create --calendar-id primary --summary \"Sync\" --start-ts 1780202400 --end-ts 1780204200",
                "feishu-bot calendar attendee add --calendar-id <calendar_id> --event-id <event_id> --user \"$FEISHU_USER_ID\"",
                "feishu-bot calendar attendee list --calendar-id <calendar_id> --event-id <event_id>"
            ]
        }),
        json!({
            "name": "vc",
            "command": "feishu-bot vc",
            "scope_group": "vc",
            "status": "typed readable and meeting-operation wrappers",
            "ai_use": "Reserve/update/delete video meetings, read active meetings/details/history/recordings/reports/rooms, invite participants, set hosts, end meetings, and start/stop/share recordings.",
            "help": ["feishu-bot vc --help", "feishu-bot vc reserve --help", "feishu-bot vc meeting --help", "feishu-bot vc recording --help", "feishu-bot vc report --help", "feishu-bot vc room --help"],
            "examples": [
                "feishu-bot vc reserve apply --end-time <sec> --owner-id <open_id> --topic \"AI sync\"",
                "feishu-bot vc reserve active-meeting --reserve-id <reserve_id> --with-participants",
                "feishu-bot vc meeting get --meeting-id <meeting_id>",
                "feishu-bot vc meeting invite --meeting-id <meeting_id> --user <open_id>",
                "feishu-bot vc meeting set-host --meeting-id <meeting_id> --user-id <open_id>",
                "feishu-bot vc recording start --meeting-id <meeting_id> --timezone 8",
                "feishu-bot vc recording set-permission --meeting-id <meeting_id> --user <open_id>",
                "feishu-bot vc report daily --start-time <sec> --end-time <sec>",
                "feishu-bot vc room list --page-size 20"
            ],
            "known_permission_edges": [
                "Tenant-token reserve apply requires --owner-id.",
                "Meeting detail reads may require vc:meeting:readonly or vc:meeting.meetingevent:read.",
                "Room reads may require vc:room, vc:room:readonly, or vc:rooms.room.basicinfo:read.",
                "Report reads may require vc:report:readonly.",
                "Set-host can require both vc:meeting and vc:meeting.participant:write.",
                "Invite/end/recording start/stop/permission usually require FEISHU_USER_ACCESS_TOKEN and the operator must be in the meeting or host.",
                "Reserve-created meetings do not create Calendar events; use calendar event commands if a calendar event is required."
            ]
        }),
        json!({
            "name": "minutes",
            "command": "feishu-bot minutes",
            "scope_group": "minutes",
            "status": "typed wrappers plus transcript binary export",
            "ai_use": "Search Feishu Minutes, read metadata, fetch AI artifacts, get media download URLs, and export transcripts.",
            "help": ["feishu-bot minutes --help", "feishu-bot minutes search --help", "feishu-bot minutes transcript --help"],
            "examples": [
                "feishu-bot minutes search --query \"周会\" --page-size 20",
                "feishu-bot minutes get --minute-token <minute_token_or_url>",
                "feishu-bot minutes transcript --minute-token <minute_token_or_url> --need-speaker --need-timestamp --file-format txt --output ./minute.txt"
            ],
            "known_permission_edges": [
                "minutes search requires user_access_token via FEISHU_USER_ACCESS_TOKEN.",
                "Metadata reads may require minutes:minutes, minutes:minutes:readonly, or minutes:minutes.basic:read.",
                "AI artifact reads require minutes:minutes.artifacts:read.",
                "Media download URLs may require minutes:minute:download or minutes:minutes.media:export.",
                "transcript/media export also depends on the Minute file export settings and app data access range."
            ]
        }),
        json!({
            "name": "search",
            "command": "feishu-bot search",
            "scope_group": "search",
            "status": "typed wrappers for docs/message search and custom search connector indexing",
            "ai_use": "Search visible Feishu docs/wiki/messages and manage custom search data sources, schemas, and indexed items.",
            "help": ["feishu-bot search --help", "feishu-bot search docs --help", "feishu-bot search message --help", "feishu-bot search source --help", "feishu-bot search item --help"],
            "examples": [
                "feishu-bot search docs --query \"飞书Bot\" --page-size 10",
                "feishu-bot search message --query \"上线\" --chat-id oc_xxx --page-size 20",
                "feishu-bot search item create --data-source-id <id> --id item_1 --title \"标题\" --url \"https://example.com\" --text \"全文\""
            ],
            "known_permission_edges": [
                "docs and message search require FEISHU_USER_ACCESS_TOKEN.",
                "custom search connector APIs may require an eligible Feishu plan in addition to search:data_source scopes."
            ]
        }),
    ]
}

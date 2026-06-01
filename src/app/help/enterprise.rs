pub(in crate::app) const CALENDAR_AFTER_HELP: &str = r#"AI-safe calendar workflow:
  feishu-bot calendar primary
  feishu-bot calendar list
  feishu-bot calendar create --summary "AI 日历"
  feishu-bot calendar event create --calendar-id <id> --summary "同步会" --start-ts 1760000000 --end-ts 1760003600
  feishu-bot calendar event list --calendar-id <id>
  feishu-bot calendar freebusy list --user-id "$FEISHU_USER_ID" --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
  feishu-bot calendar freebusy batch --user-id ou_xxx --user-id ou_yyy --time-min 2026-06-01T09:00:00+08:00 --time-max 2026-06-01T18:00:00+08:00
  feishu-bot calendar attendee add --calendar-id <id> --event-id <event_id> --user "$FEISHU_USER_ID"
  feishu-bot calendar attendee list --calendar-id <id> --event-id <event_id>
  feishu-bot calendar attendee delete --calendar-id <id> --event-id <event_id> --attendee-id <attendee_id>
  feishu-bot calendar attendee chat-members --calendar-id <id> --event-id <event_id> --attendee-id <chat_attendee_id>

For rooms, recurrence, reminders, conferencing, and complex attendee fields,
pass native Feishu JSON with --body-json/--file/--stdin.
"#;

pub(in crate::app) const APPROVAL_AFTER_HELP: &str = r#"AI-safe approval workflow:
  feishu-bot approval definition get --approval-code <code>
  feishu-bot approval definition subscribe --approval-code <code>
  feishu-bot approval instance list --approval-code <code> --start-time <ms> --end-time <ms>
  feishu-bot approval instance query --approval-code <code> --instance-status PENDING
  feishu-bot approval instance get --instance-code <code>
  feishu-bot approval instance create --body-json '{...}'
  feishu-bot approval instance cancel --approval-code <code> --instance-code <code> --user-id <open_id>
  feishu-bot approval task search --approval-code <code> --task-status PENDING
  feishu-bot approval task approve --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment OK
  feishu-bot approval task reject --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment "needs changes"
  feishu-bot approval task transfer --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --transfer-user-id <open_id>
  feishu-bot approval task add-sign --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --add-user-id <open_id> --add-sign-type 3
  feishu-bot approval task rollback --task-id <task_id> --user-id <open_id> --task-def-key START --reason "revise"
  feishu-bot approval external definition-get --approval-code <code>
  feishu-bot approval external definition-create --file external-definition.json
  feishu-bot approval external instance-sync --file external-instance.json
  feishu-bot approval external instance-check --file external-check.json

Approval forms are schema-specific. Prefer --body-json copied from the approval
definition or OpenAPI explorer. Use `definition get` before creating an
instance so the AI can inspect form widget IDs, node keys, and task IDs.
"#;

pub(in crate::app) const VC_AFTER_HELP: &str = r#"AI-safe video meeting workflow:
  feishu-bot vc reserve apply --end-time <sec> --owner-id <open_id> --topic "AI sync"
  feishu-bot vc reserve get --reserve-id <reserve_id>
  feishu-bot vc reserve active-meeting --reserve-id <reserve_id> --with-participants
  feishu-bot vc reserve update --reserve-id <reserve_id> --end-time <sec>
  feishu-bot vc reserve delete --reserve-id <reserve_id>
  feishu-bot vc meeting get --meeting-id <meeting_id>
  feishu-bot vc meeting list-by-no --meeting-no 123456789 --start-time <sec> --end-time <sec>
  feishu-bot vc meeting invite --meeting-id <meeting_id> --user <open_id>
  feishu-bot vc meeting set-host --meeting-id <meeting_id> --user-id <open_id>
  feishu-bot vc meeting end --meeting-id <meeting_id>
  feishu-bot vc recording get --meeting-id <meeting_id>
  feishu-bot vc recording start --meeting-id <meeting_id> --timezone 8
  feishu-bot vc recording stop --meeting-id <meeting_id>
  feishu-bot vc recording set-permission --meeting-id <meeting_id> --user <open_id>
  feishu-bot vc report daily --start-time <sec> --end-time <sec>
  feishu-bot vc report top-user --start-time <sec> --end-time <sec> --limit 10 --order-by 1
  feishu-bot vc room list --page-size 20
  feishu-bot vc room get --room-id <room_id>
  feishu-bot vc room mget --room-id <room_id>
  feishu-bot vc room-level list --page-size 20

Reserve APIs can use tenant or user auth. In-meeting invite/end and recording
start/stop/permission APIs usually require user_access_token and meeting host or
participant permission. Use --auth tenant only for endpoints that Feishu allows
to run as the app, such as set-host and reserve operations. Set-host may require
both vc:meeting and vc:meeting.participant:write.
"#;

pub(in crate::app) const MINUTES_AFTER_HELP: &str = r#"AI-safe Minutes workflow:
  feishu-bot minutes search --query "周会" --page-size 20
  feishu-bot minutes get --minute-token <minute_token_or_url>
  feishu-bot minutes artifacts --minute-token <minute_token_or_url>
  feishu-bot minutes media --minute-token <minute_token_or_url>
  feishu-bot minutes transcript --minute-token <minute_token_or_url> --need-speaker --need-timestamp --file-format txt --output ./minute.txt

Minute tokens can be passed directly or as full Feishu/Lark minutes URLs. Search
supports --filter-json and --body-json for native Feishu filter payloads.
"#;

pub(in crate::app) const OKR_AFTER_HELP: &str = r#"AI-safe OKR workflow:
  feishu-bot okr period list --page-size 20
  feishu-bot okr period-rule list
  feishu-bot okr user-okrs --user-id "$FEISHU_USER_ID" --offset 0 --limit 5
  feishu-bot okr batch-get --okr-id <okr_id> --lang zh_cn

OKR commands use tenant_access_token by default and require OKR scopes such as
okr:okr.period:readonly, okr:okr:readonly, or okr:okr. Some tenants also require
Feishu OKR enterprise edition.
"#;

pub(in crate::app) const ATTENDANCE_AFTER_HELP: &str = r#"AI-safe Attendance workflow:
  feishu-bot attendance group list --page-size 20
  feishu-bot attendance shift list --page-size 20
  feishu-bot attendance shift query --shift-name "早班"
  feishu-bot attendance schedule query --user-id <employee_id> --from 20260501 --to 20260531
  feishu-bot attendance task query --user-id <employee_id> --from 20260501 --to 20260531 --ignore-invalid-users
  feishu-bot attendance flow query --user-id <employee_id> --from-ts 1760000000 --to-ts 1760086400
  feishu-bot attendance stats query --user-id <employee_id> --operator-user-id <employee_id> --from 20260501 --to 20260531

Attendance commands use tenant_access_token and require attendance scopes:
attendance:rule/attendance:rule:readonly for groups and shifts, and
attendance:task/attendance:task:readonly for schedules, tasks, flows, and stats.
Employee IDs default to employee_id; use --employee-type employee-no for work
numbers. flow delete accepts at most 10 record IDs per request.
"#;

pub(in crate::app) const MAIL_AFTER_HELP: &str = r#"AI-safe Mail workflow:
  feishu-bot mail message list --mailbox me --page-size 10
  feishu-bot mail message get --mailbox me --message-id <message_id> --format metadata
  feishu-bot mail folder list --mailbox me
  feishu-bot mail settings send-as --mailbox me
  feishu-bot mail settings accessible --mailbox me
  feishu-bot mail contact list --mailbox me --page-size 20
  feishu-bot mail message send --mailbox me --to user@example.com --subject "hello" --text "body"

Mail commands use user_access_token when --mailbox me or --auth user is used.
Sending mail always requires FEISHU_USER_ACCESS_TOKEN and
mail:user_mailbox.message:send. Tenant-token reads of explicit mailboxes also
require Mail data resource permissions in the Feishu Open Platform.
"#;

pub(in crate::app) const COREHR_AFTER_HELP: &str = r#"AI-safe CoreHR workflow:
  feishu-bot corehr department search --page-size 20 --field department_name --field code
  feishu-bot corehr department get --department-id <id> --field department_name
  feishu-bot corehr job list --page-size 20
  feishu-bot corehr job get --job-id <id>
  feishu-bot corehr job batch-get --job-id <id> --field job_name
  feishu-bot corehr job-data query --employment-id <id> --page-size 20
  feishu-bot corehr job-data get --job-data-id <id>
  feishu-bot corehr person get --person-id <id>
  feishu-bot corehr process list --modify-time-from <ms> --modify-time-to <ms> --page-size 20
  feishu-bot corehr process get --process-id <id>

CoreHR commands use tenant_access_token and require CoreHR scopes plus Feishu
People data-range grants. Use --body-json/--file/--stdin for full official
CoreHR filters that are not exposed as typed flags.
"#;

pub(in crate::app) const HELPDESK_AFTER_HELP: &str = r#"AI-safe Helpdesk workflow:
  feishu-bot helpdesk ticket list --page-size 20
  feishu-bot helpdesk ticket get --ticket-id <ticket_id>
  feishu-bot helpdesk ticket messages --ticket-id <ticket_id> --page-size 20
  feishu-bot helpdesk service start --open-id <open_id> --human-service
  feishu-bot helpdesk message send --receiver-id <open_id> --text "hello"
  feishu-bot helpdesk faq categories --lang zh_cn
  feishu-bot helpdesk faq list --search "登录" --page-size 20

Helpdesk APIs require tenant_access_token plus FEISHU_HELPDESK_ID and
FEISHU_HELPDESK_TOKEN. The CLI sends X-Lark-Helpdesk-Authorization as
base64(helpdesk_id:helpdesk_token). Use --body-json/--file/--stdin for full
official bodies when typed flags are not enough.
"#;

pub(in crate::app) const HIRE_AFTER_HELP: &str = r#"AI-safe Hire workflow:
  feishu-bot hire job list --page-size 20
  feishu-bot hire job detail --job-id <job_id>
  feishu-bot hire job schemas --scenario 1
  feishu-bot hire process list --page-size 50
  feishu-bot hire talent list --keyword "张三" --page-size 10
  feishu-bot hire talent get --talent-id <talent_id>
  feishu-bot hire application list --talent-id <talent_id> --page-size 20
  feishu-bot hire application get --application-id <application_id>
  feishu-bot hire interview by-talent --talent-id <talent_id>
  feishu-bot hire requirement schemas --page-size 20

Write operations are explicit:
  feishu-bot hire talent create --name "张三" --email zhangsan@example.com
  feishu-bot hire job open --job-id <job_id> --is-never-expired true
  feishu-bot hire talent create --body-json '{...official combined_create body...}'

Hire APIs use tenant_access_token and Feishu Hire data ranges. Sensitive fields
such as user_id require contact:user.employee_id:readonly. Use
--body-json/--file/--stdin when the official Hire payload has custom fields or a
tenant-specific schema.
"#;

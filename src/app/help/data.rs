pub(in crate::app) const BASE_AFTER_HELP: &str = r#"AI-safe Base workflow:
  feishu-bot base parse-url 'https://example.feishu.cn/base/<app_token>?table=<table_id>&view=<view_id>'
  feishu-bot base create --name "AI 工作台"
  feishu-bot base update --app-token <app_token> --name "AI 工作台 v2"
  feishu-bot base copy --app-token <app_token> --name "AI 工作台副本" --folder-token <folder_token>
  feishu-bot base table list --app-token <app_token>
  feishu-bot base table create --app-token <app_token> --name "需求"
  feishu-bot base table create --app-token <app_token> --name "需求" --default-view-name "默认视图" --field "标题:text" --field "状态:single-select:待处理:0|完成:1" --field "金额:currency:0.00|CNY" --field "截止日期:date:yyyy/MM/dd"
  feishu-bot base table batch-create --app-token <app_token> --name "需求" --name "实验"
  feishu-bot base table update --app-token <app_token> --table-id <table_id> --name "需求池"
  feishu-bot base field list --app-token <app_token> --table-id <table_id> --view-id <view_id> --text-field-as-array
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "状态" --kind single-select --option "待处理:0" --option "完成:1"
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "金额" --kind currency --formatter "0.00" --currency-code CNY
  feishu-bot base field create --app-token <app_token> --table-id <table_id> --name "截止日期" --kind date --date-formatter "yyyy/MM/dd" --auto-fill false
  feishu-bot base field update --app-token <app_token> --table-id <table_id> --field-id <field_id> --name "阶段" --kind multi-select --option "进行中:2" --option "阻塞:3"
  feishu-bot base view list --app-token <app_token> --table-id <table_id>
  feishu-bot base view create --app-token <app_token> --table-id <table_id> --name "看板" --view-type kanban
  feishu-bot base view update --app-token <app_token> --table-id <table_id> --view-id <view_id> --hidden-field-id fld_internal --filter-conjunction and --filter-condition 'fld_status:3:is:json:["opt_done"]' --hierarchy-field-id fld_parent
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 标题=hello --field 分数=12.5 --field 完成=true
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field 截止日期=date:2026-06-02 --field 会议时间=datetime:2026-06-02T10:30:00+08:00
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --field '附件=json:[{"file_token":"<file_token>"}]'
  feishu-bot base record create --app-token <app_token> --table-id <table_id> --fields-json '{"标题":"hello"}'
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --view-id <view_id> --field-name "标题" --automatic-fields
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --filter-json '{"conjunction":"and","conditions":[]}' --sort-json '[]'
  feishu-bot base record search --app-token <app_token> --table-id <table_id> --body-json '{}'
  feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --records-json '[{"record_id":"rec...","fields":{"状态":"done"}}]'
  feishu-bot base record batch-create --app-token <app_token> --table-id <table_id> --record-field 0:标题=A --record-field 0:状态=open --record-field 1:标题=B
  feishu-bot base record batch-update --app-token <app_token> --table-id <table_id> --record-id rec_a --record-id rec_b --record-field 0:状态=done --record-field 1:清空=null
  feishu-bot base media upload --app-token <app_token> --kind file --file ./demo.mp4
  feishu-bot base media field-value --file-token <file_token> --field "附件"
  feishu-bot base record update --app-token <app_token> --table-id <table_id> --record-id <record_id> --field 状态=done --field 清空=null
  feishu-bot base record update --app-token <app_token> --table-id <table_id> --record-id <record_id> --fields-json '{"附件":[{"file_token":"<file_token>"}]}'
  feishu-bot base media tmp-url --file-token <file_token> --table-id <table_id> --field-id <field_id> --record-id <record_id>
  feishu-bot base media download --file-token <file_token> --output ./asset.bin --table-id <table_id> --field-id <field_id> --record-id <record_id>
  feishu-bot base dashboard list --app-token <app_token>
  feishu-bot base dashboard copy --app-token <app_token> --block-id <block_id> --name "指标副本"
  feishu-bot base workflow list --app-token <app_token>
  feishu-bot base workflow block-list --app-token <app_token>
  feishu-bot base workflow update --app-token <app_token> --workflow-id <workflow_id> --status disable
  feishu-bot base form get --app-token <app_token> --table-id <table_id> --form-id <form_id>
  feishu-bot base form update --app-token <app_token> --table-id <table_id> --form-id <form_id> --body-json '{...}'
  feishu-bot base update --app-token <app_token> --is-advanced true
  feishu-bot base role list --app-token <app_token> --api-version v2
  feishu-bot base role create --app-token <app_token> --api-version v2 --name "只读成员" --table-roles-json '[...]' --allow-base-complex-edit false --allow-copy false
  feishu-bot base role create --app-token <app_token> --api-version v2 --body-json '{"role_name":"只读成员","table_roles":[...],"base_rule":{"base_complex_edit":0,"copy":0}}'
  feishu-bot base member list --app-token <app_token> --role-id <role_id>
  feishu-bot base member add --app-token <app_token> --role-id <role_id> --member-id "$FEISHU_USER_ID" --member-id-type open_id
  feishu-bot base member batch-add --app-token <app_token> --role-id <role_id> --member open_id:ou_xxx --member chat_id:oc_xxx
  feishu-bot base field delete --app-token <app_token> --table-id <table_id> --field-id <field_id>
  feishu-bot base table batch-delete --app-token <app_token> --table-id <table_id>
  feishu-bot base table delete --app-token <app_token> --table-id <table_id>

Important:
  app_token is the token after /base/ or /app/ in the Base URL. table_id is
  usually in the table= query parameter, or from `base table list`. Use
  `base parse-url` when the user pastes a Base URL. If the URL starts with /wiki/, parse-url
  returns the wiki_node_token; run `feishu-bot wiki node --token <wiki_node_token>`
  and use obj_token as app_token when obj_type is bitable.

Tenant-token access only sees Bases that the app can access. For existing user
owned Bases, add the app as a document/Base collaborator in Feishu, or create
the Base through this CLI.

Base attachments are two-step: `base media upload` returns a file_token scoped
to the Base; write that token into an attachment field with `base record
create/update`. Use `base media field-value` to generate the attachment JSON.
For fields, prefer `base field create/update --kind ...` for common typed
fields such as text, number, currency, single-select, multi-select, date,
checkbox, user, phone, url, attachment, link, formula, location, group, and
auto-number. Use `--type`, `--ui-type`, and `--property-json` as the native
escape hatch when Feishu adds a new field capability.
For new tables, prefer `base table create --field "name:kind[:config]"` when
the AI needs fields at creation time. Config examples: select options split by
`|`, currency `formatter|CURRENCY`, date formatter, formula expression, linked
table_id, user/group `multiple=true`, or `json:{...}` for raw field.property.
For record writes, `--field name=value` parses JSON literals by default. Use
`str:` to force text, `json:` for native objects/arrays, `date:YYYY-MM-DD` for
local all-day Base date fields, and `datetime:` for RFC3339 or local
`YYYY-MM-DD HH:MM[:SS]` values. When field metadata can be read, plain
`YYYY-MM-DD`/`YYYY/MM/DD` strings are also converted automatically for Base
date fields.
For views, use typed update flags for hidden fields, filter_info, and
hierarchy_config; use --property-json for view capabilities not yet typed.
For Bases with advanced permissions, pass table/field/record IDs when
downloading so the tool can build the official bitablePerm extra.

Advanced permission role/member commands require the Base to have advanced
permissions enabled and the caller to have manageable permission on the Base.
For advanced permissions 2.0 custom roles, prefer `base role list/create
--api-version v2`; v2 adds the official `base_rule` permission points for
Base copy/download/print (`base_complex_edit`) and content copy (`copy`).
"#;

pub(in crate::app) const TASK_AFTER_HELP: &str = r#"AI-safe task workflow:
  feishu-bot task tasklist create --name "AI 项目清单"
  feishu-bot task tasklist list
  feishu-bot task tasklist tasks --tasklist-guid <tasklist_guid>
  feishu-bot task list --completed false --type my_tasks
  feishu-bot task tasklist add-member --tasklist-guid <tasklist_guid> --editor "$FEISHU_USER_ID"
  feishu-bot task tasklist remove-member --tasklist-guid <tasklist_guid> --viewer "$FEISHU_USER_ID"
  feishu-bot task create --summary "写周报" --description "整理本周进展" --assignee "$FEISHU_USER_ID"
  feishu-bot task create --summary "明天下午复核" --due-at 2026-06-02T15:00:00+08:00 --start-date 2026-06-02
  feishu-bot task create --summary "提交方案" --due-at "2026-06-03 18:00" --reminder-minute 30
  feishu-bot task create --summary "全天里程碑" --due-date 2026-06-05 --due-all-day --mode 1 --is-milestone true
  feishu-bot task create --summary "每周同步" --due-ms 1780000000000 --due-all-day --repeat-rule "FREQ=WEEKLY;INTERVAL=1"
  feishu-bot task create --summary "外部工单" --origin-json '{"platform_i18n_name":{"zh_cn":"AI系统"},"href":{"url":"https://example.com/t/1"}}' --custom-complete-json '{"pc":{"tip":{"zh_cn":"请去外部系统完成"}}}' --extra "eyJzb3VyY2UiOiJhaSJ9"
  feishu-bot task create --summary "里程碑" --due-date 2026-06-30 --mode 1 --is-milestone true --reminder-minute 30
  feishu-bot task get --guid <task_guid>
  feishu-bot task update --guid <task_guid> --summary "新标题" --due-at 2026-06-03T18:00:00+08:00
  feishu-bot task update --guid <task_guid> --clear-start --clear-repeat-rule --extra "e30="
  feishu-bot task member add --task-guid <task_guid> --assignee "$FEISHU_USER_ID"
  feishu-bot task member remove --task-guid <task_guid> --follower "$FEISHU_USER_ID"
  feishu-bot task tasklists --task-guid <task_guid>
  feishu-bot task add-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid> --section-guid <section_guid>
  feishu-bot task remove-tasklist --task-guid <task_guid> --tasklist-guid <tasklist_guid>
  feishu-bot task section list --resource-type tasklist --resource-id <tasklist_guid>
  feishu-bot task section create --resource-type tasklist --resource-id <tasklist_guid> --name "进行中"
  feishu-bot task section tasks --section-guid <section_guid>
  feishu-bot task custom-field list --resource-type tasklist --resource-id <tasklist_guid>
  feishu-bot task custom-field create --resource-id <tasklist_guid> --name "优先级" --type single_select --option 高 --option 中 --option 低
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type single-select --option-guid <option_guid>
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type text --value "复核通过"
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type multi-select --option-guid <option_a> --option-guid <option_b>
  feishu-bot task custom-field set-value --task-guid <task_guid> --custom-field-guid <field_guid> --type member --member "$FEISHU_USER_ID"
  feishu-bot task custom-field option update --custom-field-guid <field_guid> --option-guid <option_guid> --is-hidden true
  feishu-bot task attachment list --resource-id <task_guid>
  feishu-bot task attachment upload --resource-id <task_guid> --file ./image.png --file ./brief.pdf
  feishu-bot task attachment delete --attachment-guid <attachment_guid>
  feishu-bot task reminder add --task-guid <task_guid> --reminder-minute 30
  feishu-bot task reminder remove --task-guid <task_guid> --reminder-id <reminder_id>
  feishu-bot task dependency add --task-guid <task_guid> --dependency-task-guid <next_task_guid>
  feishu-bot task dependency remove --task-guid <task_guid> --dependency-task-guid <next_task_guid>
  feishu-bot task comment create --task-guid <task_guid> --content "进展说明"
  feishu-bot task comment list --task-guid <task_guid>
  feishu-bot task comment get --comment-id <comment_id>
  feishu-bot task comment update --comment-id <comment_id> --content "更新后的说明"
  feishu-bot task comment delete --comment-id <comment_id>
  feishu-bot task complete --guid <task_guid>
  feishu-bot task subtask create --task-guid <task_guid> --summary "子任务"

Task create/update exposes official typed fields for due/start all-day,
completed_at, repeat_rule, custom_complete, origin, extra, reminders, mode,
is_milestone, and custom_fields. For future fields not yet typed, pass Feishu's
native task JSON:
  feishu-bot task create --body-json '{"summary":"任务","members":[...]}'
  feishu-bot task update --guid <task_guid> --body-json '{"task":{...},"update_fields":[...]}'
  feishu-bot task custom-field create --body-json '{"name":"价格","type":"number","resource_type":"tasklist","resource_id":"<tasklist_guid>","number_setting":{"format":"cny","decimal_count":2,"separator":"thousand"}}'

`feishu-bot task list` defaults to `--auth user` because Feishu's official task-list
API is user-access-token only and lists the caller's "my tasks" view. Use
`--completed true|false` to filter that view; `--type` defaults to `my_tasks`.
Set FEISHU_USER_ACCESS_TOKEN before using that command. Core task/tasklist/member/
reminder/subtask commands plus section/custom-field/attachment/dependency/
comment wrappers accept `--auth tenant|user`; use tenant auth for app-owned task
data and user auth when matching the logged-in user's Feishu Task Center
visibility. App scopes, tasklist permissions, and resource visibility still
matter. Custom field values are typed through `custom-field set-value`; use
`--clear` to set text, number, datetime, or single-select to an empty string,
and member/multi-select to an empty array.
Use --due-at/--start-at for RFC3339 timestamps or local "YYYY-MM-DD HH:MM[:SS]"
values; use --due-date/--start-date for all-day dates. The old --due-ms and
--start-ms remain available when the AI already has Feishu millisecond values.
Task reminders are relative to the task due time in Feishu. Use
`--reminder-minute` during task/subtask creation or `task reminder add`; existing
reminders should be changed by `reminder remove` then `reminder add`. Feishu
currently supports one reminder per task.
"#;

pub(in crate::app) const WIKI_AFTER_HELP: &str = r#"AI-safe wiki workflow:
  feishu-bot wiki route-check
  feishu-bot wiki route-check --write-probe
  feishu-bot wiki route-check --write-probe --strict
  feishu-bot wiki spaces
  feishu-bot wiki nodes --space-id <space_id>
  feishu-bot wiki create-node --space-id <space_id> --title "AI 演示" --obj-type docx
  feishu-bot wiki move-docs-to-wiki --space-id <space_id> --obj-type docx --obj-token <document_id>
  feishu-bot wiki node --token <wiki_node_token>
  feishu-bot wiki task --task-id <task_id>

Admin workflows:
  feishu-bot wiki member list --space-id <space_id>
  feishu-bot wiki member add --space-id <space_id> --member-type openid --member-id <open_id> --member-role admin
  feishu-bot wiki setting update --space-id <space_id> --create-setting admin_and_member

User-token workflows:
  feishu-bot wiki create-space --name "AI 知识库"
  feishu-bot wiki create-node --auth user --space-id <space_id> --title "AI 文档" --obj-type docx
  feishu-bot doc append --auth user --document-id <obj_token> --file ./doc.md
  feishu-bot wiki search --query "关键字"

Wiki nodes reference underlying doc/sheet/bitable/file tokens. Use the matching
typed command to edit the underlying object after locating it. For dogfood,
publish one standalone docx, move it into Wiki, then read both the wiki node and
underlying docx back before reporting success.
Run route-check first when the AI must decide whether future reports can go
through Wiki by default; the normal check verifies config plus read access, and
`--write-probe` creates a proof docx and attempts the real Wiki move. Add
`--strict` in automation so the command exits non-zero unless route_ready is
true.
"#;

pub(in crate::app) const SHEET_AFTER_HELP: &str = r#"AI-safe sheets workflow:
  feishu-bot sheet create --title "AI 数据表" --folder-token <folder_token>
  feishu-bot sheet get --spreadsheet-token <token>
  feishu-bot sheet sheets --spreadsheet-token <token>
  feishu-bot sheet get-sheet --spreadsheet-token <token> --sheet-id <sheet_id>
  feishu-bot sheet add-sheet --spreadsheet-token <token> --title "数据" --index 1
  feishu-bot sheet update-sheet --spreadsheet-token <token> --sheet-id <sheet_id> --title "新标题" --frozen-row-count 1
  feishu-bot sheet copy-sheet --spreadsheet-token <token> --sheet-id <sheet_id> --title "副本"
  feishu-bot sheet delete-sheet --spreadsheet-token <token> --sheet-id <sheet_id>
  feishu-bot sheet values get --spreadsheet-token <token> --range Sheet1!A1:C10
  feishu-bot sheet values update --spreadsheet-token <token> --range Sheet1!A1:B2 --values-json '[[1,2],[3,4]]'
  feishu-bot sheet values append --spreadsheet-token <token> --range Sheet1!A:B --values-json '[["new","row"]]'
  feishu-bot sheet values prepend --spreadsheet-token <token> --range Sheet1!A:B --values-json '[["top","row"]]'
  feishu-bot sheet merge --spreadsheet-token <token> --range Sheet1!A1:C1 --merge-type MERGE_ALL
  feishu-bot sheet unmerge --spreadsheet-token <token> --range Sheet1!A1:C1
  feishu-bot sheet style --spreadsheet-token <token> --range Sheet1!A1:C1 --bold true --back-color fff2cc --border-type FULL_BORDER

Use `sheet create` to start from zero, then manage tabs with add/update/copy/delete
and write cells with values update/append/prepend. Use merge/unmerge/style to
make AI-generated tables readable before sending them. Use --body-json for
complex Sheets v2/v3 native payloads.
"#;

pub(in crate::app) const SEARCH_AFTER_HELP: &str = r#"AI-safe Search workflow:
  feishu-bot search docs --query "飞书Bot" --page-size 10
  feishu-bot search message --query "上线" --chat-id oc_xxx --page-size 20
  feishu-bot search source list --page-size 20
  feishu-bot search schema create --file ./schema.json
  feishu-bot search source create --name "AI 索引" --schema-id ai_schema --state 0
  feishu-bot search item create --data-source-id <id> --id item_1 --title "标题" --url "https://example.com" --text "全文"

Docs/message search requires FEISHU_USER_ACCESS_TOKEN. Search connector
source/schema/item commands use tenant_access_token and need search:data_source
scopes.
"#;

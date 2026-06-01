use serde_json::{json, Value};

pub(in crate::app) fn final_manifest_modules() -> Vec<Value> {
    vec![
        json!({
            "name": "wiki",
            "command": "feishu-bot wiki",
            "scope_group": "wiki",
            "status": "typed wrappers",
            "ai_use": "Diagnose the default Wiki publishing route; create/list wiki spaces, list/resolve/create/move/copy/rename nodes, move docs into Wiki, manage members/settings, search visible wiki nodes, and poll wiki tasks.",
            "help": ["feishu-bot wiki --help", "feishu-bot wiki route-check --help", "feishu-bot wiki member --help", "feishu-bot wiki setting --help"],
            "examples": [
                "feishu-bot wiki route-check",
                "feishu-bot wiki route-check --write-probe",
                "feishu-bot wiki route-check --write-probe --strict",
                "feishu-bot wiki spaces",
                "feishu-bot wiki create-node --space-id <space_id> --title \"AI 演示\" --obj-type docx",
                "feishu-bot wiki move-docs-to-wiki --space-id <space_id> --obj-type docx --obj-token <document_id>",
                "feishu-bot wiki member list --space-id <space_id>",
                "feishu-bot wiki search --query \"关键字\""
            ],
            "known_permission_edges": [
                "create-space and search require FEISHU_USER_ACCESS_TOKEN because the official APIs require user_access_token.",
                "tenant-token calls only see/edit wiki spaces where the app or bot is already a space member or admin.",
                "move-docs-to-wiki also requires management permission on the source document and edit permission on the destination wiki parent.",
                "Use route-check --write-probe --strict before claiming future AI reports can all go through Wiki; read checks alone do not prove publishing."
            ]
        }),
        json!({
            "name": "sheet",
            "command": "feishu-bot sheet",
            "scope_group": "sheet",
            "status": "typed wrappers",
            "ai_use": "Create spreadsheets, inspect and manage sheet tabs, read/write/append/prepend values, merge/unmerge ranges, and apply cell styles.",
            "help": ["feishu-bot sheet --help", "feishu-bot sheet values --help"],
            "examples": [
                "feishu-bot sheet get-sheet --spreadsheet-token sht_xxx --sheet-id <sheet_id>",
                "feishu-bot sheet add-sheet --spreadsheet-token sht_xxx --title \"数据\"",
                "feishu-bot sheet update-sheet --spreadsheet-token sht_xxx --sheet-id <sheet_id> --title \"新标题\"",
                "feishu-bot sheet values update --spreadsheet-token sht_xxx --range Sheet1!A1:B2 --values-json '[[\"a\",\"b\"]]'",
                "feishu-bot sheet values prepend --spreadsheet-token sht_xxx --range Sheet1!A:B --values-json '[[\"top\",\"row\"]]'",
                "feishu-bot sheet merge --spreadsheet-token sht_xxx --range Sheet1!A1:C1 --merge-type MERGE_ALL",
                "feishu-bot sheet style --spreadsheet-token sht_xxx --range Sheet1!A1:C1 --bold true --back-color fff2cc --border-type FULL_BORDER"
            ],
            "known_permission_edges": [
                "Sheet metadata reads may require sheets:spreadsheet, sheets:spreadsheet:readonly, drive:drive, drive:drive:readonly, or sheets:spreadsheet.meta:read.",
                "Cell value reads/writes and style/merge updates require spreadsheet file permission in addition to Sheets scopes.",
                "Wiki-hosted Sheets use the wiki node obj_token as spreadsheet_token after resolving with `feishu-bot wiki node`."
            ]
        }),
        json!({
            "name": "approval",
            "command": "feishu-bot approval",
            "scope_group": "approval",
            "status": "typed native approval and third-party connector wrappers",
            "ai_use": "Get/create/subscribe approval definitions, list/query/create/get/cancel instances, search/approve/reject/transfer/add-sign/rollback tasks, and sync/check third-party approval connector instances.",
            "help": ["feishu-bot approval --help", "feishu-bot approval definition --help", "feishu-bot approval instance --help", "feishu-bot approval task --help", "feishu-bot approval external --help"],
            "examples": [
                "feishu-bot approval definition get --approval-code <code>",
                "feishu-bot approval instance query --approval-code <code> --instance-status PENDING",
                "feishu-bot approval task search --approval-code <code> --task-status PENDING",
                "feishu-bot approval task approve --approval-code <code> --instance-code <code> --task-id <task_id> --user-id <open_id> --comment OK",
                "feishu-bot approval external definition-get --approval-code <code>",
                "feishu-bot approval external instance-sync --file external-instance.json"
            ],
            "known_permission_edges": [
                "Definition reads may require approval:approval:readonly, approval:approval, or approval:definition.",
                "Task search may require approval:approval.list:readonly or approval:approval:readonly.",
                "Approval forms and external connector payloads are schema-specific; use definition get and official JSON files.",
                "Task operations require the operator user ID and task_id from instance task_list.",
                "Rollback uses task_def_key_list from instance timeline node_key values."
            ]
        }),
        json!({
            "name": "notify",
            "command": "feishu-bot notify",
            "scope_group": "im",
            "status": "opinionated AI task card",
            "ai_use": "Send status cards to a user or project chat.",
            "help": ["feishu-bot notify --help"],
            "examples": ["feishu-bot notify --to \"$FEISHU_USER_ID\" --status done --task smoke --summary ok"]
        }),
        json!({
            "name": "api",
            "command": "feishu-bot api",
            "scope_group": "any",
            "status": "universal OpenAPI escape hatch: tenant/user auth, JSON, binary download, multipart upload",
            "ai_use": "Call any official Feishu OpenAPI path not yet wrapped by typed commands.",
            "help": ["feishu-bot api --help", "feishu-bot api download --help", "feishu-bot api multipart --help"],
            "examples": [
                "feishu-bot api get --path /im/v1/chats --query page_size=10",
                "feishu-bot api get --auth user --path /search/v2/data_sources",
                "feishu-bot api multipart --path /im/v1/images --field image_type=message --file image=./image.png"
            ]
        }),
        json!({
            "name": "browser",
            "command": "feishu-bot browser",
            "scope_group": "local",
            "status": "local Playwright MCP helper",
            "ai_use": "Verify browser bridge status and inspect the current logged-in Feishu/Open Platform page.",
            "help": ["feishu-bot browser --help"],
            "examples": ["feishu-bot browser tabs"]
        }),
    ]
}

use serde_json::{json, Value};

pub(in crate::app) fn knowledge_manifest_modules() -> Vec<Value> {
    vec![
        json!({
            "name": "doc",
            "command": "feishu-bot doc",
            "scope_group": "doc",
            "status": "typed markdown writer, official converter, media insertion, and raw block escape hatch",
            "ai_use": "Create/write/read docx docs, insert image/file media, preview block output, print templates, append raw blocks, send links with delivery proof, and optionally/default move newly created docs into Wiki.",
            "help": ["feishu-bot doc capabilities", "feishu-bot doc create --help", "feishu-bot doc insert-media --help", "feishu-bot doc template --kind all", "feishu-bot doc preview --file notes.md"],
            "examples": [
                "feishu-bot doc create --writer official --title \"Report\" --file report.md",
                "feishu-bot doc append --auth user --document-id <wiki_obj_token> --writer official --file report.md",
                "feishu-bot doc raw --auth user --document-id <wiki_obj_token>",
                "feishu-bot doc create --writer official --title \"Report\" --file report.md --send-to \"$FEISHU_USER_ID\" --send-loop-check",
                "feishu-bot doc send-link --document-id docx_xxx --to \"$FEISHU_USER_ID\" --send-loop-check",
                "feishu-bot doc insert-media --document-id docx_xxx --kind image --file ./image.png --width 640 --align 2",
                "feishu-bot doc insert-media --document-id docx_xxx --kind file --file ./attachment.pdf --view-type 1",
                "FEISHU_DOC_CREATE_WIKI_DEFAULT=true FEISHU_WIKI_SPACE_ID=<space_id> feishu-bot doc create --writer official --title \"Report\" --file report.md",
                "feishu-bot doc create --writer official --title \"Report\" --file report.md --wiki --wiki-space-id <space_id> --wiki-fallback-ok",
                "feishu-bot doc append-json --document-id docx_xxx --block-id docx_xxx --file blocks.json"
            ],
            "format_notes": [
                "Mermaid fenced code is preserved as source in docx code blocks.",
                "Use doc insert-media for normal images and file attachments; it creates the block, uploads media, and patches the token.",
                "Renderable Mermaid/PlantUML should use feishu-bot board import.",
                "Public docx OpenAPI cannot create every UI-only block, such as writable mindnote blocks."
            ],
            "known_permission_edges": [
                "Use --send-loop-check with --send-to during dogfood; it verifies the exact doc link message through message get/list, chat metadata, chat members, and read-users."
            ]
        }),
        json!({
            "name": "board",
            "command": "feishu-bot board",
            "scope_group": "board",
            "status": "typed wrappers, SVG-to-editable-board workflow, and raw node escape hatch",
            "ai_use": "Create editable Feishu whiteboards from native-shape SVG, import Mermaid/PlantUML source as board nodes, or write raw Board nodes.",
            "help": ["feishu-bot board --help"],
            "examples": [
                "feishu-bot board template --style brutal-note --title \"System map\" > board.svg",
                "feishu-bot board check-svg --file board.svg --external",
                "feishu-bot board create --title \"System map\" --file board.svg --check --external-check --send-to <chat_id> --send-to-type chat-id",
                "feishu-bot doc create --title \"Board host\" --writer official --content \"# Board host\"",
                "feishu-bot doc append-json --document-id <document_id> --block-id <document_id> --raw-json '[{\"block_type\":43,\"board\":{\"align\":1,\"height\":500,\"width\":900}}]'",
                "feishu-bot board svg --whiteboard-id <whiteboard_id> --file board.svg --check --external-check",
                "feishu-bot board import --whiteboard-id <whiteboard_id> --syntax mermaid --file graph.mmd"
            ],
            "known_permission_edges": [
                "SVG conversion requires Node/npm and npx access to @larksuite/whiteboard-cli; local template/check commands do not need Feishu credentials.",
                "board import requires a whiteboard_id from an existing Feishu Board block; create/read the host doc blocks first if needed."
            ]
        }),
        json!({
            "name": "base",
            "command": "feishu-bot base",
            "scope_group": "base",
            "status": "typed wrappers",
            "ai_use": "Parse Base links; create/copy Base apps; manage tables, typed fields, views, records, attachment media, dashboards, workflows, forms, advanced permission roles including advanced permissions 2.0 base_rule, and role members.",
            "help": [
                "feishu-bot base --help",
                "feishu-bot base parse-url --help",
                "feishu-bot base table --help",
                "feishu-bot base field --help",
                "feishu-bot base record --help",
                "feishu-bot base media --help",
                "feishu-bot base dashboard --help",
                "feishu-bot base workflow --help",
                "feishu-bot base form --help",
                "feishu-bot base role --help",
                "feishu-bot base member --help"
            ],
            "examples": [
                "feishu-bot base parse-url 'https://example.feishu.cn/base/appxxx?table=tblxxx&view=vewxxx'",
                "feishu-bot base create --name \"AI Tasks\"",
                "feishu-bot base table create --app-token app_xxx --name \"Requests\" --default-view-name \"Default\" --field \"Title:text\" --field \"Status:single-select:Open:0|Done:1\" --field \"Amount:currency:0.00|CNY\"",
                "feishu-bot base field create --app-token app_xxx --table-id tbl_xxx --name \"Status\" --kind single-select --option \"Open:0\" --option \"Done:1\"",
                "feishu-bot base field create --app-token app_xxx --table-id tbl_xxx --name \"Amount\" --kind currency --formatter \"0.00\" --currency-code CNY",
                "feishu-bot base field update --app-token app_xxx --table-id tbl_xxx --field-id fld_xxx --name \"Stage\" --kind multi-select --option \"Doing:2\" --option \"Blocked:3\"",
                "feishu-bot base field list --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --text-field-as-array",
                "feishu-bot base view update --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --hidden-field-id fld_internal --filter-conjunction and --filter-condition 'fld_status:3:is:json:[\"opt_done\"]' --hierarchy-field-id fld_parent",
                "feishu-bot base record create --app-token app_xxx --table-id tbl_xxx --field \"Name=demo\" --field \"Score=12.5\" --field \"Done=true\"",
                "feishu-bot base record create --app-token app_xxx --table-id tbl_xxx --field \"Due=date:2026-06-02\" --field \"ReviewAt=datetime:2026-06-02T10:30:00+08:00\"",
                "feishu-bot base record update --app-token app_xxx --table-id tbl_xxx --record-id rec_xxx --field \"Status=done\" --field \"Clear=null\"",
                "feishu-bot base record search --app-token app_xxx --table-id tbl_xxx --view-id vew_xxx --field-name \"Name\" --automatic-fields",
                "feishu-bot base record search --app-token app_xxx --table-id tbl_xxx --filter-json '{\"conjunction\":\"and\",\"conditions\":[]}' --sort-json '[]'",
                "feishu-bot base record batch-create --app-token app_xxx --table-id tbl_xxx --record-field \"0:Name=A\" --record-field \"1:Name=B\"",
                "feishu-bot base record batch-update --app-token app_xxx --table-id tbl_xxx --record-id rec_a --record-id rec_b --record-field \"0:Status=done\" --record-field \"1:Clear=null\"",
                "feishu-bot base record batch-create --app-token app_xxx --table-id tbl_xxx --records-json '[{\"fields\":{\"Name\":\"demo\"}}]'",
                "feishu-bot base media upload --app-token app_xxx --kind file --file ./demo.mp4",
                "feishu-bot base media field-value --file-token <file_token> --field \"附件\"",
                "feishu-bot base workflow block-list --app-token app_xxx",
                "feishu-bot base workflow update --app-token app_xxx --workflow-id wfl_xxx --status disable",
                "feishu-bot base role list --app-token app_xxx --api-version v2",
                "feishu-bot base role create --app-token app_xxx --api-version v2 --name \"Readonly\" --table-roles-json '[{\"table_id\":\"tbl_xxx\",\"table_perm\":1}]' --allow-base-complex-edit false --allow-copy false",
                "feishu-bot base member batch-add --app-token app_xxx --role-id rol_xxx --member open_id:ou_xxx"
            ],
            "known_permission_edges": [
                "Existing user-owned Bases may also require adding the app as a collaborator inside the Base.",
                "base table create supports repeated --field name:kind[:config] for common table.fields; use --fields-json for native Feishu payloads.",
                "Base media upload returns a file_token that still has to be written into an attachment field through base record create/update.",
                "Base record date fields accept date:YYYY-MM-DD and datetime:<RFC3339/local time>; when field metadata is readable, plain YYYY-MM-DD or YYYY/MM/DD strings are converted automatically for date fields.",
                "View property flags cover common hidden_fields, filter_info, and hierarchy_config edits; use --property-json for newer Feishu view capabilities.",
                "Advanced permission role/member commands require advanced permissions enabled and manageable permission on the Base.",
                "For advanced permissions 2.0 custom roles, prefer base role list/create --api-version v2; v2 supports base_rule.base_complex_edit and base_rule.copy."
            ]
        }),
    ]
}

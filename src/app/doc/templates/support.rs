use super::*;

pub(super) fn doc_support_matrix() -> Value {
    json!({
        "mermaid": {
            "preserve_source": "doc template --kind mermaid-code-child, then doc append-json",
            "rendered_diagram": "doc template --kind board-child, doc append-json, doc blocks, then board import --whiteboard-id <whiteboard_id> --syntax mermaid",
            "not_direct_docx": "diagram block has diagram_type but no Mermaid source field and is not writable through public docx OpenAPI"
        },
        "local_writer": [
            "heading1..heading9",
            "text",
            "bullet",
            "ordered",
            "quote",
            "todo",
            "divider",
            "code"
        ],
        "raw_child_templates": [
            "text-child",
            "heading-child",
            "bullet-child",
            "ordered-child",
            "todo-child",
            "quote-child",
            "code-child",
            "mermaid-code-child",
            "divider-child",
            "image-child",
            "file-child",
            "sheet-child",
            "bitable-child",
            "iframe-child",
            "chat-card-child",
            "isv-child",
            "add-ons-child",
            "jira-issue-child",
            "board-child",
            "link-preview-child",
            "sub-page-list-child",
            "wiki-catalog-child"
        ],
        "raw_descendant_templates": [
            "table-descendant",
            "grid-descendant",
            "callout-descendant",
            "quote-container-descendant",
            "agenda-descendant"
        ],
        "token_or_context_required": {
            "image": "requires an uploaded image token",
            "file": "requires an uploaded file token",
            "chat_card": "requires an oc_ chat_id and permissions",
            "isv/add_ons": "requires configured Feishu document component IDs",
            "link_preview": "currently only supports message links",
            "sub_page_list/wiki_catalog": "requires wiki context token",
            "board": "token is generated after insertion; inspect with doc blocks"
        },
        "not_writable_by_public_docx_openapi": {
            "page": "root block only",
            "diagram": "rendered flowchart/UML/Mermaid cannot be created by docx create-block API",
            "mindnote": "read placeholder only",
            "task": "read task_id only; use feishu-bot task commands to create tasks",
            "source_synced": "read-only",
            "reference_synced": "read-only",
            "ai_template": "read-only",
            "undefined": "read-only placeholder"
        },
        "requires_user_access_token_or_external_product": {
            "okr": "docx OKR insertion requires user_access_token; this CLI currently uses tenant_access_token",
            "okr_objective/okr_key_result/okr_progress": "children of an OKR block, not standalone AI-created blocks"
        }
    })
}

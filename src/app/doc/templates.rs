use super::*;

pub(in crate::app) fn doc_template(kind: DocTemplateKind) -> Value {
    match kind {
        DocTemplateKind::All => json!({
            "support-matrix": doc_template(DocTemplateKind::SupportMatrix),
            "text-child": doc_template(DocTemplateKind::TextChild),
            "heading-child": doc_template(DocTemplateKind::HeadingChild),
            "bullet-child": doc_template(DocTemplateKind::BulletChild),
            "ordered-child": doc_template(DocTemplateKind::OrderedChild),
            "todo-child": doc_template(DocTemplateKind::TodoChild),
            "quote-child": doc_template(DocTemplateKind::QuoteChild),
            "code-child": doc_template(DocTemplateKind::CodeChild),
            "mermaid-code-child": doc_template(DocTemplateKind::MermaidCodeChild),
            "divider-child": doc_template(DocTemplateKind::DividerChild),
            "image-child": doc_template(DocTemplateKind::ImageChild),
            "file-child": doc_template(DocTemplateKind::FileChild),
            "sheet-child": doc_template(DocTemplateKind::SheetChild),
            "bitable-child": doc_template(DocTemplateKind::BitableChild),
            "iframe-child": doc_template(DocTemplateKind::IframeChild),
            "chat-card-child": doc_template(DocTemplateKind::ChatCardChild),
            "isv-child": doc_template(DocTemplateKind::IsvChild),
            "add-ons-child": doc_template(DocTemplateKind::AddOnsChild),
            "jira-issue-child": doc_template(DocTemplateKind::JiraIssueChild),
            "board-child": doc_template(DocTemplateKind::BoardChild),
            "link-preview-child": doc_template(DocTemplateKind::LinkPreviewChild),
            "sub-page-list-child": doc_template(DocTemplateKind::SubPageListChild),
            "wiki-catalog-child": doc_template(DocTemplateKind::WikiCatalogChild),
            "table-descendant": doc_template(DocTemplateKind::TableDescendant),
            "grid-descendant": doc_template(DocTemplateKind::GridDescendant),
            "callout-descendant": doc_template(DocTemplateKind::CalloutDescendant),
            "quote-container-descendant": doc_template(DocTemplateKind::QuoteContainerDescendant),
            "agenda-descendant": doc_template(DocTemplateKind::AgendaDescendant),
        }),
        DocTemplateKind::SupportMatrix => doc_support_matrix(),
        DocTemplateKind::TextChild => child_body(vec![text_block(
            2,
            "text",
            "普通文本。可在 elements 内拆成多个 text_run 来做混合样式、链接、@用户、公式。",
        )]),
        DocTemplateKind::HeadingChild => child_body(vec![text_block(
            3,
            "heading1",
            "一级标题。heading1..heading9 对应 block_type 3..11。",
        )]),
        DocTemplateKind::BulletChild => child_body(vec![text_block(12, "bullet", "无序列表项")]),
        DocTemplateKind::OrderedChild => child_body(vec![text_block(13, "ordered", "有序列表项")]),
        DocTemplateKind::TodoChild => child_body(vec![todo_block("待办事项", false)]),
        DocTemplateKind::QuoteChild => child_body(vec![text_block(15, "quote", "引用内容")]),
        DocTemplateKind::CodeChild => child_body(vec![code_block(
            "fn main() {\n    println!(\"hello feishu\");\n}",
            Some("rust"),
        )]),
        DocTemplateKind::MermaidCodeChild => child_body(vec![code_block(
            "flowchart TD\n  A[AI] --> B[feishu-bot]\n  B --> C[Feishu docx code block]",
            Some("mermaid"),
        )]),
        DocTemplateKind::DividerChild => child_body(vec![divider_block()]),
        DocTemplateKind::ImageChild => child_body(vec![json!({
            "block_type": 27,
            "image": {
                "token": "<image_token_from_docx_upload>",
                "width": 640,
                "height": 360,
                "align": 2,
                "caption": {
                    "content": "图片说明"
                }
            }
        })]),
        DocTemplateKind::FileChild => child_body(vec![json!({
            "block_type": 23,
            "file": {
                "token": "<file_token_from_docx_upload>",
                "name": "example.pdf",
                "view_type": 1
            }
        })]),
        DocTemplateKind::SheetChild => child_body(vec![json!({
            "block_type": 30,
            "sheet": {
                "row_size": 5,
                "column_size": 3
            }
        })]),
        DocTemplateKind::BitableChild => child_body(vec![json!({
            "block_type": 18,
            "bitable": {
                "view_type": 1
            }
        })]),
        DocTemplateKind::BoardChild => child_body(vec![json!({
            "block_type": 43,
            "board": {
                "align": 1,
                "width": 900,
                "height": 500
            }
        })]),
        DocTemplateKind::IframeChild => child_body(vec![json!({
            "block_type": 26,
            "iframe": {
                "component": {
                    "type": 11,
                    "url": "https%3A%2F%2Fcodepen.io%2F"
                }
            }
        })]),
        DocTemplateKind::ChatCardChild => child_body(vec![json!({
            "block_type": 20,
            "chat_card": {
                "chat_id": "oc_xxx",
                "align": 1
            }
        })]),
        DocTemplateKind::IsvChild => child_body(vec![json!({
            "block_type": 28,
            "isv": {
                "component_id": "<component_id>",
                "component_type_id": "<component_type_id>"
            }
        })]),
        DocTemplateKind::AddOnsChild => child_body(vec![json!({
            "block_type": 40,
            "add_ons": {
                "component_type_id": "<component_type_id>",
                "record": "{\"key\":\"value\"}"
            }
        })]),
        DocTemplateKind::JiraIssueChild => child_body(vec![json!({
            "block_type": 41,
            "jira_issue": {
                "id": "<jira_issue_id>",
                "key": "PROJ-123"
            }
        })]),
        DocTemplateKind::LinkPreviewChild => child_body(vec![json!({
            "block_type": 48,
            "link_preview": {
                "url": "<message_link_url_encoded>",
                "url_type": "MessageLink"
            }
        })]),
        DocTemplateKind::SubPageListChild => child_body(vec![json!({
            "block_type": 51,
            "sub_page_list": {
                "wiki_token": "<current_wiki_node_token>"
            }
        })]),
        DocTemplateKind::WikiCatalogChild => child_body(vec![json!({
            "block_type": 42,
            "wiki_catalog": {
                "wiki_token": "<wiki_space_or_node_token>"
            }
        })]),
        DocTemplateKind::TableDescendant => json!({
            "index": -1,
            "children_id": ["heading_1", "table_1"],
            "descendants": [
                descendant_text_block("heading_1", 3, "heading1", "简单表格", vec![]),
                {
                    "block_id": "table_1",
                    "block_type": 31,
                    "table": {
                        "property": {
                            "row_size": 1,
                            "column_size": 2
                        }
                    },
                    "children": ["table_cell_1", "table_cell_2"]
                },
                {
                    "block_id": "table_cell_1",
                    "block_type": 32,
                    "table_cell": {},
                    "children": ["table_cell_1_text"]
                },
                {
                    "block_id": "table_cell_2",
                    "block_type": 32,
                    "table_cell": {},
                    "children": ["table_cell_2_text"]
                },
                descendant_text_block("table_cell_1_text", 2, "text", "左侧单元格", vec![]),
                descendant_text_block("table_cell_2_text", 2, "text", "右侧单元格", vec![])
            ]
        }),
        DocTemplateKind::GridDescendant => json!({
            "index": -1,
            "children_id": ["grid_1"],
            "descendants": [
                {
                    "block_id": "grid_1",
                    "block_type": 24,
                    "grid": {
                        "column_size": 2
                    },
                    "children": ["grid_col_1", "grid_col_2"]
                },
                {
                    "block_id": "grid_col_1",
                    "block_type": 25,
                    "grid_column": {
                        "width_ratio": 50
                    },
                    "children": ["grid_col_1_text"]
                },
                {
                    "block_id": "grid_col_2",
                    "block_type": 25,
                    "grid_column": {
                        "width_ratio": 50
                    },
                    "children": ["grid_col_2_text"]
                },
                descendant_text_block("grid_col_1_text", 2, "text", "左栏内容", vec![]),
                descendant_text_block("grid_col_2_text", 2, "text", "右栏内容", vec![])
            ]
        }),
        DocTemplateKind::CalloutDescendant => json!({
            "index": -1,
            "children_id": ["callout_1"],
            "descendants": [
                {
                    "block_id": "callout_1",
                    "block_type": 19,
                    "callout": {
                        "background_color": 5,
                        "border_color": 5,
                        "text_color": 7,
                        "emoji_id": "bulb"
                    },
                    "children": ["callout_1_text"]
                },
                descendant_text_block("callout_1_text", 2, "text", "高亮块内容", vec![])
            ]
        }),
        DocTemplateKind::QuoteContainerDescendant => json!({
            "index": -1,
            "children_id": ["quote_container_1"],
            "descendants": [
                {
                    "block_id": "quote_container_1",
                    "block_type": 34,
                    "quote_container": {},
                    "children": ["quote_container_text_1"]
                },
                descendant_text_block(
                    "quote_container_text_1",
                    2,
                    "text",
                    "引用容器内的内容",
                    vec![]
                )
            ]
        }),
        DocTemplateKind::AgendaDescendant => json!({
            "index": -1,
            "children_id": ["agenda_1"],
            "descendants": [
                {
                    "block_id": "agenda_1",
                    "block_type": 44,
                    "agenda": {},
                    "children": ["agenda_item_1"]
                },
                {
                    "block_id": "agenda_item_1",
                    "block_type": 45,
                    "agenda_item": {},
                    "children": ["agenda_item_title_1", "agenda_item_content_1"]
                },
                {
                    "block_id": "agenda_item_title_1",
                    "block_type": 46,
                    "agenda_item_title": {
                        "align": 1,
                        "elements": [{
                            "text_run": {
                                "content": "议题一",
                                "text_element_style": {}
                            }
                        }]
                    },
                    "children": []
                },
                {
                    "block_id": "agenda_item_content_1",
                    "block_type": 47,
                    "agenda_item_content": {},
                    "children": ["agenda_item_content_text_1"]
                },
                descendant_text_block(
                    "agenda_item_content_text_1",
                    2,
                    "text",
                    "议题内容和结论",
                    vec![]
                )
            ]
        }),
    }
}

fn doc_support_matrix() -> Value {
    json!({
        "mermaid": {
            "preserve_source": "doc template --kind mermaid-code-child, then doc append-json",
            "rendered_diagram": "doc template --kind board-child, doc append-json, doc blocks, then board import --syntax mermaid",
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

fn child_body(children: Vec<Value>) -> Value {
    json!({
        "index": -1,
        "children": children,
    })
}

fn descendant_text_block(
    block_id: &str,
    block_type: i64,
    field: &str,
    content: &str,
    children: Vec<&str>,
) -> Value {
    let mut block = text_block(block_type, field, content);
    if let Some(object) = block.as_object_mut() {
        object.insert("block_id".to_string(), Value::String(block_id.to_string()));
        object.insert(
            "children".to_string(),
            Value::Array(
                children
                    .into_iter()
                    .map(|child| Value::String(child.to_string()))
                    .collect(),
            ),
        );
    }
    block
}

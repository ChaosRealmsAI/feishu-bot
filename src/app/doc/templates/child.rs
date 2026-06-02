use super::*;

pub(super) fn child_template(kind: DocTemplateKind) -> Value {
    match kind {
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
        _ => unreachable!("non-child doc template kind routed to child templates"),
    }
}

fn child_body(children: Vec<Value>) -> Value {
    json!({
        "index": -1,
        "children": children,
    })
}

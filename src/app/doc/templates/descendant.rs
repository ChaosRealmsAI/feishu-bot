use super::*;

pub(super) fn descendant_template(kind: DocTemplateKind) -> Value {
    match kind {
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
        _ => unreachable!("non-descendant doc template kind routed to descendant templates"),
    }
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

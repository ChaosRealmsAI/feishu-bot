use super::super::*;

#[test]
fn converts_markdown_to_doc_blocks() {
    let blocks = markdown_to_blocks(
        "# Title\n\n- one\n1. two\n- [x] done\n> quote\n---\n```rust\nfn main() {}\n```\nbody",
    );
    assert_eq!(blocks.len(), 8);
    assert_eq!(blocks[0]["block_type"], 3);
    assert!(blocks[0].get("heading1").is_some());
    assert_eq!(blocks[1]["block_type"], 12);
    assert_eq!(blocks[2]["block_type"], 13);
    assert_eq!(blocks[3]["block_type"], 17);
    assert_eq!(blocks[4]["block_type"], 15);
    assert_eq!(blocks[5]["block_type"], 22);
    assert_eq!(blocks[6]["block_type"], 14);
    assert_eq!(blocks[6]["code"]["style"]["language"], 53);
    assert_eq!(blocks[7]["block_type"], 2);
}

#[test]
fn preserves_mermaid_as_plain_text_code() {
    let blocks = markdown_to_blocks("```mermaid\nflowchart TD\n  A --> B\n```");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0]["block_type"], 14);
    assert_eq!(blocks[0]["code"]["style"]["language"], 1);
    assert_eq!(
        blocks[0]["code"]["elements"][0]["text_run"]["content"],
        "flowchart TD\n  A --> B"
    );
}

#[test]
fn emits_doc_templates_for_raw_block_writing() {
    let matrix = doc_template(DocTemplateKind::SupportMatrix);
    assert_eq!(
            matrix["mermaid"]["rendered_diagram"],
            "doc template --kind board-child, doc append-json, doc blocks, then board import --syntax mermaid"
        );
    assert!(matrix["not_writable_by_public_docx_openapi"]["mindnote"].is_string());

    let mermaid = doc_template(DocTemplateKind::MermaidCodeChild);
    assert_eq!(mermaid["children"][0]["block_type"], 14);
    assert_eq!(mermaid["children"][0]["code"]["style"]["language"], 1);

    let iframe = doc_template(DocTemplateKind::IframeChild);
    assert_eq!(iframe["children"][0]["iframe"]["component"]["type"], 11);
    assert!(iframe["children"][0]["iframe"]
        .get("component_type")
        .is_none());

    let agenda = doc_template(DocTemplateKind::AgendaDescendant);
    assert_eq!(agenda["descendants"][0]["block_type"], 44);
    assert_eq!(agenda["descendants"][2]["block_type"], 46);

    let table = doc_template(DocTemplateKind::TableDescendant);
    assert_eq!(table["children_id"][1], "table_1");
    assert_eq!(table["descendants"][1]["block_type"], 31);
    assert_eq!(table["descendants"][2]["block_type"], 32);
}

#[test]
fn builds_doc_media_insert_bodies() {
    let placeholder = build_doc_media_placeholder(DocMediaKindArg::Image);
    assert_eq!(placeholder["block_type"], 27);
    assert!(placeholder["image"].is_object());

    let image = build_doc_media_replace_body(
        DocMediaKindArg::Image,
        "file_token_1",
        "image.png",
        Some(640),
        Some(360),
        Some(2),
        None,
    );
    assert_eq!(image["replace_image"]["token"], "file_token_1");
    assert_eq!(image["replace_image"]["width"], 640);
    assert_eq!(image["replace_image"]["height"], 360);
    assert_eq!(image["replace_image"]["align"], 2);

    let file = build_doc_media_replace_body(
        DocMediaKindArg::File,
        "file_token_2",
        "report.pdf",
        None,
        None,
        None,
        Some(1),
    );
    assert_eq!(file["replace_file"]["token"], "file_token_2");
    assert_eq!(file["replace_file"]["name"], "report.pdf");
    assert_eq!(file["replace_file"]["view_type"], 1);

    let response = json!({
        "data": { "children": [{ "block_id": "doxcn_block" }] }
    });
    assert_eq!(first_appended_block_id(&response).unwrap(), "doxcn_block");
}

use super::super::*;

#[test]
fn maps_board_syntax_values() {
    assert_eq!(BoardSyntaxArg::Plantuml.as_api_value(), 1);
    assert_eq!(BoardSyntaxArg::Mermaid.as_api_value(), 2);
}

#[test]
fn wraps_board_node_arrays() {
    let body = read_board_nodes_json(Some(r#"[{"id":"n1:1"}]"#.to_string()), None, false).unwrap();
    assert_eq!(body["nodes"][0]["id"], "n1:1");

    let body = read_board_nodes_json(
        Some(r#"{"nodes":[{"id":"n1:2"}]}"#.to_string()),
        None,
        false,
    )
    .unwrap();
    assert_eq!(body["nodes"][0]["id"], "n1:2");
}

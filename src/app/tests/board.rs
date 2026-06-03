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

#[test]
fn checks_svg_medium_rules() {
    let ok = check_svg_medium(
        r##"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="100" height="60" fill="#fff"/><text x="10" y="20">ok</text></svg>"##,
    );
    assert_eq!(ok["errors"], 0);

    let bad = check_svg_medium(
        r##"<svg xmlns="http://www.w3.org/2000/svg"><linearGradient id="g"/><rect opacity="0.5" font-family="Arial"/></svg>"##,
    );
    assert!(bad["errors"].as_u64().unwrap() >= 2);
}

#[test]
fn allows_marker_path_for_arrows() {
    let checked = check_svg_medium(
        r##"<svg xmlns="http://www.w3.org/2000/svg"><defs><marker id="arrow"><path d="M0 0 L10 4 L0 8 z"/></marker></defs><line x1="0" y1="0" x2="10" y2="0" marker-end="url(#arrow)"/><text x="0" y="20">ok</text></svg>"##,
    );
    assert_eq!(checked["errors"], 0);
    assert_eq!(checked["warnings"], 0);
}

#[test]
fn emits_native_shape_svg_template() {
    let svg = board_svg_template(BoardSvgStyleArg::BrutalNote, "测试画板");
    assert!(svg.contains("测试画板"));
    assert!(svg.contains("<rect"));
    assert!(svg.contains("<text"));
    assert!(!svg.contains("font-family"));
}

#[test]
fn normalizes_whiteboard_cli_openapi_output() {
    let value = json!({
        "code": 0,
        "data": {
            "result": {
                "nodes": [
                    {"id": "n1"}
                ]
            }
        }
    });
    let body = normalize_whiteboard_cli_nodes(value).unwrap();
    assert_eq!(body["nodes"][0]["id"], "n1");
}

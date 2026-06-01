use super::super::*;

#[test]
fn infers_receiver_ids() {
    assert_eq!(infer_receive_id_type("oc_abc"), "chat_id");
    assert_eq!(infer_receive_id_type("ou_abc"), "open_id");
    assert_eq!(infer_receive_id_type("on_abc"), "union_id");
    assert_eq!(infer_receive_id_type("a@example.com"), "email");
    assert_eq!(infer_receive_id_type("user123"), "user_id");
}

#[test]
fn parses_bool_env_defaults() {
    let mut values = HashMap::new();
    values.insert(
        "FEISHU_DOC_CREATE_WIKI_DEFAULT".to_string(),
        "true".to_string(),
    );
    assert_eq!(
        get_bool_any(&values, &["FEISHU_DOC_CREATE_WIKI_DEFAULT"]),
        Some(true)
    );
    values.insert(
        "FEISHU_DOC_CREATE_WIKI_DEFAULT".to_string(),
        "off".to_string(),
    );
    assert_eq!(
        get_bool_any(&values, &["FEISHU_DOC_CREATE_WIKI_DEFAULT"]),
        Some(false)
    );
    values.insert(
        "FEISHU_DOC_CREATE_WIKI_DEFAULT".to_string(),
        "maybe".to_string(),
    );
    assert_eq!(
        get_bool_any(&values, &["FEISHU_DOC_CREATE_WIKI_DEFAULT"]),
        None
    );
}

#[test]
fn parses_raw_api_query_pairs() {
    let pairs = parse_query_pairs(vec!["a=1".to_string(), "b=two".to_string()]).unwrap();
    assert_eq!(
        pairs,
        vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "two".to_string())
        ]
    );
    assert!(parse_query_pairs(vec!["missing".to_string()]).is_err());

    let headers = parse_header_pairs(vec!["X-Test=ok".to_string()]).unwrap();
    assert_eq!(headers, vec![("X-Test".to_string(), "ok".to_string())]);
    let file_parts = parse_file_part_pairs(vec!["image=/tmp/a.png".to_string()]).unwrap();
    assert_eq!(file_parts[0].0, "image");
    assert_eq!(file_parts[0].1, PathBuf::from("/tmp/a.png"));
}

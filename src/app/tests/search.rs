use super::super::*;

#[test]
fn parses_search_commands_after_cli_split() {
    let docs = Cli::parse_from([
        "feishu",
        "search",
        "docs",
        "--query",
        "飞书Bot",
        "--doc-type",
        "DOCX",
        "--doc-type",
        "BITABLE",
        "--space-id",
        "spc_1",
        "--only-title",
        "--page-size",
        "10",
    ]);
    match docs.command {
        Commands::Search(SearchCommand::Docs(args)) => {
            assert_eq!(args.query.as_deref(), Some("飞书Bot"));
            assert_eq!(args.doc_types, vec!["DOCX", "BITABLE"]);
            assert_eq!(args.space_ids, vec!["spc_1"]);
            assert!(args.only_title);
            assert_eq!(args.page_size, 10);
        }
        _ => panic!("expected search docs"),
    }

    let item = Cli::parse_from([
        "feishu",
        "search",
        "item",
        "create",
        "--data-source-id",
        "ds_1",
        "--id",
        "item_1",
        "--title",
        "标题",
        "--url",
        "https://example.com",
        "--structured-json",
        r#"{"summary":"摘要"}"#,
        "--text",
        "全文",
    ]);
    match item.command {
        Commands::Search(SearchCommand::Item(SearchItemCommand::Create(args))) => {
            assert_eq!(args.data_source_id, "ds_1");
            assert_eq!(args.id.as_deref(), Some("item_1"));
            assert_eq!(args.title.as_deref(), Some("标题"));
            assert_eq!(args.url.as_deref(), Some("https://example.com"));
            assert_eq!(
                args.structured_json.as_deref(),
                Some(r#"{"summary":"摘要"}"#)
            );
            assert_eq!(args.text.as_deref(), Some("全文"));
        }
        _ => panic!("expected search item create"),
    }
}

#[test]
fn builds_search_docs_and_message_bodies() {
    let docs = build_search_docs_body(SearchDocsArgs {
        query: Some("飞书Bot".to_string()),
        page_size: 10,
        page_token: Some("next".to_string()),
        doc_types: vec!["DOCX".to_string(), "BITABLE".to_string()],
        folder_tokens: vec!["fld_1".to_string()],
        space_ids: vec!["spc_1".to_string()],
        only_title: true,
        sort_type: Some("CREATE_TIME".to_string()),
        create_start: Some(1760000000),
        create_end: Some(1760003600),
        open_start: None,
        open_end: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(docs["query"], "飞书Bot");
    assert_eq!(docs["page_size"], 10);
    assert_eq!(docs["doc_filter"]["doc_types"][0], "DOCX");
    assert_eq!(docs["doc_filter"]["folder_tokens"][0], "fld_1");
    assert_eq!(docs["wiki_filter"]["space_ids"][0], "spc_1");
    assert_eq!(docs["doc_filter"]["create_time"]["start"], 1760000000);

    let message = build_search_message_body(SearchMessageArgs {
        query: Some("上线".to_string()),
        page_size: 20,
        page_token: None,
        from_ids: vec!["ou_1".to_string()],
        chat_ids: vec!["oc_1".to_string()],
        at_chatter_ids: vec!["ou_2".to_string()],
        message_type: Some("image".to_string()),
        from_type: Some("user".to_string()),
        chat_type: Some("group_chat".to_string()),
        start_time: Some("1760000000".to_string()),
        end_time: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(message["query"], "上线");
    assert_eq!(message["chat_ids"][0], "oc_1");
    assert_eq!(message["message_type"], "image");
}

#[test]
fn builds_search_source_and_item_bodies() {
    let source = build_search_source_body(
        SearchSourceWriteArgs {
            name: Some("AI 索引".to_string()),
            description: Some("project index".to_string()),
            icon_url: None,
            schema_id: Some("ai_schema".to_string()),
            template: Some("search_common_card".to_string()),
            state: Some(0),
            body_json: None,
            file: None,
            stdin: false,
        },
        true,
    )
    .unwrap();
    assert_eq!(source["name"], "AI 索引");
    assert_eq!(source["schema_id"], "ai_schema");
    assert_eq!(source["state"], 0);

    let item = build_search_item_body(SearchItemCreateArgs {
        data_source_id: "ds_1".to_string(),
        id: Some("item_1".to_string()),
        title: Some("标题".to_string()),
        url: Some("https://example.com".to_string()),
        mobile_url: None,
        structured_json: Some(r#"{"summary":"摘要"}"#.to_string()),
        text: Some("全文".to_string()),
        content_format: "plaintext".to_string(),
        acl_json: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(item["id"], "item_1");
    assert_eq!(item["metadata"]["title"], "标题");
    assert_eq!(item["acl"][0]["value"], "everyone");
    assert_eq!(item["structured_data"], "{\"summary\":\"摘要\"}");
    assert_eq!(item["content"]["content_data"], "全文");
}

use super::super::*;

#[test]
fn normalizes_bot_info_for_wiki_member_grant() {
    let response = json!({
        "code": 0,
        "msg": "ok",
        "bot": {
            "open_id": "ou_bot",
            "app_name": "LarkPilot"
        }
    });
    let normalized = normalize_bot_info_response(response);
    assert_eq!(normalized["data"]["open_id"], "ou_bot");
    assert!(normalized["data"]["wiki_member_add_example"]
        .as_str()
        .unwrap()
        .contains("--member-id ou_bot"));
}

#[test]
fn recommends_wiki_route_next_step() {
    assert!(wiki_route_recommendation(false, false, false, false, None)
        .contains("FEISHU_WIKI_SPACE_ID"));
    assert!(wiki_route_recommendation(false, true, false, false, None)
        .contains("FEISHU_DOC_CREATE_WIKI_DEFAULT"));
    assert!(
        wiki_route_recommendation(true, true, false, false, None).contains("OpenAPI checks failed")
    );
    assert!(wiki_route_recommendation(true, true, true, false, None).contains("write-probe"));
    assert!(
        wiki_route_recommendation(true, true, true, true, Some(false))
            .contains("write probe did not prove")
    );
    assert!(
        wiki_route_recommendation(true, true, true, true, Some(true))
            .contains("Wiki write route is ready")
    );
}

#[test]
fn formats_wiki_route_strict_error() {
    let data = json!({
        "data": {
            "recommendation": "Grant Wiki scopes and add the app to the target Wiki space.",
            "checks": [
                {
                    "name": "list_spaces",
                    "ok": false,
                    "error": "missing wiki:wiki"
                }
            ]
        }
    });
    let message = wiki_route_check_strict_error(&data);
    assert!(message.contains("list_spaces"));
    assert!(message.contains("Grant Wiki scopes"));
    assert!(message.contains("missing wiki:wiki"));
}

#[test]
fn default_wiki_route_keeps_doc_fallback_unless_strict() {
    let mut args = DocCreateArgs {
        title: "Dogfood".to_string(),
        folder_token: None,
        auth: ApiAuthArg::Tenant,
        writer: WriterArg::Local,
        content_type: ContentTypeArg::Markdown,
        content: None,
        file: None,
        stdin: false,
        send_to: None,
        send_to_type: ReceiveIdTypeArg::Auto,
        send_loop_check: false,
        wiki: false,
        no_wiki: false,
        wiki_space_id: None,
        wiki_parent_token: None,
        wiki_apply: false,
        wiki_fallback_ok: false,
        wiki_strict: false,
        wiki_auth: ApiAuthArg::Tenant,
    };

    assert!(!doc_create_allows_wiki_fallback(&args, false));
    assert!(doc_create_allows_wiki_fallback(&args, true));

    args.wiki_strict = true;
    assert!(!doc_create_allows_wiki_fallback(&args, true));

    args.wiki_fallback_ok = true;
    assert!(doc_create_allows_wiki_fallback(&args, false));
    assert!(doc_create_allows_wiki_fallback(&args, true));
}

#[test]
fn builds_wiki_typed_bodies() {
    let auto_publish =
        build_doc_create_wiki_move_body("docx_1", Some("wik_parent".to_string()), true);
    assert_eq!(auto_publish["obj_type"], "docx");
    assert_eq!(auto_publish["obj_token"], "docx_1");
    assert_eq!(auto_publish["parent_wiki_token"], "wik_parent");
    assert_eq!(auto_publish["apply"], true);

    let node = build_wiki_create_node_body(WikiCreateNodeArgs {
        space_id: "spc_1".to_string(),
        obj_type: "docx".to_string(),
        node_type: "origin".to_string(),
        parent_node_token: Some("wik_parent".to_string()),
        origin_node_token: None,
        title: Some("AI 演示".to_string()),
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(node["obj_type"], "docx");
    assert_eq!(node["parent_node_token"], "wik_parent");
    assert_eq!(node["title"], "AI 演示");

    let move_docs = build_wiki_move_docs_to_wiki_body(WikiMoveDocsToWikiArgs {
        space_id: "spc_1".to_string(),
        parent_wiki_token: Some("wik_parent".to_string()),
        obj_type: Some("docx".to_string()),
        obj_token: Some("docx_1".to_string()),
        apply: true,
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(move_docs["obj_type"], "docx");
    assert_eq!(move_docs["obj_token"], "docx_1");
    assert_eq!(move_docs["apply"], true);

    let member = build_wiki_member_add_body(WikiMemberAddArgs {
        space_id: "spc_1".to_string(),
        member_type: Some("openid".to_string()),
        member_id: Some("ou_1".to_string()),
        member_role: "admin".to_string(),
        need_notification: Some(false),
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(member["member_type"], "openid");
    assert_eq!(member["member_role"], "admin");

    let search = build_wiki_search_body(WikiSearchArgs {
        query: Some("dogfood".to_string()),
        space_id: Some("spc_1".to_string()),
        node_id: Some("wik_parent".to_string()),
        page_size: 10,
        page_token: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(search["query"], "dogfood");
    assert_eq!(search["space_id"], "spc_1");
    assert_eq!(search["node_id"], "wik_parent");
}

use super::*;
use std::collections::HashMap;

mod base;
mod drive;
mod message;
mod office;
mod sheet;
mod task;

#[test]
fn infers_receiver_ids() {
    assert_eq!(infer_receive_id_type("oc_abc"), "chat_id");
    assert_eq!(infer_receive_id_type("ou_abc"), "open_id");
    assert_eq!(infer_receive_id_type("on_abc"), "union_id");
    assert_eq!(infer_receive_id_type("a@example.com"), "email");
    assert_eq!(infer_receive_id_type("user123"), "user_id");
}

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
fn detects_loop_check_probe_content() {
    let probe = probe_value(Ok(json!({
        "data": {
            "items": [
                {
                    "message_id": "om_1",
                    "body": {
                        "content": "{\"text\":\"loop ok\"}"
                    }
                }
            ]
        }
    })));
    assert_eq!(probe["ok"], true);
    assert!(response_contains(&probe, "om_1"));
    assert!(response_contains(&probe, "loop ok"));
    assert!(!response_contains_multiline_text(
        &probe,
        "loop ok\nsecond line"
    ));
    assert!(response_contains_multiline_text(&probe, "om_1\nloop ok"));
}

#[test]
fn resolves_dogfood_defaults() {
    assert_eq!(
        resolve_dogfood_receiver(Some("ou_explicit".to_string()), Some("ou_default")).unwrap(),
        "ou_explicit"
    );
    assert_eq!(
        resolve_dogfood_receiver(None, Some("ou_default")).unwrap(),
        "ou_default"
    );
    assert!(resolve_dogfood_receiver(None, None).is_err());

    assert!(
        dogfood_wiki_target(false, false, false, None, None, None, None)
            .unwrap()
            .is_none()
    );
    assert!(dogfood_wiki_target(false, true, false, None, None, None, None).is_err());
    let target = dogfood_wiki_target(
        false,
        false,
        true,
        None,
        Some("spc_default".to_string()),
        None,
        Some("wik_parent".to_string()),
    )
    .unwrap()
    .unwrap();
    assert_eq!(target.0, "spc_default");
    assert_eq!(target.1.as_deref(), Some("wik_parent"));
    assert!(dogfood_wiki_target(
        true,
        true,
        true,
        Some("spc_1".to_string()),
        None,
        None,
        None,
    )
    .unwrap()
    .is_none());

    let markers = dogfood_readback_markers(
        "Demo title",
        "# Demo title\n\n- 创建独立 docx。\n```bash\nfeishu-bot dogfood publish\n```",
    );
    assert_eq!(markers[0], "Demo title");
    assert!(markers.contains(&"创建独立 docx。".to_string()));
    assert!(markers.contains(&"feishu-bot dogfood publish".to_string()));
    assert!(!markers.iter().any(|marker| marker.contains('`')));
}

#[test]
fn classifies_dogfood_probe_errors() {
    let missing_scope = classify_dogfood_error(
        r#"Feishu HTTP 400 Bad Request: {
          "code": 99991672,
          "error": {
            "log_id": "20260601ABC",
            "permission_violations": [
              { "subject": "calendar:calendar:readonly", "type": "action_scope_required" }
            ]
          },
          "msg": "Access denied"
        }"#,
    );
    assert_eq!(missing_scope["status"], "missing_scope");
    assert_eq!(missing_scope["log_id"], "20260601ABC");
    assert_eq!(
        missing_scope["missing_scopes"][0],
        "calendar:calendar:readonly"
    );

    let missing_user = classify_dogfood_error(
        "this Feishu API requires user_access_token; set FEISHU_USER_ACCESS_TOKEN",
    );
    assert_eq!(missing_user["status"], "missing_user_token");

    let helpdesk = classify_dogfood_error("helpdesk APIs require FEISHU_HELPDESK_TOKEN");
    assert_eq!(helpdesk["status"], "missing_helpdesk_config");
}

#[test]
fn dogfood_probe_outputs_remediation() {
    let probe = dogfood_probe_from_result(
        "calendar",
        "calendar.primary",
        "feishu-bot --json calendar primary",
        "GET /calendar/v4/calendars/primary",
        "calendar",
        probe_value(Err(anyhow!(
            r#"Feishu HTTP 400 Bad Request: {{
              "code": 99991672,
              "error": {{
                "log_id": "20260601ABC",
                "permission_violations": [
                  {{ "subject": "calendar:calendar:readonly", "type": "action_scope_required" }}
                ]
              }},
              "msg": "Access denied"
            }}"#
        ))),
        false,
        "cli_test",
    );
    assert_eq!(probe["status"], "missing_scope");
    assert_eq!(probe["remediation"]["action"], "grant_scopes");
    assert_eq!(
        probe["remediation"]["missing_scopes"][0],
        "calendar:calendar:readonly"
    );
    assert!(probe["remediation"]["grant_url"]
        .as_str()
        .unwrap()
        .contains("open.feishu.cn/app/cli_test/auth"));
    assert!(probe["remediation"]["browser_command"]
        .as_str()
        .unwrap()
        .contains("feishu-bot browser open --url"));

    let user_token_probe = dogfood_probe_from_result(
        "task",
        "task.my_tasks.list",
        "feishu-bot --json task list",
        "GET /task/v2/tasks",
        "task",
        probe_value(Err(anyhow!(
            "this Feishu API requires user_access_token; set FEISHU_USER_ACCESS_TOKEN"
        ))),
        false,
        "cli_test",
    );
    assert_eq!(user_token_probe["status"], "missing_user_token");
    assert!(user_token_probe["remediation"]["oauth_url_command"]
        .as_str()
        .unwrap()
        .contains("task:task:read"));
}

#[test]
fn builds_oauth_helpers_for_user_token_flow() {
    assert_eq!(
        code_challenge_s256("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );
    let scopes = resolve_oauth_scopes(vec![
        "offline_access auth:user.id:read".to_string(),
        "task:task:read".to_string(),
    ]);
    assert_eq!(
        scopes,
        vec!["offline_access", "auth:user.id:read", "task:task:read"]
    );
    let default_scopes = resolve_oauth_scopes(Vec::new());
    assert!(default_scopes.contains(&"offline_access".to_string()));

    let config = Config {
        app_id: "cli_test".to_string(),
        app_secret: "secret".to_string(),
        base_url: FEISHU_BASE_URL.to_string(),
        default_user_id: None,
        user_access_token: None,
        helpdesk_id: None,
        helpdesk_token: None,
        default_wiki_space_id: None,
        default_wiki_parent_node_token: None,
        default_doc_create_wiki: false,
        doc_base_url: "https://my.feishu.cn/docx".to_string(),
    };
    assert_eq!(
        oauth_authorize_url(&config),
        "https://accounts.feishu.cn/open-apis/authen/v1/authorize"
    );

    let masked = mask_oauth_token_response(&json!({
        "access_token": "u-1234567890",
        "refresh_token": "r-1234567890",
        "scope": "task:task:read",
    }));
    assert_ne!(masked["access_token"], "u-1234567890");
    assert_ne!(masked["refresh_token"], "r-1234567890");
}

#[test]
fn filters_dogfood_verify_modules() {
    assert!(dogfood_module_selected(
        &["calendar".to_string()],
        "calendar",
        "calendar.primary"
    ));
    assert!(dogfood_module_selected(
        &["calendar.primary".to_string()],
        "calendar",
        "calendar.primary"
    ));
    assert!(!dogfood_module_selected(
        &["task".to_string()],
        "calendar",
        "calendar.primary"
    ));
    assert!(dogfood_module_selected(&[], "calendar", "calendar.primary"));
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

#[test]
fn builds_vc_room_mget_body() {
    let body = build_vc_room_mget_body(VcRoomMgetArgs {
        room_ids: vec!["omm_1".to_string(), "omm_2".to_string()],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(body["room_ids"][0], "omm_1");
    assert_eq!(body["room_ids"][1], "omm_2");

    let empty = build_vc_room_mget_body(VcRoomMgetArgs {
        room_ids: vec![],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    });
    assert!(empty.is_err());
}

#[test]
fn builds_vc_reserve_meeting_and_recording_bodies() {
    let reserve = build_vc_reserve_apply_body(VcReserveApplyArgs {
        end_time: Some("1780300000".to_string()),
        owner_id: Some("ou_owner".to_string()),
        topic: Some("AI sync".to_string()),
        auto_record: Some(true),
        assign_hosts: vec!["ou_host".to_string()],
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(reserve["end_time"], "1780300000");
    assert_eq!(reserve["owner_id"], "ou_owner");
    assert_eq!(reserve["meeting_settings"]["topic"], "AI sync");
    assert_eq!(reserve["meeting_settings"]["auto_record"], true);
    assert_eq!(
        reserve["meeting_settings"]["assign_host_list"][0]["id"],
        "ou_host"
    );

    let invite = build_vc_meeting_invite_body(VcMeetingInviteArgs {
        meeting_id: "mtg_1".to_string(),
        users: vec!["ou_1".to_string(), "ou_2".to_string()],
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::User,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(invite["invitees"][1]["id"], "ou_2");
    assert_eq!(invite["invitees"][1]["user_type"], 1);

    let host = build_vc_meeting_set_host_body(VcMeetingSetHostArgs {
        meeting_id: "mtg_1".to_string(),
        user_id: "ou_host".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(host["host_user"]["id"], "ou_host");

    let recording = build_vc_recording_permission_body(VcRecordingSetPermissionArgs {
        meeting_id: "mtg_1".to_string(),
        users: vec!["ou_1".to_string()],
        chats: vec!["oc_1".to_string()],
        tenant: true,
        public: false,
        auth: ApiAuthArg::User,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(recording["permission_objects"][0]["type"], 1);
    assert_eq!(recording["permission_objects"][1]["type"], 2);
    assert_eq!(recording["permission_objects"][2]["type"], 3);
}

#[test]
fn extracts_minutes_tokens_from_urls() {
    assert_eq!(
        extract_minute_token("obcnq3b9jl72l83w4f14xxxx").unwrap(),
        "obcnq3b9jl72l83w4f14xxxx"
    );
    assert_eq!(
        extract_minute_token("https://sample.feishu.cn/minutes/obcnq3b9jl72l83w4f14xxxx?from=copy")
            .unwrap(),
        "obcnq3b9jl72l83w4f14xxxx"
    );
}

#[test]
fn builds_minutes_search_body() {
    let body = build_minutes_search_body(MinutesSearchArgs {
            query: Some("周会".to_string()),
            filter_json: Some(
                r#"{"create_time":{"start_time":"2026-05-01T00:00:00+08:00","end_time":"2026-05-31T23:59:59+08:00"}}"#
                    .to_string(),
            ),
            sorter: Some("create_time_desc".to_string()),
            page_size: 20,
            page_token: None,
            body_json: None,
            file: None,
            stdin: false,
            user_id_type: UserIdTypeArg::OpenId,
        })
        .unwrap();
    assert_eq!(body["query"], "周会");
    assert_eq!(
        body["filter"]["create_time"]["start_time"],
        "2026-05-01T00:00:00+08:00"
    );
    assert_eq!(body["sorter"], "create_time_desc");
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

#[test]
fn builds_okr_queries_and_validates_ids() {
    assert_eq!(OkrUserIdTypeArg::OpenId.as_api_value(), "open_id");
    assert_eq!(
        OkrUserIdTypeArg::PeopleAdminId.as_api_value(),
        "people_admin_id"
    );

    let mut query = build_okr_query(OkrUserIdTypeArg::PeopleAdminId, "zh_cn".to_string());
    push_query_repeated(
        &mut query,
        "okr_ids",
        vec!["okr_1".to_string(), "".to_string(), "okr_2".to_string()],
    );
    assert!(query.contains(&("user_id_type".to_string(), "people_admin_id".to_string())));
    assert!(query.contains(&("lang".to_string(), "zh_cn".to_string())));
    assert_eq!(
        query
            .iter()
            .filter(|(key, _)| key == "okr_ids")
            .collect::<Vec<_>>()
            .len(),
        2
    );

    assert!(validate_okr_id_list("period-id", &[], 10, false).is_ok());
    assert!(validate_okr_id_list("okr-id", &[], 10, true).is_err());
    assert!(validate_okr_id_list("okr-id", &vec!["x".to_string(); 11], 10, true).is_err());
}

#[test]
fn builds_attendance_queries_and_bodies() {
    assert_eq!(
        AttendanceEmployeeTypeArg::EmployeeId.as_api_value(),
        "employee_id"
    );
    assert_eq!(
        AttendanceEmployeeTypeArg::EmployeeNo.as_api_value(),
        "employee_no"
    );

    let page = attendance_page_query(AttendancePageArgs {
        page_size: 20,
        page_token: Some("next".to_string()),
    })
    .unwrap();
    assert!(page.contains(&("page_size".to_string(), "20".to_string())));
    assert!(page.contains(&("page_token".to_string(), "next".to_string())));
    assert!(attendance_page_query(AttendancePageArgs {
        page_size: 51,
        page_token: None,
    })
    .is_err());

    let schedule = build_attendance_schedule_body(AttendanceScheduleQueryArgs {
        user_ids: vec!["u1".to_string(), "".to_string()],
        check_date_from: Some(20260501),
        check_date_to: Some(20260531),
        employee_type: AttendanceEmployeeTypeArg::EmployeeId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(schedule["user_ids"][0], "u1");
    assert_eq!(schedule["check_date_from"], 20260501);

    let task = build_attendance_task_body(AttendanceTaskQueryArgs {
        user_ids: vec!["u1".to_string()],
        check_date_from: Some(20260501),
        check_date_to: Some(20260531),
        need_overtime_result: true,
        ignore_invalid_users: true,
        include_terminated_user: true,
        employee_type: AttendanceEmployeeTypeArg::EmployeeId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(task["need_overtime_result"], true);

    let flow = build_attendance_flow_query_body(AttendanceFlowQueryArgs {
        user_ids: vec!["u1".to_string()],
        check_time_from: Some("1760000000".to_string()),
        check_time_to: Some("1760086400".to_string()),
        include_terminated_user: false,
        employee_type: AttendanceEmployeeTypeArg::EmployeeId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(flow["check_time_from"], "1760000000");

    let delete = build_attendance_flow_delete_body(AttendanceFlowDeleteArgs {
        record_ids: vec!["rec_1".to_string(), "".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(delete["record_ids"][0], "rec_1");
    assert!(build_attendance_flow_delete_body(AttendanceFlowDeleteArgs {
        record_ids: vec!["rec".to_string(); 11],
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let stats = build_attendance_stats_body(AttendanceStatsQueryArgs {
        user_ids: vec!["u1".to_string()],
        operator_user_id: Some("admin_1".to_string()),
        start_date: Some(20260501),
        end_date: Some(20260531),
        locale: "zh".to_string(),
        stats_type: "daily".to_string(),
        need_history: true,
        current_group_only: true,
        employee_type: AttendanceEmployeeTypeArg::EmployeeId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(stats["user_id"], "admin_1");
    assert_eq!(stats["need_history"], true);
    assert_eq!(stats["current_group_only"], true);
}

#[test]
fn builds_mail_paths_auth_and_send_body() {
    assert_eq!(
        encode_path_segment("user@example.com/TUlH=="),
        "user%40example.com%2FTUlH%3D%3D"
    );
    assert!(mail_should_use_user(MailAuthArg::Auto, "me").unwrap());
    assert!(!mail_should_use_user(MailAuthArg::Auto, "user@example.com").unwrap());
    assert!(mail_should_use_user(MailAuthArg::Tenant, "me").is_err());

    let body = build_mail_send_body(MailMessageSendArgs {
        mailbox: "me".to_string(),
        to: vec!["a@example.com".to_string(), "".to_string()],
        cc: vec!["c@example.com".to_string()],
        bcc: vec![],
        subject: Some("hello".to_string()),
        text: Some("body".to_string()),
        html: None,
        raw_base64url: None,
        raw_file: None,
        dedupe_key: Some("k1".to_string()),
        from_address: Some("me@example.com".to_string()),
        from_name: Some("Me".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(body["subject"], "hello");
    assert_eq!(body["to"][0]["mail_address"], "a@example.com");
    assert_eq!(body["cc"][0]["mail_address"], "c@example.com");
    assert_eq!(body["body_plain_text"], "body");
    assert_eq!(body["dedupe_key"], "k1");
    assert_eq!(body["head_from"]["mail_address"], "me@example.com");

    assert!(build_mail_send_body(MailMessageSendArgs {
        mailbox: "me".to_string(),
        to: vec![],
        cc: vec![],
        bcc: vec![],
        subject: None,
        text: None,
        html: None,
        raw_base64url: None,
        raw_file: None,
        dedupe_key: None,
        from_address: None,
        from_name: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());
}

#[test]
fn builds_corehr_queries_and_bodies() {
    assert_eq!(
        CorehrUserIdTypeArg::PeopleCorehrId.as_api_value(),
        "people_corehr_id"
    );
    assert_eq!(
        CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId.as_api_value(),
        "people_corehr_department_id"
    );

    let page = corehr_page_query(20, Some("next".to_string())).unwrap();
    assert!(page.contains(&("page_size".to_string(), "20".to_string())));
    assert!(page.contains(&("page_token".to_string(), "next".to_string())));
    assert!(corehr_page_query(101, None).is_err());

    let department = build_corehr_department_search_body(CorehrDepartmentSearchArgs {
        page_size: 20,
        page_token: None,
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec!["dept_1".to_string()],
        names: vec!["研发".to_string()],
        manager_ids: vec!["emp_1".to_string()],
        parent_department_id: Some("parent_1".to_string()),
        codes: vec!["D001".to_string()],
        fields: vec!["department_name".to_string()],
        active: Some(true),
        get_all_children: true,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(department["department_id_list"][0], "dept_1");
    assert_eq!(department["name_list"][0], "研发");
    assert_eq!(department["active"], true);
    assert_eq!(department["get_all_children"], true);

    let department_get = build_corehr_department_get_body(CorehrDepartmentGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec!["dept_1".to_string()],
        names: vec![],
        fields: vec!["version_id".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(department_get["department_id_list"][0], "dept_1");
    assert_eq!(department_get["fields"][0], "version_id");
    assert!(build_corehr_department_get_body(CorehrDepartmentGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec![],
        names: vec![],
        fields: vec![],
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let job = build_corehr_job_batch_get_body(CorehrJobBatchGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        job_ids: vec!["job_1".to_string()],
        job_codes: vec!["JP001".to_string()],
        fields: vec!["job_name".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(job["job_ids"][0], "job_1");
    assert_eq!(job["job_codes"][0], "JP001");

    let job_data = build_corehr_job_data_query_body(CorehrJobDataQueryArgs {
        page_size: 20,
        page_token: None,
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId,
        employment_ids: vec!["emp_1".to_string()],
        department_id: Some("dept_1".to_string()),
        data_date: Some("2026-05-31".to_string()),
        effective_date_start: None,
        effective_date_end: None,
        all_version: true,
        primary_job_data: Some(true),
        assignment_start_reasons: vec!["onboarding".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(job_data["employment_ids"][0], "emp_1");
    assert_eq!(job_data["get_all_version"], true);
    assert_eq!(job_data["primary_job_data"], true);

    let process = build_corehr_process_list_query(CorehrProcessListArgs {
        page_size: 10,
        page_token: None,
        statuses: vec![1, 9],
        modify_time_from: "1760000000000".to_string(),
        modify_time_to: "1760003600000".to_string(),
        flow_definition_id: Some("flow_1".to_string()),
    })
    .unwrap();
    assert_eq!(
        process
            .iter()
            .filter(|(key, _)| key == "statuses")
            .collect::<Vec<_>>()
            .len(),
        2
    );
    assert!(build_corehr_process_list_query(CorehrProcessListArgs {
        page_size: 10,
        page_token: None,
        statuses: vec![3],
        modify_time_from: "1760000000000".to_string(),
        modify_time_to: "1760003600000".to_string(),
        flow_definition_id: None,
    })
    .is_err());
}

#[test]
fn builds_directory_queries_and_bodies() {
    assert_eq!(
        DirectoryEmployeeIdTypeArg::EmployeeId.as_api_value(),
        "employee_id"
    );
    assert_eq!(
        DirectoryDepartmentIdTypeArg::DepartmentId.as_api_value(),
        "department_id"
    );

    let query = directory_query(
        DirectoryEmployeeIdTypeArg::UnionId,
        DirectoryDepartmentIdTypeArg::DepartmentId,
    );
    assert!(query.contains(&("employee_id_type".to_string(), "union_id".to_string())));
    assert!(query.contains(&(
        "department_id_type".to_string(),
        "department_id".to_string()
    )));

    let search = build_directory_employee_search_body(DirectoryEmployeeSearchArgs {
        query: Some("user@example.com".to_string()),
        page_size: 10,
        page_token: Some("next".to_string()),
        fields: vec![
            "base_info.employee_id".to_string(),
            "base_info.email".to_string(),
        ],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(search["query"], "user@example.com");
    assert_eq!(search["page_request"]["page_size"], 10);
    assert_eq!(search["page_request"]["page_token"], "next");
    assert_eq!(search["required_fields"][1], "base_info.email");

    let default_fields = build_directory_employee_search_body(DirectoryEmployeeSearchArgs {
        query: Some("张三".to_string()),
        page_size: 20,
        page_token: None,
        fields: vec![],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        default_fields["required_fields"][0],
        "base_info.employee_id"
    );
    assert_eq!(default_fields["required_fields"][1], "base_info.name.name");

    let mget = build_directory_employee_mget_body(DirectoryEmployeeMgetArgs {
        employee_ids: vec!["ou_1".to_string(), "".to_string(), "ou_2".to_string()],
        fields: vec!["work_info.job_title".to_string()],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(mget["employee_ids"][0], "ou_1");
    assert_eq!(mget["employee_ids"][1], "ou_2");
    assert_eq!(mget["required_fields"][0], "work_info.job_title");
    assert!(
        build_directory_employee_mget_body(DirectoryEmployeeMgetArgs {
            employee_ids: vec![],
            fields: vec![],
            employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
            department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
            auth: DirectoryAuthArg::Tenant,
            body_json: None,
            file: None,
            stdin: false,
        })
        .is_err()
    );

    let filter = build_directory_employee_filter_body(DirectoryEmployeeFilterArgs {
        conditions: vec![
            "base_info.email=eq=\"user@example.com\"".to_string(),
            "base_info.is_resigned=eq=false".to_string(),
        ],
        filter_json: None,
        page_size: 5,
        page_token: None,
        fields: vec!["base_info.name.name".to_string()],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        filter["filter"]["conditions"][0]["field"],
        "base_info.email"
    );
    assert_eq!(filter["filter"]["conditions"][0]["operator"], "eq");
    assert_eq!(
        filter["filter"]["conditions"][0]["value"],
        "user@example.com"
    );
    assert_eq!(filter["filter"]["conditions"][1]["value"], false);
}

#[test]
fn builds_helpdesk_queries_bodies_and_auth() {
    assert_eq!(HelpdeskReceiveTypeArg::Chat.as_api_value(), "chat");
    assert_eq!(HelpdeskReceiveTypeArg::User.as_api_value(), "user");

    let query = helpdesk_ticket_list_query(HelpdeskTicketListArgs {
        ticket_id: Some("t1".to_string()),
        agent_id: Some("ou_agent".to_string()),
        closed_by_id: None,
        ticket_type: Some(2),
        channel: None,
        solved: Some(1),
        score: None,
        status_list: vec![2, 5],
        guest_name: Some("张三".to_string()),
        guest_id: None,
        tags: vec!["urgent".to_string(), "".to_string()],
        page: 2,
        page_size: 50,
        create_time_start: Some(1760000000000),
        create_time_end: Some(1760003600000),
        update_time_start: None,
        update_time_end: None,
    })
    .unwrap();
    assert!(query.contains(&("page".to_string(), "2".to_string())));
    assert!(query.contains(&("page_size".to_string(), "50".to_string())));
    assert!(query.contains(&("type".to_string(), "2".to_string())));
    assert_eq!(
        query.iter().filter(|(key, _)| key == "status_list").count(),
        2
    );
    assert!(query.contains(&("tags".to_string(), "urgent".to_string())));
    assert!(helpdesk_page_number_query(0, 20, 200).is_err());
    assert!(helpdesk_page_number_query(1, 201, 200).is_err());

    let start = build_helpdesk_service_start_body(HelpdeskServiceStartArgs {
        open_id: Some("ou_user".to_string()),
        human_service: true,
        appointed_agents: vec!["ou_agent".to_string()],
        customized_info: Some("from cli".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(start["open_id"], "ou_user");
    assert_eq!(start["human_service"], true);
    assert_eq!(start["appointed_agents"][0], "ou_agent");
    assert!(build_helpdesk_service_start_body(HelpdeskServiceStartArgs {
        open_id: Some("ou_user".to_string()),
        human_service: false,
        appointed_agents: vec!["ou_agent".to_string()],
        customized_info: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let message = build_helpdesk_message_send_body(HelpdeskMessageSendArgs {
        receiver_id: Some("ou_user".to_string()),
        msg_type: "text".to_string(),
        text: Some("hello".to_string()),
        content_json: None,
        receive_type: HelpdeskReceiveTypeArg::User,
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(message["msg_type"], "text");
    assert_eq!(message["receiver_id"], "ou_user");
    assert_eq!(message["receive_type"], "user");
    let content: Value = serde_json::from_str(message["content"].as_str().unwrap()).unwrap();
    assert_eq!(content["text"], "hello");

    let api = FeishuClient::new(Config {
        app_id: "cli_xxx".to_string(),
        app_secret: "secret".to_string(),
        base_url: FEISHU_BASE_URL.to_string(),
        default_user_id: None,
        user_access_token: None,
        helpdesk_id: Some("12345".to_string()),
        helpdesk_token: Some("ht-token".to_string()),
        default_wiki_space_id: None,
        default_wiki_parent_node_token: None,
        default_doc_create_wiki: false,
        doc_base_url: "https://my.feishu.cn/docx".to_string(),
    });
    assert_eq!(api.helpdesk_auth_header().unwrap(), "MTIzNDU6aHQtdG9rZW4=");
}

#[test]
fn builds_hire_queries_and_bodies() {
    assert_eq!(
        HireUserIdTypeArg::PeopleAdminId.as_api_value(),
        "people_admin_id"
    );
    assert_eq!(
        HireJobLevelIdTypeArg::PeopleAdminJobLevelId.as_api_value(),
        "people_admin_job_level_id"
    );
    assert_eq!(
        HireJobFamilyIdTypeArg::JobFamilyId.as_api_value(),
        "job_family_id"
    );
    assert_eq!(
        HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId.as_api_value(),
        "employee_type_enum_id"
    );

    let jobs = hire_job_list_query(HireJobListArgs {
        update_start_time: Some("1760000000000".to_string()),
        update_end_time: Some("1760003600000".to_string()),
        page_size: 20,
        page_token: Some("tok".to_string()),
        user_id_type: HireUserIdTypeArg::OpenId,
        department_id_type: DepartmentIdTypeArg::OpenDepartmentId,
        job_level_id_type: HireJobLevelIdTypeArg::PeopleAdminJobLevelId,
        job_family_id_type: HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId,
    })
    .unwrap();
    assert!(jobs.contains(&("page_size".to_string(), "20".to_string())));
    assert!(jobs.contains(&("page_token".to_string(), "tok".to_string())));
    assert!(jobs.contains(&("user_id_type".to_string(), "open_id".to_string())));
    assert!(hire_page_query(0, 20, None).is_err());
    assert!(hire_page_query(21, 20, None).is_err());

    let talents = hire_talent_list_query(HireTalentListArgs {
        keyword: Some("张三 and 产品".to_string()),
        update_start_time: None,
        update_end_time: None,
        page_size: 10,
        sort_by: Some(2),
        page_token: None,
        user_id_type: HireUserIdTypeArg::PeopleAdminId,
        query_option: Some("ignore_empty_error".to_string()),
    })
    .unwrap();
    assert!(talents.contains(&("keyword".to_string(), "张三 and 产品".to_string())));
    assert!(talents.contains(&("sort_by".to_string(), "2".to_string())));
    assert!(talents.contains(&("query_option".to_string(), "ignore_empty_error".to_string())));

    let apps = hire_application_list_query(HireApplicationListArgs {
        process_id: Some("p1".to_string()),
        stage_id: Some("s1".to_string()),
        talent_id: Some("t1".to_string()),
        active_status: Some("1".to_string()),
        job_id: Some("j1".to_string()),
        lock_status: vec![1, 3],
        page_token: None,
        page_size: 200,
        update_start_time: None,
        update_end_time: None,
    })
    .unwrap();
    assert_eq!(
        apps.iter().filter(|(key, _)| key == "lock_status").count(),
        2
    );
    assert!(apps.contains(&("page_size".to_string(), "200".to_string())));

    let detail = hire_application_detail_query(HireApplicationDetailArgs {
        application_id: "a1".to_string(),
        user_id_type: HireUserIdTypeArg::OpenId,
        department_id_type: DepartmentIdTypeArg::DepartmentId,
        job_level_id_type: HireJobLevelIdTypeArg::JobLevelId,
        job_family_id_type: HireJobFamilyIdTypeArg::JobFamilyId,
        employee_type_id_type: HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId,
        options: vec!["with_job".to_string(), "with_talent".to_string()],
    });
    assert!(detail.contains(&(
        "department_id_type".to_string(),
        "department_id".to_string()
    )));
    assert_eq!(detail.iter().filter(|(key, _)| key == "options").count(), 2);

    let open = build_hire_job_open_body(HireJobOpenArgs {
        job_id: "j1".to_string(),
        is_never_expired: Some(false),
        expiry_time: Some(1830259120000),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(open["is_never_expired"], false);
    assert_eq!(open["expiry_time"], 1830259120000_i64);
    assert!(build_hire_job_open_body(HireJobOpenArgs {
        job_id: "j1".to_string(),
        is_never_expired: Some(false),
        expiry_time: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let talent = build_hire_talent_create_body(HireTalentCreateArgs {
        name: Some("张三".to_string()),
        email: Some("zhangsan@example.com".to_string()),
        mobile: None,
        mobile_country_code: Some("CN_1".to_string()),
        current_city_code: Some("CT_11".to_string()),
        resume_source_id: Some("10000".to_string()),
        folder_ids: vec!["f1".to_string(), "".to_string()],
        creator_id: None,
        creator_account_type: Some(3),
        resume_attachment_id: None,
        user_id_type: HireUserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(talent["basic_info"]["name"], "张三");
    assert_eq!(talent["basic_info"]["email"], "zhangsan@example.com");
    assert_eq!(talent["folder_id_list"][0], "f1");
    assert_eq!(talent["creator_account_type"], 3);

    let location = build_hire_location_query_body(HireLocationQueryArgs {
        location_type: Some(1),
        code_list: vec!["CN_1".to_string()],
        page_size: 100,
        page_token: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(location["location_type"], 1);
    assert_eq!(location["code_list"][0], "CN_1");
}

#[test]
fn builds_calendar_event_body() {
    let body = build_calendar_event_create_body(CalendarEventCreateArgs {
        calendar_id: "cal_1".to_string(),
        summary: Some("sync".to_string()),
        description: Some("notes".to_string()),
        start_ts: Some("1760000000".to_string()),
        end_ts: Some("1760003600".to_string()),
        time_zone: "Asia/Shanghai".to_string(),
        idempotency_key: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(body["summary"], "sync");
    assert_eq!(body["start_time"]["timestamp"], "1760000000");
    assert_eq!(body["end_time"]["timezone"], "Asia/Shanghai");

    let attendees = build_calendar_attendee_add_body(CalendarAttendeeAddArgs {
        calendar_id: "cal_1".to_string(),
        event_id: "evt_1".to_string(),
        users: vec!["ou_1".to_string()],
        chats: vec!["oc_1".to_string()],
        optional: true,
        attendees_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(attendees["attendees"][0]["type"], "user");
    assert_eq!(attendees["attendees"][0]["is_optional"], true);
    assert_eq!(attendees["attendees"][1]["chat_id"], "oc_1");

    let delete = build_calendar_attendee_delete_body(CalendarAttendeeDeleteArgs {
        calendar_id: "cal_1".to_string(),
        event_id: "evt_1".to_string(),
        attendee_ids: vec!["att_1".to_string()],
        delete_ids: vec!["ou_1".to_string()],
        attendee_ids_json: None,
        delete_ids_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(delete["attendee_ids"][0], "att_1");
    assert_eq!(delete["delete_ids"][0], "ou_1");

    let freebusy = build_calendar_freebusy_list_body(CalendarFreebusyListArgs {
        time_min: "2026-06-01T09:00:00+08:00".to_string(),
        time_max: "2026-06-01T18:00:00+08:00".to_string(),
        user_id: Some("ou_1".to_string()),
        room_id: None,
        include_external_calendar: Some(false),
        only_busy: Some(true),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(freebusy["user_id"], "ou_1");
    assert_eq!(freebusy["include_external_calendar"], false);
    assert_eq!(freebusy["only_busy"], true);

    let batch = build_calendar_freebusy_batch_body(CalendarFreebusyBatchArgs {
        time_min: "2026-06-01T09:00:00+08:00".to_string(),
        time_max: "2026-06-01T18:00:00+08:00".to_string(),
        user_ids: vec!["ou_1".to_string(), "ou_2".to_string()],
        user_ids_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(batch["user_ids"][0], "ou_1");
    assert_eq!(batch["user_ids"][1], "ou_2");
}

#[test]
fn builds_approval_bodies() {
    let action = build_approval_task_action_body(ApprovalTaskActionArgs {
        approval_code: "appr_1".to_string(),
        instance_code: "inst_1".to_string(),
        user_id: "ou_1".to_string(),
        task_id: "task_1".to_string(),
        comment: Some("OK".to_string()),
        form_json: Some(r#"[{"id":"field_1","type":"input","value":"done"}]"#.to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(action["approval_code"], "appr_1");
    assert_eq!(action["task_id"], "task_1");
    assert_eq!(
        serde_json::from_str::<Value>(action["form"].as_str().unwrap()).unwrap()[0]["id"],
        "field_1"
    );

    let add_sign = build_approval_task_add_sign_body(ApprovalTaskAddSignArgs {
        approval_code: "appr_1".to_string(),
        instance_code: "inst_1".to_string(),
        user_id: "ou_1".to_string(),
        task_id: "task_1".to_string(),
        add_sign_user_ids: vec!["ou_2".to_string(), "ou_3".to_string()],
        add_sign_type: Some(3),
        approval_method: None,
        comment: Some("join".to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(add_sign["add_sign_user_ids"][1], "ou_3");
    assert_eq!(add_sign["add_sign_type"], 3);

    let rollback = build_approval_task_rollback_body(ApprovalTaskRollbackArgs {
        user_id: "ou_1".to_string(),
        task_id: "task_1".to_string(),
        task_def_key_list: vec!["START".to_string()],
        reason: Some("revise".to_string()),
        extra: None,
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(rollback["task_def_key_list"][0], "START");

    let search = build_approval_search_body(
        ApprovalSearchArgs {
            page_size: 10,
            page_token: None,
            user_id_type: UserIdTypeArg::OpenId,
            user_id: Some("ou_1".to_string()),
            approval_code: Some("appr_1".to_string()),
            instance_code: None,
            instance_external_id: None,
            group_external_id: None,
            instance_title: None,
            instance_status: None,
            instance_start_time_from: None,
            instance_start_time_to: None,
            task_title: None,
            task_status: Some("PENDING".to_string()),
            task_status_list: vec!["APPROVED".to_string()],
            task_start_time_from: None,
            task_start_time_to: None,
            locale: Some("zh-CN".to_string()),
            order: Some(2),
            body_json: None,
            file: None,
            stdin: false,
        },
        "approval task search body",
    )
    .unwrap();
    assert_eq!(search["approval_code"], "appr_1");
    assert_eq!(search["task_status_list"][0], "APPROVED");
}

use super::*;
use std::collections::HashMap;

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
fn validates_drive_upload_inputs() {
    assert_eq!(
        drive_upload_file_name(Path::new("/tmp/report.pdf"), None).unwrap(),
        "report.pdf"
    );
    assert_eq!(
        drive_upload_file_name(Path::new("/tmp/report.pdf"), Some("out.pdf".to_string())).unwrap(),
        "out.pdf"
    );
    assert!(validate_drive_upload_size(0).is_err());
    assert!(validate_drive_upload_size(20 * 1024 * 1024).is_ok());
    assert!(validate_drive_upload_size(20 * 1024 * 1024 + 1).is_err());

    let body = build_drive_upload_prepare_body(
        "large.mov".to_string(),
        "explorer".to_string(),
        "fld_1".to_string(),
        20 * 1024 * 1024 + 1,
    )
    .unwrap();
    assert_eq!(body["file_name"], "large.mov");
    assert_eq!(body["parent_type"], "explorer");
    assert_eq!(body["parent_node"], "fld_1");
    assert_eq!(body["size"], 20 * 1024 * 1024 + 1);
    assert!(build_drive_upload_prepare_body(
        "empty.txt".to_string(),
        "explorer".to_string(),
        String::new(),
        0,
    )
    .is_err());
}

#[test]
fn parses_base_urls_for_ai() {
    let parsed = parse_base_reference(
        "https://example.feishu.cn/base/bascn123456789?table=tblABC&view=vewXYZ&record=rec1",
    )
    .unwrap();
    assert_eq!(parsed["app_token"], "bascn123456789");
    assert_eq!(parsed["table_id"], "tblABC");
    assert_eq!(parsed["view_id"], "vewXYZ");
    assert_eq!(parsed["record_id"], "rec1");
    assert_eq!(parsed["is_wiki_url"], false);

    let wiki =
        parse_base_reference("https://example.feishu.cn/wiki/WIKTOKEN123?table=tblABC").unwrap();
    assert_eq!(wiki["wiki_node_token"], "WIKTOKEN123");
    assert_eq!(wiki["table_id"], "tblABC");
    assert_eq!(wiki["is_wiki_url"], true);
    assert!(wiki.get("app_token").is_none());

    let raw = parse_base_reference("bascnRawToken").unwrap();
    assert_eq!(raw["input_kind"], "app_token");
    assert_eq!(raw["app_token"], "bascnRawToken");

    let app =
        parse_base_reference("https://example.feishu.cn/app/appToken123?pageId=pge123").unwrap();
    assert_eq!(app["app_token"], "appToken123");
    assert_eq!(app["page_id"], "pge123");
}

#[test]
fn parses_task_auth_defaults_for_ai() {
    let list = Cli::parse_from(["feishu", "task", "list"]);
    match list.command {
        Commands::Task(TaskCommand::List(args)) => {
            assert!(matches!(args.auth, ApiAuthArg::User));
        }
        _ => panic!("expected task list"),
    }

    let get = Cli::parse_from([
        "feishu", "task", "get", "--guid", "task_1", "--auth", "user",
    ]);
    match get.command {
        Commands::Task(TaskCommand::Get(args)) => {
            assert!(matches!(args.auth, ApiAuthArg::User));
        }
        _ => panic!("expected task get"),
    }
}

#[test]
fn parses_office_workflow_commands_for_ai() {
    let list = Cli::parse_from(["feishu", "office", "list", "--details"]);
    match list.command {
        Commands::Office(OfficeCommand::List(args)) => {
            assert!(args.details);
        }
        _ => panic!("expected office list"),
    }

    let bootstrap = Cli::parse_from([
        "feishu",
        "office",
        "bootstrap",
        "--project",
        "AI Project",
        "--user",
        "ou_1",
        "--space-id",
        "spc_1",
        "--send-summary",
        "--dry-run",
    ]);
    match bootstrap.command {
        Commands::Office(OfficeCommand::Bootstrap(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.users, vec!["ou_1"]);
            assert_eq!(args.space_id.as_deref(), Some("spc_1"));
            assert!(args.send_summary);
            assert!(args.dry_run);
            assert!(!args.skip_wiki);
        }
        _ => panic!("expected office bootstrap"),
    }

    let report = Cli::parse_from([
        "feishu",
        "--json",
        "office",
        "report",
        "--project",
        "AI Project",
        "--title",
        "HTML Demo",
        "--content-type",
        "html",
        "--file",
        "demo.html",
        "--base-record",
        "--pin",
        "--dry-run",
    ]);
    match report.command {
        Commands::Office(OfficeCommand::Report(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.title, "HTML Demo");
            assert!(matches!(args.content_type, ContentTypeArg::Html));
            assert_eq!(args.file.unwrap(), PathBuf::from("demo.html"));
            assert!(args.base_record);
            assert!(args.pin);
            assert!(args.dry_run);
        }
        _ => panic!("expected office report"),
    }

    let progress = Cli::parse_from([
        "feishu",
        "office",
        "progress",
        "--project",
        "AI Project",
        "--title",
        "Progress",
        "--status",
        "doing",
        "--summary",
        "Current status",
        "--wiki-report",
        "--pin",
    ]);
    match progress.command {
        Commands::Office(OfficeCommand::Progress(args)) => {
            assert_eq!(args.project, "AI Project");
            assert_eq!(args.title, "Progress");
            assert_eq!(args.status, "doing");
            assert_eq!(args.summary.as_deref(), Some("Current status"));
            assert!(args.wiki_report);
            assert!(args.pin);
            assert!(!args.no_base_record);
        }
        _ => panic!("expected office progress"),
    }

    let inbox = Cli::parse_from([
        "feishu",
        "office",
        "inbox",
        "--project",
        "AI Project",
        "--from-now",
        "--reply-text",
        "Received",
    ]);
    match inbox.command {
        Commands::Office(OfficeCommand::Inbox(args)) => {
            assert_eq!(args.project, "AI Project");
            assert!(args.from_now);
            assert_eq!(args.ack_emoji, "OK");
            assert_eq!(args.reply_text.as_deref(), Some("Received"));
            assert!(!args.no_mark_seen);
        }
        _ => panic!("expected office inbox"),
    }

    let cleanup = Cli::parse_from([
        "feishu",
        "office",
        "cleanup",
        "--project",
        "AI Project",
        "--dry-run",
    ]);
    match cleanup.command {
        Commands::Office(OfficeCommand::Cleanup(args)) => {
            assert_eq!(args.project, "AI Project");
            assert!(args.dry_run);
            assert!(!args.confirm);
        }
        _ => panic!("expected office cleanup"),
    }
}

#[test]
fn parses_setup_automation_commands_for_ai() {
    let plan = Cli::parse_from(["feishu", "setup", "plan", "--group", "office"]);
    match plan.command {
        Commands::Setup(SetupCommand::Plan(args)) => {
            assert_eq!(args.groups, vec!["office"]);
            assert!(matches!(args.token_type, ApiAuthArg::Tenant));
        }
        _ => panic!("expected setup plan"),
    }

    let open = Cli::parse_from([
        "feishu",
        "setup",
        "open-scopes",
        "--group",
        "wiki",
        "--browser",
    ]);
    match open.command {
        Commands::Setup(SetupCommand::OpenScopes(args)) => {
            assert_eq!(args.groups, vec!["wiki"]);
            assert!(args.browser);
            assert!(!args.system_browser);
        }
        _ => panic!("expected setup open-scopes"),
    }

    let wiki_bot = Cli::parse_from([
        "feishu",
        "setup",
        "wiki-bot",
        "--space-id",
        "spc_1",
        "--auth",
        "user",
    ]);
    match wiki_bot.command {
        Commands::Setup(SetupCommand::WikiBot(args)) => {
            assert_eq!(args.space_id.as_deref(), Some("spc_1"));
            assert!(matches!(args.auth, ApiAuthArg::User));
        }
        _ => panic!("expected setup wiki-bot"),
    }

    let auto = Cli::parse_from(["feishu", "setup", "auto", "--open-browser"]);
    match auto.command {
        Commands::Setup(SetupCommand::Auto(args)) => {
            assert!(args.open_browser);
            assert!(!args.no_wiki_bot);
        }
        _ => panic!("expected setup auto"),
    }

    let quickstart = Cli::parse_from([
        "feishu",
        "setup",
        "quickstart",
        "--open-browser",
        "--system-browser",
        "--project",
        "AI Project",
    ]);
    match quickstart.command {
        Commands::Setup(SetupCommand::Quickstart(args)) => {
            assert!(args.open_browser);
            assert!(args.system_browser);
            assert_eq!(args.project, "AI Project");
            assert!(!args.no_wiki_bot);
        }
        _ => panic!("expected setup quickstart"),
    }
}

#[test]
fn serializes_office_project_registry() {
    assert_eq!(office_project_key("  AI Project  ").unwrap(), "AI Project");
    assert!(office_project_key("   ").is_err());

    let mut registry = OfficeProjectRegistry::default();
    registry.projects.insert(
        "AI Project".to_string(),
        OfficeProject {
            project: "AI Project".to_string(),
            name: "AI Project".to_string(),
            chat_id: Some("oc_1".to_string()),
            wiki_space_id: Some("spc_1".to_string()),
            wiki_index_node_token: Some("wik_1".to_string()),
            wiki_index_obj_token: Some("docx_1".to_string()),
            base_app_token: Some("base_1".to_string()),
            base_table_id: Some("tbl_1".to_string()),
            ..OfficeProject::default()
        },
    );
    let text = serde_json::to_string(&registry).unwrap();
    assert!(text.contains("AI Project"));
    assert!(text.contains("base_1"));
    let parsed: OfficeProjectRegistry = serde_json::from_str(&text).unwrap();
    assert_eq!(
        parsed.projects["AI Project"]
            .wiki_index_obj_token
            .as_deref(),
        Some("docx_1")
    );
}

#[test]
fn builds_base_field_list_query() {
    let query = build_base_field_list_query(&BaseFieldListArgs {
        app_token: "app_token".to_string(),
        table_id: "tbl123".to_string(),
        page_size: 50,
        page_token: Some("next".to_string()),
        view_id: Some("vew123".to_string()),
        text_field_as_array: true,
    });

    assert_eq!(
        query,
        vec![
            ("page_size".to_string(), "50".to_string()),
            ("page_token".to_string(), "next".to_string()),
            ("view_id".to_string(), "vew123".to_string()),
            ("text_field_as_array".to_string(), "true".to_string()),
        ]
    );
}

#[test]
fn builds_media_and_import_helpers() {
    assert!(validate_upload_size(1, 10, "test").is_ok());
    assert!(validate_upload_size(0, 10, "test").is_err());
    assert!(validate_upload_size(11, 10, "test").is_err());

    let extra = build_drive_media_extra(None, Some("docx_token".to_string())).expect("route extra");
    assert_eq!(
        extra,
        Some(r#"{"drive_route_token":"docx_token"}"#.to_string())
    );
    assert!(
        build_drive_media_extra(Some("{}".to_string()), Some("docx_token".to_string())).is_err()
    );

    assert_eq!(BaseMediaKindArg::Image.parent_type(), "bitable_image");
    assert_eq!(BaseMediaKindArg::File.parent_type(), "bitable_file");
    let bitable_extra = build_base_media_extra(
        None,
        Some("tbl_1".to_string()),
        Some("fld_1".to_string()),
        Some("rec_1".to_string()),
        &["file_1".to_string(), "file_2".to_string()],
    )
    .unwrap()
    .unwrap();
    let bitable_extra: Value = serde_json::from_str(&bitable_extra).unwrap();
    assert_eq!(bitable_extra["bitablePerm"]["tableId"], "tbl_1");
    assert_eq!(
        bitable_extra["bitablePerm"]["attachments"]["fld_1"]["rec_1"][0],
        "file_1"
    );
    assert!(build_base_media_extra(
        Some("{}".to_string()),
        Some("tbl_1".to_string()),
        None,
        None,
        &["file_1".to_string()]
    )
    .is_err());
    assert!(build_base_media_extra(
        None,
        Some("tbl_1".to_string()),
        None,
        Some("rec_1".to_string()),
        &["file_1".to_string()]
    )
    .is_err());
    let field_value = build_base_media_field_value(
        vec!["file_1".to_string(), "file_2".to_string()],
        Some("附件".to_string()),
    )
    .unwrap();
    assert_eq!(field_value["data"]["value"][0]["file_token"], "file_1");
    assert_eq!(
        field_value["data"]["fields"]["附件"][1]["file_token"],
        "file_2"
    );

    assert_eq!(
        infer_upload_extension(Path::new("/tmp/page.html"), "page.html", None).unwrap(),
        "html"
    );
    assert_eq!(
        infer_upload_extension(
            Path::new("/tmp/no-extension"),
            "source.markdown",
            Some(".md".to_string())
        )
        .unwrap(),
        "md"
    );

    let body = build_drive_import_task_body(
        "box_token".to_string(),
        "html".to_string(),
        "docx".to_string(),
        Some("HTML Preview".to_string()),
        "".to_string(),
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(body["file_token"], "box_token");
    assert_eq!(body["file_extension"], "html");
    assert_eq!(body["type"], "docx");
    assert_eq!(body["file_name"], "HTML Preview");
    assert_eq!(body["point"]["mount_type"], 1);
    assert_eq!(body["point"]["mount_key"], "");

    let export = build_drive_export_task_body(
        "docx_token".to_string(),
        "docx".to_string(),
        "pdf".to_string(),
        None,
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(export["token"], "docx_token");
    assert_eq!(export["type"], "docx");
    assert_eq!(export["file_extension"], "pdf");
}

#[test]
fn builds_drive_permission_bodies() {
    let public = build_drive_public_update_body(DrivePermissionPublicUpdateArgs {
        token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        external_access: Some(false),
        invite_external: Some(true),
        security_entity: Some("anyone_can_view".to_string()),
        comment_entity: None,
        share_entity: Some("only_full_access".to_string()),
        link_share_entity: Some("tenant_readable".to_string()),
    })
    .unwrap();
    assert_eq!(public["external_access"], false);
    assert_eq!(public["invite_external"], true);
    assert_eq!(public["security_entity"], "anyone_can_view");
    assert_eq!(public["share_entity"], "only_full_access");

    let add = build_drive_member_add_body(DrivePermissionMemberAddArgs {
        token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        member_id: "ou_1".to_string(),
        member_type: "openid".to_string(),
        perm: "edit".to_string(),
        perm_type: "container".to_string(),
        collaborator_type: "user".to_string(),
        need_notification: false,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(add["member_id"], "ou_1");
    assert_eq!(add["perm"], "edit");

    let list_query = drive_permission_member_list_query(&DrivePermissionMemberListArgs {
        token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        page_size: 25,
        page_token: Some("next".to_string()),
        member_type: Some("openid".to_string()),
    })
    .unwrap();
    assert!(list_query.contains(&("type".to_string(), "docx".to_string())));
    assert!(list_query.contains(&("page_size".to_string(), "25".to_string())));
    assert!(list_query.contains(&("page_token".to_string(), "next".to_string())));
    assert!(list_query.contains(&("member_type".to_string(), "openid".to_string())));

    let query = drive_permission_member_query("docx", true, Some("openid"));
    assert!(query.contains(&("type".to_string(), "docx".to_string())));
    assert!(query.contains(&("need_notification".to_string(), "true".to_string())));
    assert!(query.contains(&("member_type".to_string(), "openid".to_string())));
}

#[test]
fn builds_drive_comment_version_subscription_inputs() {
    let create = build_drive_comment_create_body(DriveCommentCreateArgs {
        file_token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        text: Some("需要复核".to_string()),
        docs_links: vec!["https://example.feishu.cn/docx/docx_1".to_string()],
        mention_users: vec!["ou_1".to_string()],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    let elements = create["reply_list"]["replies"][0]["content"]["elements"]
        .as_array()
        .unwrap();
    assert_eq!(elements.len(), 3);
    assert_eq!(elements[0]["text_run"]["text"], "需要复核");
    assert_eq!(
        elements[1]["docs_link"]["url"],
        "https://example.feishu.cn/docx/docx_1"
    );
    assert_eq!(elements[2]["person"]["user_id"], "ou_1");

    let batch = build_drive_comment_batch_body(DriveCommentBatchGetArgs {
        file_token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        comment_ids: vec!["c1".to_string()],
        need_reaction: true,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(batch["comment_ids"][0], "c1");
    assert_eq!(batch["need_reaction"], true);

    let version = build_drive_version_create_body(DriveVersionCreateArgs {
        file_token: "docx_1".to_string(),
        name: Some("AI 修订版".to_string()),
        obj_type: "docx".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(version["name"], "AI 修订版");
    assert_eq!(version["obj_type"], "docx");
    assert!(build_drive_version_create_body(DriveVersionCreateArgs {
        file_token: "docx_1".to_string(),
        name: Some("bad".to_string()),
        obj_type: "file".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .is_err());

    let subscription = build_drive_subscription_create_body(DriveSubscriptionCreateArgs {
        file_token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        subscription_type: "comment_update".to_string(),
        subscription_id: Some("sub_1".to_string()),
        is_subscribe: Some(true),
        auth: ApiAuthArg::User,
    });
    assert_eq!(subscription["subscription_id"], "sub_1");
    assert_eq!(subscription["is_subcribe"], true);

    let (view_query, auth) = drive_view_record_query(DriveViewRecordArgs {
        file_token: "docx_1".to_string(),
        file_type: "docx".to_string(),
        page_size: 10,
        page_token: Some("next".to_string()),
        viewer_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert!(matches!(auth, ApiAuthArg::Tenant));
    assert!(view_query.contains(&("file_type".to_string(), "docx".to_string())));
    assert!(view_query.contains(&("page_token".to_string(), "next".to_string())));
}

#[test]
fn builds_sheet_create_body() {
    let body = build_sheet_create_body(SheetCreateArgs {
        title: Some("AI 数据表".to_string()),
        folder_token: Some("fld_1".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(body["title"], "AI 数据表");
    assert_eq!(body["folder_token"], "fld_1");

    let empty = build_sheet_create_body(SheetCreateArgs {
        title: None,
        folder_token: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert!(empty.as_object().unwrap().is_empty());
}

#[test]
fn builds_sheet_tab_operation_bodies() {
    let add = build_sheet_add_body(SheetAddArgs {
        spreadsheet_token: "sht_1".to_string(),
        title: Some("数据".to_string()),
        index: Some(1),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        add["requests"][0]["addSheet"]["properties"]["title"],
        "数据"
    );
    assert_eq!(add["requests"][0]["addSheet"]["properties"]["index"], 1);

    let copy = build_sheet_copy_body(SheetCopyArgs {
        spreadsheet_token: "sht_1".to_string(),
        sheet_id: "sh_1".to_string(),
        title: Some("数据副本".to_string()),
        index: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        copy["requests"][0]["copySheet"]["source"]["sheetId"],
        "sh_1"
    );
    assert_eq!(
        copy["requests"][0]["copySheet"]["destination"]["title"],
        "数据副本"
    );

    let delete = build_sheet_delete_body(SheetDeleteArgs {
        spreadsheet_token: "sht_1".to_string(),
        sheet_id: "sh_1".to_string(),
    });
    assert_eq!(delete["requests"][0]["deleteSheet"]["sheetId"], "sh_1");

    let update = build_sheet_update_body(SheetUpdateArgs {
        spreadsheet_token: "sht_1".to_string(),
        sheet_id: "sh_1".to_string(),
        title: Some("新数据".to_string()),
        index: Some(0),
        hidden: Some(false),
        frozen_row_count: Some(1),
        frozen_col_count: Some(2),
        protect_lock: Some("LOCK".to_string()),
        lock_info: Some("重要表".to_string()),
        protect_users: vec!["ou_1".to_string()],
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    let properties = &update["requests"][0]["updateSheet"]["properties"];
    assert_eq!(properties["sheetId"], "sh_1");
    assert_eq!(properties["title"], "新数据");
    assert_eq!(properties["frozenRowCount"], 1);
    assert_eq!(properties["frozenColCount"], 2);
    assert_eq!(properties["protect"]["userIDs"][0], "ou_1");
}

#[test]
fn builds_sheet_merge_and_style_bodies() {
    let merge = build_sheet_merge_body(SheetMergeArgs {
        spreadsheet_token: "sht_1".to_string(),
        range: Some("Sheet1!A1:C1".to_string()),
        merge_type: "rows".to_string(),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(merge["range"], "Sheet1!A1:C1");
    assert_eq!(merge["mergeType"], "MERGE_ROWS");

    let unmerge = build_sheet_unmerge_body(SheetUnmergeArgs {
        spreadsheet_token: "sht_1".to_string(),
        range: Some("Sheet1!A1:C1".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(unmerge["range"], "Sheet1!A1:C1");

    let style = build_sheet_style_body(SheetStyleArgs {
        spreadsheet_token: "sht_1".to_string(),
        ranges: vec!["Sheet1!A1:C1".to_string(), "Sheet1!A2:C2".to_string()],
        style_json: Some(r#"{"formatter":"@","font":{"italic":true}}"#.to_string()),
        bold: Some(true),
        italic: None,
        font_size: Some("10pt/1.5".to_string()),
        font_clean: None,
        text_decoration: Some(1),
        formatter: Some("0.00%".to_string()),
        h_align: Some(1),
        v_align: Some(1),
        fore_color: Some("000000".to_string()),
        back_color: Some("#fff2cc".to_string()),
        border_type: Some("full_border".to_string()),
        border_color: Some("ff0000".to_string()),
        clean: Some(false),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    let first = &style["data"][0];
    assert_eq!(first["ranges"][0], "Sheet1!A1:C1");
    assert_eq!(first["ranges"][1], "Sheet1!A2:C2");
    assert_eq!(first["style"]["font"]["italic"], true);
    assert_eq!(first["style"]["font"]["bold"], true);
    assert_eq!(first["style"]["font"]["fontSize"], "10pt/1.5");
    assert_eq!(first["style"]["formatter"], "0.00%");
    assert_eq!(first["style"]["hAlign"], 1);
    assert_eq!(first["style"]["vAlign"], 1);
    assert_eq!(first["style"]["foreColor"], "#000000");
    assert_eq!(first["style"]["backColor"], "#fff2cc");
    assert_eq!(first["style"]["borderType"], "FULL_BORDER");
    assert_eq!(first["style"]["borderColor"], "#ff0000");
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
fn normalizes_base_record_inputs() {
    let fields = read_base_record_fields(
        Vec::new(),
        Some(r#"{"fields":{"标题":"A"}}"#.to_string()),
        None,
        false,
    )
    .unwrap();
    assert_eq!(fields["标题"], "A");
    let typed_fields = read_base_record_fields(
        vec![
            "标题=任务 A".to_string(),
            "分数=12.5".to_string(),
            "完成=true".to_string(),
            "备注=str:true".to_string(),
            "日期=date:2026-06-02".to_string(),
            "时间=datetime:2026-06-02 10:30".to_string(),
            r#"附件=json:[{"file_token":"file_1"}]"#.to_string(),
            "清空=null".to_string(),
        ],
        Some(r#"{"fields":{"标题":"旧标题","状态":"open"}}"#.to_string()),
        None,
        false,
    )
    .unwrap();
    assert_eq!(typed_fields["标题"], "任务 A");
    assert_eq!(typed_fields["状态"], "open");
    assert_eq!(typed_fields["分数"], 12.5);
    assert_eq!(typed_fields["完成"], true);
    assert_eq!(typed_fields["备注"], "true");
    assert_eq!(
        typed_fields["日期"],
        Local
            .with_ymd_and_hms(2026, 6, 2, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
    assert_eq!(
        typed_fields["时间"],
        Local
            .with_ymd_and_hms(2026, 6, 2, 10, 30, 0)
            .single()
            .unwrap()
            .timestamp_millis()
    );
    assert_eq!(typed_fields["附件"][0]["file_token"], "file_1");
    assert!(typed_fields["清空"].is_null());

    let records = read_base_record_batch_records(
        Vec::new(),
        Vec::new(),
        Some(r#"[{"标题":"A"},{"fields":{"标题":"B"}}]"#.to_string()),
        None,
        false,
        false,
    )
    .unwrap();
    assert_eq!(records[0]["fields"]["标题"], "A");
    assert_eq!(records[1]["fields"]["标题"], "B");

    let batch_create = read_base_record_batch_records(
        vec![
            "0:标题=A".to_string(),
            "0:分数=1.5".to_string(),
            "1:标题=B".to_string(),
            r#"1:附件=json:[{"file_token":"file_1"}]"#.to_string(),
        ],
        Vec::new(),
        None,
        None,
        false,
        false,
    )
    .unwrap();
    assert_eq!(batch_create[0]["fields"]["标题"], "A");
    assert_eq!(batch_create[0]["fields"]["分数"], 1.5);
    assert_eq!(batch_create[1]["fields"]["标题"], "B");
    assert_eq!(batch_create[1]["fields"]["附件"][0]["file_token"], "file_1");

    let batch_update = read_base_record_batch_records(
        vec![
            "0:状态=done".to_string(),
            "1:清空=null".to_string(),
            "1:备注=str:true".to_string(),
        ],
        vec!["rec_a".to_string(), "rec_b".to_string()],
        Some(r#"[{"record_id":"old","fields":{"状态":"open"}}]"#.to_string()),
        None,
        false,
        true,
    )
    .unwrap();
    assert_eq!(batch_update[0]["record_id"], "rec_a");
    assert_eq!(batch_update[0]["fields"]["状态"], "done");
    assert_eq!(batch_update[1]["record_id"], "rec_b");
    assert!(batch_update[1]["fields"]["清空"].is_null());
    assert_eq!(batch_update[1]["fields"]["备注"], "true");
}

#[test]
fn builds_chat_member_inputs() {
    let body = build_chat_members_body(
        vec!["ou_1".to_string(), "ou_2".to_string()],
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(body["id_list"][0], "ou_1");
    assert_eq!(body["id_list"][1], "ou_2");

    let body =
        build_chat_members_body(vec![], Some(r#"["cli_bot"]"#.to_string()), None, false).unwrap();
    assert_eq!(body["id_list"][0], "cli_bot");

    let query = chat_member_query(ChatMemberIdTypeArg::AppId, 1);
    assert!(query.contains(&("member_id_type".to_string(), "app_id".to_string())));
    assert!(query.contains(&("succeed_type".to_string(), "1".to_string())));

    let reaction = build_reaction_body(MessageReactionAddArgs {
        message_id: "om_1".to_string(),
        emoji_type: Some("SMILE".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(reaction["reaction_type"]["emoji_type"], "SMILE");
}

#[test]
fn builds_message_reply_and_poll_helpers() {
    assert_eq!(message_text_content("收到")["text"], "收到");

    let messages = vec![
        json!({
            "message_id": "om_old",
            "message_position": "10",
            "msg_type": "text",
            "sender": { "sender_type": "user" }
        }),
        json!({
            "message_id": "om_app",
            "message_position": "11",
            "msg_type": "text",
            "sender": { "sender_type": "app" }
        }),
        json!({
            "message_id": "om_system",
            "message_position": "12",
            "msg_type": "system",
            "sender": { "sender_type": "user" }
        }),
        json!({
            "message_id": "om_new",
            "message_position": "13",
            "msg_type": "text",
            "sender": { "sender_type": "user" }
        }),
    ];

    assert_eq!(message_position(&messages[0]), Some(10));
    assert_eq!(
        latest_message_cursor(&messages),
        Some((13, "om_new".to_string()))
    );

    let filtered = filter_poll_items(&messages, Some(10), false, false);
    assert_eq!(filtered.len(), 1);
    assert_eq!(message_id_of(&filtered[0]).as_deref(), Some("om_new"));

    let all = filter_poll_items(&messages, Some(10), true, true);
    assert_eq!(
        all.iter().filter_map(message_id_of).collect::<Vec<_>>(),
        vec![
            "om_app".to_string(),
            "om_system".to_string(),
            "om_new".to_string()
        ]
    );
}

#[test]
fn builds_uploaded_message_file_content() {
    assert_eq!(resolve_upload_message_type("mp4", "auto").unwrap(), "media");
    assert_eq!(
        resolve_upload_message_type("opus", "auto").unwrap(),
        "audio"
    );
    assert_eq!(
        resolve_upload_message_type("stream", "auto").unwrap(),
        "file"
    );

    let media = build_uploaded_file_message_content(
        "file_v2_xxx",
        "demo.mp4",
        "media",
        Some(3000),
        Some("img_v2_xxx".to_string()),
    );
    assert_eq!(media["file_key"], "file_v2_xxx");
    assert_eq!(media["image_key"], "img_v2_xxx");
    assert!(media.get("file_name").is_none());

    let file = build_uploaded_file_message_content("file_v2_xxx", "demo.pdf", "file", None, None);
    assert_eq!(file["file_key"], "file_v2_xxx");
    assert_eq!(file["file_name"], "demo.pdf");
}

#[test]
fn builds_voice_message_helpers() {
    assert!(is_opus_path(Path::new("voice.OPUS")));
    assert!(!is_opus_path(Path::new("voice.mp3")));
    assert_eq!(source_voice_stem(Path::new("/tmp/demo.mp3")), "demo");

    let candidates = voice_output_candidates(Path::new("/tmp/voice-work"), "feishu-voice.mp3");
    assert_eq!(
        candidates[0],
        PathBuf::from("/tmp/voice-work/feishu-voice.mp3")
    );
    assert_eq!(
        candidates[1],
        PathBuf::from("/tmp/voice-work/feishu-voice/feishu-voice.mp3")
    );
}

#[test]
fn builds_chat_create_and_tab_bodies() {
    let create = build_chat_create_body(&ChatCreateArgs {
        name: "AI 项目群".to_string(),
        description: Some("demo".to_string()),
        avatar: Some("img_avatar".to_string()),
        avatar_file: None,
        users: vec!["ou_user".to_string()],
        bots: vec!["cli_bot".to_string()],
        owner_id: Some("ou_user".to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        chat_type: "private".to_string(),
        group_message_type: "chat".to_string(),
        set_bot_manager: true,
        uuid: Some("uuid".to_string()),
        body_json: None,
        body_file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(create["name"], "AI 项目群");
    assert_eq!(create["avatar"], "img_avatar");
    assert_eq!(create["user_id_list"][0], "ou_user");
    assert_eq!(create["bot_id_list"][0], "cli_bot");

    let tab = build_chat_tab_body(
        &ChatTabWriteArgs {
            chat_id: "oc_chat".to_string(),
            tab_id: None,
            name: Some("项目页".to_string()),
            tab_type: "url".to_string(),
            url: Some("https://example.com".to_string()),
            doc: None,
            icon_key: Some("img_icon".to_string()),
            icon_file: None,
            built_in: true,
            body_json: None,
            body_file: None,
            stdin: false,
        },
        false,
        None,
    )
    .unwrap();
    assert_eq!(tab["chat_tabs"][0]["tab_name"], "项目页");
    assert_eq!(
        tab["chat_tabs"][0]["tab_content"]["url"],
        "https://example.com"
    );
    assert_eq!(tab["chat_tabs"][0]["tab_config"]["icon_key"], "img_icon");
    assert_eq!(tab["chat_tabs"][0]["tab_config"]["is_built_in"], true);
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
fn filters_manifest_by_module_identity() {
    let base = json!({
        "name": "base",
        "command": "feishu-bot base",
        "scope_group": "base",
        "examples": ["feishu-bot base create --name \"AI Tasks\""]
    });
    let task = json!({
        "name": "task",
        "command": "feishu-bot task",
        "scope_group": "task"
    });
    assert!(!manifest_module_matches(&base, "task"));
    assert!(manifest_module_matches(&task, "task"));
    assert!(manifest_module_matches(&base, "feishu-bot base"));

    let setup = json!({
        "name": "setup",
        "command": "feishu-bot setup",
        "scope_group": "im,doc,wiki,base,search,user-token"
    });
    let office = json!({
        "name": "office",
        "command": "feishu-bot office",
        "scope_group": "im,wiki,doc,base,search"
    });
    let mut modules = vec![setup, office, base];
    retain_manifest_modules(&mut modules, "base");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0]["name"], "base");
}

#[test]
fn manifest_exposes_office_workflow_layer() {
    let manifest = build_manifest().unwrap();
    let workflow_modules = manifest
        .pointer("/layers/workflow_modules")
        .and_then(Value::as_array)
        .unwrap();
    assert!(workflow_modules.iter().any(|item| item == "office"));
    let modules = manifest.get("modules").and_then(Value::as_array).unwrap();
    let office = modules
        .iter()
        .find(|module| module.get("name").and_then(Value::as_str) == Some("office"))
        .unwrap();
    assert_eq!(office["layer"], "workflow");
    assert!(office["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("office report")));
    assert!(office["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("office progress")));

    let setup = modules
        .iter()
        .find(|module| module.get("name").and_then(Value::as_str) == Some("setup"))
        .unwrap();
    assert_eq!(setup["layer"], "setup");
    assert!(setup["examples"]
        .as_array()
        .unwrap()
        .iter()
        .any(|example| example.as_str().unwrap().contains("setup quickstart")));
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
fn builds_base_view_and_record_batch_inputs() {
    let app_update = build_base_app_update_body(BaseAppUpdateArgs {
        app_token: "base_1".to_string(),
        name: Some("AI 工作台 v2".to_string()),
        is_advanced: Some(true),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(app_update["name"], "AI 工作台 v2");
    assert_eq!(app_update["is_advanced"], true);

    let copied = build_base_copy_body(BaseCopyArgs {
        app_token: "base_1".to_string(),
        name: Some("AI 工作台副本".to_string()),
        folder_token: Some("fld_1".to_string()),
        without_content: Some(true),
        time_zone: Some("Asia/Shanghai".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(copied["name"], "AI 工作台副本");
    assert_eq!(copied["folder_token"], "fld_1");
    assert_eq!(copied["without_content"], true);

    let table_batch = build_base_table_batch_create_body(BaseTableBatchCreateArgs {
        app_token: "base_1".to_string(),
        name: vec!["需求".to_string(), "实验".to_string()],
        tables_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(table_batch["tables"][0]["name"], "需求");
    assert_eq!(table_batch["tables"][1]["name"], "实验");

    let table_create = build_base_table_create_body(BaseTableCreateArgs {
        app_token: "base_1".to_string(),
        name: Some("需求".to_string()),
        default_view_name: Some("默认视图".to_string()),
        field_specs: vec![
            "标题:text".to_string(),
            "状态:single-select:待处理:0|完成:1".to_string(),
            "金额:currency:0.00|CNY".to_string(),
            "截止日期:date:yyyy/MM/dd|auto_fill=false".to_string(),
        ],
        fields_json: Some(r#"[{"field_name":"备注","type":1}]"#.to_string()),
        fields_file: None,
        fields_stdin: false,
    })
    .unwrap();
    assert_eq!(table_create["table"]["name"], "需求");
    assert_eq!(table_create["table"]["default_view_name"], "默认视图");
    assert_eq!(table_create["table"]["fields"][0]["field_name"], "备注");
    assert_eq!(table_create["table"]["fields"][1]["field_name"], "标题");
    assert_eq!(table_create["table"]["fields"][1]["type"], 1);
    assert_eq!(table_create["table"]["fields"][2]["type"], 3);
    assert_eq!(
        table_create["table"]["fields"][2]["property"]["options"][1]["name"],
        "完成"
    );
    assert_eq!(
        table_create["table"]["fields"][2]["property"]["options"][1]["color"],
        1
    );
    assert_eq!(
        table_create["table"]["fields"][3]["property"]["currency_code"],
        "CNY"
    );
    assert_eq!(
        table_create["table"]["fields"][4]["property"]["date_formatter"],
        "yyyy/MM/dd"
    );
    assert_eq!(
        table_create["table"]["fields"][4]["property"]["auto_fill"],
        false
    );
    assert!(parse_base_table_field_spec("bad").is_err());

    let table_ids = read_table_ids_json(
        vec!["tbl_1".to_string()],
        Some(r#"{"table_ids":["tbl_2"]}"#.to_string()),
        None,
        false,
    )
    .unwrap();
    assert_eq!(table_ids[0], "tbl_2");

    let table_update = build_base_table_update_body(BaseTableUpdateArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        name: Some("需求池".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(table_update["name"], "需求池");

    let field_create = build_base_field_create_body(BaseFieldCreateArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        name: "状态".to_string(),
        field_type: None,
        kind: Some(BaseFieldKindArg::SingleSelect),
        options: vec!["待处理:0".to_string(), "完成:1".to_string()],
        formatter: None,
        currency_code: None,
        date_formatter: None,
        auto_fill: None,
        multiple: None,
        linked_table_id: None,
        formula: None,
        location_input_type: None,
        property_json: None,
        description_json: Some(r#"{"disable_sync":false,"text":"阶段字段"}"#.to_string()),
        ui_type: None,
        client_token: Some("token_1".to_string()),
    })
    .unwrap();
    assert_eq!(field_create["field_name"], "状态");
    assert_eq!(field_create["type"], 3);
    assert_eq!(field_create["property"]["options"][0]["name"], "待处理");
    assert_eq!(field_create["property"]["options"][0]["color"], 0);
    assert_eq!(field_create["description"]["text"], "阶段字段");

    let field_update = build_base_field_update_body(BaseFieldUpdateArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        field_id: "fld_1".to_string(),
        name: Some("阶段".to_string()),
        field_type: None,
        kind: Some(BaseFieldKindArg::Currency),
        options: vec![],
        formatter: Some("0.00".to_string()),
        currency_code: Some("CNY".to_string()),
        date_formatter: None,
        auto_fill: None,
        multiple: None,
        linked_table_id: None,
        formula: None,
        location_input_type: None,
        property_json: Some(r#"{"separator":"thousand"}"#.to_string()),
        description_json: None,
        ui_type: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(field_update["field_name"], "阶段");
    assert_eq!(field_update["type"], 2);
    assert_eq!(field_update["ui_type"], "Currency");
    assert_eq!(field_update["property"]["formatter"], "0.00");
    assert_eq!(field_update["property"]["currency_code"], "CNY");
    assert_eq!(field_update["property"]["separator"], "thousand");

    let dashboard = build_base_dashboard_copy_body(BaseDashboardCopyArgs {
        app_token: "base_1".to_string(),
        block_id: "blk_1".to_string(),
        name: Some("指标副本".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(dashboard["name"], "指标副本");

    let workflow = build_base_workflow_update_body(BaseWorkflowUpdateArgs {
        app_token: "base_1".to_string(),
        workflow_id: "wfl_1".to_string(),
        status: Some(BaseWorkflowStatusArg::Disable),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(workflow["status"], "Disable");

    let view = build_base_view_create_body(BaseViewCreateArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        name: Some("看板".to_string()),
        view_type: "kanban".to_string(),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(view["view_name"], "看板");
    assert_eq!(view["view_type"], "kanban");

    let view_update = build_base_view_update_body(BaseViewUpdateArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        view_id: "vew_1".to_string(),
        name: Some("新视图".to_string()),
        hidden_field_ids: vec!["fld_hidden".to_string()],
        filter_conjunction: Some("and".to_string()),
        filter_conditions: vec![
            r#"fld_status:3:is:json:["opt_done"]"#.to_string(),
            "fld_text:1:contains:AI".to_string(),
        ],
        filter_condition_omitted: Some(true),
        hierarchy_field_id: Some("fld_parent".to_string()),
        property_json: Some(
            r#"{"filter_info":{"conditions":[]},"hidden_fields":["fld_old"]}"#.to_string(),
        ),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(view_update["view_name"], "新视图");
    assert_eq!(view_update["property"]["hidden_fields"][0], "fld_old");
    assert_eq!(view_update["property"]["hidden_fields"][1], "fld_hidden");
    assert_eq!(view_update["property"]["filter_info"]["conjunction"], "and");
    assert_eq!(
        view_update["property"]["filter_info"]["conditions"][0]["value"],
        r#"["opt_done"]"#
    );
    assert_eq!(
        view_update["property"]["filter_info"]["conditions"][1]["value"],
        "AI"
    );
    assert_eq!(
        view_update["property"]["filter_info"]["condition_omitted"],
        true
    );
    assert_eq!(
        view_update["property"]["hierarchy_config"]["field_id"],
        "fld_parent"
    );

    let record_ids = read_record_ids_json(
        vec!["rec_1".to_string()],
        Some(r#"{"records":["rec_2"]}"#.to_string()),
        None,
        false,
    )
    .unwrap();
    assert_eq!(record_ids[0], "rec_2");

    let search = build_base_record_search_body(&BaseRecordSearchArgs {
        app_token: "base_1".to_string(),
        table_id: "tbl_1".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        view_id: Some("vew_1".to_string()),
        field_names: vec!["标题".to_string()],
        field_names_json: Some(r#"{"field_names":["状态"]}"#.to_string()),
        filter_json: Some(r#"{"conjunction":"and","conditions":[]}"#.to_string()),
        sort_json: Some(r#"[{"field_name":"标题","desc":true}]"#.to_string()),
        automatic_fields: true,
        page_size: 100,
        page_token: None,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(search["view_id"], "vew_1");
    assert_eq!(search["field_names"][0], "标题");
    assert_eq!(search["field_names"][1], "状态");
    assert_eq!(search["filter"]["conjunction"], "and");
    assert_eq!(search["sort"][0]["field_name"], "标题");
    assert_eq!(search["automatic_fields"], true);

    let role = build_base_role_write_body(
        Some("只读成员".to_string()),
        Some(r#"[{"table_id":"tbl_1","table_perm":1}]"#.to_string()),
        None,
        Some(r#"{"copy":0}"#.to_string()),
        Some(true),
        None,
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(role["role_name"], "只读成员");
    assert_eq!(role["table_roles"][0]["table_id"], "tbl_1");
    assert_eq!(role["base_rule"]["base_complex_edit"], 1);
    assert_eq!(role["base_rule"]["copy"], 0);

    let member = build_base_member_add_body(Some("ou_1".to_string()), None, None, false).unwrap();
    assert_eq!(member["member_id"], "ou_1");

    let batch_members = build_base_member_batch_body(
        vec!["open_id:ou_1".to_string(), "chat_id:oc_1".to_string()],
        None,
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(batch_members["member_list"][0]["type"], "open_id");
    assert_eq!(batch_members["member_list"][1]["id"], "oc_1");
}

#[test]
fn builds_tasklist_and_comment_bodies() {
    let tasklist = build_tasklist_create_body(TasklistCreateArgs {
        name: Some("AI 项目清单".to_string()),
        members: vec!["ou_1".to_string()],
        member_role: "editor".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(tasklist["name"], "AI 项目清单");
    assert_eq!(tasklist["members"][0]["role"], "editor");

    let update = build_tasklist_update_body(TasklistUpdateArgs {
        tasklist_guid: "tl_1".to_string(),
        name: Some("新清单".to_string()),
        owner_json: None,
        origin_owner_to_role: "none".to_string(),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(update["tasklist"]["name"], "新清单");
    assert_eq!(update["update_fields"][0], "name");

    let tasklist_members = build_tasklist_member_body(TasklistMemberWriteArgs {
        tasklist_guid: "tl_1".to_string(),
        editors: vec!["ou_editor".to_string()],
        viewers: vec!["ou_viewer".to_string()],
        member_type: None,
        members_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(tasklist_members["members"][0]["role"], "editor");
    assert_eq!(tasklist_members["members"][1]["role"], "viewer");

    let task_members = build_task_member_body(
        TaskMemberWriteArgs {
            task_guid: "task_1".to_string(),
            assignees: vec!["ou_assignee".to_string()],
            followers: vec!["ou_follower".to_string()],
            member_type: None,
            members_json: None,
            body_json: None,
            file: None,
            stdin: false,
            client_token: Some("client_12345".to_string()),
            user_id_type: UserIdTypeArg::OpenId,
            auth: ApiAuthArg::Tenant,
        },
        true,
    )
    .unwrap();
    assert_eq!(task_members["members"][0]["role"], "assignee");
    assert_eq!(task_members["members"][1]["role"], "follower");
    assert_eq!(task_members["client_token"], "client_12345");

    let relation = build_task_tasklist_body(TaskTasklistWriteArgs {
        task_guid: "task_1".to_string(),
        tasklist_guid: Some("tl_1".to_string()),
        section_guid: Some("section_1".to_string()),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(relation["tasklist_guid"], "tl_1");
    assert_eq!(relation["section_guid"], "section_1");

    let reminder = build_task_reminder_add_body(TaskReminderAddArgs {
        task_guid: "task_1".to_string(),
        relative_fire_minute: Some(30),
        reminders_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(reminder["reminders"][0]["relative_fire_minute"], 30);
    assert!(build_task_reminder_add_body(TaskReminderAddArgs {
        task_guid: "task_1".to_string(),
        relative_fire_minute: Some(-1),
        reminders_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .is_err());

    let remove_reminder = build_task_reminder_remove_body(TaskReminderRemoveArgs {
        task_guid: "task_1".to_string(),
        reminder_ids: vec!["10".to_string()],
        reminder_ids_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(remove_reminder["reminder_ids"][0], "10");

    let add_dependency = build_task_dependency_add_body(TaskDependencyAddArgs {
        task_guid: "task_1".to_string(),
        dependency_task_guids: vec!["task_2".to_string()],
        dependency_type: "next".to_string(),
        dependencies_json: None,
        body_json: None,
        file: None,
        stdin: false,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(add_dependency["dependencies"][0]["task_guid"], "task_2");
    assert_eq!(add_dependency["dependencies"][0]["type"], "next");

    let remove_dependency = build_task_dependency_remove_body(TaskDependencyRemoveArgs {
        task_guid: "task_1".to_string(),
        dependency_task_guids: vec!["task_2".to_string()],
        dependencies_json: None,
        body_json: None,
        file: None,
        stdin: false,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(remove_dependency["dependencies"][0]["task_guid"], "task_2");

    let comment = build_task_comment_create_body(TaskCommentCreateArgs {
        task_guid: "task_1".to_string(),
        content: Some("进展说明".to_string()),
        reply_to_comment_id: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(comment["resource_type"], "task");
    assert_eq!(comment["resource_id"], "task_1");
    assert_eq!(comment["content"], "进展说明");

    let comment_update = build_task_comment_update_body(TaskCommentUpdateArgs {
        comment_id: "comment_1".to_string(),
        content: Some("更新后的进展".to_string()),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(comment_update["comment"]["content"], "更新后的进展");
    assert_eq!(comment_update["update_fields"][0], "content");

    assert!(build_task_comment_update_body(TaskCommentUpdateArgs {
        comment_id: "comment_1".to_string(),
        content: Some(" ".to_string()),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .is_err());
}

#[test]
fn builds_task_section_and_custom_field_bodies() {
    let section = build_task_section_create_body(TaskSectionCreateArgs {
        name: Some("进行中".to_string()),
        resource_type: "tasklist".to_string(),
        resource_id: Some("tl_1".to_string()),
        insert_before: None,
        insert_after: Some("section_0".to_string()),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(section["name"], "进行中");
    assert_eq!(section["resource_type"], "tasklist");
    assert_eq!(section["resource_id"], "tl_1");
    assert_eq!(section["insert_after"], "section_0");

    let section_update = build_task_section_update_body(TaskSectionUpdateArgs {
        section_guid: "section_1".to_string(),
        name: Some("已完成".to_string()),
        insert_before: None,
        insert_after: None,
        update_fields: vec![],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(section_update["section"]["name"], "已完成");
    assert_eq!(section_update["update_fields"][0], "name");

    let field = build_task_custom_field_create_body(TaskCustomFieldCreateArgs {
        name: Some("优先级".to_string()),
        field_type: Some("single_select".to_string()),
        resource_type: "tasklist".to_string(),
        resource_id: Some("tl_1".to_string()),
        setting_key: None,
        setting_json: None,
        options: vec!["高".to_string(), "中".to_string(), "低".to_string()],
        options_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(field["name"], "优先级");
    assert_eq!(field["type"], "single_select");
    assert_eq!(field["single_select_setting"]["options"][0]["name"], "高");

    let option_update =
        build_task_custom_field_option_update_body(TaskCustomFieldOptionUpdateArgs {
            custom_field_guid: "field_1".to_string(),
            option_guid: "option_1".to_string(),
            name: None,
            color_index: Some(8),
            is_hidden: Some(true),
            insert_before: None,
            insert_after: None,
            update_fields: vec![],
            body_json: None,
            file: None,
            stdin: false,
            auth: ApiAuthArg::Tenant,
        })
        .unwrap();
    assert_eq!(option_update["option"]["color_index"], 8);
    assert_eq!(option_update["option"]["is_hidden"], true);
    assert_eq!(option_update["update_fields"][0], "color_index");
    assert_eq!(option_update["update_fields"][1], "is_hidden");

    let text_value = build_task_custom_field_value_update_body(TaskCustomFieldSetValueArgs {
        task_guid: "task_1".to_string(),
        custom_field_guid: "field_1".to_string(),
        value_type: TaskCustomFieldValueTypeArg::Text,
        value: Some("reviewed".to_string()),
        members: vec![],
        option_guids: vec![],
        clear: false,
        member_type: "user".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(
        text_value["task"]["custom_fields"][0]["text_value"],
        "reviewed"
    );
    assert_eq!(text_value["update_fields"][0], "custom_fields");

    let member_value = build_task_custom_field_value_update_body(TaskCustomFieldSetValueArgs {
        task_guid: "task_1".to_string(),
        custom_field_guid: "field_2".to_string(),
        value_type: TaskCustomFieldValueTypeArg::Member,
        value: None,
        members: vec!["ou_user".to_string()],
        option_guids: vec![],
        clear: false,
        member_type: "user".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(
        member_value["task"]["custom_fields"][0]["member_value"][0]["id"],
        "ou_user"
    );

    let multi_value = build_task_custom_field_value_update_body(TaskCustomFieldSetValueArgs {
        task_guid: "task_1".to_string(),
        custom_field_guid: "field_3".to_string(),
        value_type: TaskCustomFieldValueTypeArg::MultiSelect,
        value: None,
        members: vec![],
        option_guids: vec!["opt_1".to_string(), "opt_2".to_string()],
        clear: false,
        member_type: "user".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(
        multi_value["task"]["custom_fields"][0]["multi_select_value"][1],
        "opt_2"
    );

    let cleared = build_task_custom_field_value_update_body(TaskCustomFieldSetValueArgs {
        task_guid: "task_1".to_string(),
        custom_field_guid: "field_4".to_string(),
        value_type: TaskCustomFieldValueTypeArg::SingleSelect,
        value: None,
        members: vec![],
        option_guids: vec![],
        clear: true,
        member_type: "user".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(
        cleared["task"]["custom_fields"][0]["single_select_value"],
        ""
    );

    let upload_path =
        std::env::temp_dir().join(format!("feishu-task-attachment-{}.txt", Uuid::new_v4()));
    fs::write(&upload_path, b"dogfood").unwrap();
    let (fields, files) = build_task_attachment_upload_parts(TaskAttachmentUploadArgs {
        resource_type: "task".to_string(),
        resource_id: "task_1".to_string(),
        files: vec![upload_path.clone()],
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    let _ = fs::remove_file(upload_path);
    assert_eq!(fields[0], ("resource_type".to_string(), "task".to_string()));
    assert_eq!(fields[1], ("resource_id".to_string(), "task_1".to_string()));
    assert_eq!(files[0].0, "file");
}

#[test]
fn builds_task_list_query_filters() {
    let query = build_task_list_query(&TaskListArgs {
        page_size: 25,
        page_token: Some("next".to_string()),
        completed: Some(false),
        list_type: "my_tasks".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::User,
    })
    .unwrap();

    assert!(query.contains(&("page_size".to_string(), "25".to_string())));
    assert!(query.contains(&("page_token".to_string(), "next".to_string())));
    assert!(query.contains(&("completed".to_string(), "false".to_string())));
    assert!(query.contains(&("type".to_string(), "my_tasks".to_string())));
    assert!(query.contains(&("user_id_type".to_string(), "open_id".to_string())));

    assert!(build_task_list_query(&TaskListArgs {
        page_size: 101,
        page_token: None,
        completed: None,
        list_type: "my_tasks".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::User,
    })
    .is_err());
    assert!(build_task_list_query(&TaskListArgs {
        page_size: 50,
        page_token: None,
        completed: None,
        list_type: "  ".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::User,
    })
    .is_err());
}

#[test]
fn builds_task_create_and_update_bodies() {
    let create = build_task_create_body(TaskCreateArgs {
        summary: Some("plan".to_string()),
        description: Some("typed create".to_string()),
        due_ms: None,
        due_at: Some("2026-06-01T09:30:00+08:00".to_string()),
        due_date: None,
        due_all_day: false,
        start_ms: None,
        start_at: None,
        start_date: Some("2026-06-03".to_string()),
        start_all_day: false,
        completed_at: Some("0".to_string()),
        repeat_rule: Some("FREQ=WEEKLY;INTERVAL=1".to_string()),
        custom_complete_json: Some(r#"{"pc":{"tip":{"zh_cn":"去系统完成"}}}"#.to_string()),
        origin_json: Some(r#"{"platform_i18n_name":{"zh_cn":"AI系统"},"href":{"url":"https://example.com/task/1"}}"#.to_string()),
        extra: Some("eyJzb3VyY2UiOiJhaSJ9".to_string()),
        mode: Some(2),
        is_milestone: Some(true),
        reminders_json: None,
        reminder_minute: Some(30),
        custom_fields_json: Some(r#"[{"custom_field_guid":"field_1","type":"text","text_value":"ok"}]"#.to_string()),
        docx_source_json: Some(r#"{"document_id":"docx_1","block_id":"blk_1"}"#.to_string()),
        assignees: vec!["ou_a".to_string()],
        followers: vec!["ou_f".to_string()],
        tasklist_guids: vec!["tl_1".to_string()],
        body_json: None,
        file: None,
        stdin: false,
        client_token: Some("client-token-123".to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(create["summary"], "plan");
    assert_eq!(create["due"]["timestamp"], "1780277400000");
    assert_eq!(create["due"]["is_all_day"], false);
    assert_eq!(create["start"]["is_all_day"], true);
    assert_eq!(
        create["start"]["timestamp"],
        Local
            .with_ymd_and_hms(2026, 6, 3, 0, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis()
            .to_string()
    );
    assert_eq!(create["completed_at"], "0");
    assert_eq!(create["repeat_rule"], "FREQ=WEEKLY;INTERVAL=1");
    assert_eq!(
        create["custom_complete"]["pc"]["tip"]["zh_cn"],
        "去系统完成"
    );
    assert_eq!(create["origin"]["platform_i18n_name"]["zh_cn"], "AI系统");
    assert_eq!(create["mode"], 2);
    assert_eq!(create["is_milestone"], true);
    assert_eq!(create["reminders"][0]["relative_fire_minute"], 30);
    assert_eq!(create["custom_fields"][0]["custom_field_guid"], "field_1");
    assert_eq!(create["docx_source"]["document_id"], "docx_1");

    assert!(build_task_create_body(TaskCreateArgs {
        summary: Some("missing due".to_string()),
        description: None,
        due_ms: None,
        due_at: None,
        due_date: None,
        due_all_day: false,
        start_ms: None,
        start_at: None,
        start_date: None,
        start_all_day: false,
        completed_at: None,
        repeat_rule: None,
        custom_complete_json: None,
        origin_json: None,
        extra: None,
        mode: None,
        is_milestone: None,
        reminders_json: None,
        reminder_minute: Some(30),
        custom_fields_json: None,
        docx_source_json: None,
        assignees: vec![],
        followers: vec![],
        tasklist_guids: vec![],
        body_json: None,
        file: None,
        stdin: false,
        client_token: Some("client-token-456".to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .is_err());

    let body = build_task_update_body(TaskUpdateArgs {
        guid: "task_1".to_string(),
        summary: Some("next".to_string()),
        description: None,
        clear_description: false,
        due_ms: None,
        due_at: None,
        due_date: Some("2026-06-04".to_string()),
        due_all_day: false,
        clear_due: false,
        start_ms: None,
        start_at: None,
        start_date: None,
        start_all_day: false,
        clear_start: true,
        completed_at: None,
        repeat_rule: Some("FREQ=DAILY;INTERVAL=1".to_string()),
        clear_repeat_rule: false,
        custom_complete_json: Some(r#"{"pc":{"tip":{"en_us":"finish elsewhere"}}}"#.to_string()),
        clear_custom_complete: false,
        extra: Some("e30=".to_string()),
        clear_extra: false,
        mode: Some(1),
        is_milestone: Some(false),
        custom_fields_json: Some(r#"{"custom_fields":[{"custom_field_guid":"field_2","type":"number","number_value":"12"}]}"#.to_string()),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
    })
    .unwrap();
    assert_eq!(body["task"]["summary"], "next");
    assert_eq!(body["update_fields"][0], "summary");
    assert_eq!(body["task"]["due"]["is_all_day"], true);
    assert_eq!(body["task"].get("start"), None);
    assert!(body["update_fields"]
        .as_array()
        .unwrap()
        .contains(&json!("start")));
    assert_eq!(body["task"]["repeat_rule"], "FREQ=DAILY;INTERVAL=1");
    assert_eq!(
        body["task"]["custom_complete"]["pc"]["tip"]["en_us"],
        "finish elsewhere"
    );
    assert_eq!(body["task"]["extra"], "e30=");
    assert_eq!(body["task"]["mode"], 1);
    assert_eq!(body["task"]["is_milestone"], false);
    assert_eq!(
        body["task"]["custom_fields"][0]["custom_field_guid"],
        "field_2"
    );
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
fn builds_sheet_values_body() {
    let body = build_sheet_values_body(SheetValuesWriteArgs {
        spreadsheet_token: "sht_1".to_string(),
        range: "Sheet1!A1:B1".to_string(),
        values_json: Some(r#"[["a","b"]]"#.to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(body["valueRange"]["range"], "Sheet1!A1:B1");
    assert_eq!(body["valueRange"]["values"][0][0], "a");
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

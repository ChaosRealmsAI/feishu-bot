use super::super::*;

#[test]
fn parses_dogfood_auto_refresh_args_for_ai() {
    let cli = Cli::parse_from([
        "feishu",
        "dogfood",
        "verify",
        "--module",
        "task",
        "--auto-refresh-user-token",
        "--refresh-env-file",
        "private/local.env",
    ]);
    match cli.command {
        Commands::Dogfood(DogfoodCommand::Verify(args)) => {
            assert_eq!(args.module, vec!["task"]);
            assert!(args.auto_refresh_user_token);
            assert_eq!(
                args.refresh_env_file.unwrap(),
                PathBuf::from("private/local.env")
            );
        }
        _ => panic!("expected dogfood verify"),
    }
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

    let expired_user = classify_dogfood_error(
        r#"Feishu HTTP 401 Unauthorized: {
          "code": 99991677,
          "error": {
            "log_id": "20260602ABC"
          },
          "msg": "Authentication token expired. Please request a new one."
        }"#,
    );
    assert_eq!(expired_user["status"], "expired_user_token");
    assert_eq!(expired_user["log_id"], "20260602ABC");
    assert_eq!(expired_user["code"], 99991677);

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
    assert!(user_token_probe["remediation"]["oauth_token_command"]
        .as_str()
        .unwrap()
        .contains("private/local.env"));

    let expired_token_probe = dogfood_probe_from_result(
        "search",
        "search.docs",
        "feishu-bot --json search docs --query dogfood --page-size 1",
        "POST /search/v2/doc_wiki/search",
        "search",
        probe_value(Err(anyhow!(
            r#"Feishu HTTP 401 Unauthorized: {{
              "code": 99991677,
              "error": {{
                "log_id": "20260602ABC"
              }},
              "msg": "Authentication token expired. Please request a new one."
            }}"#
        ))),
        false,
        "cli_test",
    );
    assert_eq!(expired_token_probe["status"], "expired_user_token");
    assert_eq!(
        expired_token_probe["remediation"]["action"],
        "refresh_user_access_token"
    );
    assert!(expired_token_probe["remediation"]["oauth_refresh_command"]
        .as_str()
        .unwrap()
        .contains("oauth refresh"));
    let expired_mail_probe = dogfood_probe_from_result(
        "mail",
        "mail.me.folders.list",
        "feishu-bot --json mail folder list --mailbox me",
        "GET /mail/v1/user_mailboxes/me/folders",
        "mail",
        probe_value(Err(anyhow!(
            r#"Feishu HTTP 401 Unauthorized: {{
              "code": 99991677,
              "msg": "Authentication token expired. Please request a new one."
            }}"#
        ))),
        false,
        "cli_test",
    );
    let summary = summarize_dogfood_probes(&[expired_token_probe, expired_mail_probe]);
    assert_eq!(summary["counts"]["expired_user_token"], 2);
    let action_modules = summary["next_actions"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|action| action.get("module").and_then(Value::as_str))
        .collect::<Vec<_>>();
    assert!(action_modules.contains(&"search"));
    assert!(action_modules.contains(&"mail"));
    assert!(summary["next_actions"][0]["oauth_refresh_command"]
        .as_str()
        .unwrap()
        .contains("oauth refresh"));
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

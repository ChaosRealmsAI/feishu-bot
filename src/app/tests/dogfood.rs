use super::super::*;

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

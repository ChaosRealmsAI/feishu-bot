use super::super::*;

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

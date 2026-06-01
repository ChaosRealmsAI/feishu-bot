use super::super::*;

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

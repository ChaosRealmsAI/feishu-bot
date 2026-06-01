use super::super::*;

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

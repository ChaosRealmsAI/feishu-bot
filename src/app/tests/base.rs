use super::super::*;

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

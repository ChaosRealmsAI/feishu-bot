use super::super::*;

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

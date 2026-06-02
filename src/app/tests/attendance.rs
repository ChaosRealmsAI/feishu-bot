use super::super::*;

#[test]
fn parses_attendance_commands_after_cli_split() {
    let group = Cli::parse_from([
        "feishu",
        "attendance",
        "group",
        "list",
        "--page-size",
        "20",
        "--page-token",
        "next",
    ]);
    match group.command {
        Commands::Attendance(AttendanceCommand::Group(AttendanceGroupCommand::List(args))) => {
            assert_eq!(args.page_size, 20);
            assert_eq!(args.page_token.as_deref(), Some("next"));
        }
        _ => panic!("expected attendance group list"),
    }

    let flow = Cli::parse_from([
        "feishu",
        "attendance",
        "flow",
        "query",
        "--user-id",
        "u1",
        "--from-ts",
        "1760000000",
        "--to-ts",
        "1760086400",
        "--include-terminated-user",
        "--employee-type",
        "employee-no",
    ]);
    match flow.command {
        Commands::Attendance(AttendanceCommand::Flow(AttendanceFlowCommand::Query(args))) => {
            assert_eq!(args.user_ids, vec!["u1"]);
            assert_eq!(args.check_time_from.as_deref(), Some("1760000000"));
            assert_eq!(args.check_time_to.as_deref(), Some("1760086400"));
            assert!(args.include_terminated_user);
            assert!(matches!(
                args.employee_type,
                AttendanceEmployeeTypeArg::EmployeeNo
            ));
        }
        _ => panic!("expected attendance flow query"),
    }
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

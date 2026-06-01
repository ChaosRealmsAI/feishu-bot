use super::*;

pub(super) async fn run_attendance_command(
    api: &mut FeishuClient,
    command: AttendanceCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        AttendanceCommand::Group(AttendanceGroupCommand::List(args)) => {
            let query = attendance_page_query(args)?;
            api.get_json("/attendance/v1/groups", &query).await?
        }
        AttendanceCommand::Group(AttendanceGroupCommand::Get(args)) => {
            let query = vec![
                (
                    "employee_type".to_string(),
                    args.employee_type.as_api_value().to_string(),
                ),
                ("dept_type".to_string(), args.dept_type),
            ];
            let path = format!("/attendance/v1/groups/{}", args.group_id);
            api.get_json(&path, &query).await?
        }
        AttendanceCommand::Shift(AttendanceShiftCommand::List(args)) => {
            let query = attendance_page_query(args)?;
            api.get_json("/attendance/v1/shifts", &query).await?
        }
        AttendanceCommand::Shift(AttendanceShiftCommand::Get(args)) => {
            let path = format!("/attendance/v1/shifts/{}", args.shift_id);
            api.get_json(&path, &[]).await?
        }
        AttendanceCommand::Shift(AttendanceShiftCommand::Query(args)) => {
            api.post_json(
                "/attendance/v1/shifts/query",
                &[("shift_name".to_string(), args.shift_name)],
                json!({}),
            )
            .await?
        }
        AttendanceCommand::Schedule(AttendanceScheduleCommand::Query(args)) => {
            let query = attendance_employee_query(args.employee_type);
            let body = build_attendance_schedule_body(args)?;
            api.post_json("/attendance/v1/user_daily_shifts/query", &query, body)
                .await?
        }
        AttendanceCommand::Task(AttendanceTaskCommand::Query(args)) => {
            let mut query = attendance_employee_query(args.employee_type);
            if args.ignore_invalid_users {
                query.push(("ignore_invalid_users".to_string(), "true".to_string()));
            }
            if args.include_terminated_user {
                query.push(("include_terminated_user".to_string(), "true".to_string()));
            }
            let body = build_attendance_task_body(args)?;
            api.post_json("/attendance/v1/user_tasks/query", &query, body)
                .await?
        }
        AttendanceCommand::Flow(AttendanceFlowCommand::Get(args)) => {
            let query = attendance_employee_query(args.employee_type);
            let path = format!("/attendance/v1/user_flows/{}", args.user_flow_id);
            api.get_json(&path, &query).await?
        }
        AttendanceCommand::Flow(AttendanceFlowCommand::Query(args)) => {
            let mut query = attendance_employee_query(args.employee_type);
            if args.include_terminated_user {
                query.push(("include_terminated_user".to_string(), "true".to_string()));
            }
            let body = build_attendance_flow_query_body(args)?;
            api.post_json("/attendance/v1/user_flows/query", &query, body)
                .await?
        }
        AttendanceCommand::Flow(AttendanceFlowCommand::Import(args)) => {
            let query = attendance_employee_query(args.employee_type);
            if !has_json_input(&args.body_json, &args.file, args.stdin) {
                bail!("attendance flow import requires --body-json, --file, or --stdin");
            }
            let body = ensure_json_object(
                read_json_value(args.body_json, args.file, args.stdin)?,
                "attendance flow import body",
            )?;
            api.post_json("/attendance/v1/user_flows/batch_create", &query, body)
                .await?
        }
        AttendanceCommand::Flow(AttendanceFlowCommand::Delete(args)) => {
            let body = build_attendance_flow_delete_body(args)?;
            api.post_json("/attendance/v1/user_flows/batch_del", &[], body)
                .await?
        }
        AttendanceCommand::Stats(AttendanceStatsCommand::Query(args)) => {
            let query = attendance_employee_query(args.employee_type);
            let body = build_attendance_stats_body(args)?;
            api.post_json("/attendance/v1/user_stats_datas/query", &query, body)
                .await?
        }
    };
    print_response(raw_json, "attendance operation completed", data)
}

pub(super) fn attendance_page_query(args: AttendancePageArgs) -> Result<Vec<(String, String)>> {
    if args.page_size == 0 || args.page_size > 50 {
        bail!("attendance page_size must be between 1 and 50");
    }
    let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
    push_query_opt(&mut query, "page_token", args.page_token);
    Ok(query)
}

pub(super) fn attendance_employee_query(
    employee_type: AttendanceEmployeeTypeArg,
) -> Vec<(String, String)> {
    vec![(
        "employee_type".to_string(),
        employee_type.as_api_value().to_string(),
    )]
}

pub(super) fn build_attendance_schedule_body(args: AttendanceScheduleQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "attendance schedule query body",
        );
    }
    let user_ids = clean_string_values(args.user_ids);
    validate_value_count("user-id", user_ids.len(), 50, true)?;
    Ok(json!({
        "user_ids": user_ids,
        "check_date_from": args
            .check_date_from
            .ok_or_else(|| anyhow!("--from is required unless --body-json/--file/--stdin is used"))?,
        "check_date_to": args
            .check_date_to
            .ok_or_else(|| anyhow!("--to is required unless --body-json/--file/--stdin is used"))?,
    }))
}

pub(super) fn build_attendance_task_body(args: AttendanceTaskQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "attendance task query body",
        );
    }
    let user_ids = clean_string_values(args.user_ids);
    validate_value_count("user-id", user_ids.len(), 50, true)?;
    let mut body = Map::new();
    body.insert("user_ids".to_string(), json!(user_ids));
    body.insert(
        "check_date_from".to_string(),
        json!(args.check_date_from.ok_or_else(|| anyhow!(
            "--from is required unless --body-json/--file/--stdin is used"
        ))?),
    );
    body.insert(
        "check_date_to".to_string(),
        json!(args.check_date_to.ok_or_else(|| anyhow!(
            "--to is required unless --body-json/--file/--stdin is used"
        ))?),
    );
    if args.need_overtime_result {
        body.insert("need_overtime_result".to_string(), Value::Bool(true));
    }
    Ok(Value::Object(body))
}

pub(super) fn build_attendance_flow_query_body(args: AttendanceFlowQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "attendance flow query body",
        );
    }
    let user_ids = clean_string_values(args.user_ids);
    validate_value_count("user-id", user_ids.len(), 50, true)?;
    Ok(json!({
        "user_ids": user_ids,
        "check_time_from": args
            .check_time_from
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| anyhow!("--from-ts is required unless --body-json/--file/--stdin is used"))?,
        "check_time_to": args
            .check_time_to
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| anyhow!("--to-ts is required unless --body-json/--file/--stdin is used"))?,
    }))
}

pub(super) fn build_attendance_flow_delete_body(args: AttendanceFlowDeleteArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "attendance flow delete body",
        );
    }
    let record_ids = clean_string_values(args.record_ids);
    validate_value_count("record-id", record_ids.len(), 10, true)?;
    Ok(json!({ "record_ids": record_ids }))
}

pub(super) fn build_attendance_stats_body(args: AttendanceStatsQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "attendance stats query body",
        );
    }
    let user_ids = clean_string_values(args.user_ids);
    validate_value_count("user-id", user_ids.len(), 200, true)?;
    let mut body = Map::new();
    body.insert("locale".to_string(), Value::String(args.locale));
    body.insert("stats_type".to_string(), Value::String(args.stats_type));
    body.insert(
        "start_date".to_string(),
        json!(args.start_date.ok_or_else(|| anyhow!(
            "--from is required unless --body-json/--file/--stdin is used"
        ))?),
    );
    body.insert(
        "end_date".to_string(),
        json!(args.end_date.ok_or_else(|| anyhow!(
            "--to is required unless --body-json/--file/--stdin is used"
        ))?),
    );
    body.insert("user_ids".to_string(), json!(user_ids));
    if args.need_history {
        body.insert("need_history".to_string(), Value::Bool(true));
    }
    if args.current_group_only {
        body.insert("current_group_only".to_string(), Value::Bool(true));
    }
    if let Some(user_id) = args
        .operator_user_id
        .filter(|value| !value.trim().is_empty())
    {
        body.insert("user_id".to_string(), Value::String(user_id));
    }
    Ok(Value::Object(body))
}

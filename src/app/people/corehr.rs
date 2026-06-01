use super::*;

pub(in crate::app) async fn run_corehr_command(
    api: &mut FeishuClient,
    command: CorehrCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        CorehrCommand::Department(CorehrDepartmentCommand::Search(args)) => {
            let mut query = corehr_page_query(args.page_size, args.page_token.clone())?;
            append_corehr_user_query(&mut query, args.user_id_type);
            append_corehr_department_query(&mut query, args.department_id_type);
            let body = build_corehr_department_search_body(args)?;
            api.post_json("/corehr/v2/departments/search", &query, body)
                .await?
        }
        CorehrCommand::Department(CorehrDepartmentCommand::Get(args)) => {
            let mut query = Vec::new();
            append_corehr_user_query(&mut query, args.user_id_type);
            append_corehr_department_query(&mut query, args.department_id_type);
            let body = build_corehr_department_get_body(args)?;
            api.post_json("/corehr/v2/departments/batch_get", &query, body)
                .await?
        }
        CorehrCommand::Job(CorehrJobCommand::List(args)) => {
            let mut query = corehr_page_query(args.page_size, args.page_token)?;
            push_query_opt(&mut query, "name", args.name);
            push_query_opt(&mut query, "query_language", args.query_language);
            api.get_json("/corehr/v2/jobs", &query).await?
        }
        CorehrCommand::Job(CorehrJobCommand::Get(args)) => {
            let path = format!("/corehr/v2/jobs/{}", args.job_id);
            api.get_json(&path, &[]).await?
        }
        CorehrCommand::Job(CorehrJobCommand::BatchGet(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            let body = build_corehr_job_batch_get_body(args)?;
            api.post_json("/corehr/v2/jobs/batch_get", &query, body)
                .await?
        }
        CorehrCommand::JobData(CorehrJobDataCommand::Query(args)) => {
            let mut query = corehr_page_query(args.page_size, args.page_token.clone())?;
            append_corehr_user_query(&mut query, args.user_id_type);
            append_corehr_department_query(&mut query, args.department_id_type);
            let body = build_corehr_job_data_query_body(args)?;
            api.post_json("/corehr/v2/employees/job_datas/query", &query, body)
                .await?
        }
        CorehrCommand::JobData(CorehrJobDataCommand::Get(args)) => {
            let path = format!("/corehr/v1/job_datas/{}", args.job_data_id);
            let mut query = Vec::new();
            append_corehr_user_query(&mut query, args.user_id_type);
            append_corehr_department_query(&mut query, args.department_id_type);
            api.get_json(&path, &query).await?
        }
        CorehrCommand::Person(CorehrPersonCommand::Get(args)) => {
            let path = format!("/corehr/v1/persons/{}", args.person_id);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            api.get_json(&path, &query).await?
        }
        CorehrCommand::Process(CorehrProcessCommand::List(args)) => {
            let query = build_corehr_process_list_query(args)?;
            api.get_json("/corehr/v2/processes", &query).await?
        }
        CorehrCommand::Process(CorehrProcessCommand::Get(args)) => {
            let path = format!("/corehr/v2/processes/{}", args.process_id);
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            api.get_json(&path, &query).await?
        }
    };
    print_response(raw_json, "corehr operation completed", data)
}

pub(in crate::app) fn corehr_page_query(
    page_size: u16,
    page_token: Option<String>,
) -> Result<Vec<(String, String)>> {
    if page_size == 0 || page_size > 100 {
        bail!("corehr page_size must be between 1 and 100");
    }
    let mut query = vec![("page_size".to_string(), page_size.to_string())];
    push_query_opt(&mut query, "page_token", page_token);
    Ok(query)
}

fn append_corehr_user_query(query: &mut Vec<(String, String)>, user_id_type: CorehrUserIdTypeArg) {
    query.push((
        "user_id_type".to_string(),
        user_id_type.as_api_value().to_string(),
    ));
}

fn append_corehr_department_query(
    query: &mut Vec<(String, String)>,
    department_id_type: CorehrDepartmentIdTypeArg,
) {
    query.push((
        "department_id_type".to_string(),
        department_id_type.as_api_value().to_string(),
    ));
}

fn insert_opt_bool(object: &mut Map<String, Value>, key: &str, value: Option<bool>) {
    if let Some(value) = value {
        object.insert(key.to_string(), Value::Bool(value));
    }
}

fn insert_checked_string_array(
    object: &mut Map<String, Value>,
    key: &str,
    values: Vec<String>,
    max: usize,
) -> Result<usize> {
    let values = clean_string_values(values);
    validate_value_count(key, values.len(), max, false)?;
    if !values.is_empty() {
        object.insert(
            key.to_string(),
            Value::Array(values.into_iter().map(Value::String).collect()),
        );
        return Ok(object
            .get(key)
            .and_then(Value::as_array)
            .map_or(0, Vec::len));
    }
    Ok(0)
}

pub(in crate::app) fn build_corehr_department_search_body(
    args: CorehrDepartmentSearchArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "corehr department search body",
        );
    }

    let mut body = Map::new();
    insert_opt_bool(&mut body, "active", args.active);
    if args.get_all_children {
        body.insert("get_all_children".to_string(), Value::Bool(true));
    }
    insert_opt_string(&mut body, "parent_department_id", args.parent_department_id);
    insert_checked_string_array(&mut body, "department_id_list", args.department_ids, 100)?;
    insert_checked_string_array(&mut body, "name_list", args.names, 100)?;
    insert_checked_string_array(&mut body, "manager_list", args.manager_ids, 100)?;
    insert_checked_string_array(&mut body, "code_list", args.codes, 100)?;
    insert_checked_string_array(&mut body, "fields", args.fields, 100)?;
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_corehr_department_get_body(
    args: CorehrDepartmentGetArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "corehr department get body",
        );
    }

    let mut body = Map::new();
    let id_count =
        insert_checked_string_array(&mut body, "department_id_list", args.department_ids, 100)?;
    let name_count =
        insert_checked_string_array(&mut body, "department_name_list", args.names, 100)?;
    insert_checked_string_array(&mut body, "fields", args.fields, 100)?;
    if id_count == 0 && name_count == 0 {
        bail!("corehr department get needs --department-id or --name unless raw JSON is used");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_corehr_job_batch_get_body(
    args: CorehrJobBatchGetArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "corehr job batch-get body",
        );
    }

    let mut body = Map::new();
    let id_count = insert_checked_string_array(&mut body, "job_ids", args.job_ids, 100)?;
    let code_count = insert_checked_string_array(&mut body, "job_codes", args.job_codes, 100)?;
    insert_checked_string_array(&mut body, "fields", args.fields, 100)?;
    if id_count == 0 && code_count == 0 {
        bail!("corehr job batch-get needs --job-id or --job-code unless raw JSON is used");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_corehr_job_data_query_body(
    args: CorehrJobDataQueryArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "corehr job-data query body",
        );
    }

    let mut body = Map::new();
    if args.all_version {
        body.insert("get_all_version".to_string(), Value::Bool(true));
    }
    insert_opt_string(&mut body, "data_date", args.data_date);
    insert_opt_string(&mut body, "effective_date_start", args.effective_date_start);
    insert_opt_string(&mut body, "effective_date_end", args.effective_date_end);
    insert_opt_string(&mut body, "department_id", args.department_id);
    insert_opt_bool(&mut body, "primary_job_data", args.primary_job_data);
    insert_checked_string_array(&mut body, "employment_ids", args.employment_ids, 100)?;
    insert_checked_string_array(
        &mut body,
        "assignment_start_reasons",
        args.assignment_start_reasons,
        100,
    )?;
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_corehr_process_list_query(
    args: CorehrProcessListArgs,
) -> Result<Vec<(String, String)>> {
    let mut query = corehr_page_query(args.page_size, args.page_token)?;
    let valid_statuses = [1_u8, 2, 4, 8, 9, 15];
    for status in args.statuses {
        if !valid_statuses.contains(&status) {
            bail!("corehr process status must be one of 1, 2, 4, 8, 9, or 15");
        }
        query.push(("statuses".to_string(), status.to_string()));
    }
    query.push(("modify_time_from".to_string(), args.modify_time_from));
    query.push(("modify_time_to".to_string(), args.modify_time_to));
    push_query_opt(&mut query, "flow_definition_id", args.flow_definition_id);
    Ok(query)
}

impl CorehrUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            CorehrUserIdTypeArg::OpenId => "open_id",
            CorehrUserIdTypeArg::UnionId => "union_id",
            CorehrUserIdTypeArg::UserId => "user_id",
            CorehrUserIdTypeArg::PeopleCorehrId => "people_corehr_id",
        }
    }
}

impl CorehrPersonUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            CorehrPersonUserIdTypeArg::OpenId => "open_id",
            CorehrPersonUserIdTypeArg::PeopleEmployeeId => "people_employee_id",
        }
    }
}

impl CorehrDepartmentIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            CorehrDepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            CorehrDepartmentIdTypeArg::DepartmentId => "department_id",
            CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId => "people_corehr_department_id",
        }
    }
}

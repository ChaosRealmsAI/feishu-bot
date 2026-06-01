use super::*;

pub(in crate::app) async fn run_directory_command(
    api: &mut FeishuClient,
    command: DirectoryCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        DirectoryCommand::Employee(DirectoryEmployeeCommand::Search(args)) => {
            let query = directory_query(args.employee_id_type, args.department_id_type);
            let auth = args.auth;
            let body = build_directory_employee_search_body(args)?;
            directory_post_json(api, "/directory/v1/employees/search", &query, body, auth).await?
        }
        DirectoryCommand::Employee(DirectoryEmployeeCommand::Mget(args)) => {
            let query = directory_query(args.employee_id_type, args.department_id_type);
            let auth = args.auth;
            let body = build_directory_employee_mget_body(args)?;
            directory_post_json(api, "/directory/v1/employees/mget", &query, body, auth).await?
        }
        DirectoryCommand::Employee(DirectoryEmployeeCommand::Filter(args)) => {
            let query = directory_query(args.employee_id_type, args.department_id_type);
            let auth = args.auth;
            let body = build_directory_employee_filter_body(args)?;
            directory_post_json(api, "/directory/v1/employees/filter", &query, body, auth).await?
        }
    };
    print_response(raw_json, "directory operation completed", data)
}

async fn directory_post_json(
    api: &mut FeishuClient,
    path: &str,
    query: &[(String, String)],
    body: Value,
    auth: DirectoryAuthArg,
) -> Result<Value> {
    match auth {
        DirectoryAuthArg::Tenant => api.post_json(path, query, body).await,
        DirectoryAuthArg::User => api.post_json_user(path, query, body).await,
    }
}

fn directory_default_fields() -> Vec<String> {
    vec![
        "base_info.employee_id".to_string(),
        "base_info.name.name".to_string(),
    ]
}

fn directory_fields(fields: Vec<String>) -> Result<Vec<String>> {
    let fields = clean_string_values(fields);
    validate_value_count("field", fields.len(), 100, false)?;
    if fields.is_empty() {
        Ok(directory_default_fields())
    } else {
        Ok(fields)
    }
}

fn directory_page_request(page_size: u16, page_token: Option<String>) -> Result<Value> {
    if page_size == 0 || page_size > 100 {
        bail!("directory page_size must be between 1 and 100");
    }
    let mut page = Map::new();
    page.insert("page_size".to_string(), json!(page_size));
    insert_opt_string(&mut page, "page_token", page_token);
    Ok(Value::Object(page))
}

pub(in crate::app) fn build_directory_employee_search_body(
    args: DirectoryEmployeeSearchArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "directory employee search body",
        );
    }
    let query = args
        .query
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("directory employee search needs --query unless raw JSON is used")
        })?;
    Ok(json!({
        "query": query,
        "page_request": directory_page_request(args.page_size, args.page_token)?,
        "required_fields": directory_fields(args.fields)?,
    }))
}

pub(in crate::app) fn build_directory_employee_mget_body(
    args: DirectoryEmployeeMgetArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "directory employee mget body",
        );
    }
    let employee_ids = clean_string_values(args.employee_ids);
    validate_value_count("employee-id", employee_ids.len(), 100, true)?;
    Ok(json!({
        "employee_ids": employee_ids,
        "required_fields": directory_fields(args.fields)?,
    }))
}

pub(in crate::app) fn build_directory_employee_filter_body(
    args: DirectoryEmployeeFilterArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "directory employee filter body",
        );
    }

    let filter =
        if let Some(filter_json) = args.filter_json.filter(|value| !value.trim().is_empty()) {
            ensure_json_object(
                parse_json_value(&filter_json, "directory filter JSON")?,
                "directory filter",
            )?
        } else {
            let conditions = args
                .conditions
                .into_iter()
                .filter(|value| !value.trim().is_empty())
                .map(parse_directory_condition)
                .collect::<Result<Vec<_>>>()?;
            validate_value_count("condition", conditions.len(), 10, true)?;
            json!({ "conditions": conditions })
        };

    Ok(json!({
        "filter": filter,
        "page_request": directory_page_request(args.page_size, args.page_token)?,
        "required_fields": directory_fields(args.fields)?,
    }))
}

fn parse_directory_condition(value: String) -> Result<Value> {
    let mut parts = value.splitn(3, '=');
    let field = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("directory condition must be field=operator=value"))?;
    let operator = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("directory condition must include operator"))?;
    let condition_value = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("directory condition must include value"))?;
    let condition_value = parse_json_value(condition_value.trim(), "directory condition value")
        .unwrap_or_else(|_| json!(condition_value.trim()));
    Ok(json!({
        "field": field.trim(),
        "operator": operator.trim(),
        "value": condition_value,
    }))
}

pub(in crate::app) fn directory_query(
    employee_id_type: DirectoryEmployeeIdTypeArg,
    department_id_type: DirectoryDepartmentIdTypeArg,
) -> Vec<(String, String)> {
    vec![
        (
            "employee_id_type".to_string(),
            employee_id_type.as_api_value().to_string(),
        ),
        (
            "department_id_type".to_string(),
            department_id_type.as_api_value().to_string(),
        ),
    ]
}

impl DirectoryEmployeeIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            DirectoryEmployeeIdTypeArg::OpenId => "open_id",
            DirectoryEmployeeIdTypeArg::UnionId => "union_id",
            DirectoryEmployeeIdTypeArg::EmployeeId => "employee_id",
        }
    }
}

impl DirectoryDepartmentIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            DirectoryDepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            DirectoryDepartmentIdTypeArg::DepartmentId => "department_id",
        }
    }
}

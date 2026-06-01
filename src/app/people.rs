use super::*;

pub(super) async fn run_contact_command(
    api: &mut FeishuClient,
    command: ContactCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        ContactCommand::User(ContactUserCommand::Get(args)) => {
            let path = format!("/contact/v3/users/{}", args.user_id);
            api.get_json(
                &path,
                &contact_query(args.user_id_type, args.department_id_type),
            )
            .await?
        }
        ContactCommand::User(ContactUserCommand::List(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "department_id", args.department_id);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/contact/v3/users", &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Get(args)) => {
            let path = format!("/contact/v3/departments/{}", args.department_id);
            api.get_json(
                &path,
                &contact_query(args.user_id_type, args.department_id_type),
            )
            .await?
        }
        ContactCommand::Department(ContactDepartmentCommand::List(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(
                &mut query,
                "parent_department_id",
                args.parent_department_id,
            );
            if args.fetch_child {
                query.push(("fetch_child".to_string(), "true".to_string()));
            }
            api.get_json("/contact/v3/departments", &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Children(args)) => {
            let path = format!("/contact/v3/departments/{}/children", args.department_id);
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            if args.fetch_child {
                query.push(("fetch_child".to_string(), "true".to_string()));
            }
            api.get_json(&path, &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Search(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("query".to_string(), args.query));
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/contact/v3/departments/search", &query)
                .await?
        }
    };
    print_response(raw_json, "contact operation completed", data)
}

pub(super) async fn run_directory_command(
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

pub(super) async fn run_corehr_command(
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

pub(super) async fn run_hire_command(
    api: &mut FeishuClient,
    command: HireCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        HireCommand::Job(HireJobCommand::List(args)) => {
            let query = hire_job_list_query(args)?;
            api.get_json("/hire/v1/jobs", &query).await?
        }
        HireCommand::Job(HireJobCommand::Get(args)) => {
            let path = format!("/hire/v1/jobs/{}", encode_path_segment(&args.job_id));
            let query = hire_job_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Job(HireJobCommand::Detail(args)) => {
            let path = format!(
                "/hire/v1/jobs/{}/get_detail",
                encode_path_segment(&args.job_id)
            );
            let query = hire_job_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Job(HireJobCommand::Schemas(args)) => {
            let mut query = hire_page_query(args.page_size, 100, args.page_token)?;
            push_query_opt_u8(&mut query, "scenario", args.scenario);
            api.get_json("/hire/v1/job_schemas", &query).await?
        }
        HireCommand::Job(HireJobCommand::Open(args)) => {
            let path = format!("/hire/v1/jobs/{}/open", encode_path_segment(&args.job_id));
            let body = build_hire_job_open_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        HireCommand::Talent(HireTalentCommand::List(args)) => {
            let query = hire_talent_list_query(args)?;
            api.get_json("/hire/v1/talents", &query).await?
        }
        HireCommand::Talent(HireTalentCommand::Get(args)) => {
            let path = format!("/hire/v1/talents/{}", encode_path_segment(&args.talent_id));
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            api.get_json(&path, &query).await?
        }
        HireCommand::Talent(HireTalentCommand::Create(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            let body = build_hire_talent_create_body(args)?;
            api.post_json("/hire/v1/talents/combined_create", &query, body)
                .await?
        }
        HireCommand::Application(HireApplicationCommand::List(args)) => {
            let query = hire_application_list_query(args)?;
            api.get_json("/hire/v1/applications", &query).await?
        }
        HireCommand::Application(HireApplicationCommand::Get(args)) => {
            let path = format!(
                "/hire/v1/applications/{}",
                encode_path_segment(&args.application_id)
            );
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_repeated(&mut query, "options", args.options);
            api.get_json(&path, &query).await?
        }
        HireCommand::Application(HireApplicationCommand::Detail(args)) => {
            let path = format!(
                "/hire/v1/applications/{}/get_detail",
                encode_path_segment(&args.application_id)
            );
            let query = hire_application_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Interview(HireInterviewCommand::ByTalent(args)) => {
            let query = vec![
                ("talent_id".to_string(), args.talent_id),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.as_api_value().to_string(),
                ),
                (
                    "job_level_id_type".to_string(),
                    args.job_level_id_type.as_api_value().to_string(),
                ),
            ];
            api.get_json("/hire/v1/interviews/get_by_talent", &query)
                .await?
        }
        HireCommand::Process(HireProcessCommand::List(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_processes", &query).await?
        }
        HireCommand::Requirement(HireRequirementCommand::Schemas(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_requirement_schemas", &query)
                .await?
        }
        HireCommand::Metadata(HireMetadataCommand::ResumeSources(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/resume_sources", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::JobTypes(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_types", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::JobFunctions(args)) => {
            let query = hire_page_query(args.page_size, 50, args.page_token)?;
            api.get_json("/hire/v1/job_functions", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::Subjects(args)) => {
            let mut query = hire_page_query(args.page_size, 200, args.page_token)?;
            query.push((
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            ));
            api.get_json("/hire/v1/subjects", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::Websites(args)) => {
            let query = hire_page_query(args.page_size, 10, args.page_token)?;
            api.get_json("/hire/v1/websites", &query).await?
        }
        HireCommand::Attachment(HireAttachmentCommand::Get(args)) => {
            let path = format!(
                "/hire/v1/attachments/{}",
                encode_path_segment(&args.attachment_id)
            );
            let mut query = Vec::new();
            push_query_opt_u8(&mut query, "type", args.attachment_type);
            api.get_json(&path, &query).await?
        }
        HireCommand::Location(HireLocationCommand::Query(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token.clone())?;
            let body = build_hire_location_query_body(args)?;
            api.post_json("/hire/v1/locations/query", &query, body)
                .await?
        }
    };
    print_response(raw_json, "hire operation completed", data)
}

pub(super) fn hire_page_query(
    page_size: u16,
    max_page_size: u16,
    page_token: Option<String>,
) -> Result<Vec<(String, String)>> {
    if page_size == 0 || page_size > max_page_size {
        bail!("hire page_size must be between 1 and {max_page_size}");
    }
    let mut query = vec![("page_size".to_string(), page_size.to_string())];
    push_query_opt(&mut query, "page_token", page_token);
    Ok(query)
}

pub(super) fn hire_job_list_query(args: HireJobListArgs) -> Result<Vec<(String, String)>> {
    let mut query = hire_page_query(args.page_size, 20, args.page_token)?;
    push_query_opt(&mut query, "update_start_time", args.update_start_time);
    push_query_opt(&mut query, "update_end_time", args.update_end_time);
    query.extend(hire_job_id_type_query(
        args.user_id_type,
        args.department_id_type,
        args.job_level_id_type,
        args.job_family_id_type,
    ));
    Ok(query)
}

fn hire_job_detail_query(args: HireJobGetArgs) -> Vec<(String, String)> {
    hire_job_id_type_query(
        args.user_id_type,
        args.department_id_type,
        args.job_level_id_type,
        args.job_family_id_type,
    )
}

fn hire_job_id_type_query(
    user_id_type: HireUserIdTypeArg,
    department_id_type: DepartmentIdTypeArg,
    job_level_id_type: HireJobLevelIdTypeArg,
    job_family_id_type: HireJobFamilyIdTypeArg,
) -> Vec<(String, String)> {
    vec![
        (
            "user_id_type".to_string(),
            user_id_type.as_api_value().to_string(),
        ),
        (
            "department_id_type".to_string(),
            department_id_type.as_api_value().to_string(),
        ),
        (
            "job_level_id_type".to_string(),
            job_level_id_type.as_api_value().to_string(),
        ),
        (
            "job_family_id_type".to_string(),
            job_family_id_type.as_api_value().to_string(),
        ),
    ]
}

pub(super) fn hire_talent_list_query(args: HireTalentListArgs) -> Result<Vec<(String, String)>> {
    let mut query = hire_page_query(args.page_size, 20, args.page_token)?;
    push_query_opt(&mut query, "keyword", args.keyword);
    push_query_opt(&mut query, "update_start_time", args.update_start_time);
    push_query_opt(&mut query, "update_end_time", args.update_end_time);
    push_query_opt_u8(&mut query, "sort_by", args.sort_by);
    query.push((
        "user_id_type".to_string(),
        args.user_id_type.as_api_value().to_string(),
    ));
    push_query_opt(&mut query, "query_option", args.query_option);
    Ok(query)
}

pub(super) fn hire_application_list_query(
    args: HireApplicationListArgs,
) -> Result<Vec<(String, String)>> {
    let mut query = hire_page_query(args.page_size, 200, args.page_token)?;
    push_query_opt(&mut query, "process_id", args.process_id);
    push_query_opt(&mut query, "stage_id", args.stage_id);
    push_query_opt(&mut query, "talent_id", args.talent_id);
    push_query_opt(&mut query, "active_status", args.active_status);
    push_query_opt(&mut query, "job_id", args.job_id);
    for status in args.lock_status {
        query.push(("lock_status".to_string(), status.to_string()));
    }
    push_query_opt(&mut query, "update_start_time", args.update_start_time);
    push_query_opt(&mut query, "update_end_time", args.update_end_time);
    Ok(query)
}

pub(super) fn hire_application_detail_query(
    args: HireApplicationDetailArgs,
) -> Vec<(String, String)> {
    let mut query = vec![
        (
            "user_id_type".to_string(),
            args.user_id_type.as_api_value().to_string(),
        ),
        (
            "department_id_type".to_string(),
            args.department_id_type.as_api_value().to_string(),
        ),
        (
            "job_level_id_type".to_string(),
            args.job_level_id_type.as_api_value().to_string(),
        ),
        (
            "job_family_id_type".to_string(),
            args.job_family_id_type.as_api_value().to_string(),
        ),
        (
            "employee_type_id_type".to_string(),
            args.employee_type_id_type.as_api_value().to_string(),
        ),
    ];
    push_query_repeated(&mut query, "options", args.options);
    query
}

pub(super) fn build_hire_job_open_body(args: HireJobOpenArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire job open body",
        );
    }
    let is_never_expired = args
        .is_never_expired
        .ok_or_else(|| anyhow!("hire job open needs --is-never-expired unless raw JSON is used"))?;
    if !is_never_expired && args.expiry_time.is_none() {
        bail!("hire job open needs --expiry-time when --is-never-expired false");
    }
    let mut body = Map::new();
    body.insert(
        "is_never_expired".to_string(),
        Value::Bool(is_never_expired),
    );
    insert_opt_i64(&mut body, "expiry_time", args.expiry_time);
    Ok(Value::Object(body))
}

pub(super) fn build_hire_talent_create_body(args: HireTalentCreateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire talent combined_create body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("hire talent create needs --name unless raw JSON is used"))?;
    let mut basic_info = Map::new();
    basic_info.insert("name".to_string(), Value::String(name));
    insert_opt_string(&mut basic_info, "email", args.email);
    insert_opt_string(&mut basic_info, "mobile", args.mobile);
    insert_opt_string(
        &mut basic_info,
        "mobile_country_code",
        args.mobile_country_code,
    );
    insert_opt_string(&mut basic_info, "current_city_code", args.current_city_code);

    let mut body = Map::new();
    insert_opt_string(&mut body, "resume_source_id", args.resume_source_id);
    let folder_ids = clean_string_values(args.folder_ids);
    if !folder_ids.is_empty() {
        body.insert("folder_id_list".to_string(), json!(folder_ids));
    }
    insert_opt_string(&mut body, "creator_id", args.creator_id);
    insert_opt_u8(&mut body, "creator_account_type", args.creator_account_type);
    insert_opt_string(&mut body, "resume_attachment_id", args.resume_attachment_id);
    body.insert("basic_info".to_string(), Value::Object(basic_info));
    Ok(Value::Object(body))
}

pub(super) fn build_hire_location_query_body(args: HireLocationQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire location query body",
        );
    }
    let location_type = args.location_type.ok_or_else(|| {
        anyhow!("hire location query needs --location-type unless raw JSON is used")
    })?;
    let mut body = Map::new();
    insert_opt_u8(&mut body, "location_type", Some(location_type));
    let code_list = clean_string_values(args.code_list);
    if !code_list.is_empty() {
        body.insert("code_list".to_string(), json!(code_list));
    }
    Ok(Value::Object(body))
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

pub(super) fn build_directory_employee_search_body(
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

pub(super) fn build_directory_employee_mget_body(args: DirectoryEmployeeMgetArgs) -> Result<Value> {
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

pub(super) fn build_directory_employee_filter_body(
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

pub(super) fn corehr_page_query(
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

pub(super) fn build_corehr_department_search_body(
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

pub(super) fn build_corehr_department_get_body(args: CorehrDepartmentGetArgs) -> Result<Value> {
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

pub(super) fn build_corehr_job_batch_get_body(args: CorehrJobBatchGetArgs) -> Result<Value> {
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

pub(super) fn build_corehr_job_data_query_body(args: CorehrJobDataQueryArgs) -> Result<Value> {
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

pub(super) fn build_corehr_process_list_query(
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

fn contact_query(
    user_id_type: UserIdTypeArg,
    department_id_type: DepartmentIdTypeArg,
) -> Vec<(String, String)> {
    vec![
        (
            "user_id_type".to_string(),
            user_id_type.resolve(None).to_string(),
        ),
        (
            "department_id_type".to_string(),
            department_id_type.as_api_value().to_string(),
        ),
    ]
}

pub(super) fn directory_query(
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
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            DirectoryEmployeeIdTypeArg::OpenId => "open_id",
            DirectoryEmployeeIdTypeArg::UnionId => "union_id",
            DirectoryEmployeeIdTypeArg::EmployeeId => "employee_id",
        }
    }
}

impl DirectoryDepartmentIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            DirectoryDepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            DirectoryDepartmentIdTypeArg::DepartmentId => "department_id",
        }
    }
}

impl CorehrUserIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            CorehrUserIdTypeArg::OpenId => "open_id",
            CorehrUserIdTypeArg::UnionId => "union_id",
            CorehrUserIdTypeArg::UserId => "user_id",
            CorehrUserIdTypeArg::PeopleCorehrId => "people_corehr_id",
        }
    }
}

impl CorehrPersonUserIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            CorehrPersonUserIdTypeArg::OpenId => "open_id",
            CorehrPersonUserIdTypeArg::PeopleEmployeeId => "people_employee_id",
        }
    }
}

impl CorehrDepartmentIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            CorehrDepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            CorehrDepartmentIdTypeArg::DepartmentId => "department_id",
            CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId => "people_corehr_department_id",
        }
    }
}

impl HireUserIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            HireUserIdTypeArg::OpenId => "open_id",
            HireUserIdTypeArg::UnionId => "union_id",
            HireUserIdTypeArg::UserId => "user_id",
            HireUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl HireJobLevelIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            HireJobLevelIdTypeArg::PeopleAdminJobLevelId => "people_admin_job_level_id",
            HireJobLevelIdTypeArg::JobLevelId => "job_level_id",
        }
    }
}

impl HireJobFamilyIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId => "people_admin_job_category_id",
            HireJobFamilyIdTypeArg::JobFamilyId => "job_family_id",
        }
    }
}

impl HireEmployeeTypeIdTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            HireEmployeeTypeIdTypeArg::PeopleAdminEmployeeTypeId => "people_admin_employee_type_id",
            HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId => "employee_type_enum_id",
        }
    }
}

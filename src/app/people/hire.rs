use super::*;

pub(in crate::app) async fn run_hire_command(
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

pub(in crate::app) fn hire_page_query(
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

pub(in crate::app) fn hire_job_list_query(args: HireJobListArgs) -> Result<Vec<(String, String)>> {
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

pub(in crate::app) fn hire_talent_list_query(
    args: HireTalentListArgs,
) -> Result<Vec<(String, String)>> {
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

pub(in crate::app) fn hire_application_list_query(
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

pub(in crate::app) fn hire_application_detail_query(
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

pub(in crate::app) fn build_hire_job_open_body(args: HireJobOpenArgs) -> Result<Value> {
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

pub(in crate::app) fn build_hire_talent_create_body(args: HireTalentCreateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_hire_location_query_body(args: HireLocationQueryArgs) -> Result<Value> {
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

impl HireUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireUserIdTypeArg::OpenId => "open_id",
            HireUserIdTypeArg::UnionId => "union_id",
            HireUserIdTypeArg::UserId => "user_id",
            HireUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl HireJobLevelIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireJobLevelIdTypeArg::PeopleAdminJobLevelId => "people_admin_job_level_id",
            HireJobLevelIdTypeArg::JobLevelId => "job_level_id",
        }
    }
}

impl HireJobFamilyIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId => "people_admin_job_category_id",
            HireJobFamilyIdTypeArg::JobFamilyId => "job_family_id",
        }
    }
}

impl HireEmployeeTypeIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireEmployeeTypeIdTypeArg::PeopleAdminEmployeeTypeId => "people_admin_employee_type_id",
            HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId => "employee_type_enum_id",
        }
    }
}

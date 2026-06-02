use super::*;

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

pub(super) fn hire_job_detail_query(args: HireJobGetArgs) -> Vec<(String, String)> {
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

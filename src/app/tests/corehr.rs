use super::super::*;

#[test]
fn builds_corehr_queries_and_bodies() {
    assert_eq!(
        CorehrUserIdTypeArg::PeopleCorehrId.as_api_value(),
        "people_corehr_id"
    );
    assert_eq!(
        CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId.as_api_value(),
        "people_corehr_department_id"
    );

    let page = corehr_page_query(20, Some("next".to_string())).unwrap();
    assert!(page.contains(&("page_size".to_string(), "20".to_string())));
    assert!(page.contains(&("page_token".to_string(), "next".to_string())));
    assert!(corehr_page_query(101, None).is_err());

    let department = build_corehr_department_search_body(CorehrDepartmentSearchArgs {
        page_size: 20,
        page_token: None,
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec!["dept_1".to_string()],
        names: vec!["研发".to_string()],
        manager_ids: vec!["emp_1".to_string()],
        parent_department_id: Some("parent_1".to_string()),
        codes: vec!["D001".to_string()],
        fields: vec!["department_name".to_string()],
        active: Some(true),
        get_all_children: true,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(department["department_id_list"][0], "dept_1");
    assert_eq!(department["name_list"][0], "研发");
    assert_eq!(department["active"], true);
    assert_eq!(department["get_all_children"], true);

    let department_get = build_corehr_department_get_body(CorehrDepartmentGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec!["dept_1".to_string()],
        names: vec![],
        fields: vec!["version_id".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(department_get["department_id_list"][0], "dept_1");
    assert_eq!(department_get["fields"][0], "version_id");
    assert!(build_corehr_department_get_body(CorehrDepartmentGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::OpenDepartmentId,
        department_ids: vec![],
        names: vec![],
        fields: vec![],
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let job = build_corehr_job_batch_get_body(CorehrJobBatchGetArgs {
        user_id_type: CorehrUserIdTypeArg::OpenId,
        job_ids: vec!["job_1".to_string()],
        job_codes: vec!["JP001".to_string()],
        fields: vec!["job_name".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(job["job_ids"][0], "job_1");
    assert_eq!(job["job_codes"][0], "JP001");

    let job_data = build_corehr_job_data_query_body(CorehrJobDataQueryArgs {
        page_size: 20,
        page_token: None,
        user_id_type: CorehrUserIdTypeArg::OpenId,
        department_id_type: CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId,
        employment_ids: vec!["emp_1".to_string()],
        department_id: Some("dept_1".to_string()),
        data_date: Some("2026-05-31".to_string()),
        effective_date_start: None,
        effective_date_end: None,
        all_version: true,
        primary_job_data: Some(true),
        assignment_start_reasons: vec!["onboarding".to_string()],
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(job_data["employment_ids"][0], "emp_1");
    assert_eq!(job_data["get_all_version"], true);
    assert_eq!(job_data["primary_job_data"], true);

    let process = build_corehr_process_list_query(CorehrProcessListArgs {
        page_size: 10,
        page_token: None,
        statuses: vec![1, 9],
        modify_time_from: "1760000000000".to_string(),
        modify_time_to: "1760003600000".to_string(),
        flow_definition_id: Some("flow_1".to_string()),
    })
    .unwrap();
    assert_eq!(
        process
            .iter()
            .filter(|(key, _)| key == "statuses")
            .collect::<Vec<_>>()
            .len(),
        2
    );
    assert!(build_corehr_process_list_query(CorehrProcessListArgs {
        page_size: 10,
        page_token: None,
        statuses: vec![3],
        modify_time_from: "1760000000000".to_string(),
        modify_time_to: "1760003600000".to_string(),
        flow_definition_id: None,
    })
    .is_err());
}

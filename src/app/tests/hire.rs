use super::super::*;

#[test]
fn builds_hire_queries_and_bodies() {
    assert_eq!(
        HireUserIdTypeArg::PeopleAdminId.as_api_value(),
        "people_admin_id"
    );
    assert_eq!(
        HireJobLevelIdTypeArg::PeopleAdminJobLevelId.as_api_value(),
        "people_admin_job_level_id"
    );
    assert_eq!(
        HireJobFamilyIdTypeArg::JobFamilyId.as_api_value(),
        "job_family_id"
    );
    assert_eq!(
        HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId.as_api_value(),
        "employee_type_enum_id"
    );

    let jobs = hire_job_list_query(HireJobListArgs {
        update_start_time: Some("1760000000000".to_string()),
        update_end_time: Some("1760003600000".to_string()),
        page_size: 20,
        page_token: Some("tok".to_string()),
        user_id_type: HireUserIdTypeArg::OpenId,
        department_id_type: DepartmentIdTypeArg::OpenDepartmentId,
        job_level_id_type: HireJobLevelIdTypeArg::PeopleAdminJobLevelId,
        job_family_id_type: HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId,
    })
    .unwrap();
    assert!(jobs.contains(&("page_size".to_string(), "20".to_string())));
    assert!(jobs.contains(&("page_token".to_string(), "tok".to_string())));
    assert!(jobs.contains(&("user_id_type".to_string(), "open_id".to_string())));
    assert!(hire_page_query(0, 20, None).is_err());
    assert!(hire_page_query(21, 20, None).is_err());

    let talents = hire_talent_list_query(HireTalentListArgs {
        keyword: Some("张三 and 产品".to_string()),
        update_start_time: None,
        update_end_time: None,
        page_size: 10,
        sort_by: Some(2),
        page_token: None,
        user_id_type: HireUserIdTypeArg::PeopleAdminId,
        query_option: Some("ignore_empty_error".to_string()),
    })
    .unwrap();
    assert!(talents.contains(&("keyword".to_string(), "张三 and 产品".to_string())));
    assert!(talents.contains(&("sort_by".to_string(), "2".to_string())));
    assert!(talents.contains(&("query_option".to_string(), "ignore_empty_error".to_string())));

    let apps = hire_application_list_query(HireApplicationListArgs {
        process_id: Some("p1".to_string()),
        stage_id: Some("s1".to_string()),
        talent_id: Some("t1".to_string()),
        active_status: Some("1".to_string()),
        job_id: Some("j1".to_string()),
        lock_status: vec![1, 3],
        page_token: None,
        page_size: 200,
        update_start_time: None,
        update_end_time: None,
    })
    .unwrap();
    assert_eq!(
        apps.iter().filter(|(key, _)| key == "lock_status").count(),
        2
    );
    assert!(apps.contains(&("page_size".to_string(), "200".to_string())));

    let detail = hire_application_detail_query(HireApplicationDetailArgs {
        application_id: "a1".to_string(),
        user_id_type: HireUserIdTypeArg::OpenId,
        department_id_type: DepartmentIdTypeArg::DepartmentId,
        job_level_id_type: HireJobLevelIdTypeArg::JobLevelId,
        job_family_id_type: HireJobFamilyIdTypeArg::JobFamilyId,
        employee_type_id_type: HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId,
        options: vec!["with_job".to_string(), "with_talent".to_string()],
    });
    assert!(detail.contains(&(
        "department_id_type".to_string(),
        "department_id".to_string()
    )));
    assert_eq!(detail.iter().filter(|(key, _)| key == "options").count(), 2);

    let open = build_hire_job_open_body(HireJobOpenArgs {
        job_id: "j1".to_string(),
        is_never_expired: Some(false),
        expiry_time: Some(1830259120000),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(open["is_never_expired"], false);
    assert_eq!(open["expiry_time"], 1830259120000_i64);
    assert!(build_hire_job_open_body(HireJobOpenArgs {
        job_id: "j1".to_string(),
        is_never_expired: Some(false),
        expiry_time: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let talent = build_hire_talent_create_body(HireTalentCreateArgs {
        name: Some("张三".to_string()),
        email: Some("zhangsan@example.com".to_string()),
        mobile: None,
        mobile_country_code: Some("CN_1".to_string()),
        current_city_code: Some("CT_11".to_string()),
        resume_source_id: Some("10000".to_string()),
        folder_ids: vec!["f1".to_string(), "".to_string()],
        creator_id: None,
        creator_account_type: Some(3),
        resume_attachment_id: None,
        user_id_type: HireUserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(talent["basic_info"]["name"], "张三");
    assert_eq!(talent["basic_info"]["email"], "zhangsan@example.com");
    assert_eq!(talent["folder_id_list"][0], "f1");
    assert_eq!(talent["creator_account_type"], 3);

    let location = build_hire_location_query_body(HireLocationQueryArgs {
        location_type: Some(1),
        code_list: vec!["CN_1".to_string()],
        page_size: 100,
        page_token: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(location["location_type"], 1);
    assert_eq!(location["code_list"][0], "CN_1");
}

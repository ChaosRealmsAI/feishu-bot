use super::super::*;

#[test]
fn builds_directory_queries_and_bodies() {
    assert_eq!(
        DirectoryEmployeeIdTypeArg::EmployeeId.as_api_value(),
        "employee_id"
    );
    assert_eq!(
        DirectoryDepartmentIdTypeArg::DepartmentId.as_api_value(),
        "department_id"
    );

    let query = directory_query(
        DirectoryEmployeeIdTypeArg::UnionId,
        DirectoryDepartmentIdTypeArg::DepartmentId,
    );
    assert!(query.contains(&("employee_id_type".to_string(), "union_id".to_string())));
    assert!(query.contains(&(
        "department_id_type".to_string(),
        "department_id".to_string()
    )));

    let search = build_directory_employee_search_body(DirectoryEmployeeSearchArgs {
        query: Some("user@example.com".to_string()),
        page_size: 10,
        page_token: Some("next".to_string()),
        fields: vec![
            "base_info.employee_id".to_string(),
            "base_info.email".to_string(),
        ],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(search["query"], "user@example.com");
    assert_eq!(search["page_request"]["page_size"], 10);
    assert_eq!(search["page_request"]["page_token"], "next");
    assert_eq!(search["required_fields"][1], "base_info.email");

    let default_fields = build_directory_employee_search_body(DirectoryEmployeeSearchArgs {
        query: Some("张三".to_string()),
        page_size: 20,
        page_token: None,
        fields: vec![],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        default_fields["required_fields"][0],
        "base_info.employee_id"
    );
    assert_eq!(default_fields["required_fields"][1], "base_info.name.name");

    let mget = build_directory_employee_mget_body(DirectoryEmployeeMgetArgs {
        employee_ids: vec!["ou_1".to_string(), "".to_string(), "ou_2".to_string()],
        fields: vec!["work_info.job_title".to_string()],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(mget["employee_ids"][0], "ou_1");
    assert_eq!(mget["employee_ids"][1], "ou_2");
    assert_eq!(mget["required_fields"][0], "work_info.job_title");
    assert!(
        build_directory_employee_mget_body(DirectoryEmployeeMgetArgs {
            employee_ids: vec![],
            fields: vec![],
            employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
            department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
            auth: DirectoryAuthArg::Tenant,
            body_json: None,
            file: None,
            stdin: false,
        })
        .is_err()
    );

    let filter = build_directory_employee_filter_body(DirectoryEmployeeFilterArgs {
        conditions: vec![
            "base_info.email=eq=\"user@example.com\"".to_string(),
            "base_info.is_resigned=eq=false".to_string(),
        ],
        filter_json: None,
        page_size: 5,
        page_token: None,
        fields: vec!["base_info.name.name".to_string()],
        employee_id_type: DirectoryEmployeeIdTypeArg::OpenId,
        department_id_type: DirectoryDepartmentIdTypeArg::OpenDepartmentId,
        auth: DirectoryAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(
        filter["filter"]["conditions"][0]["field"],
        "base_info.email"
    );
    assert_eq!(filter["filter"]["conditions"][0]["operator"], "eq");
    assert_eq!(
        filter["filter"]["conditions"][0]["value"],
        "user@example.com"
    );
    assert_eq!(filter["filter"]["conditions"][1]["value"], false);
}

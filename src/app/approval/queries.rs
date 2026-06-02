use super::*;

pub(super) fn approval_id_query(
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

pub(super) fn approval_task_user_query(
    user_id_type: UserIdTypeArg,
    user_id: &str,
) -> Vec<(String, String)> {
    vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(Some(user_id)).to_string(),
    )]
}

pub(super) fn approval_search_query(args: &ApprovalSearchArgs) -> Vec<(String, String)> {
    let mut query = vec![
        ("page_size".to_string(), args.page_size.to_string()),
        (
            "user_id_type".to_string(),
            args.user_id_type
                .resolve(args.user_id.as_deref())
                .to_string(),
        ),
    ];
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    query
}

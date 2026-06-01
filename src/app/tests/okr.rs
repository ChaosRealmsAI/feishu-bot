use super::super::*;

#[test]
fn builds_okr_queries_and_validates_ids() {
    assert_eq!(OkrUserIdTypeArg::OpenId.as_api_value(), "open_id");
    assert_eq!(
        OkrUserIdTypeArg::PeopleAdminId.as_api_value(),
        "people_admin_id"
    );

    let mut query = build_okr_query(OkrUserIdTypeArg::PeopleAdminId, "zh_cn".to_string());
    push_query_repeated(
        &mut query,
        "okr_ids",
        vec!["okr_1".to_string(), "".to_string(), "okr_2".to_string()],
    );
    assert!(query.contains(&("user_id_type".to_string(), "people_admin_id".to_string())));
    assert!(query.contains(&("lang".to_string(), "zh_cn".to_string())));
    assert_eq!(
        query
            .iter()
            .filter(|(key, _)| key == "okr_ids")
            .collect::<Vec<_>>()
            .len(),
        2
    );

    assert!(validate_okr_id_list("period-id", &[], 10, false).is_ok());
    assert!(validate_okr_id_list("okr-id", &[], 10, true).is_err());
    assert!(validate_okr_id_list("okr-id", &vec!["x".to_string(); 11], 10, true).is_err());
}

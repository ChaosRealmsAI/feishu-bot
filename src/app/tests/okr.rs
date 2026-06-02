use super::super::*;

#[test]
fn parses_okr_commands_after_cli_split() {
    let period = Cli::parse_from([
        "feishu",
        "okr",
        "period",
        "list",
        "--page-size",
        "20",
        "--page-token",
        "next",
    ]);
    match period.command {
        Commands::Okr(OkrCommand::Period(OkrPeriodCommand::List(args))) => {
            assert_eq!(args.page_size, 20);
            assert_eq!(args.page_token.as_deref(), Some("next"));
        }
        _ => panic!("expected okr period list"),
    }

    let batch = Cli::parse_from([
        "feishu",
        "okr",
        "batch-get",
        "--okr-id",
        "okr_1",
        "--okr-id",
        "okr_2",
        "--lang",
        "en_us",
        "--user-id-type",
        "people-admin-id",
    ]);
    match batch.command {
        Commands::Okr(OkrCommand::BatchGet(args)) => {
            assert_eq!(args.okr_ids, vec!["okr_1", "okr_2"]);
            assert_eq!(args.lang, "en_us");
            assert!(matches!(args.user_id_type, OkrUserIdTypeArg::PeopleAdminId));
        }
        _ => panic!("expected okr batch-get"),
    }
}

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

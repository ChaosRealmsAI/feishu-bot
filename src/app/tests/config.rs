use super::super::*;

#[test]
fn loads_env_sources_in_order_and_skips_empty_overrides() {
    let dir = std::env::temp_dir().join(format!("feishu-bot-config-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let first = dir.join("first.env");
    let second = dir.join("second.env");
    std::fs::write(
        &first,
        "FEISHU_APP_ID=cli_from_first\nFEISHU_APP_SECRET=secret_from_first\nFEISHU_USER_ACCESS_TOKEN=u_from_first\n",
    )
    .unwrap();
    std::fs::write(
        &second,
        "FEISHU_APP_ID=cli_from_second\nFEISHU_APP_SECRET=\nFEISHU_USER_ID=ou_from_second\n",
    )
    .unwrap();

    let values = load_env_values_from_sources(
        vec![first, second],
        vec![
            ("FEISHU_APP_ID".to_string(), "cli_from_env".to_string()),
            ("FEISHU_USER_ACCESS_TOKEN".to_string(), String::new()),
            ("LARK_USER_ID".to_string(), "ou_from_env".to_string()),
        ],
    )
    .unwrap();

    assert_eq!(values["FEISHU_APP_ID"], "cli_from_env");
    assert_eq!(values["FEISHU_APP_SECRET"], "secret_from_first");
    assert_eq!(values["FEISHU_USER_ACCESS_TOKEN"], "u_from_first");
    assert_eq!(values["FEISHU_USER_ID"], "ou_from_second");
    assert_eq!(values["LARK_USER_ID"], "ou_from_env");

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn gets_first_non_empty_env_value() {
    let mut values = HashMap::new();
    values.insert("FEISHU_USER_ID".to_string(), String::new());
    values.insert("LARK_USER_ID".to_string(), "ou_lark".to_string());

    assert_eq!(
        get_any(&values, &["FEISHU_USER_ID", "LARK_USER_ID"]),
        Some("ou_lark".to_string())
    );
}

#[test]
fn masks_secrets_without_exposing_middle_content() {
    assert_eq!(mask_secret("12345678"), "***");
    assert_eq!(mask_secret("123456789"), "1234...6789");
    assert_eq!(mask_app_id("cli_1234567890"), "cli_1234...7890");
}

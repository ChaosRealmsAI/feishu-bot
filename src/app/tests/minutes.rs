use super::super::*;

#[test]
fn extracts_minutes_tokens_from_urls() {
    assert_eq!(
        extract_minute_token("obcnq3b9jl72l83w4f14xxxx").unwrap(),
        "obcnq3b9jl72l83w4f14xxxx"
    );
    assert_eq!(
        extract_minute_token("https://sample.feishu.cn/minutes/obcnq3b9jl72l83w4f14xxxx?from=copy")
            .unwrap(),
        "obcnq3b9jl72l83w4f14xxxx"
    );
}

#[test]
fn builds_minutes_search_body() {
    let body = build_minutes_search_body(MinutesSearchArgs {
            query: Some("周会".to_string()),
            filter_json: Some(
                r#"{"create_time":{"start_time":"2026-05-01T00:00:00+08:00","end_time":"2026-05-31T23:59:59+08:00"}}"#
                    .to_string(),
            ),
            sorter: Some("create_time_desc".to_string()),
            page_size: 20,
            page_token: None,
            body_json: None,
            file: None,
            stdin: false,
            user_id_type: UserIdTypeArg::OpenId,
        })
        .unwrap();
    assert_eq!(body["query"], "周会");
    assert_eq!(
        body["filter"]["create_time"]["start_time"],
        "2026-05-01T00:00:00+08:00"
    );
    assert_eq!(body["sorter"], "create_time_desc");
}

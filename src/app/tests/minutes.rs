use super::super::*;

#[test]
fn parses_minutes_commands_after_cli_split() {
    let search = Cli::parse_from([
        "feishu",
        "minutes",
        "search",
        "--query",
        "AI sync",
        "--sorter",
        "create_time_desc",
        "--page-size",
        "10",
    ]);
    match search.command {
        Commands::Minutes(MinutesCommand::Search(args)) => {
            assert_eq!(args.query.as_deref(), Some("AI sync"));
            assert_eq!(args.sorter.as_deref(), Some("create_time_desc"));
            assert_eq!(args.page_size, 10);
        }
        _ => panic!("expected minutes search"),
    }

    let transcript = Cli::parse_from([
        "feishu",
        "minutes",
        "transcript",
        "--minute-token",
        "obcnq3b9jl72l83w4f14xxxx",
        "--need-speaker",
        "--need-timestamp",
        "--file-format",
        "srt",
        "--output",
        "meeting.srt",
    ]);
    match transcript.command {
        Commands::Minutes(MinutesCommand::Transcript(args)) => {
            assert_eq!(args.minute_token, "obcnq3b9jl72l83w4f14xxxx");
            assert!(args.need_speaker);
            assert!(args.need_timestamp);
            assert_eq!(args.file_format.as_deref(), Some("srt"));
            assert_eq!(args.output, PathBuf::from("meeting.srt"));
        }
        _ => panic!("expected minutes transcript"),
    }
}

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

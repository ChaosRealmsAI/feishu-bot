use super::super::*;

#[test]
fn parses_mail_commands_after_cli_split() {
    let list = Cli::parse_from([
        "feishu",
        "mail",
        "message",
        "list",
        "--mailbox",
        "me",
        "--auth",
        "user",
        "--folder-id",
        "INBOX",
        "--only-unread",
        "--page-size",
        "10",
    ]);
    match list.command {
        Commands::Mail(MailCommand::Message(MailMessageCommand::List(args))) => {
            assert_eq!(args.mailbox.mailbox, "me");
            assert!(matches!(args.mailbox.auth, MailAuthArg::User));
            assert_eq!(args.folder_id.as_deref(), Some("INBOX"));
            assert!(args.only_unread);
            assert_eq!(args.page_size, 10);
        }
        _ => panic!("expected mail message list"),
    }

    let send = Cli::parse_from([
        "feishu",
        "mail",
        "message",
        "send",
        "--mailbox",
        "me",
        "--to",
        "a@example.com",
        "--subject",
        "hello",
        "--text",
        "body",
        "--dedupe-key",
        "k1",
    ]);
    match send.command {
        Commands::Mail(MailCommand::Message(MailMessageCommand::Send(args))) => {
            assert_eq!(args.mailbox, "me");
            assert_eq!(args.to, vec!["a@example.com"]);
            assert_eq!(args.subject.as_deref(), Some("hello"));
            assert_eq!(args.text.as_deref(), Some("body"));
            assert_eq!(args.dedupe_key.as_deref(), Some("k1"));
        }
        _ => panic!("expected mail message send"),
    }
}

#[test]
fn builds_mail_paths_auth_and_send_body() {
    assert_eq!(
        encode_path_segment("user@example.com/TUlH=="),
        "user%40example.com%2FTUlH%3D%3D"
    );
    assert!(mail_should_use_user(MailAuthArg::Auto, "me").unwrap());
    assert!(!mail_should_use_user(MailAuthArg::Auto, "user@example.com").unwrap());
    assert!(mail_should_use_user(MailAuthArg::Tenant, "me").is_err());

    let body = build_mail_send_body(MailMessageSendArgs {
        mailbox: "me".to_string(),
        to: vec!["a@example.com".to_string(), "".to_string()],
        cc: vec!["c@example.com".to_string()],
        bcc: vec![],
        subject: Some("hello".to_string()),
        text: Some("body".to_string()),
        html: None,
        raw_base64url: None,
        raw_file: None,
        dedupe_key: Some("k1".to_string()),
        from_address: Some("me@example.com".to_string()),
        from_name: Some("Me".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(body["subject"], "hello");
    assert_eq!(body["to"][0]["mail_address"], "a@example.com");
    assert_eq!(body["cc"][0]["mail_address"], "c@example.com");
    assert_eq!(body["body_plain_text"], "body");
    assert_eq!(body["dedupe_key"], "k1");
    assert_eq!(body["head_from"]["mail_address"], "me@example.com");

    assert!(build_mail_send_body(MailMessageSendArgs {
        mailbox: "me".to_string(),
        to: vec![],
        cc: vec![],
        bcc: vec![],
        subject: None,
        text: None,
        html: None,
        raw_base64url: None,
        raw_file: None,
        dedupe_key: None,
        from_address: None,
        from_name: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());
}

use super::super::*;

#[test]
fn builds_chat_member_inputs() {
    let body = build_chat_members_body(
        vec!["ou_1".to_string(), "ou_2".to_string()],
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(body["id_list"][0], "ou_1");
    assert_eq!(body["id_list"][1], "ou_2");

    let body =
        build_chat_members_body(vec![], Some(r#"["cli_bot"]"#.to_string()), None, false).unwrap();
    assert_eq!(body["id_list"][0], "cli_bot");

    let query = chat_member_query(ChatMemberIdTypeArg::AppId, 1);
    assert!(query.contains(&("member_id_type".to_string(), "app_id".to_string())));
    assert!(query.contains(&("succeed_type".to_string(), "1".to_string())));

    let reaction = build_reaction_body(MessageReactionAddArgs {
        message_id: "om_1".to_string(),
        emoji_type: Some("SMILE".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(reaction["reaction_type"]["emoji_type"], "SMILE");
}

#[test]
fn builds_message_reply_and_poll_helpers() {
    assert_eq!(message_text_content("收到")["text"], "收到");

    let messages = vec![
        json!({
            "message_id": "om_old",
            "message_position": "10",
            "msg_type": "text",
            "sender": { "sender_type": "user" }
        }),
        json!({
            "message_id": "om_app",
            "message_position": "11",
            "msg_type": "text",
            "sender": { "sender_type": "app" }
        }),
        json!({
            "message_id": "om_system",
            "message_position": "12",
            "msg_type": "system",
            "sender": { "sender_type": "user" }
        }),
        json!({
            "message_id": "om_new",
            "message_position": "13",
            "msg_type": "text",
            "sender": { "sender_type": "user" }
        }),
    ];

    assert_eq!(message_position(&messages[0]), Some(10));
    assert_eq!(
        latest_message_cursor(&messages),
        Some((13, "om_new".to_string()))
    );

    let filtered = filter_poll_items(&messages, Some(10), false, false);
    assert_eq!(filtered.len(), 1);
    assert_eq!(message_id_of(&filtered[0]).as_deref(), Some("om_new"));

    let all = filter_poll_items(&messages, Some(10), true, true);
    assert_eq!(
        all.iter().filter_map(message_id_of).collect::<Vec<_>>(),
        vec![
            "om_app".to_string(),
            "om_system".to_string(),
            "om_new".to_string()
        ]
    );
}

#[test]
fn builds_uploaded_message_file_content() {
    assert_eq!(resolve_upload_message_type("mp4", "auto").unwrap(), "media");
    assert_eq!(
        resolve_upload_message_type("opus", "auto").unwrap(),
        "audio"
    );
    assert_eq!(
        resolve_upload_message_type("stream", "auto").unwrap(),
        "file"
    );

    let media = build_uploaded_file_message_content(
        "file_v2_xxx",
        "demo.mp4",
        "media",
        Some(3000),
        Some("img_v2_xxx".to_string()),
    );
    assert_eq!(media["file_key"], "file_v2_xxx");
    assert_eq!(media["image_key"], "img_v2_xxx");
    assert!(media.get("file_name").is_none());

    let file = build_uploaded_file_message_content("file_v2_xxx", "demo.pdf", "file", None, None);
    assert_eq!(file["file_key"], "file_v2_xxx");
    assert_eq!(file["file_name"], "demo.pdf");
}

#[test]
fn builds_voice_message_helpers() {
    assert!(is_opus_path(Path::new("voice.OPUS")));
    assert!(!is_opus_path(Path::new("voice.mp3")));
    assert_eq!(source_voice_stem(Path::new("/tmp/demo.mp3")), "demo");

    let candidates = voice_output_candidates(Path::new("/tmp/voice-work"), "feishu-voice.mp3");
    assert_eq!(
        candidates[0],
        PathBuf::from("/tmp/voice-work/feishu-voice.mp3")
    );
    assert_eq!(
        candidates[1],
        PathBuf::from("/tmp/voice-work/feishu-voice/feishu-voice.mp3")
    );
}

#[test]
fn builds_chat_create_and_tab_bodies() {
    let create = build_chat_create_body(&ChatCreateArgs {
        name: "AI 项目群".to_string(),
        description: Some("demo".to_string()),
        avatar: Some("img_avatar".to_string()),
        avatar_file: None,
        users: vec!["ou_user".to_string()],
        bots: vec!["cli_bot".to_string()],
        owner_id: Some("ou_user".to_string()),
        user_id_type: UserIdTypeArg::OpenId,
        chat_type: "private".to_string(),
        group_message_type: "chat".to_string(),
        set_bot_manager: true,
        uuid: Some("uuid".to_string()),
        body_json: None,
        body_file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(create["name"], "AI 项目群");
    assert_eq!(create["avatar"], "img_avatar");
    assert_eq!(create["user_id_list"][0], "ou_user");
    assert_eq!(create["bot_id_list"][0], "cli_bot");

    let tab = build_chat_tab_body(
        &ChatTabWriteArgs {
            chat_id: "oc_chat".to_string(),
            tab_id: None,
            name: Some("项目页".to_string()),
            tab_type: "url".to_string(),
            url: Some("https://example.com".to_string()),
            doc: None,
            icon_key: Some("img_icon".to_string()),
            icon_file: None,
            built_in: true,
            body_json: None,
            body_file: None,
            stdin: false,
        },
        false,
        None,
    )
    .unwrap();
    assert_eq!(tab["chat_tabs"][0]["tab_name"], "项目页");
    assert_eq!(
        tab["chat_tabs"][0]["tab_content"]["url"],
        "https://example.com"
    );
    assert_eq!(tab["chat_tabs"][0]["tab_config"]["icon_key"], "img_icon");
    assert_eq!(tab["chat_tabs"][0]["tab_config"]["is_built_in"], true);
}

use super::super::*;

#[test]
fn builds_helpdesk_queries_bodies_and_auth() {
    assert_eq!(HelpdeskReceiveTypeArg::Chat.as_api_value(), "chat");
    assert_eq!(HelpdeskReceiveTypeArg::User.as_api_value(), "user");

    let query = helpdesk_ticket_list_query(HelpdeskTicketListArgs {
        ticket_id: Some("t1".to_string()),
        agent_id: Some("ou_agent".to_string()),
        closed_by_id: None,
        ticket_type: Some(2),
        channel: None,
        solved: Some(1),
        score: None,
        status_list: vec![2, 5],
        guest_name: Some("张三".to_string()),
        guest_id: None,
        tags: vec!["urgent".to_string(), "".to_string()],
        page: 2,
        page_size: 50,
        create_time_start: Some(1760000000000),
        create_time_end: Some(1760003600000),
        update_time_start: None,
        update_time_end: None,
    })
    .unwrap();
    assert!(query.contains(&("page".to_string(), "2".to_string())));
    assert!(query.contains(&("page_size".to_string(), "50".to_string())));
    assert!(query.contains(&("type".to_string(), "2".to_string())));
    assert_eq!(
        query.iter().filter(|(key, _)| key == "status_list").count(),
        2
    );
    assert!(query.contains(&("tags".to_string(), "urgent".to_string())));
    assert!(helpdesk_page_number_query(0, 20, 200).is_err());
    assert!(helpdesk_page_number_query(1, 201, 200).is_err());

    let start = build_helpdesk_service_start_body(HelpdeskServiceStartArgs {
        open_id: Some("ou_user".to_string()),
        human_service: true,
        appointed_agents: vec!["ou_agent".to_string()],
        customized_info: Some("from cli".to_string()),
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(start["open_id"], "ou_user");
    assert_eq!(start["human_service"], true);
    assert_eq!(start["appointed_agents"][0], "ou_agent");
    assert!(build_helpdesk_service_start_body(HelpdeskServiceStartArgs {
        open_id: Some("ou_user".to_string()),
        human_service: false,
        appointed_agents: vec!["ou_agent".to_string()],
        customized_info: None,
        body_json: None,
        file: None,
        stdin: false,
    })
    .is_err());

    let message = build_helpdesk_message_send_body(HelpdeskMessageSendArgs {
        receiver_id: Some("ou_user".to_string()),
        msg_type: "text".to_string(),
        text: Some("hello".to_string()),
        content_json: None,
        receive_type: HelpdeskReceiveTypeArg::User,
        user_id_type: UserIdTypeArg::OpenId,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(message["msg_type"], "text");
    assert_eq!(message["receiver_id"], "ou_user");
    assert_eq!(message["receive_type"], "user");
    let content: Value = serde_json::from_str(message["content"].as_str().unwrap()).unwrap();
    assert_eq!(content["text"], "hello");

    let api = FeishuClient::new(Config {
        app_id: "cli_xxx".to_string(),
        app_secret: "secret".to_string(),
        base_url: FEISHU_BASE_URL.to_string(),
        default_user_id: None,
        user_access_token: None,
        helpdesk_id: Some("12345".to_string()),
        helpdesk_token: Some("ht-token".to_string()),
        default_wiki_space_id: None,
        default_wiki_parent_node_token: None,
        default_doc_create_wiki: false,
        doc_base_url: "https://my.feishu.cn/docx".to_string(),
    });
    assert_eq!(api.helpdesk_auth_header().unwrap(), "MTIzNDU6aHQtdG9rZW4=");
}

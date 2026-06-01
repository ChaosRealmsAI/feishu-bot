use super::*;

pub(super) async fn run_helpdesk_command(
    api: &mut FeishuClient,
    command: HelpdeskCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        HelpdeskCommand::Ticket(HelpdeskTicketCommand::List(args)) => {
            let query = helpdesk_ticket_list_query(args)?;
            api.get_helpdesk_json("/helpdesk/v1/tickets", &query)
                .await?
        }
        HelpdeskCommand::Ticket(HelpdeskTicketCommand::Get(args)) => {
            let path = format!(
                "/helpdesk/v1/tickets/{}",
                encode_path_segment(&args.ticket_id)
            );
            api.get_helpdesk_json(&path, &[]).await?
        }
        HelpdeskCommand::Ticket(HelpdeskTicketCommand::Messages(args)) => {
            let mut query = helpdesk_page_number_query(args.page, args.page_size, 200)?;
            push_query_opt_i64(&mut query, "time_start", args.time_start);
            push_query_opt_i64(&mut query, "time_end", args.time_end);
            let path = format!(
                "/helpdesk/v1/tickets/{}/messages",
                encode_path_segment(&args.ticket_id)
            );
            api.get_helpdesk_json(&path, &query).await?
        }
        HelpdeskCommand::Service(HelpdeskServiceCommand::Start(args)) => {
            let body = build_helpdesk_service_start_body(args)?;
            api.post_helpdesk_json("/helpdesk/v1/start_service", &[], body)
                .await?
        }
        HelpdeskCommand::Message(HelpdeskMessageCommand::Send(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_helpdesk_message_send_body(args)?;
            api.post_helpdesk_json("/helpdesk/v1/message", &query, body)
                .await?
        }
        HelpdeskCommand::Faq(HelpdeskFaqCommand::Categories(args)) => {
            let mut query = Vec::new();
            push_query_opt(&mut query, "lang", args.lang);
            push_query_opt_u8(&mut query, "order_by", args.order_by);
            if let Some(asc) = args.asc {
                query.push(("asc".to_string(), asc.to_string()));
            }
            api.get_helpdesk_json("/helpdesk/v1/categories", &query)
                .await?
        }
        HelpdeskCommand::Faq(HelpdeskFaqCommand::List(args)) => {
            if args.page_size == 0 || args.page_size > 100 {
                bail!("helpdesk faq list page_size must be between 1 and 100");
            }
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "category_id", args.category_id);
            push_query_opt(&mut query, "status", args.status);
            push_query_opt(&mut query, "search", args.search);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_helpdesk_json("/helpdesk/v1/faqs", &query).await?
        }
    };
    print_response(raw_json, "helpdesk operation completed", data)
}

pub(super) fn helpdesk_page_number_query(
    page: u32,
    page_size: u16,
    max_page_size: u16,
) -> Result<Vec<(String, String)>> {
    if page == 0 {
        bail!("helpdesk page must be at least 1");
    }
    if page_size == 0 || page_size > max_page_size {
        bail!("helpdesk page_size must be between 1 and {max_page_size}");
    }
    Ok(vec![
        ("page".to_string(), page.to_string()),
        ("page_size".to_string(), page_size.to_string()),
    ])
}

pub(super) fn helpdesk_ticket_list_query(
    args: HelpdeskTicketListArgs,
) -> Result<Vec<(String, String)>> {
    let mut query = helpdesk_page_number_query(args.page, args.page_size, 200)?;
    push_query_opt(&mut query, "ticket_id", args.ticket_id);
    push_query_opt(&mut query, "agent_id", args.agent_id);
    push_query_opt(&mut query, "closed_by_id", args.closed_by_id);
    push_query_opt_u8(&mut query, "type", args.ticket_type);
    push_query_opt_u8(&mut query, "channel", args.channel);
    push_query_opt_u8(&mut query, "solved", args.solved);
    push_query_opt_u8(&mut query, "score", args.score);
    for status in args.status_list {
        query.push(("status_list".to_string(), status.to_string()));
    }
    push_query_opt(&mut query, "guest_name", args.guest_name);
    push_query_opt(&mut query, "guest_id", args.guest_id);
    push_query_repeated(&mut query, "tags", args.tags);
    push_query_opt_i64(&mut query, "create_time_start", args.create_time_start);
    push_query_opt_i64(&mut query, "create_time_end", args.create_time_end);
    push_query_opt_i64(&mut query, "update_time_start", args.update_time_start);
    push_query_opt_i64(&mut query, "update_time_end", args.update_time_end);
    Ok(query)
}

pub(super) fn build_helpdesk_service_start_body(args: HelpdeskServiceStartArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "helpdesk start_service body",
        );
    }
    let open_id = args
        .open_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("helpdesk service start needs --open-id unless raw JSON is used"))?;
    let appointed_agents = clean_string_values(args.appointed_agents);
    if !appointed_agents.is_empty() && !args.human_service {
        bail!("helpdesk service start with --appointed-agent also needs --human-service");
    }
    let mut body = Map::new();
    body.insert("open_id".to_string(), Value::String(open_id));
    body.insert("human_service".to_string(), Value::Bool(args.human_service));
    if !appointed_agents.is_empty() {
        body.insert("appointed_agents".to_string(), json!(appointed_agents));
    }
    insert_opt_string(&mut body, "customized_info", args.customized_info);
    Ok(Value::Object(body))
}

pub(super) fn build_helpdesk_message_send_body(args: HelpdeskMessageSendArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "helpdesk message body",
        );
    }
    let receiver_id = args
        .receiver_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("helpdesk message send needs --receiver-id unless raw JSON is used")
        })?;
    let content = if let Some(text) = args.text.filter(|value| !value.trim().is_empty()) {
        serde_json::to_string(&json!({ "text": text }))
            .context("serialize helpdesk text content")?
    } else if let Some(content_json) = args.content_json.filter(|value| !value.trim().is_empty()) {
        let value = parse_json_value(&content_json, "helpdesk message content JSON")?;
        if !value.is_object() {
            bail!("helpdesk --content-json must be a JSON object");
        }
        serde_json::to_string(&value).context("serialize helpdesk message content")?
    } else {
        bail!("helpdesk message send needs --text or --content-json unless raw JSON is used");
    };
    Ok(json!({
        "msg_type": args.msg_type,
        "content": content,
        "receiver_id": receiver_id,
        "receive_type": args.receive_type.as_api_value(),
    }))
}

impl HelpdeskReceiveTypeArg {
    pub(super) fn as_api_value(self) -> &'static str {
        match self {
            HelpdeskReceiveTypeArg::Chat => "chat",
            HelpdeskReceiveTypeArg::User => "user",
        }
    }
}

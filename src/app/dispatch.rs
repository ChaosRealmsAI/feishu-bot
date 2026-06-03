use super::*;

pub(super) async fn run_api_command(
    command: Commands,
    config: Config,
    raw_json: bool,
) -> Result<()> {
    match command {
        Commands::Ai
        | Commands::Manifest(_)
        | Commands::Scopes(_)
        | Commands::Browser(_)
        | Commands::Setup(_)
        | Commands::Board(BoardCommand::Template(_))
        | Commands::Board(BoardCommand::CheckSvg(_))
        | Commands::Board(BoardCommand::Svg(BoardSvgArgs {
            print_nodes: true,
            whiteboard_id: None,
            ..
        }))
        | Commands::Doc(DocCommand::Capabilities)
        | Commands::Doc(DocCommand::Template(_))
        | Commands::Doc(DocCommand::Preview(_)) => {
            unreachable!("non-API commands are handled before config loading")
        }
        Commands::Doctor => run_doctor_command(&config, raw_json).await,
        Commands::Token(args) => run_token_command(config, args, raw_json).await,
        Commands::Oauth(command) => run_oauth_command(&config, command, raw_json).await,
        Commands::Message(command) => {
            let mut api = FeishuClient::new(config);
            run_message_command(&mut api, command, raw_json).await
        }
        Commands::Bot(command) => {
            let mut api = FeishuClient::new(config);
            run_bot_command(&mut api, command, raw_json).await
        }
        Commands::Dogfood(command) => {
            let mut api = FeishuClient::new(config);
            run_dogfood_command(&mut api, command, raw_json).await
        }
        Commands::Office(command) => {
            let mut api = FeishuClient::new(config);
            run_office_command(&mut api, command, raw_json).await
        }
        Commands::Contact(command) => {
            let mut api = FeishuClient::new(config);
            run_contact_command(&mut api, command, raw_json).await
        }
        Commands::Directory(command) => {
            let mut api = FeishuClient::new(config);
            run_directory_command(&mut api, command, raw_json).await
        }
        Commands::Board(command) => {
            let mut api = FeishuClient::new(config);
            run_board_command(&mut api, command, raw_json).await
        }
        Commands::Notify(args) => run_notify_command(config, args, raw_json).await,
        Commands::Chat(command) => {
            let mut api = FeishuClient::new(config);
            run_chat_command(&mut api, command, raw_json).await
        }
        Commands::Base(command) => {
            let mut api = FeishuClient::new(config);
            run_base_command(&mut api, command, raw_json).await
        }
        Commands::Task(command) => {
            let mut api = FeishuClient::new(config);
            run_task_command(&mut api, command, raw_json).await
        }
        Commands::Drive(command) => {
            let mut api = FeishuClient::new(config);
            run_drive_command(&mut api, command, raw_json).await
        }
        Commands::Calendar(command) => {
            let mut api = FeishuClient::new(config);
            run_calendar_command(&mut api, command, raw_json).await
        }
        Commands::Vc(command) => {
            let mut api = FeishuClient::new(config);
            run_vc_command(&mut api, command, raw_json).await
        }
        Commands::Minutes(command) => {
            let mut api = FeishuClient::new(config);
            run_minutes_command(&mut api, command, raw_json).await
        }
        Commands::Search(command) => {
            let mut api = FeishuClient::new(config);
            run_search_command(&mut api, command, raw_json).await
        }
        Commands::Okr(command) => {
            let mut api = FeishuClient::new(config);
            run_okr_command(&mut api, command, raw_json).await
        }
        Commands::Attendance(command) => {
            let mut api = FeishuClient::new(config);
            run_attendance_command(&mut api, command, raw_json).await
        }
        Commands::Mail(command) => {
            let mut api = FeishuClient::new(config);
            run_mail_command(&mut api, command, raw_json).await
        }
        Commands::Corehr(command) => {
            let mut api = FeishuClient::new(config);
            run_corehr_command(&mut api, command, raw_json).await
        }
        Commands::Helpdesk(command) => {
            let mut api = FeishuClient::new(config);
            run_helpdesk_command(&mut api, command, raw_json).await
        }
        Commands::Hire(command) => {
            let mut api = FeishuClient::new(config);
            run_hire_command(&mut api, command, raw_json).await
        }
        Commands::Wiki(command) => {
            let mut api = FeishuClient::new(config);
            run_wiki_command(&mut api, command, raw_json).await
        }
        Commands::Sheet(command) => {
            let mut api = FeishuClient::new(config);
            run_sheet_command(&mut api, command, raw_json).await
        }
        Commands::Approval(command) => {
            let mut api = FeishuClient::new(config);
            run_approval_command(&mut api, command, raw_json).await
        }
        Commands::Api(command) => {
            let mut api = FeishuClient::new(config);
            run_raw_api_command(&mut api, command, raw_json).await
        }
        Commands::Doc(command) => {
            let mut api = FeishuClient::new(config);
            run_doc_command(&mut api, command, raw_json).await
        }
    }
}

async fn run_token_command(config: Config, args: TokenArgs, raw_json: bool) -> Result<()> {
    let mut api = FeishuClient::new(config);
    let token = api.tenant_token().await?;
    if raw_json {
        let output = if args.raw {
            json!({ "tenant_access_token": token })
        } else {
            json!({ "tenant_access_token": mask_secret(&token) })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if args.raw {
        println!("{token}");
    } else {
        println!("tenant_access_token={} ", mask_secret(&token));
    }
    Ok(())
}

async fn run_notify_command(config: Config, args: NotifyArgs, raw_json: bool) -> Result<()> {
    let mut api = FeishuClient::new(config);
    let body = read_content(args.text.clone(), args.file.clone(), args.stdin)?;
    let receive_id = if let Some(to) = args.to.clone() {
        to
    } else {
        get_or_create_project_chat(&mut api, &args.project).await?
    };
    let receive_id_type = if args.to.is_some() {
        args.to_type.resolve(&receive_id)
    } else {
        "chat_id"
    };
    let card = build_notification_card(&args, &body);
    let data = api
        .send_interactive(&receive_id, receive_id_type, card, None)
        .await?;
    print_response(raw_json, "notification sent", data)
}

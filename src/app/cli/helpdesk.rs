use super::*;

#[derive(Subcommand)]
#[command(after_long_help = HELPDESK_AFTER_HELP)]
pub(in crate::app) enum HelpdeskCommand {
    #[command(subcommand, about = "Read helpdesk tickets and ticket messages")]
    Ticket(HelpdeskTicketCommand),
    #[command(subcommand, about = "Create helpdesk conversations")]
    Service(HelpdeskServiceCommand),
    #[command(subcommand, about = "Send messages through the helpdesk bot")]
    Message(HelpdeskMessageCommand),
    #[command(subcommand, about = "Read helpdesk FAQ categories and articles")]
    Faq(HelpdeskFaqCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskTicketCommand {
    #[command(about = "List tickets with official filters")]
    List(HelpdeskTicketListArgs),
    #[command(about = "Get one ticket detail")]
    Get(HelpdeskTicketGetArgs),
    #[command(about = "List ticket messages")]
    Messages(HelpdeskTicketMessagesArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskServiceCommand {
    #[command(about = "Create a helpdesk conversation for a user")]
    Start(HelpdeskServiceStartArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskMessageCommand {
    #[command(about = "Send text/post/image/card through the helpdesk bot")]
    Send(HelpdeskMessageSendArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskFaqCommand {
    #[command(about = "List FAQ categories")]
    Categories(HelpdeskFaqCategoriesArgs),
    #[command(about = "List FAQ articles")]
    List(HelpdeskFaqListArgs),
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketListArgs {
    #[arg(long, help = "Ticket ID filter")]
    pub(in crate::app) ticket_id: Option<String>,

    #[arg(long, help = "Agent open_id filter")]
    pub(in crate::app) agent_id: Option<String>,

    #[arg(long, help = "Closed-by agent open_id filter")]
    pub(in crate::app) closed_by_id: Option<String>,

    #[arg(long = "type", help = "Ticket type: 1 bot, 2 human")]
    pub(in crate::app) ticket_type: Option<u8>,

    #[arg(long, help = "Ticket channel")]
    pub(in crate::app) channel: Option<u8>,

    #[arg(long, help = "Solved filter: 1 unsolved, 2 solved")]
    pub(in crate::app) solved: Option<u8>,

    #[arg(long, help = "Score filter: 1 dissatisfied, 2 normal, 3 satisfied")]
    pub(in crate::app) score: Option<u8>,

    #[arg(long = "status", help = "Ticket status; can repeat")]
    pub(in crate::app) status_list: Vec<u8>,

    #[arg(long, help = "Guest name filter")]
    pub(in crate::app) guest_name: Option<String>,

    #[arg(long, help = "Guest open_id filter")]
    pub(in crate::app) guest_id: Option<String>,

    #[arg(long = "tag", help = "Ticket tag; can repeat")]
    pub(in crate::app) tags: Vec<String>,

    #[arg(long, default_value_t = 1, help = "Page number, starts from 1")]
    pub(in crate::app) page: u32,

    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Create time start, Unix milliseconds")]
    pub(in crate::app) create_time_start: Option<i64>,

    #[arg(long, help = "Create time end, Unix milliseconds")]
    pub(in crate::app) create_time_end: Option<i64>,

    #[arg(long, help = "Update time start, Unix milliseconds")]
    pub(in crate::app) update_time_start: Option<i64>,

    #[arg(long, help = "Update time end, Unix milliseconds")]
    pub(in crate::app) update_time_end: Option<i64>,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketGetArgs {
    #[arg(long, help = "Ticket ID")]
    pub(in crate::app) ticket_id: String,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketMessagesArgs {
    #[arg(long, help = "Ticket ID")]
    pub(in crate::app) ticket_id: String,

    #[arg(long, help = "Message create time start")]
    pub(in crate::app) time_start: Option<i64>,

    #[arg(long, help = "Message create time end")]
    pub(in crate::app) time_end: Option<i64>,

    #[arg(long, default_value_t = 1, help = "Page number")]
    pub(in crate::app) page: u32,

    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskServiceStartArgs {
    #[arg(long, help = "User open_id")]
    pub(in crate::app) open_id: Option<String>,

    #[arg(long, help = "Directly enter human service")]
    pub(in crate::app) human_service: bool,

    #[arg(long = "appointed-agent", help = "Appointed agent open_id; can repeat")]
    pub(in crate::app) appointed_agents: Vec<String>,

    #[arg(long, help = "Customized source info, max 1024 chars")]
    pub(in crate::app) customized_info: Option<String>,

    #[arg(long, help = "Raw Feishu start_service body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw start_service body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw start_service body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskMessageSendArgs {
    #[arg(long, help = "Receiver user ID")]
    pub(in crate::app) receiver_id: Option<String>,

    #[arg(long, default_value = "text", help = "text, post, image, interactive")]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Plain text content; builds the official content string")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Native Feishu message content JSON object")]
    pub(in crate::app) content_json: Option<String>,

    #[arg(long, value_enum, default_value_t = HelpdeskReceiveTypeArg::Chat)]
    pub(in crate::app) receive_type: HelpdeskReceiveTypeArg,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw Feishu helpdesk message body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw helpdesk message body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw helpdesk message body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskFaqCategoriesArgs {
    #[arg(long, help = "Category language, for example zh_cn")]
    pub(in crate::app) lang: Option<String>,

    #[arg(long, help = "Sort key: 1 update time")]
    pub(in crate::app) order_by: Option<u8>,

    #[arg(long, help = "Sort ascending")]
    pub(in crate::app) asc: Option<bool>,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskFaqListArgs {
    #[arg(long, help = "FAQ category ID")]
    pub(in crate::app) category_id: Option<String>,

    #[arg(long, help = "FAQ status: 1 online, 0 deleted recoverable, 2 deleted")]
    pub(in crate::app) status: Option<String>,

    #[arg(long, help = "Search keyword")]
    pub(in crate::app) search: Option<String>,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,
}

use super::*;
#[derive(Parser)]
#[command(name = "feishu-bot")]
#[command(version)]
#[command(about = "Feishu Bot command-line automation for messages, office workflows, and docs")]
#[command(
    long_about = "AI-ready Feishu Bot command-line automation. Use it to run office workflows over project chats/Wiki/Base, verify bot credentials, send messages, send task notification cards, create/write/read native docx documents, operate Base records, create/update tasks, use Directory/Drive/Calendar/VC/Minutes/Search/OKR/Attendance/Mail/CoreHR/Helpdesk/Hire/Wiki/Sheets/Approval, call raw OpenAPI endpoints, and operate the local Playwright MCP browser bridge."
)]
#[command(after_long_help = ROOT_AFTER_HELP)]
pub(in crate::app) struct Cli {
    #[arg(
        long,
        global = true,
        help = "Use open.larksuite.com instead of open.feishu.cn"
    )]
    pub(in crate::app) lark: bool,

    #[arg(long, global = true, help = "Override OpenAPI base URL")]
    pub(in crate::app) base_url: Option<String>,

    #[arg(long, global = true, help = "Print raw JSON for machine parsing")]
    pub(in crate::app) json: bool,

    #[command(subcommand)]
    pub(in crate::app) command: Commands,
}

#[derive(Subcommand)]
pub(in crate::app) enum Commands {
    #[command(about = "Print the AI operator playbook")]
    Ai,
    #[command(about = "Print a machine-readable AI command/scope manifest")]
    Manifest(ManifestArgs),
    #[command(about = "Inspect local config and API connectivity")]
    Doctor,
    #[command(about = "Get a tenant_access_token")]
    Token(TokenArgs),
    #[command(subcommand, about = "Generate and exchange OAuth user tokens")]
    Oauth(OauthCommand),
    #[command(about = "Print known scope groups and Open Platform grant links")]
    Scopes(ScopesArgs),
    #[command(subcommand, about = "Inspect current Feishu app bot identity")]
    Bot(BotCommand),
    #[command(
        subcommand,
        about = "Automate initial env, scope, OAuth, browser, and Wiki permission setup"
    )]
    Setup(SetupCommand),
    #[command(subcommand, about = "Publish closed-loop AI dogfood demos")]
    Dogfood(DogfoodCommand),
    #[command(
        subcommand,
        about = "Run AI-friendly office workflows over chat, Wiki, Base, messages, and voice"
    )]
    Office(OfficeCommand),
    #[command(subcommand, about = "Send bot messages")]
    Message(MessageCommand),
    #[command(subcommand, about = "Query Feishu Contact users and departments")]
    Contact(ContactCommand),
    #[command(subcommand, about = "Search and batch-read Feishu Directory employees")]
    Directory(DirectoryCommand),
    #[command(about = "Send a task notification card")]
    Notify(NotifyArgs),
    #[command(subcommand, about = "Create or inspect chats")]
    Chat(ChatCommand),
    #[command(subcommand, about = "Create and write Feishu docx documents")]
    Doc(DocCommand),
    #[command(subcommand, about = "Operate Feishu Board / whiteboard nodes")]
    Board(BoardCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Base / Bitable apps, tables, fields, and records"
    )]
    Base(BaseCommand),
    #[command(subcommand, about = "Create, inspect, and update Feishu Task v2 tasks")]
    Task(TaskCommand),
    #[command(subcommand, about = "Operate Feishu Drive files and folders")]
    Drive(DriveCommand),
    #[command(subcommand, about = "Operate Feishu Calendar and events")]
    Calendar(CalendarCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Video Conferencing meetings, reports, rooms, and recordings"
    )]
    Vc(VcCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Minutes search, metadata, AI artifacts, media, and transcripts"
    )]
    Minutes(MinutesCommand),
    #[command(
        subcommand,
        about = "Search Feishu docs/messages and manage custom search connector indexes"
    )]
    Search(SearchCommand),
    #[command(
        subcommand,
        about = "Read Feishu OKR periods, period rules, and user OKRs"
    )]
    Okr(OkrCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Attendance groups, shifts, schedules, flows, and stats"
    )]
    Attendance(AttendanceCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Mail messages, folders, contacts, aliases, and settings"
    )]
    Mail(MailCommand),
    #[command(
        subcommand,
        about = "Operate Feishu CoreHR departments, jobs, job data, persons, and processes"
    )]
    Corehr(CorehrCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Helpdesk tickets, messages, and FAQs"
    )]
    Helpdesk(HelpdeskCommand),
    #[command(
        subcommand,
        about = "Operate Feishu Hire jobs, talents, applications, and interviews"
    )]
    Hire(HireCommand),
    #[command(subcommand, about = "Operate Feishu Wiki spaces and nodes")]
    Wiki(WikiCommand),
    #[command(subcommand, about = "Operate Feishu Sheets")]
    Sheet(SheetCommand),
    #[command(subcommand, about = "Operate Feishu Approval instances")]
    Approval(ApprovalCommand),
    #[command(
        subcommand,
        about = "Call raw Feishu OpenAPI endpoints with tenant token"
    )]
    Api(ApiCommand),
    #[command(subcommand, about = "Use the local Playwright MCP browser bridge")]
    Browser(BrowserCommand),
}

#[derive(Subcommand)]
#[command(after_long_help = BOT_AFTER_HELP)]
pub(in crate::app) enum BotCommand {
    #[command(about = "Get current app bot info, including bot open_id")]
    Info,
}

#[derive(Subcommand)]
#[command(after_long_help = DOGFOOD_AFTER_HELP)]
pub(in crate::app) enum DogfoodCommand {
    #[command(about = "Create a standalone docx, send it, and verify delivery loop")]
    Publish(DogfoodPublishArgs),
    #[command(about = "Run real OpenAPI probes and classify current capability gaps")]
    Verify(DogfoodVerifyArgs),
}

#[derive(Args)]
#[command(after_long_help = DOGFOOD_AFTER_HELP)]
pub(in crate::app) struct DogfoodVerifyArgs {
    #[arg(
        long,
        help = "Filter modules/probes; repeat for multiple modules, such as --module calendar --module task"
    )]
    pub(in crate::app) module: Vec<String>,

    #[arg(
        long,
        help = "Include raw API response/error payloads in each probe result"
    )]
    pub(in crate::app) include_response: bool,

    #[arg(
        long,
        help = "Also run side-effect write probes for doc/base/task where possible"
    )]
    pub(in crate::app) write: bool,

    #[arg(
        long,
        help = "Send a real message and read it back as part of the verification"
    )]
    pub(in crate::app) send_loop_check: bool,

    #[arg(
        long,
        help = "When user-token probes return expired_user_token, refresh FEISHU_USER_ACCESS_TOKEN with FEISHU_REFRESH_TOKEN, save it, and retry those probes"
    )]
    pub(in crate::app) auto_refresh_user_token: bool,

    #[arg(
        long,
        help = "Env file for --auto-refresh-user-token; defaults to FEISHU_ENV_FILE/LARK_ENV_FILE or private/local.env"
    )]
    pub(in crate::app) refresh_env_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Receiver for --send-loop-check; defaults to FEISHU_USER_ID"
    )]
    pub(in crate::app) to: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type for --send-loop-check"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,
}

#[derive(Args)]
#[command(after_long_help = DOGFOOD_AFTER_HELP)]
pub(in crate::app) struct DogfoodPublishArgs {
    #[arg(long, help = "Dogfood document title")]
    pub(in crate::app) title: String,

    #[arg(long, help = "Optional Drive folder token for document placement")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(long, value_enum, default_value_t = WriterArg::Official)]
    pub(in crate::app) writer: WriterArg,

    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(long, help = "Markdown-ish, Markdown, or HTML content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read content from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Receiver for the demo link; defaults to FEISHU_USER_ID")]
    pub(in crate::app) to: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        help = "Attempt Wiki move even when FEISHU_DOC_CREATE_WIKI_DEFAULT is false"
    )]
    pub(in crate::app) wiki: bool,

    #[arg(long, help = "Do not attempt Wiki publishing")]
    pub(in crate::app) no_wiki: bool,

    #[arg(long, help = "Target Wiki space ID")]
    pub(in crate::app) wiki_space_id: Option<String>,

    #[arg(long, help = "Target parent Wiki node token")]
    pub(in crate::app) wiki_parent_token: Option<String>,

    #[arg(long, help = "Ask Feishu to apply for Wiki move approval")]
    pub(in crate::app) wiki_apply: bool,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for Wiki move"
    )]
    pub(in crate::app) wiki_auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TokenArgs {
    #[arg(long, help = "Print the full token. Treat output as a secret.")]
    pub(in crate::app) raw: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = OAUTH_AFTER_HELP)]
pub(in crate::app) enum OauthCommand {
    #[command(about = "Build a Feishu OAuth authorization URL with PKCE")]
    Url(OauthUrlArgs),
    #[command(about = "Exchange an OAuth authorization code for a user_access_token")]
    Token(OauthTokenArgs),
    #[command(about = "Refresh a user_access_token with refresh_token")]
    Refresh(OauthRefreshArgs),
    #[command(about = "Read user info with FEISHU_USER_ACCESS_TOKEN")]
    UserInfo(OauthUserInfoArgs),
}

#[derive(Args)]
#[command(after_long_help = OAUTH_AFTER_HELP)]
pub(in crate::app) struct OauthUrlArgs {
    #[arg(
        long,
        help = "Registered OAuth redirect URI; defaults to FEISHU_OAUTH_REDIRECT_URI or http://localhost:8080/callback"
    )]
    pub(in crate::app) redirect_uri: Option<String>,

    #[arg(long, help = "OAuth scopes; repeat or pass a space-separated string")]
    pub(in crate::app) scope: Vec<String>,

    #[arg(long, help = "OAuth state; defaults to a random UUID")]
    pub(in crate::app) state: Option<String>,

    #[arg(long, help = "PKCE code_verifier; defaults to a generated value")]
    pub(in crate::app) code_verifier: Option<String>,

    #[arg(long, help = "Disable PKCE code_challenge generation")]
    pub(in crate::app) no_pkce: bool,
}

#[derive(Args)]
#[command(after_long_help = OAUTH_AFTER_HELP)]
pub(in crate::app) struct OauthTokenArgs {
    #[arg(long, help = "Authorization code from redirect_uri?code=...")]
    pub(in crate::app) code: String,

    #[arg(long, help = "Registered OAuth redirect URI used by oauth url")]
    pub(in crate::app) redirect_uri: Option<String>,

    #[arg(long, help = "PKCE code_verifier printed by oauth url")]
    pub(in crate::app) code_verifier: Option<String>,

    #[arg(long, help = "Print full token JSON. Treat output as a secret.")]
    pub(in crate::app) raw: bool,

    #[arg(
        long,
        help = "Print shell export lines with full tokens. Treat output as secret."
    )]
    pub(in crate::app) print_env: bool,

    #[arg(
        long,
        help = "Persist FEISHU_USER_ACCESS_TOKEN and FEISHU_REFRESH_TOKEN into an env file"
    )]
    pub(in crate::app) save_env: bool,

    #[arg(long, help = "Env file for --save-env; defaults to ./.env")]
    pub(in crate::app) env_file: Option<PathBuf>,
}

#[derive(Args)]
#[command(after_long_help = OAUTH_AFTER_HELP)]
pub(in crate::app) struct OauthRefreshArgs {
    #[arg(
        long,
        help = "Refresh token; defaults to FEISHU_REFRESH_TOKEN or LARK_REFRESH_TOKEN"
    )]
    pub(in crate::app) refresh_token: Option<String>,

    #[arg(long, help = "Print full token JSON. Treat output as a secret.")]
    pub(in crate::app) raw: bool,

    #[arg(
        long,
        help = "Print shell export lines with full tokens. Treat output as secret."
    )]
    pub(in crate::app) print_env: bool,

    #[arg(
        long,
        help = "Persist FEISHU_USER_ACCESS_TOKEN and FEISHU_REFRESH_TOKEN into an env file"
    )]
    pub(in crate::app) save_env: bool,

    #[arg(long, help = "Env file for --save-env; defaults to ./.env")]
    pub(in crate::app) env_file: Option<PathBuf>,
}

#[derive(Args)]
#[command(after_long_help = OAUTH_AFTER_HELP)]
pub(in crate::app) struct OauthUserInfoArgs {
    #[arg(long, help = "User access token; defaults to FEISHU_USER_ACCESS_TOKEN")]
    pub(in crate::app) access_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct ManifestArgs {
    #[arg(long, help = "Filter module names/commands, such as base, task, doc")]
    pub(in crate::app) module: Option<String>,

    #[arg(long, help = "Print compact one-line JSON")]
    pub(in crate::app) compact: bool,
}

#[derive(Args)]
pub(in crate::app) struct ScopesArgs {
    #[arg(
        long,
        default_value = "all",
        help = "Scope group: all, user-token, im, contact, directory, doc, board, base, task, drive, permission, calendar, vc, minutes, search, okr, attendance, mail, corehr, helpdesk, hire, wiki, sheet, approval"
    )]
    pub(in crate::app) group: String,

    #[arg(
        long = "token-type",
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Grant URL token type"
    )]
    pub(in crate::app) token_type: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum MessageCommand {
    #[command(about = "List historical messages in a chat or thread")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    List(MessageListArgs),
    #[command(about = "Get one message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Get(MessageGetArgs),
    #[command(about = "Send a text message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Send(SendMessageArgs),
    #[command(about = "Send a text message and read back message/chat proof")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    LoopCheck(MessageLoopCheckArgs),
    #[command(about = "Send any Feishu message type with native content JSON")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    SendJson(SendJsonMessageArgs),
    #[command(about = "Upload an image for message/avatar use and print image_key")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    UploadImage(UploadImageArgs),
    #[command(about = "Upload a file/video/audio for message use and print file_key")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    UploadFile(UploadFileArgs),
    #[command(about = "Upload and send an image message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    SendImage(SendImageMessageArgs),
    #[command(about = "Upload and send a file/video/audio message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    SendFile(SendFileMessageArgs),
    #[command(about = "Create/send a Feishu voice message from text or an audio file")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    SendVoice(SendVoiceMessageArgs),
    #[command(about = "Download an app-uploaded message image by image_key")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    DownloadImage(DownloadImageArgs),
    #[command(about = "Download an app-uploaded message file by file_key")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    DownloadFile(DownloadFileArgs),
    #[command(about = "Reply to a message with native content JSON")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    ReplyJson(ReplyJsonMessageArgs),
    #[command(about = "Reply to a message with plain text")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Reply(MessageReplyArgs),
    #[command(about = "Acknowledge a message with an emoji status and optional reply")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Ack(MessageAckArgs),
    #[command(
        about = "Poll new chat messages and optionally ack/reply while maintaining a local cursor"
    )]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Poll(MessagePollArgs),
    #[command(about = "Edit a sent message with native content JSON")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    EditJson(EditJsonMessageArgs),
    #[command(about = "Delete/revoke a message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Delete(DeleteMessageArgs),
    #[command(about = "List read users for one sent message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    ReadUsers(MessageReadUsersArgs),
    #[command(about = "Download a resource from a message")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Resource(MessageResourceArgs),
    #[command(subcommand, about = "Operate message reactions")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Reaction(MessageReactionCommand),
    #[command(subcommand, about = "Operate pinned messages")]
    #[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
    Pin(MessagePinCommand),
}

#[derive(Args)]
pub(in crate::app) struct MessageListArgs {
    #[arg(long, help = "Chat/thread/open_message_id container ID")]
    pub(in crate::app) container_id: String,

    #[arg(
        long,
        default_value = "chat",
        help = "Container type: chat, thread, or open_message_id"
    )]
    pub(in crate::app) container_id_type: String,

    #[arg(long, help = "Start Unix timestamp in seconds")]
    pub(in crate::app) start_time: Option<String>,

    #[arg(long, help = "End Unix timestamp in seconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, default_value = "ByCreateTimeDesc", help = "Sort type")]
    pub(in crate::app) sort_type: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct MessageGetArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Return original card JSON for card messages")]
    pub(in crate::app) user_card_content: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct SendMessageArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(long, help = "Message text")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Read message text from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message text from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct MessageLoopCheckArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(long, help = "Loop-check text. Generated when omitted.")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Read loop-check text from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read loop-check text from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency UUID for Feishu send API")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct SendJsonMessageArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        help = "Feishu msg_type: text, post, interactive, image, file, etc."
    )]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Raw Feishu message content JSON object")]
    pub(in crate::app) content_json: Option<String>,

    #[arg(long, help = "Read message content JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message content JSON object from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct UploadImageArgs {
    #[arg(
        long,
        help = "Local image path. Supported by Feishu: JPG/JPEG/PNG/WEBP/GIF/BMP/ICO/TIFF/HEIC"
    )]
    pub(in crate::app) file: PathBuf,

    #[arg(
        long,
        default_value = "message",
        help = "Image type: message or avatar"
    )]
    pub(in crate::app) image_type: String,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct UploadFileArgs {
    #[arg(long, help = "Local file path. For video messages use an MP4 file.")]
    pub(in crate::app) file: PathBuf,

    #[arg(
        long = "file-type",
        default_value = "stream",
        help = "Feishu file_type: opus, mp4, pdf, doc, xls, ppt, or stream"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Override uploaded file name, including extension")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Audio/video duration in milliseconds")]
    pub(in crate::app) duration: Option<u64>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct SendImageMessageArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(long, help = "Local image path")]
    pub(in crate::app) file: PathBuf,

    #[arg(long, default_value = "message", help = "Image type, normally message")]
    pub(in crate::app) image_type: String,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct SendFileMessageArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(long, help = "Local file path. For video messages use an MP4 file.")]
    pub(in crate::app) file: PathBuf,

    #[arg(
        long = "file-type",
        default_value = "stream",
        help = "Feishu file_type: opus, mp4, pdf, doc, xls, ppt, or stream"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Override uploaded file name, including extension")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Audio/video duration in milliseconds")]
    pub(in crate::app) duration: Option<u64>,

    #[arg(
        long = "msg-type",
        default_value = "auto",
        help = "Message type to send after upload: auto, file, media, or audio. auto maps mp4 -> media, opus -> audio, otherwise file"
    )]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Optional image_key cover for media/video messages")]
    pub(in crate::app) cover_image_key: Option<String>,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct SendVoiceMessageArgs {
    #[arg(long, short = 't', help = "Receiver ID: open_id/user_id/email/chat_id")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type; auto infers from prefix"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        help = "Existing audio file. OPUS is sent directly; MP3/WAV/M4A/etc. are converted to OPUS with ffmpeg"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Text to synthesize with vox before sending")]
    pub(in crate::app) text: Option<String>,

    #[arg(long = "text-file", help = "Read synthesis text from file")]
    pub(in crate::app) text_file: Option<PathBuf>,

    #[arg(long, help = "Read synthesis text from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(
        long = "vox-bin",
        default_value = "vox",
        help = "vox binary used by --text/--text-file/--stdin"
    )]
    pub(in crate::app) vox_bin: PathBuf,

    #[arg(long, help = "vox voice name/id, passed as --voice")]
    pub(in crate::app) voice: Option<String>,

    #[arg(
        long = "vox-timeout-ms",
        default_value_t = 120_000,
        help = "Maximum milliseconds to wait for vox output"
    )]
    pub(in crate::app) vox_timeout_ms: u64,

    #[arg(
        long = "ffmpeg-bin",
        default_value = "ffmpeg",
        help = "ffmpeg binary used to convert non-OPUS audio"
    )]
    pub(in crate::app) ffmpeg_bin: PathBuf,

    #[arg(
        long = "ffprobe-bin",
        default_value = "ffprobe",
        help = "ffprobe binary used to detect audio duration"
    )]
    pub(in crate::app) ffprobe_bin: PathBuf,

    #[arg(
        long,
        help = "Audio duration in milliseconds. Auto-detected with ffprobe when omitted"
    )]
    pub(in crate::app) duration: Option<u64>,

    #[arg(
        long,
        help = "Uploaded OPUS file name. Defaults to the source stem with .opus"
    )]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long,
        help = "Keep generated/transcoded files and return their temp directory"
    )]
    pub(in crate::app) keep: bool,

    #[arg(
        long,
        help = "Read back the sent message by message_id and include proof"
    )]
    pub(in crate::app) readback: bool,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct DownloadImageArgs {
    #[arg(long, help = "image_key returned by message upload-image")]
    pub(in crate::app) image_key: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,
}

#[derive(Args)]
pub(in crate::app) struct DownloadFileArgs {
    #[arg(long, help = "file_key returned by message upload-file")]
    pub(in crate::app) file_key: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct ReplyJsonMessageArgs {
    #[arg(long, help = "Message ID to reply to")]
    pub(in crate::app) message_id: String,

    #[arg(
        long,
        help = "Feishu msg_type: text, post, interactive, image, file, etc."
    )]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Raw Feishu message content JSON object")]
    pub(in crate::app) content_json: Option<String>,

    #[arg(long, help = "Read message content JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message content JSON object from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct MessageReplyArgs {
    #[arg(long, help = "Message ID to reply to")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Reply text")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Read reply text from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reply text from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency UUID. Defaults to a random UUID.")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct MessageAckArgs {
    #[arg(long, help = "Message ID to acknowledge")]
    pub(in crate::app) message_id: String,

    #[arg(
        long = "emoji-type",
        default_value = "OK",
        help = "Feishu emoji_type used as the status marker. Common: OK, THUMBSUP, SMILE, THANKS"
    )]
    pub(in crate::app) emoji_type: String,

    #[arg(
        long,
        default_value = "read",
        help = "Local status label included in output, e.g. read, working, done, blocked"
    )]
    pub(in crate::app) status: String,

    #[arg(
        long = "reply-text",
        help = "Optional text reply after adding the reaction"
    )]
    pub(in crate::app) reply_text: Option<String>,

    #[arg(long = "reply-file", help = "Read optional reply text from file")]
    pub(in crate::app) reply_file: Option<PathBuf>,

    #[arg(long = "reply-stdin", help = "Read optional reply text from stdin")]
    pub(in crate::app) reply_stdin: bool,

    #[arg(long, help = "Read back the message and reaction list after ack")]
    pub(in crate::app) readback: bool,

    #[arg(long, help = "Idempotency UUID for optional reply")]
    pub(in crate::app) uuid: Option<String>,
}

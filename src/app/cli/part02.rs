use super::*;
#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct MessagePollArgs {
    #[arg(long, help = "Chat ID to poll")]
    pub(in crate::app) chat_id: String,

    #[arg(long, default_value_t = 20, help = "Page size for message list")]
    pub(in crate::app) page_size: u16,

    #[arg(
        long,
        help = "Local state file. Defaults to ~/.config/feishu/message-state.json"
    )]
    pub(in crate::app) state_file: Option<PathBuf>,

    #[arg(long, help = "State key. Defaults to --chat-id")]
    pub(in crate::app) state_key: Option<String>,

    #[arg(long, help = "Override local cursor with this message_position")]
    pub(in crate::app) since_position: Option<u64>,

    #[arg(
        long,
        help = "On first run, save the latest cursor and return no messages"
    )]
    pub(in crate::app) from_now: bool,

    #[arg(long, help = "Save the newest fetched message_position after polling")]
    pub(in crate::app) mark_seen: bool,

    #[arg(
        long = "ack-emoji",
        help = "Add this emoji reaction to each new human message, e.g. OK or THUMBSUP"
    )]
    pub(in crate::app) ack_emoji: Option<String>,

    #[arg(
        long = "reply-text",
        help = "Reply with this text to each new human message"
    )]
    pub(in crate::app) reply_text: Option<String>,

    #[arg(
        long,
        help = "Include messages sent by apps/bots in returned/actioned messages"
    )]
    pub(in crate::app) include_app_messages: bool,

    #[arg(long, help = "Include system messages in returned/actioned messages")]
    pub(in crate::app) include_system_messages: bool,
}

#[derive(Args)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) struct EditJsonMessageArgs {
    #[arg(long, help = "Message ID to edit")]
    pub(in crate::app) message_id: String,

    #[arg(
        long,
        help = "Feishu msg_type; editing is usually limited to text/post"
    )]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Raw Feishu message content JSON object")]
    pub(in crate::app) content_json: Option<String>,

    #[arg(long, help = "Read message content JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message content JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DeleteMessageArgs {
    #[arg(long, help = "Message ID to delete/revoke")]
    pub(in crate::app) message_id: String,
}

#[derive(Args)]
pub(in crate::app) struct MessageReadUsersArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MessageResourceArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Resource file key from message content")]
    pub(in crate::app) file_key: String,

    #[arg(
        long = "type",
        default_value = "file",
        help = "Resource type: image or file"
    )]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Output file path")]
    pub(in crate::app) output: PathBuf,
}

#[derive(Subcommand)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) enum MessageReactionCommand {
    #[command(about = "List reactions on a message")]
    List(MessageReactionListArgs),
    #[command(about = "Add a reaction to a message")]
    Add(MessageReactionAddArgs),
    #[command(about = "Delete a reaction from a message")]
    Delete(MessageReactionDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct MessageReactionListArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Emoji type such as SMILE or LAUGH")]
    pub(in crate::app) reaction_type: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MessageReactionAddArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Emoji type such as SMILE or LAUGH")]
    pub(in crate::app) emoji_type: Option<String>,

    #[arg(long, help = "Raw reaction_type JSON object or full request body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read reaction body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reaction body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct MessageReactionDeleteArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,

    #[arg(long, help = "Reaction ID")]
    pub(in crate::app) reaction_id: String,
}

#[derive(Subcommand)]
#[command(after_long_help = MESSAGE_SEND_AFTER_HELP)]
pub(in crate::app) enum MessagePinCommand {
    #[command(about = "List pinned messages in a chat")]
    List(MessagePinListArgs),
    #[command(about = "Pin one message")]
    Add(MessagePinArgs),
    #[command(about = "Remove one pinned message")]
    Delete(MessagePinArgs),
}

#[derive(Args)]
pub(in crate::app) struct MessagePinListArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long, help = "Start Unix timestamp in seconds or milliseconds")]
    pub(in crate::app) start_time: Option<String>,

    #[arg(long, help = "End Unix timestamp in seconds or milliseconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MessagePinArgs {
    #[arg(long, help = "Message ID")]
    pub(in crate::app) message_id: String,
}

#[derive(Subcommand)]
#[command(after_long_help = CONTACT_AFTER_HELP)]
pub(in crate::app) enum ContactCommand {
    #[command(subcommand, about = "Query users")]
    User(ContactUserCommand),
    #[command(subcommand, about = "Query departments")]
    Department(ContactDepartmentCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum ContactUserCommand {
    #[command(about = "Get one user")]
    Get(ContactUserGetArgs),
    #[command(about = "List users, optionally under a department")]
    List(ContactUserListArgs),
}

#[derive(Args)]
pub(in crate::app) struct ContactUserGetArgs {
    #[arg(long, help = "User ID")]
    pub(in crate::app) user_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ContactUserListArgs {
    #[arg(
        long,
        help = "Department ID. Omit for all visible users when API allows it."
    )]
    pub(in crate::app) department_id: Option<String>,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum ContactDepartmentCommand {
    #[command(about = "Get one department")]
    Get(ContactDepartmentGetArgs),
    #[command(about = "List departments")]
    List(ContactDepartmentListArgs),
    #[command(about = "List child departments")]
    Children(ContactDepartmentChildrenArgs),
    #[command(about = "Search departments")]
    Search(ContactDepartmentSearchArgs),
}

#[derive(Args)]
pub(in crate::app) struct ContactDepartmentGetArgs {
    #[arg(long, help = "Department ID")]
    pub(in crate::app) department_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ContactDepartmentListArgs {
    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Optional parent department ID")]
    pub(in crate::app) parent_department_id: Option<String>,

    #[arg(long, help = "Whether to fetch deleted departments")]
    pub(in crate::app) fetch_child: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ContactDepartmentChildrenArgs {
    #[arg(
        long,
        default_value = "0",
        help = "Department ID; root department is 0"
    )]
    pub(in crate::app) department_id: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Fetch recursively")]
    pub(in crate::app) fetch_child: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ContactDepartmentSearchArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,
}

#[derive(Subcommand)]
#[command(after_long_help = DIRECTORY_AFTER_HELP)]
pub(in crate::app) enum DirectoryCommand {
    #[command(subcommand, about = "Search, filter, and batch-read employees")]
    Employee(DirectoryEmployeeCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum DirectoryEmployeeCommand {
    #[command(about = "Search employees by keyword, name, phone, email, or employee ID")]
    Search(DirectoryEmployeeSearchArgs),
    #[command(about = "Batch get employee details by IDs")]
    Mget(DirectoryEmployeeMgetArgs),
    #[command(about = "Filter employees by email, mobile, department/status, or job number")]
    Filter(DirectoryEmployeeFilterArgs),
}

#[derive(Args)]
pub(in crate::app) struct DirectoryEmployeeSearchArgs {
    #[arg(long, help = "Search keyword: employee ID, name, phone, or email")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long = "field", help = "Required field to return; can repeat, max 100")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, value_enum, default_value_t = DirectoryEmployeeIdTypeArg::OpenId)]
    pub(in crate::app) employee_id_type: DirectoryEmployeeIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryDepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DirectoryDepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryAuthArg::Tenant)]
    pub(in crate::app) auth: DirectoryAuthArg,

    #[arg(long, help = "Raw Feishu employees search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu employees search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu employees search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DirectoryEmployeeMgetArgs {
    #[arg(long = "employee-id", help = "Employee ID; can repeat, max 100")]
    pub(in crate::app) employee_ids: Vec<String>,

    #[arg(long = "field", help = "Required field to return; can repeat, max 100")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, value_enum, default_value_t = DirectoryEmployeeIdTypeArg::OpenId)]
    pub(in crate::app) employee_id_type: DirectoryEmployeeIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryDepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DirectoryDepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryAuthArg::Tenant)]
    pub(in crate::app) auth: DirectoryAuthArg,

    #[arg(long, help = "Raw Feishu employees mget body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu employees mget body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu employees mget body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DirectoryEmployeeFilterArgs {
    #[arg(
        long = "condition",
        help = "Condition as field=operator=value; can repeat, for example base_info.email=eq=\"user@example.com\""
    )]
    pub(in crate::app) conditions: Vec<String>,

    #[arg(
        long,
        help = "Raw filter object JSON, for example {\"conditions\":[...]}"
    )]
    pub(in crate::app) filter_json: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long = "field", help = "Required field to return; can repeat, max 100")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, value_enum, default_value_t = DirectoryEmployeeIdTypeArg::OpenId)]
    pub(in crate::app) employee_id_type: DirectoryEmployeeIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryDepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DirectoryDepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = DirectoryAuthArg::Tenant)]
    pub(in crate::app) auth: DirectoryAuthArg,

    #[arg(long, help = "Raw Feishu employees filter request body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long,
        help = "Read raw Feishu employees filter request body JSON from file"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw Feishu employees filter request body JSON from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = NOTIFY_AFTER_HELP)]
pub(in crate::app) struct NotifyArgs {
    #[arg(
        long,
        default_value = "feishu-bot",
        help = "Project name used for notification grouping"
    )]
    pub(in crate::app) project: String,

    #[arg(
        long,
        value_enum,
        default_value_t = StatusArg::Info,
        help = "Notification status and card color"
    )]
    pub(in crate::app) status: StatusArg,

    #[arg(
        long,
        help = "Session ID shown in the card footer; defaults to a random UUID"
    )]
    pub(in crate::app) session: Option<String>,

    #[arg(long, help = "High-level objective shown at the top of the card")]
    pub(in crate::app) goal: Option<String>,

    #[arg(long, help = "Current task; also used as the card header")]
    pub(in crate::app) task: Option<String>,

    #[arg(long, help = "One-line bold summary")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Details separated by |")]
    pub(in crate::app) details: Option<String>,

    #[arg(long, help = "Next action shown as a quoted block")]
    pub(in crate::app) next: Option<String>,

    #[arg(long, help = "Progress marker such as 3/10 or phase-2")]
    pub(in crate::app) progress: Option<String>,

    #[arg(long, help = "URL for the primary card button")]
    pub(in crate::app) link: Option<String>,

    #[arg(long, short = 'm', help = "Notification body text")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Read notification body from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read notification body from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Send to this receiver instead of project chat mapping")]
    pub(in crate::app) to: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type when --to is used"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,
}

#[derive(Subcommand)]
#[command(after_long_help = CHAT_AFTER_HELP)]
pub(in crate::app) enum ChatCommand {
    #[command(about = "List chats visible to the bot/user")]
    List(ChatListArgs),
    #[command(about = "Search visible chats")]
    Search(ChatSearchArgs),
    #[command(about = "Get chat metadata")]
    Get(ChatGetArgs),
    #[command(about = "Create a private group chat and add users")]
    Create(ChatCreateArgs),
    #[command(about = "Update chat name, avatar, description, or settings")]
    Update(ChatUpdateArgs),
    #[command(about = "Delete/dissolve a chat")]
    Delete(ChatGetArgs),
    #[command(subcommand, about = "Operate chat members")]
    Member(ChatMemberCommand),
    #[command(subcommand, about = "Operate chat tabs")]
    Tab(ChatTabCommand),
    #[command(subcommand, about = "Operate chat menu tree")]
    Menu(ChatMenuCommand),
}

#[derive(Args)]
pub(in crate::app) struct ChatListArgs {
    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long,
        help = "Sort type, for example ByCreateTimeAsc or ByActiveTimeDesc"
    )]
    pub(in crate::app) sort_type: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ChatSearchArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ChatGetArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ChatCreateArgs {
    #[arg(long, help = "Group chat name")]
    pub(in crate::app) name: String,

    #[arg(long, help = "Optional group description")]
    pub(in crate::app) description: Option<String>,

    #[arg(
        long,
        help = "Image key for the group avatar. Use --avatar-file to upload one"
    )]
    pub(in crate::app) avatar: Option<String>,

    #[arg(
        long = "avatar-file",
        help = "Upload local image as avatar and use returned image_key"
    )]
    pub(in crate::app) avatar_file: Option<PathBuf>,

    #[arg(long = "user", help = "User ID to invite. Can be repeated.")]
    pub(in crate::app) users: Vec<String>,

    #[arg(long = "bot", help = "Bot app_id to invite. Can be repeated.")]
    pub(in crate::app) bots: Vec<String>,

    #[arg(long = "owner-id", help = "Open/user/union ID of the group owner")]
    pub(in crate::app) owner_id: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = UserIdTypeArg::Auto,
        help = "Type for --user and --owner-id values"
    )]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(
        long = "chat-type",
        default_value = "private",
        help = "Chat type: private or public"
    )]
    pub(in crate::app) chat_type: String,

    #[arg(
        long = "group-message-type",
        default_value = "chat",
        help = "Group message style: chat or thread"
    )]
    pub(in crate::app) group_message_type: String,

    #[arg(
        long = "set-bot-manager",
        help = "Set the creating bot as admin when --owner-id is used"
    )]
    pub(in crate::app) set_bot_manager: bool,

    #[arg(long, help = "Idempotency UUID for chat creation")]
    pub(in crate::app) uuid: Option<String>,

    #[arg(
        long = "body-json",
        help = "Raw official chat create body JSON object; typed flags still fill missing avatar-file only when omitted"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official chat create body JSON object from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official chat create body JSON object from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ChatUpdateArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long, help = "New group chat name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "New group description")]
    pub(in crate::app) description: Option<String>,

    #[arg(
        long,
        help = "Image key for the group avatar. Use --avatar-file to upload one"
    )]
    pub(in crate::app) avatar: Option<String>,

    #[arg(
        long = "avatar-file",
        help = "Upload local image as avatar and use returned image_key"
    )]
    pub(in crate::app) avatar_file: Option<PathBuf>,

    #[arg(long = "owner-id", help = "Transfer owner to this user ID")]
    pub(in crate::app) owner_id: Option<String>,

    #[arg(long = "chat-type", help = "Chat type: private or public")]
    pub(in crate::app) chat_type: Option<String>,

    #[arg(
        long = "group-message-type",
        help = "Group message style: chat or thread"
    )]
    pub(in crate::app) group_message_type: Option<String>,

    #[arg(long = "add-member-permission", help = "all_members or only_owner")]
    pub(in crate::app) add_member_permission: Option<String>,

    #[arg(long = "share-card-permission", help = "allowed or not_allowed")]
    pub(in crate::app) share_card_permission: Option<String>,

    #[arg(long = "at-all-permission", help = "all_members or only_owner")]
    pub(in crate::app) at_all_permission: Option<String>,

    #[arg(long = "edit-permission", help = "all_members or only_owner")]
    pub(in crate::app) edit_permission: Option<String>,

    #[arg(
        long = "membership-approval",
        help = "no_approval_required or approval_required"
    )]
    pub(in crate::app) membership_approval: Option<String>,

    #[arg(
        long = "join-message-visibility",
        help = "only_owner, all_members, or not_anyone"
    )]
    pub(in crate::app) join_message_visibility: Option<String>,

    #[arg(
        long = "leave-message-visibility",
        help = "only_owner, all_members, or not_anyone"
    )]
    pub(in crate::app) leave_message_visibility: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long = "body-json", help = "Raw official chat update body JSON object")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official chat update body JSON object from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official chat update body JSON object from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum ChatMemberCommand {
    #[command(about = "List chat members")]
    List(ChatMemberListArgs),
    #[command(about = "Add users or bots to a chat")]
    Add(ChatMemberWriteArgs),
    #[command(about = "Remove users or bots from a chat")]
    Delete(ChatMemberWriteArgs),
    #[command(about = "Check whether the current bot/user is in a chat")]
    IsInChat(ChatMemberIsInChatArgs),
}

#[derive(Args)]
pub(in crate::app) struct ChatMemberListArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) member_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ChatMemberWriteArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long = "id", help = "Member ID. Repeat for multiple users/bots.")]
    pub(in crate::app) ids: Vec<String>,

    #[arg(long, value_enum, default_value_t = ChatMemberIdTypeArg::OpenId)]
    pub(in crate::app) member_id_type: ChatMemberIdTypeArg,

    #[arg(long, default_value_t = 0, help = "Feishu succeed_type: 0, 1, or 2")]
    pub(in crate::app) succeed_type: u8,

    #[arg(long, help = "Raw JSON object or id_list array")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read member body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ChatMemberIsInChatArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,
}

#[derive(Subcommand)]
pub(in crate::app) enum ChatTabCommand {
    #[command(about = "List chat tabs")]
    List(ChatTabListArgs),
    #[command(about = "Add a doc or URL tab to a chat")]
    Add(ChatTabWriteArgs),
    #[command(about = "Update an existing doc or URL chat tab")]
    Update(ChatTabWriteArgs),
    #[command(about = "Delete doc or URL chat tabs")]
    Delete(ChatTabDeleteArgs),
    #[command(about = "Sort chat tabs by tab_id")]
    Sort(ChatTabSortArgs),
}

#[derive(Args)]
pub(in crate::app) struct ChatTabListArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,
}

#[derive(Args)]
pub(in crate::app) struct ChatTabWriteArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long = "tab-id", help = "Tab ID. Required for update")]
    pub(in crate::app) tab_id: Option<String>,

    #[arg(long, help = "Tab name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long = "tab-type",
        default_value = "url",
        help = "Tab type: url or doc"
    )]
    pub(in crate::app) tab_type: String,

    #[arg(long, help = "URL tab target")]
    pub(in crate::app) url: Option<String>,

    #[arg(long, help = "Doc/wiki/docx URL for doc tab")]
    pub(in crate::app) doc: Option<String>,

    #[arg(long = "icon-key", help = "Image key for tab icon")]
    pub(in crate::app) icon_key: Option<String>,

    #[arg(
        long = "icon-file",
        help = "Upload local image as message image and use returned image_key"
    )]
    pub(in crate::app) icon_file: Option<PathBuf>,

    #[arg(
        long = "built-in",
        help = "Open tab in Feishu embedded webview when supported"
    )]
    pub(in crate::app) built_in: bool,

    #[arg(
        long = "body-json",
        help = "Raw official body JSON object or chat_tabs array"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official body JSON object or chat_tabs array from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official body JSON object or chat_tabs array from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

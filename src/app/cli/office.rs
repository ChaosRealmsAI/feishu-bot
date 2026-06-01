use super::*;

#[derive(Subcommand)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) enum OfficeCommand {
    #[command(about = "List locally bootstrapped office projects without Feishu API calls")]
    List(OfficeListArgs),
    #[command(about = "Create/reuse a project chat, Wiki index, Base log, tabs, and summary")]
    Bootstrap(OfficeBootstrapArgs),
    #[command(
        about = "Write one project report to Wiki/docx, notify the project chat, and read back"
    )]
    Report(OfficeReportArgs),
    #[command(about = "Send a lightweight project progress update and append the Base log")]
    Progress(OfficeProgressArgs),
    #[command(about = "Send a project voice update from an audio file or vox-generated speech")]
    VoiceReport(OfficeVoiceReportArgs),
    #[command(about = "Poll the project inbox with safe defaults for ack/reply/mark-seen")]
    Inbox(OfficeInboxArgs),
    #[command(about = "Poll new human messages in a project chat and optionally ack/reply")]
    Poll(OfficePollArgs),
    #[command(about = "Show local project state and optionally probe Feishu resources")]
    Status(OfficeStatusArgs),
    #[command(about = "Search project chat messages and project Wiki/docs")]
    Search(OfficeSearchArgs),
    #[command(about = "Preview or apply project cleanup for known messages/local state")]
    Cleanup(OfficeCleanupArgs),
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeListArgs {
    #[arg(long, help = "Include full local project state for each project")]
    pub(in crate::app) details: bool,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeBootstrapArgs {
    #[arg(long, help = "Stable project name used as the office workflow key")]
    pub(in crate::app) project: String,

    #[arg(
        long = "user",
        help = "User to add to a newly created group. Can repeat."
    )]
    pub(in crate::app) users: Vec<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = UserIdTypeArg::Auto,
        help = "ID type for --user values"
    )]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(
        long,
        help = "Reuse an existing project chat_id instead of creating a group"
    )]
    pub(in crate::app) chat_id: Option<String>,

    #[arg(
        long,
        help = "Wiki space ID. Defaults to FEISHU_WIKI_SPACE_ID when set."
    )]
    pub(in crate::app) space_id: Option<String>,

    #[arg(
        long = "parent-node-token",
        help = "Parent Wiki node token. Defaults to FEISHU_WIKI_PARENT_NODE_TOKEN when set."
    )]
    pub(in crate::app) parent_node_token: Option<String>,

    #[arg(
        long = "avatar-file",
        help = "Upload and set a group avatar when creating a chat"
    )]
    pub(in crate::app) avatar_file: Option<PathBuf>,

    #[arg(long, help = "Do not create a Wiki index doc")]
    pub(in crate::app) skip_wiki: bool,

    #[arg(long, help = "Do not create a project Base log")]
    pub(in crate::app) skip_base: bool,

    #[arg(long, help = "Do not add chat tabs for the Wiki/Base resources")]
    pub(in crate::app) skip_tabs: bool,

    #[arg(long, help = "Send and pin a project summary message after bootstrap")]
    pub(in crate::app) send_summary: bool,

    #[arg(
        long,
        help = "Preview planned writes without creating chats, Wiki docs, Base apps, tabs, messages, or local state"
    )]
    pub(in crate::app) dry_run: bool,

    #[arg(
        long = "auth",
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Token type for Wiki/docx/Base writes"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeReportArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, help = "Report title")]
    pub(in crate::app) title: String,

    #[arg(long, help = "Markdown or HTML report content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read report content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read report content from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(long, help = "Pin the chat notification message")]
    pub(in crate::app) pin: bool,

    #[arg(
        long,
        help = "Create a standalone docx instead of writing into the project Wiki"
    )]
    pub(in crate::app) no_wiki: bool,

    #[arg(
        long,
        help = "Also append one row to the project Base log when configured"
    )]
    pub(in crate::app) base_record: bool,

    #[arg(
        long = "auth",
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Token type for Wiki/docx writes"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(
        long,
        help = "Preview planned writes without creating docs, messages, pins, Base records, or local state"
    )]
    pub(in crate::app) dry_run: bool,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeProgressArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, help = "Short progress title")]
    pub(in crate::app) title: String,

    #[arg(long, default_value = "doing", help = "Progress status label")]
    pub(in crate::app) status: String,

    #[arg(long, help = "Short summary for chat and Base")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Optional Markdown or HTML detail content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read optional detail content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read optional detail content from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(
        long,
        help = "Also create a Wiki/docx detail report and link it in chat/Base"
    )]
    pub(in crate::app) wiki_report: bool,

    #[arg(long, help = "Pin the progress chat message")]
    pub(in crate::app) pin: bool,

    #[arg(long, help = "Do not append the project Base log")]
    pub(in crate::app) no_base_record: bool,

    #[arg(
        long = "auth",
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Token type for optional Wiki/docx writes"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeVoiceReportArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, help = "Existing audio file. Non-OPUS files require ffmpeg.")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Text to synthesize with vox before sending")]
    pub(in crate::app) text: Option<String>,

    #[arg(long = "text-file", help = "Read synthesis text from file")]
    pub(in crate::app) text_file: Option<PathBuf>,

    #[arg(long, help = "Read synthesis text from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Send this text message after the voice message")]
    pub(in crate::app) reply_text: Option<String>,

    #[arg(long, help = "Pin the voice message")]
    pub(in crate::app) pin: bool,

    #[arg(long = "vox-bin", default_value = "vox", help = "vox binary")]
    pub(in crate::app) vox_bin: PathBuf,

    #[arg(long, help = "vox voice name/id")]
    pub(in crate::app) voice: Option<String>,

    #[arg(long = "vox-timeout-ms", default_value_t = 120_000)]
    pub(in crate::app) vox_timeout_ms: u64,

    #[arg(long = "ffmpeg-bin", default_value = "ffmpeg")]
    pub(in crate::app) ffmpeg_bin: PathBuf,

    #[arg(long = "ffprobe-bin", default_value = "ffprobe")]
    pub(in crate::app) ffprobe_bin: PathBuf,

    #[arg(long, help = "Audio duration in milliseconds")]
    pub(in crate::app) duration: Option<u64>,

    #[arg(long, help = "Uploaded OPUS file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Keep generated/transcoded files")]
    pub(in crate::app) keep: bool,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) uuid: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficePollArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, default_value_t = 20, help = "Page size for message list")]
    pub(in crate::app) page_size: u16,

    #[arg(
        long,
        help = "Local state file. Defaults to ~/.config/feishu/message-state.json"
    )]
    pub(in crate::app) state_file: Option<PathBuf>,

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
        help = "Add this emoji reaction to each new human message"
    )]
    pub(in crate::app) ack_emoji: Option<String>,

    #[arg(
        long = "reply-text",
        help = "Reply with this text to each new human message"
    )]
    pub(in crate::app) reply_text: Option<String>,

    #[arg(long, help = "Include messages sent by apps/bots")]
    pub(in crate::app) include_app_messages: bool,

    #[arg(long, help = "Include system messages")]
    pub(in crate::app) include_system_messages: bool,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeInboxArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, default_value_t = 20, help = "Page size for message list")]
    pub(in crate::app) page_size: u16,

    #[arg(
        long,
        help = "Local state file. Defaults to ~/.config/feishu/message-state.json"
    )]
    pub(in crate::app) state_file: Option<PathBuf>,

    #[arg(long, help = "Override local cursor with this message_position")]
    pub(in crate::app) since_position: Option<u64>,

    #[arg(
        long,
        help = "On first run, save the latest cursor and return no messages"
    )]
    pub(in crate::app) from_now: bool,

    #[arg(
        long = "ack-emoji",
        default_value = "OK",
        help = "Emoji reaction used as the default workflow status marker"
    )]
    pub(in crate::app) ack_emoji: String,

    #[arg(long, help = "Do not add an emoji reaction")]
    pub(in crate::app) no_ack: bool,

    #[arg(
        long = "reply-text",
        help = "Reply with this text to each new human message"
    )]
    pub(in crate::app) reply_text: Option<String>,

    #[arg(long, help = "Do not save the newest fetched cursor after polling")]
    pub(in crate::app) no_mark_seen: bool,

    #[arg(long, help = "Include messages sent by apps/bots")]
    pub(in crate::app) include_app_messages: bool,

    #[arg(long, help = "Include system messages")]
    pub(in crate::app) include_system_messages: bool,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeStatusArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(
        long,
        help = "Probe known chat/wiki/base resources through Feishu OpenAPI"
    )]
    pub(in crate::app) check: bool,

    #[arg(
        long = "auth",
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Token type for Wiki checks"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeSearchArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: String,

    #[arg(long, help = "Search project chat messages only")]
    pub(in crate::app) messages: bool,

    #[arg(long, help = "Search project docs/Wiki only")]
    pub(in crate::app) docs: bool,

    #[arg(long, default_value_t = 10, help = "Page size for each search")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token for Feishu search APIs")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = OFFICE_AFTER_HELP)]
pub(in crate::app) struct OfficeCleanupArgs {
    #[arg(long, help = "Project name created by office bootstrap")]
    pub(in crate::app) project: String,

    #[arg(
        long,
        help = "Preview only. This is the default unless --confirm is set."
    )]
    pub(in crate::app) dry_run: bool,

    #[arg(long, help = "Apply the cleanup plan")]
    pub(in crate::app) confirm: bool,

    #[arg(long, help = "Only remove local office project state")]
    pub(in crate::app) local_only: bool,

    #[arg(long, help = "Delete/revoke known messages such as the pinned summary")]
    pub(in crate::app) delete_messages: bool,
}

use super::*;

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

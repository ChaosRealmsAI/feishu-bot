use super::*;

#[derive(Subcommand)]
#[command(after_long_help = MAIL_AFTER_HELP)]
pub(in crate::app) enum MailCommand {
    #[command(subcommand, about = "Operate mailbox messages")]
    Message(MailMessageCommand),
    #[command(subcommand, about = "Operate mailbox folders")]
    Folder(MailFolderCommand),
    #[command(subcommand, about = "Operate mailbox contacts")]
    Contact(MailContactCommand),
    #[command(subcommand, about = "Operate user mailbox aliases")]
    Alias(MailAliasCommand),
    #[command(subcommand, about = "Read mailbox settings")]
    Settings(MailSettingsCommand),
    #[command(subcommand, about = "Read mailbox rules")]
    Rule(MailRuleCommand),
    #[command(subcommand, about = "Read mailbox labels")]
    Label(MailLabelCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailMessageCommand {
    #[command(about = "List message IDs in a mailbox")]
    List(MailMessageListArgs),
    #[command(about = "Get one message")]
    Get(MailMessageGetArgs),
    #[command(about = "Send a message as the current user")]
    Send(MailMessageSendArgs),
    #[command(name = "get-by-card", about = "Get message IDs from a mail card")]
    GetByCard(MailMessageGetByCardArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailFolderCommand {
    #[command(about = "List mailbox folders")]
    List(MailFolderListArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailContactCommand {
    #[command(about = "List mailbox contacts")]
    List(MailContactListArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailAliasCommand {
    #[command(about = "List user mailbox aliases")]
    List(MailAliasListArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailSettingsCommand {
    #[command(name = "send-as", about = "List sendable mailbox addresses")]
    SendAs(MailMailboxAuthArgs),
    #[command(about = "List mailboxes accessible by this mailbox/user")]
    Accessible(MailMailboxAuthArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailRuleCommand {
    #[command(about = "List mailbox receiving rules")]
    List(MailMailboxAuthArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum MailLabelCommand {
    #[command(about = "Get one mailbox label")]
    Get(MailLabelGetArgs),
}

#[derive(Args)]
pub(in crate::app) struct MailboxAuthFields {
    #[arg(
        long,
        default_value = "me",
        help = "Mailbox address, or me with user token"
    )]
    pub(in crate::app) mailbox: String,

    #[arg(long, value_enum, default_value_t = MailAuthArg::Auto)]
    pub(in crate::app) auth: MailAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct MailMailboxAuthArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,
}

#[derive(Args)]
pub(in crate::app) struct MailMessageListArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, default_value_t = 10, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Folder ID, for example INBOX")]
    pub(in crate::app) folder_id: Option<String>,

    #[arg(long, help = "Only list unread messages")]
    pub(in crate::app) only_unread: bool,

    #[arg(long, help = "Label ID, for example FLAGGED")]
    pub(in crate::app) label_id: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct MailMessageGetArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, help = "Message ID from mail message list")]
    pub(in crate::app) message_id: String,

    #[arg(
        long,
        default_value = "metadata",
        help = "full, plain_text_full, or metadata"
    )]
    pub(in crate::app) format: String,
}

#[derive(Args)]
pub(in crate::app) struct MailMessageSendArgs {
    #[arg(long, default_value = "me", help = "Mailbox address, or me")]
    pub(in crate::app) mailbox: String,

    #[arg(long, help = "Recipient email address; can repeat")]
    pub(in crate::app) to: Vec<String>,

    #[arg(long, help = "CC email address; can repeat")]
    pub(in crate::app) cc: Vec<String>,

    #[arg(long, help = "BCC email address; can repeat")]
    pub(in crate::app) bcc: Vec<String>,

    #[arg(long, help = "Email subject")]
    pub(in crate::app) subject: Option<String>,

    #[arg(long, help = "Plain text body")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "HTML body")]
    pub(in crate::app) html: Option<String>,

    #[arg(long, help = "Raw EML content, already base64url encoded")]
    pub(in crate::app) raw_base64url: Option<String>,

    #[arg(long, help = "Read raw EML bytes from file and base64url encode")]
    pub(in crate::app) raw_file: Option<PathBuf>,

    #[arg(long, help = "Dedupe key")]
    pub(in crate::app) dedupe_key: Option<String>,

    #[arg(long = "from", help = "Head From email address")]
    pub(in crate::app) from_address: Option<String>,

    #[arg(long = "from-name", help = "Head From display name")]
    pub(in crate::app) from_name: Option<String>,

    #[arg(long, help = "Raw Feishu send body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu send body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu send body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct MailMessageGetByCardArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, help = "Mail card ID")]
    pub(in crate::app) card_id: String,

    #[arg(long, help = "Mail card owner ID")]
    pub(in crate::app) owner_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MailFolderListArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, help = "Folder type: 1 system, 2 user")]
    pub(in crate::app) folder_type: Option<u8>,
}

#[derive(Args)]
pub(in crate::app) struct MailContactListArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, default_value_t = 20, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct MailAliasListArgs {
    #[arg(long, help = "User mailbox address; tenant token only")]
    pub(in crate::app) mailbox: String,

    #[arg(long, default_value_t = 20, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct MailLabelGetArgs {
    #[command(flatten)]
    pub(in crate::app) mailbox: MailboxAuthFields,

    #[arg(long, help = "Label ID")]
    pub(in crate::app) label_id: String,
}

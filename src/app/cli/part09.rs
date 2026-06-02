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

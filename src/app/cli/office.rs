use super::*;

mod interaction;
mod project;

pub(in crate::app) use interaction::*;
pub(in crate::app) use project::*;

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

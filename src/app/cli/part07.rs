use super::*;
#[derive(Args)]
pub(in crate::app) struct DriveMediaDownloadArgs {
    #[arg(long, help = "Cloud-document media file_token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,

    #[arg(long, help = "Optional HTTP Range header, e.g. bytes=0-1023")]
    pub(in crate::app) range: Option<String>,

    #[arg(
        long,
        help = "Raw extra string, required for some advanced-permission Bitable media"
    )]
    pub(in crate::app) extra: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct DriveMediaTmpUrlArgs {
    #[arg(
        long = "file-token",
        help = "Cloud-document media file_token; repeat up to 5 times"
    )]
    pub(in crate::app) file_tokens: Vec<String>,

    #[arg(
        long,
        help = "Raw extra string, required for some advanced-permission Bitable media"
    )]
    pub(in crate::app) extra: Option<String>,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveImportCommand {
    #[command(about = "Create an import task from an uploaded source file token")]
    Create(DriveImportCreateArgs),
    #[command(about = "Query an import task result")]
    Get(DriveImportGetArgs),
    #[command(about = "Upload a local file and import it as an online docx/sheet/bitable")]
    File(DriveImportFileArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveImportCreateArgs {
    #[arg(
        long,
        help = "Source file token returned by drive media upload or drive upload"
    )]
    pub(in crate::app) file_token: String,

    #[arg(
        long,
        help = "Source file extension, exactly matching the uploaded file suffix"
    )]
    pub(in crate::app) file_extension: String,

    #[arg(
        long = "type",
        default_value = "docx",
        help = "Target online document type: docx, sheet, or bitable"
    )]
    pub(in crate::app) target_type: String,

    #[arg(long, help = "Imported online document name")]
    pub(in crate::app) title: Option<String>,

    #[arg(
        long,
        default_value = "",
        help = "Destination Drive folder token; empty means root"
    )]
    pub(in crate::app) folder_token: String,

    #[arg(long, help = "Raw Feishu import_tasks request body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long,
        help = "Read raw Feishu import_tasks request body JSON from file"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw Feishu import_tasks request body JSON from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DriveImportGetArgs {
    #[arg(long, help = "Import task ticket")]
    pub(in crate::app) ticket: String,
}

#[derive(Args)]
pub(in crate::app) struct DriveImportFileArgs {
    #[arg(
        long,
        help = "Local file to upload and import. HTML should use .html/.htm."
    )]
    pub(in crate::app) file: PathBuf,

    #[arg(long, help = "Override uploaded source file name, including extension")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long = "file-extension",
        help = "Source extension. Defaults to the path/name suffix and must match it."
    )]
    pub(in crate::app) file_extension: Option<String>,

    #[arg(
        long = "type",
        default_value = "docx",
        help = "Target online document type: docx, sheet, or bitable"
    )]
    pub(in crate::app) target_type: String,

    #[arg(
        long,
        help = "Imported online document title. Defaults to uploaded file name."
    )]
    pub(in crate::app) title: Option<String>,

    #[arg(
        long,
        default_value = "",
        help = "Destination Drive folder token; empty means root"
    )]
    pub(in crate::app) folder_token: String,

    #[arg(
        long,
        default_value_t = 30,
        help = "How many times to poll import result; 0 only creates the task"
    )]
    pub(in crate::app) polls: u16,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Milliseconds between import result polls"
    )]
    pub(in crate::app) poll_interval_ms: u64,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveExportCommand {
    #[command(about = "Create an export task")]
    Create(DriveExportCreateArgs),
    #[command(about = "Query an export task result")]
    Get(DriveExportGetArgs),
    #[command(about = "Download an exported file token")]
    Download(DriveExportDownloadArgs),
    #[command(about = "Create, poll, and download an export in one command")]
    File(DriveExportFileArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveExportCreateArgs {
    #[arg(long, help = "Source cloud document token")]
    pub(in crate::app) token: String,

    #[arg(
        long = "type",
        default_value = "docx",
        help = "Source document type: doc, docx, sheet, or bitable"
    )]
    pub(in crate::app) file_type: String,

    #[arg(
        long,
        default_value = "pdf",
        help = "Export extension: pdf, docx, xlsx, or csv"
    )]
    pub(in crate::app) file_extension: String,

    #[arg(long, help = "Sheet ID or Base table ID when exporting CSV")]
    pub(in crate::app) sub_id: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu export_tasks request body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long,
        help = "Read raw Feishu export_tasks request body JSON from file"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw Feishu export_tasks request body JSON from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DriveExportGetArgs {
    #[arg(long, help = "Export task ticket")]
    pub(in crate::app) ticket: String,

    #[arg(long, help = "Source cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveExportDownloadArgs {
    #[arg(long, help = "Exported file token returned by export get")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveExportFileArgs {
    #[arg(long, help = "Source cloud document token")]
    pub(in crate::app) token: String,

    #[arg(
        long = "type",
        default_value = "docx",
        help = "Source document type: doc, docx, sheet, or bitable"
    )]
    pub(in crate::app) file_type: String,

    #[arg(
        long,
        default_value = "pdf",
        help = "Export extension: pdf, docx, xlsx, or csv"
    )]
    pub(in crate::app) file_extension: String,

    #[arg(long, help = "Sheet ID or Base table ID when exporting CSV")]
    pub(in crate::app) sub_id: Option<String>,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(
        long,
        default_value_t = 30,
        help = "How many times to poll export result"
    )]
    pub(in crate::app) polls: u16,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Milliseconds between export result polls"
    )]
    pub(in crate::app) poll_interval_ms: u64,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveCommentCommand {
    #[command(about = "List cloud document comments")]
    List(DriveCommentListArgs),
    #[command(about = "Get one cloud document comment")]
    Get(DriveCommentGetArgs),
    #[command(about = "Batch get cloud document comments by ID")]
    BatchGet(DriveCommentBatchGetArgs),
    #[command(about = "Create a global cloud document comment")]
    Create(DriveCommentCreateArgs),
    #[command(about = "Reply to a cloud document comment")]
    Reply(DriveCommentReplyArgs),
    #[command(about = "Update one comment reply")]
    UpdateReply(DriveCommentUpdateReplyArgs),
    #[command(about = "Delete one comment reply")]
    DeleteReply(DriveCommentDeleteReplyArgs),
    #[command(about = "Resolve or reopen a cloud document comment")]
    Resolve(DriveCommentResolveArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentListArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Filter to whole-document comments")]
    pub(in crate::app) is_whole: Option<bool>,

    #[arg(long, help = "Filter solved/unresolved comments")]
    pub(in crate::app) is_solved: Option<bool>,

    #[arg(long, default_value_t = 50, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Include comment card reactions")]
    pub(in crate::app) need_reaction: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentGetArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentBatchGetArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long = "comment-id", help = "Comment ID; repeat up to 100")]
    pub(in crate::app) comment_ids: Vec<String>,

    #[arg(long, help = "Include comment card reactions")]
    pub(in crate::app) need_reaction: bool,

    #[arg(long, help = "Raw Feishu batch_query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw batch_query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw batch_query body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentCreateArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long = "file-type", default_value = "docx", help = "doc or docx")]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Plain text comment content")]
    pub(in crate::app) text: Option<String>,

    #[arg(long = "docs-link", help = "Cloud document link element; repeatable")]
    pub(in crate::app) docs_links: Vec<String>,

    #[arg(
        long = "mention-user",
        help = "Mention user ID matching --user-id-type; repeatable"
    )]
    pub(in crate::app) mention_users: Vec<String>,

    #[arg(long, help = "Raw Feishu create-comment request body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw create-comment request body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw create-comment request body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentReplyArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, or file"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Plain text reply content")]
    pub(in crate::app) text: Option<String>,

    #[arg(long = "docs-link", help = "Cloud document link element; repeatable")]
    pub(in crate::app) docs_links: Vec<String>,

    #[arg(
        long = "mention-user",
        help = "Mention user ID matching --user-id-type; repeatable"
    )]
    pub(in crate::app) mention_users: Vec<String>,

    #[arg(long, help = "Raw Feishu reply body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw reply body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw reply body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentUpdateReplyArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(long, help = "Reply ID")]
    pub(in crate::app) reply_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Plain text reply content")]
    pub(in crate::app) text: Option<String>,

    #[arg(long = "docs-link", help = "Cloud document link element; repeatable")]
    pub(in crate::app) docs_links: Vec<String>,

    #[arg(
        long = "mention-user",
        help = "Mention user ID matching --user-id-type; repeatable"
    )]
    pub(in crate::app) mention_users: Vec<String>,

    #[arg(long, help = "Raw Feishu update-reply body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw update-reply body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw update-reply body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentDeleteReplyArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(long, help = "Reply ID")]
    pub(in crate::app) reply_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveCommentResolveArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, file, or slides"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, default_value_t = true, help = "true resolves, false reopens")]
    pub(in crate::app) is_solved: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveVersionCommand {
    #[command(about = "Create a document version")]
    Create(DriveVersionCreateArgs),
    #[command(about = "List document versions")]
    List(DriveVersionListArgs),
    #[command(about = "Get one document version")]
    Get(DriveVersionGetArgs),
    #[command(about = "Delete one document version")]
    Delete(DriveVersionGetArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveVersionCreateArgs {
    #[arg(long, help = "Source document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Version title")]
    pub(in crate::app) name: Option<String>,

    #[arg(long = "obj-type", default_value = "docx", help = "docx or sheet")]
    pub(in crate::app) obj_type: String,

    #[arg(long, help = "Raw Feishu version-create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw version-create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw version-create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveVersionListArgs {
    #[arg(long, help = "Source document token")]
    pub(in crate::app) file_token: String,

    #[arg(long = "obj-type", default_value = "docx", help = "docx or sheet")]
    pub(in crate::app) obj_type: String,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveVersionGetArgs {
    #[arg(long, help = "Source document token")]
    pub(in crate::app) file_token: String,

    #[arg(long = "version-id", help = "Version ID")]
    pub(in crate::app) version_id: String,

    #[arg(long = "obj-type", default_value = "docx", help = "docx or sheet")]
    pub(in crate::app) obj_type: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveSubscriptionCommand {
    #[command(about = "Create a user subscription for comment updates")]
    Create(DriveSubscriptionCreateArgs),
    #[command(about = "Get a user subscription status")]
    Get(DriveSubscriptionGetArgs),
    #[command(about = "Update a user subscription status")]
    Update(DriveSubscriptionUpdateArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveSubscriptionCreateArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, or wiki"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, default_value = "comment_update", help = "Subscription type")]
    pub(in crate::app) subscription_type: String,

    #[arg(long, help = "Optional existing subscription ID")]
    pub(in crate::app) subscription_id: Option<String>,

    #[arg(long, help = "Whether to subscribe; omitted lets Feishu default")]
    pub(in crate::app) is_subscribe: Option<bool>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Subscription APIs require user access token")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveSubscriptionGetArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Subscription ID")]
    pub(in crate::app) subscription_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, or wiki"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Subscription APIs require user access token")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveSubscriptionUpdateArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Subscription ID")]
    pub(in crate::app) subscription_id: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, or wiki"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "true subscribes, false unsubscribes")]
    pub(in crate::app) is_subscribe: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Subscription APIs require user access token")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveViewRecordArgs {
    #[arg(long, help = "File token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        default_value = "docx",
        help = "doc, docx, sheet, bitable, mindnote, wiki, or file"
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, default_value_t = 10, help = "Page size, 1..=50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId, help = "Viewer ID type")]
    pub(in crate::app) viewer_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DriveDownloadArgs {
    #[arg(long, help = "Drive resource file token")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,

    #[arg(long, help = "Optional HTTP Range header, e.g. bytes=0-1023")]
    pub(in crate::app) range: Option<String>,
}

#[derive(Subcommand)]
pub(in crate::app) enum DrivePermissionCommand {
    #[command(about = "Get public permission settings")]
    PublicGet(DrivePermissionRefArgs),
    #[command(about = "Patch public permission settings")]
    PublicUpdate(DrivePermissionPublicUpdateArgs),
    #[command(about = "Disable public password")]
    PublicPasswordOff(DrivePermissionRefArgs),
    #[command(about = "List document collaborators")]
    MemberList(DrivePermissionMemberListArgs),
    #[command(about = "Add a document collaborator")]
    MemberAdd(DrivePermissionMemberAddArgs),
    #[command(about = "Update a collaborator permission")]
    MemberUpdate(DrivePermissionMemberUpdateArgs),
    #[command(about = "Remove a document collaborator")]
    MemberDelete(DrivePermissionMemberDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct DrivePermissionRefArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,
}

#[derive(Args)]
pub(in crate::app) struct DrivePermissionPublicUpdateArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Raw Feishu public permission body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw public permission body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw public permission body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Allow external access: true/false")]
    pub(in crate::app) external_access: Option<bool>,

    #[arg(long, help = "Allow non-full-access users to invite external users")]
    pub(in crate::app) invite_external: Option<bool>,

    #[arg(long, help = "security_entity, e.g. anyone_can_view")]
    pub(in crate::app) security_entity: Option<String>,

    #[arg(long, help = "comment_entity, e.g. anyone_can_view")]
    pub(in crate::app) comment_entity: Option<String>,

    #[arg(long, help = "share_entity, e.g. anyone or only_full_access")]
    pub(in crate::app) share_entity: Option<String>,

    #[arg(long, help = "link_share_entity, e.g. tenant_readable or closed")]
    pub(in crate::app) link_share_entity: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct DrivePermissionMemberListArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long,
        help = "Filter by collaborator member type: email, openid, userid, unionid, openchat, opendepartmentid"
    )]
    pub(in crate::app) member_type: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct DrivePermissionMemberAddArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Collaborator ID")]
    pub(in crate::app) member_id: String,

    #[arg(
        long,
        default_value = "openid",
        help = "email, openid, userid, openchat"
    )]
    pub(in crate::app) member_type: String,

    #[arg(long, default_value = "view", help = "view, edit, full_access")]
    pub(in crate::app) perm: String,

    #[arg(long, default_value = "container", help = "container or single_page")]
    pub(in crate::app) perm_type: String,

    #[arg(
        long = "collaborator-type",
        default_value = "user",
        help = "user, chat, department, group"
    )]
    pub(in crate::app) collaborator_type: String,

    #[arg(long, help = "Set need_notification=true")]
    pub(in crate::app) need_notification: bool,

    #[arg(long, help = "Raw Feishu member body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw member body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

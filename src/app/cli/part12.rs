use super::*;
#[derive(Args)]
pub(in crate::app) struct SheetStyleArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long = "range", help = "Range such as Sheet1!A1:C1. Can repeat.")]
    pub(in crate::app) ranges: Vec<String>,

    #[arg(long, help = "Raw JSON object for the Feishu style field")]
    pub(in crate::app) style_json: Option<String>,

    #[arg(long, help = "Font bold flag")]
    pub(in crate::app) bold: Option<bool>,

    #[arg(long, help = "Font italic flag")]
    pub(in crate::app) italic: Option<bool>,

    #[arg(long, help = "Font size, for example 10pt/1.5")]
    pub(in crate::app) font_size: Option<String>,

    #[arg(long, help = "Clear font style")]
    pub(in crate::app) font_clean: Option<bool>,

    #[arg(
        long,
        help = "Text decoration: 0 none, 1 underline, 2 strikethrough, 3 both"
    )]
    pub(in crate::app) text_decoration: Option<i64>,

    #[arg(long, help = "Number/date formatter, for example 0.00% or yyyy/MM/dd")]
    pub(in crate::app) formatter: Option<String>,

    #[arg(long, help = "Horizontal align: 0 left, 1 center, 2 right")]
    pub(in crate::app) h_align: Option<i64>,

    #[arg(long, help = "Vertical align: 0 top, 1 middle, 2 bottom")]
    pub(in crate::app) v_align: Option<i64>,

    #[arg(long, help = "Font color, for example #000000 or 000000")]
    pub(in crate::app) fore_color: Option<String>,

    #[arg(long, help = "Background color, for example #fff2cc or fff2cc")]
    pub(in crate::app) back_color: Option<String>,

    #[arg(
        long,
        help = "Border type: FULL_BORDER, OUTER_BORDER, INNER_BORDER, NO_BORDER, LEFT_BORDER, RIGHT_BORDER, TOP_BORDER, BOTTOM_BORDER"
    )]
    pub(in crate::app) border_type: Option<String>,

    #[arg(long, help = "Border color, for example #ff0000 or ff0000")]
    pub(in crate::app) border_color: Option<String>,

    #[arg(long, help = "Clear all cell styles")]
    pub(in crate::app) clean: Option<bool>,

    #[arg(long, help = "Raw Feishu styles_batch_update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read styles_batch_update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read styles_batch_update body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum SheetValuesCommand {
    #[command(about = "Read one range")]
    Get(SheetRangeArgs),
    #[command(about = "Read multiple ranges")]
    BatchGet(SheetBatchGetArgs),
    #[command(about = "Update one range")]
    Update(SheetValuesWriteArgs),
    #[command(about = "Append rows to a range")]
    Append(SheetValuesWriteArgs),
    #[command(about = "Prepend rows before a range")]
    Prepend(SheetValuesWriteArgs),
}

#[derive(Args)]
pub(in crate::app) struct SheetRangeArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Range such as Sheet1!A1:C10")]
    pub(in crate::app) range: String,
}

#[derive(Args)]
pub(in crate::app) struct SheetBatchGetArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long = "range", help = "Range. Can repeat.")]
    pub(in crate::app) ranges: Vec<String>,
}

#[derive(Args)]
pub(in crate::app) struct SheetValuesWriteArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Range such as Sheet1!A1:C10")]
    pub(in crate::app) range: String,

    #[arg(long, help = "Raw JSON array for values")]
    pub(in crate::app) values_json: Option<String>,

    #[arg(long, help = "Raw Feishu values body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read JSON body or values array from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read JSON body or values array from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetBodyArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Raw Feishu sheet body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read sheet body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read sheet body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = APPROVAL_AFTER_HELP)]
pub(in crate::app) enum ApprovalCommand {
    #[command(subcommand, about = "Operate approval definitions")]
    Definition(ApprovalDefinitionCommand),
    #[command(subcommand, about = "Operate approval instances")]
    Instance(ApprovalInstanceCommand),
    #[command(subcommand, about = "Search and operate approval tasks")]
    Task(ApprovalTaskCommand),
    #[command(subcommand, about = "Operate third-party approval connector resources")]
    External(ApprovalExternalCommand),
    #[command(about = "Create approval definition")]
    CreateDefinition(ApprovalBodyArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum ApprovalDefinitionCommand {
    #[command(about = "Get one approval definition schema")]
    Get(ApprovalDefinitionGetArgs),
    #[command(about = "Create or update approval definition from official JSON")]
    Create(ApprovalDefinitionCreateArgs),
    #[command(about = "Subscribe to approval events for a definition")]
    Subscribe(ApprovalDefinitionCodeArgs),
    #[command(about = "Unsubscribe from approval events for a definition")]
    Unsubscribe(ApprovalDefinitionCodeArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum ApprovalInstanceCommand {
    #[command(about = "List approval instance codes")]
    List(ApprovalInstanceListArgs),
    #[command(about = "Query approval instances with filters")]
    Query(ApprovalSearchArgs),
    #[command(about = "Get one approval instance")]
    Get(ApprovalInstanceGetArgs),
    #[command(about = "Create approval instance")]
    Create(ApprovalBodyArgs),
    #[command(about = "Cancel/revoke an approval instance")]
    Cancel(ApprovalInstanceCancelArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum ApprovalTaskCommand {
    #[command(about = "Search approval tasks with filters")]
    Search(ApprovalSearchArgs),
    #[command(about = "Approve one approval task")]
    Approve(ApprovalTaskActionArgs),
    #[command(about = "Reject one approval task")]
    Reject(ApprovalTaskActionArgs),
    #[command(about = "Transfer one approval task")]
    Transfer(ApprovalTaskTransferArgs),
    #[command(about = "Add signers to one approval task")]
    AddSign(ApprovalTaskAddSignArgs),
    #[command(about = "Roll back one approval task to previous node keys")]
    Rollback(ApprovalTaskRollbackArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum ApprovalExternalCommand {
    #[command(about = "Get one third-party approval definition")]
    DefinitionGet(ApprovalDefinitionGetArgs),
    #[command(about = "Create third-party approval definition from official JSON")]
    DefinitionCreate(ApprovalDefinitionCreateArgs),
    #[command(about = "Sync third-party approval instance from official JSON")]
    InstanceSync(ApprovalBodyArgs),
    #[command(about = "Check third-party approval instance sync status")]
    InstanceCheck(ApprovalBodyArgs),
    #[command(about = "List third-party approval task status with filters")]
    TaskList(ApprovalExternalTaskListArgs),
}

#[derive(Args)]
pub(in crate::app) struct ApprovalDefinitionGetArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Locale: zh-CN, en-US, ja-JP, zh-HK, or zh-TW")]
    pub(in crate::app) locale: Option<String>,

    #[arg(long, help = "Return approval_admin_ids")]
    pub(in crate::app) with_admin_id: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalDefinitionCodeArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalDefinitionCreateArgs {
    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,

    #[arg(long, help = "Raw Feishu approval definition body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read approval definition body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read approval definition body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalInstanceListArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Start timestamp in milliseconds")]
    pub(in crate::app) start_time: String,

    #[arg(long, help = "End timestamp in milliseconds")]
    pub(in crate::app) end_time: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalInstanceGetArgs {
    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: String,

    #[arg(long, help = "Locale: zh-CN, en-US, ja-JP, zh-HK, or zh-TW")]
    pub(in crate::app) locale: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalInstanceCancelArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: String,

    #[arg(long, help = "Submitter user ID matching --user-id-type")]
    pub(in crate::app) user_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw cancel request body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read cancel request body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read cancel request body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalSearchArgs {
    #[arg(
        long,
        default_value_t = 10,
        help = "Page size; task search supports 5..200"
    )]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Approval user ID matching --user-id-type")]
    pub(in crate::app) user_id: Option<String>,

    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: Option<String>,

    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: Option<String>,

    #[arg(long, help = "Third-party instance ID")]
    pub(in crate::app) instance_external_id: Option<String>,

    #[arg(long, help = "Third-party approval group external ID")]
    pub(in crate::app) group_external_id: Option<String>,

    #[arg(long, help = "Instance title; only for third-party approval instances")]
    pub(in crate::app) instance_title: Option<String>,

    #[arg(
        long,
        help = "Instance status: PENDING, RECALL, REJECT, DELETED, APPROVED"
    )]
    pub(in crate::app) instance_status: Option<String>,

    #[arg(long, help = "Instance start time from, Unix milliseconds")]
    pub(in crate::app) instance_start_time_from: Option<String>,

    #[arg(long, help = "Instance start time to, Unix milliseconds")]
    pub(in crate::app) instance_start_time_to: Option<String>,

    #[arg(long, help = "Task title; only for third-party approval tasks")]
    pub(in crate::app) task_title: Option<String>,

    #[arg(
        long,
        help = "Task status: PENDING, REJECTED, APPROVED, TRANSFERRED, DONE"
    )]
    pub(in crate::app) task_status: Option<String>,

    #[arg(
        long = "task-status-list",
        help = "Task statuses; can repeat and overrides --task-status"
    )]
    pub(in crate::app) task_status_list: Vec<String>,

    #[arg(long, help = "Task start time from, Unix milliseconds")]
    pub(in crate::app) task_start_time_from: Option<String>,

    #[arg(long, help = "Task start time to, Unix milliseconds")]
    pub(in crate::app) task_start_time_to: Option<String>,

    #[arg(long, help = "Locale: zh-CN, en-US, or ja-JP")]
    pub(in crate::app) locale: Option<String>,

    #[arg(long, help = "Sort order enum used by Feishu approval task search")]
    pub(in crate::app) order: Option<i64>,

    #[arg(long, help = "Raw Feishu search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalTaskActionArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: String,

    #[arg(long, help = "Operator user ID matching --user-id-type")]
    pub(in crate::app) user_id: String,

    #[arg(long, help = "Approval task ID from instance task_list")]
    pub(in crate::app) task_id: String,

    #[arg(long, help = "Approval comment")]
    pub(in crate::app) comment: Option<String>,

    #[arg(
        long,
        help = "Serialized form JSON array; validated and sent as Feishu string field"
    )]
    pub(in crate::app) form_json: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw approval task body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read approval task body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read approval task body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalTaskTransferArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: String,

    #[arg(long, help = "Operator user ID matching --user-id-type")]
    pub(in crate::app) user_id: String,

    #[arg(long, help = "Approval task ID from instance task_list")]
    pub(in crate::app) task_id: String,

    #[arg(long, help = "Target user ID receiving the task")]
    pub(in crate::app) transfer_user_id: String,

    #[arg(long, help = "Transfer comment")]
    pub(in crate::app) comment: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw transfer body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read transfer body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read transfer body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalTaskAddSignArgs {
    #[arg(long, help = "Approval definition code")]
    pub(in crate::app) approval_code: String,

    #[arg(long, help = "Approval instance code")]
    pub(in crate::app) instance_code: String,

    #[arg(long, help = "Operator user ID matching --user-id-type")]
    pub(in crate::app) user_id: String,

    #[arg(long, help = "Approval task ID from instance task_list")]
    pub(in crate::app) task_id: String,

    #[arg(long = "add-user-id", help = "User ID to add as signer; can repeat")]
    pub(in crate::app) add_sign_user_ids: Vec<String>,

    #[arg(long, help = "1=pre-sign, 2=post-sign, 3=parallel-sign")]
    pub(in crate::app) add_sign_type: Option<i64>,

    #[arg(long, help = "1=or-sign, 2=and-sign; for pre/post add-sign")]
    pub(in crate::app) approval_method: Option<i64>,

    #[arg(long, help = "Add-sign comment")]
    pub(in crate::app) comment: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw add-sign body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read add-sign body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read add-sign body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalTaskRollbackArgs {
    #[arg(long, help = "Current operator user ID matching --user-id-type")]
    pub(in crate::app) user_id: String,

    #[arg(long, help = "Current approval task ID")]
    pub(in crate::app) task_id: String,

    #[arg(
        long = "task-def-key",
        help = "Previous timeline node_key to roll back to; can repeat"
    )]
    pub(in crate::app) task_def_key_list: Vec<String>,

    #[arg(long, help = "Rollback reason")]
    pub(in crate::app) reason: Option<String>,

    #[arg(long, help = "Extra string for Feishu rollback API")]
    pub(in crate::app) extra: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw rollback body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read rollback body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read rollback body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalExternalTaskListArgs {
    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long = "approval-code",
        help = "Third-party approval definition code; can repeat"
    )]
    pub(in crate::app) approval_codes: Vec<String>,

    #[arg(long = "instance-id", help = "Third-party instance ID; can repeat")]
    pub(in crate::app) instance_ids: Vec<String>,

    #[arg(long = "user-id", help = "User ID; can repeat")]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(
        long,
        help = "Task status: PENDING, APPROVED, REJECTED, TRANSFERRED, DONE"
    )]
    pub(in crate::app) status: Option<String>,

    #[arg(long, help = "Raw external task list body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read external task list body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read external task list body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApprovalBodyArgs {
    #[arg(long, help = "Raw Feishu approval body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read approval body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read approval body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = API_AFTER_HELP)]
pub(in crate::app) enum ApiCommand {
    #[command(about = "Raw GET request")]
    Get(ApiPathArgs),
    #[command(about = "Raw POST request with JSON body")]
    Post(ApiBodyArgs),
    #[command(about = "Raw PUT request with JSON body")]
    Put(ApiBodyArgs),
    #[command(about = "Raw PATCH request with JSON body")]
    Patch(ApiBodyArgs),
    #[command(about = "Raw DELETE request")]
    Delete(ApiMaybeBodyArgs),
    #[command(about = "Raw binary GET download to a local file")]
    Download(ApiDownloadArgs),
    #[command(about = "Raw multipart/form-data request with text fields and file parts")]
    Multipart(ApiMultipartArgs),
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(in crate::app) enum ApiAuthArg {
    Tenant,
    User,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub(in crate::app) enum ApiMethodArg {
    Post,
    Put,
    Patch,
}

impl ApiMethodArg {
    pub(in crate::app) fn as_method(self) -> Method {
        match self {
            ApiMethodArg::Post => Method::POST,
            ApiMethodArg::Put => Method::PUT,
            ApiMethodArg::Patch => Method::PATCH,
        }
    }
}

#[derive(Args)]
pub(in crate::app) struct ApiPathArgs {
    #[arg(long, help = "OpenAPI path under /open-apis, must start with /")]
    pub(in crate::app) path: String,

    #[arg(long = "query", help = "Query pair key=value. Can repeat.")]
    pub(in crate::app) query: Vec<String>,

    #[arg(long = "header", help = "Extra header key=value. Can repeat.")]
    pub(in crate::app) headers: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct ApiBodyArgs {
    #[arg(long, help = "OpenAPI path under /open-apis, must start with /")]
    pub(in crate::app) path: String,

    #[arg(long = "query", help = "Query pair key=value. Can repeat.")]
    pub(in crate::app) query: Vec<String>,

    #[arg(long = "header", help = "Extra header key=value. Can repeat.")]
    pub(in crate::app) headers: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw JSON body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw JSON body from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw JSON body from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApiMaybeBodyArgs {
    #[arg(long, help = "OpenAPI path under /open-apis, must start with /")]
    pub(in crate::app) path: String,

    #[arg(long = "query", help = "Query pair key=value. Can repeat.")]
    pub(in crate::app) query: Vec<String>,

    #[arg(long = "header", help = "Extra header key=value. Can repeat.")]
    pub(in crate::app) headers: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Optional raw JSON body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read optional raw JSON body from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read optional raw JSON body from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ApiDownloadArgs {
    #[arg(long, help = "OpenAPI binary download path under /open-apis")]
    pub(in crate::app) path: String,

    #[arg(long = "query", help = "Query pair key=value. Can repeat.")]
    pub(in crate::app) query: Vec<String>,

    #[arg(long = "header", help = "Extra header key=value. Can repeat.")]
    pub(in crate::app) headers: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Optional HTTP Range header, e.g. bytes=0-1023")]
    pub(in crate::app) range: Option<String>,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,
}

#[derive(Args)]
pub(in crate::app) struct ApiMultipartArgs {
    #[arg(long, help = "OpenAPI path under /open-apis, must start with /")]
    pub(in crate::app) path: String,

    #[arg(long, value_enum, default_value_t = ApiMethodArg::Post, help = "HTTP method")]
    pub(in crate::app) method: ApiMethodArg,

    #[arg(long = "query", help = "Query pair key=value. Can repeat.")]
    pub(in crate::app) query: Vec<String>,

    #[arg(long = "header", help = "Extra header key=value. Can repeat.")]
    pub(in crate::app) headers: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long = "field", help = "Multipart text field key=value. Can repeat.")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(
        long = "file",
        help = "Multipart file part part_name=./path. Can repeat."
    )]
    pub(in crate::app) files: Vec<String>,
}

#[derive(Subcommand)]
pub(in crate::app) enum BrowserCommand {
    #[command(about = "Start or verify the Playwright MCP extension bridge")]
    Ensure,
    #[command(about = "List controlled Chrome tabs")]
    Tabs,
    #[command(about = "Navigate the controlled Chrome tab to a URL")]
    Open(BrowserOpenArgs),
    #[command(about = "Open Feishu Drive in the controlled Chrome tab")]
    Drive,
}

#[derive(Args)]
pub(in crate::app) struct BrowserOpenArgs {
    #[arg(long, help = "URL to open in the controlled Chrome tab")]
    pub(in crate::app) url: String,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum ReceiveIdTypeArg {
    Auto,
    OpenId,
    UnionId,
    UserId,
    Email,
    ChatId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum UserIdTypeArg {
    Auto,
    OpenId,
    UnionId,
    UserId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum OkrUserIdTypeArg {
    OpenId,
    UnionId,
    UserId,
    PeopleAdminId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum AttendanceEmployeeTypeArg {
    EmployeeId,
    EmployeeNo,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum MailAuthArg {
    Auto,
    Tenant,
    User,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DirectoryAuthArg {
    Tenant,
    User,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DirectoryEmployeeIdTypeArg {
    OpenId,
    UnionId,
    EmployeeId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DirectoryDepartmentIdTypeArg {
    OpenDepartmentId,
    DepartmentId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum CorehrUserIdTypeArg {
    OpenId,
    UnionId,
    UserId,
    PeopleCorehrId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum CorehrPersonUserIdTypeArg {
    OpenId,
    PeopleEmployeeId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum CorehrDepartmentIdTypeArg {
    OpenDepartmentId,
    DepartmentId,
    PeopleCorehrDepartmentId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum HelpdeskReceiveTypeArg {
    Chat,
    User,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum HireUserIdTypeArg {
    OpenId,
    UnionId,
    UserId,
    PeopleAdminId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum HireJobLevelIdTypeArg {
    PeopleAdminJobLevelId,
    JobLevelId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum HireJobFamilyIdTypeArg {
    PeopleAdminJobCategoryId,
    JobFamilyId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum HireEmployeeTypeIdTypeArg {
    PeopleAdminEmployeeTypeId,
    EmployeeTypeEnumId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum ChatMemberIdTypeArg {
    OpenId,
    UnionId,
    UserId,
    AppId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DepartmentIdTypeArg {
    OpenDepartmentId,
    DepartmentId,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum StatusArg {
    Done,
    Error,
    Info,
    Warning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(in crate::app) enum WriterArg {
    Local,
    Official,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum ContentTypeArg {
    Markdown,
    Html,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DocMediaKindArg {
    Image,
    File,
}

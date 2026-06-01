use super::*;
#[derive(Args)]
pub(in crate::app) struct VcRoomGetArgs {
    #[arg(long, help = "Meeting room ID")]
    pub(in crate::app) room_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct VcRoomMgetArgs {
    #[arg(long = "room-id", help = "Meeting room ID. Can repeat.")]
    pub(in crate::app) room_ids: Vec<String>,

    #[arg(long, help = "Raw Feishu mget body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read mget body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read mget body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum VcRoomLevelCommand {
    #[command(about = "List child meeting room levels")]
    List(VcRoomLevelListArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcRoomLevelListArgs {
    #[arg(long, help = "Room level ID. Omit for tenant root when API allows it.")]
    pub(in crate::app) room_level_id: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Subcommand)]
#[command(after_long_help = MINUTES_AFTER_HELP)]
pub(in crate::app) enum MinutesCommand {
    #[command(about = "Search minutes by keyword and native filters")]
    Search(MinutesSearchArgs),
    #[command(about = "Get minutes metadata")]
    Get(MinutesGetArgs),
    #[command(about = "Get minutes AI artifacts such as summary, actions, and chapters")]
    Artifacts(MinutesTokenArgs),
    #[command(about = "Get minutes audio/video download URL")]
    Media(MinutesTokenArgs),
    #[command(about = "Export minutes transcript to a local file")]
    Transcript(MinutesTranscriptArgs),
}

#[derive(Args)]
pub(in crate::app) struct MinutesSearchArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, help = "Native Feishu minutes filter JSON object")]
    pub(in crate::app) filter_json: Option<String>,

    #[arg(long, help = "Sorter such as create_time_desc")]
    pub(in crate::app) sorter: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Raw Feishu search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read search body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MinutesGetArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MinutesTokenArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,
}

#[derive(Args)]
pub(in crate::app) struct MinutesTranscriptArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,

    #[arg(long, help = "Include speaker names")]
    pub(in crate::app) need_speaker: bool,

    #[arg(long, help = "Include timestamps")]
    pub(in crate::app) need_timestamp: bool,

    #[arg(long, help = "Export format, usually txt or srt")]
    pub(in crate::app) file_format: Option<String>,

    #[arg(long, help = "Output file path, or - for stdout")]
    pub(in crate::app) output: PathBuf,
}

#[derive(Subcommand)]
#[command(after_long_help = SEARCH_AFTER_HELP)]
pub(in crate::app) enum SearchCommand {
    #[command(about = "Search current user's visible Feishu docs and wiki")]
    Docs(SearchDocsArgs),
    #[command(about = "Search current user's visible messages")]
    Message(SearchMessageArgs),
    #[command(subcommand, about = "Manage custom search data sources")]
    Source(SearchSourceCommand),
    #[command(subcommand, about = "Manage custom search schemas")]
    Schema(SearchSchemaCommand),
    #[command(subcommand, about = "Index or inspect custom search data items")]
    Item(SearchItemCommand),
}

#[derive(Args)]
pub(in crate::app) struct SearchDocsArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, default_value_t = 10, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long = "doc-type",
        help = "DOC, SHEET, BITABLE, MINDNOTE, FILE, WIKI, DOCX, FOLDER, CATALOG, SLIDES, SHORTCUT"
    )]
    pub(in crate::app) doc_types: Vec<String>,

    #[arg(
        long = "folder-token",
        help = "Restrict document search to Drive folder token; can repeat"
    )]
    pub(in crate::app) folder_tokens: Vec<String>,

    #[arg(
        long = "space-id",
        help = "Restrict wiki search to space ID; can repeat"
    )]
    pub(in crate::app) space_ids: Vec<String>,

    #[arg(long, help = "Only search titles")]
    pub(in crate::app) only_title: bool,

    #[arg(
        long,
        help = "DEFAULT_TYPE, OPEN_TIME, EDIT_TIME, EDIT_TIME_ASC, CREATE_TIME"
    )]
    pub(in crate::app) sort_type: Option<String>,

    #[arg(long, help = "Create time range start, Unix seconds")]
    pub(in crate::app) create_start: Option<i64>,

    #[arg(long, help = "Create time range end, Unix seconds")]
    pub(in crate::app) create_end: Option<i64>,

    #[arg(long, help = "Open time range start, Unix seconds")]
    pub(in crate::app) open_start: Option<i64>,

    #[arg(long, help = "Open time range end, Unix seconds")]
    pub(in crate::app) open_end: Option<i64>,

    #[arg(long, help = "Raw Feishu docs search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read docs search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read docs search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchMessageArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long = "from-id", help = "Sender user/open ID; can repeat")]
    pub(in crate::app) from_ids: Vec<String>,

    #[arg(long = "chat-id", help = "Chat ID; can repeat")]
    pub(in crate::app) chat_ids: Vec<String>,

    #[arg(long = "at-chatter-id", help = "Mentioned user/open ID; can repeat")]
    pub(in crate::app) at_chatter_ids: Vec<String>,

    #[arg(long, help = "Message type: file, image, or media")]
    pub(in crate::app) message_type: Option<String>,

    #[arg(long, help = "Sender type: bot or user")]
    pub(in crate::app) from_type: Option<String>,

    #[arg(long, help = "Chat type: group_chat or p2p_chat")]
    pub(in crate::app) chat_type: Option<String>,

    #[arg(long, help = "Message send start time, Unix seconds")]
    pub(in crate::app) start_time: Option<String>,

    #[arg(long, help = "Message send end time, Unix seconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, help = "Raw Feishu message search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read message search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message search body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchSourceCommand {
    #[command(about = "List custom search data sources")]
    List(SearchSourceListArgs),
    #[command(about = "Get one custom search data source")]
    Get(SearchSourceRefArgs),
    #[command(about = "Create a custom search data source")]
    Create(SearchSourceWriteArgs),
    #[command(about = "Update a custom search data source")]
    Update(SearchSourceUpdateArgs),
    #[command(about = "Delete a custom search data source")]
    Delete(SearchSourceRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceListArgs {
    #[arg(long, help = "View format: 0 full, 1 summary")]
    pub(in crate::app) view: Option<u8>,

    #[arg(long, default_value_t = 20, help = "Page size, max 50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceRefArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceWriteArgs {
    #[arg(long, help = "Data source display name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Data source description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Icon URL")]
    pub(in crate::app) icon_url: Option<String>,

    #[arg(long, help = "Associated schema ID")]
    pub(in crate::app) schema_id: Option<String>,

    #[arg(long, help = "Display template, usually search_common_card")]
    pub(in crate::app) template: Option<String>,

    #[arg(long, help = "Data source state: 0 online, 1 offline")]
    pub(in crate::app) state: Option<i64>,

    #[arg(long, help = "Raw Feishu data source JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read data source JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read data source JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceUpdateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[command(flatten)]
    pub(in crate::app) body: SearchSourceWriteArgs,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchSchemaCommand {
    #[command(about = "Get a custom search schema")]
    Get(SearchSchemaRefArgs),
    #[command(about = "Create a custom search schema")]
    Create(SearchSchemaCreateArgs),
    #[command(about = "Update a custom search schema display config")]
    Update(SearchSchemaUpdateArgs),
    #[command(about = "Delete a custom search schema")]
    Delete(SearchSchemaRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaRefArgs {
    #[arg(long, help = "Schema ID")]
    pub(in crate::app) schema_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaCreateArgs {
    #[arg(long, help = "Raw Feishu schema JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read schema JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read schema JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Only validate schema without creating it")]
    pub(in crate::app) validate_only: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaUpdateArgs {
    #[arg(long, help = "Schema ID")]
    pub(in crate::app) schema_id: String,

    #[arg(long, help = "Raw Feishu schema update JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read schema update JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read schema update JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchItemCommand {
    #[command(about = "Get one indexed item")]
    Get(SearchItemRefArgs),
    #[command(about = "Create/index one item")]
    Create(SearchItemCreateArgs),
    #[command(about = "Batch create/index items")]
    BatchCreate(SearchItemBatchCreateArgs),
    #[command(about = "Delete one indexed item")]
    Delete(SearchItemRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchItemRefArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Item ID")]
    pub(in crate::app) item_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchItemCreateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Item ID")]
    pub(in crate::app) id: Option<String>,

    #[arg(long, help = "Item title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "Desktop source URL")]
    pub(in crate::app) url: Option<String>,

    #[arg(long, help = "Mobile source URL")]
    pub(in crate::app) mobile_url: Option<String>,

    #[arg(
        long,
        help = "Structured data JSON object; encoded as string for Feishu"
    )]
    pub(in crate::app) structured_json: Option<String>,

    #[arg(long, help = "Full text content for recall")]
    pub(in crate::app) text: Option<String>,

    #[arg(
        long,
        default_value = "plaintext",
        help = "Content format: plaintext or html"
    )]
    pub(in crate::app) content_format: String,

    #[arg(long, help = "ACL JSON array. Defaults to allow everyone.")]
    pub(in crate::app) acl_json: Option<String>,

    #[arg(long, help = "Raw Feishu item JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read item JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read item JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchItemBatchCreateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Raw Feishu batch item JSON, usually {\"items\":[...]}")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read batch item JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read batch item JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = OKR_AFTER_HELP)]
pub(in crate::app) enum OkrCommand {
    #[command(subcommand, about = "Operate OKR periods")]
    Period(OkrPeriodCommand),
    #[command(subcommand, name = "period-rule", about = "Operate OKR period rules")]
    PeriodRule(OkrPeriodRuleCommand),
    #[command(about = "Get one user's OKR list")]
    UserOkrs(OkrUserOkrsArgs),
    #[command(about = "Batch get OKRs by OKR IDs")]
    BatchGet(OkrBatchGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum OkrPeriodCommand {
    #[command(about = "List OKR periods")]
    List(OkrPeriodListArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum OkrPeriodRuleCommand {
    #[command(about = "List OKR period rules")]
    List,
}

#[derive(Args)]
pub(in crate::app) struct OkrPeriodListArgs {
    #[arg(long, default_value_t = 10, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct OkrUserOkrsArgs {
    #[arg(long, help = "User ID")]
    pub(in crate::app) user_id: String,

    #[arg(long, default_value_t = 0, help = "Offset, required by Feishu")]
    pub(in crate::app) offset: u32,

    #[arg(long, default_value_t = 5, help = "Limit, max 10")]
    pub(in crate::app) limit: u16,

    #[arg(
        long,
        default_value = "zh_cn",
        help = "Language, for example zh_cn or en_us"
    )]
    pub(in crate::app) lang: String,

    #[arg(long = "period-id", help = "Restrict to OKR period ID; can repeat")]
    pub(in crate::app) period_ids: Vec<String>,

    #[arg(long, value_enum, default_value_t = OkrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: OkrUserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct OkrBatchGetArgs {
    #[arg(long = "okr-id", help = "OKR ID; can repeat, max 10")]
    pub(in crate::app) okr_ids: Vec<String>,

    #[arg(
        long,
        default_value = "zh_cn",
        help = "Language, for example zh_cn or en_us"
    )]
    pub(in crate::app) lang: String,

    #[arg(long, value_enum, default_value_t = OkrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: OkrUserIdTypeArg,
}

#[derive(Subcommand)]
#[command(after_long_help = ATTENDANCE_AFTER_HELP)]
pub(in crate::app) enum AttendanceCommand {
    #[command(subcommand, about = "Operate attendance groups")]
    Group(AttendanceGroupCommand),
    #[command(subcommand, about = "Operate attendance shifts")]
    Shift(AttendanceShiftCommand),
    #[command(subcommand, about = "Query user daily schedules")]
    Schedule(AttendanceScheduleCommand),
    #[command(subcommand, about = "Query attendance task results")]
    Task(AttendanceTaskCommand),
    #[command(subcommand, about = "Operate attendance clock-in flows")]
    Flow(AttendanceFlowCommand),
    #[command(subcommand, about = "Query attendance statistics")]
    Stats(AttendanceStatsCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceGroupCommand {
    #[command(about = "List attendance groups")]
    List(AttendancePageArgs),
    #[command(about = "Get one attendance group")]
    Get(AttendanceGroupGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceShiftCommand {
    #[command(about = "List attendance shifts")]
    List(AttendancePageArgs),
    #[command(about = "Get one attendance shift")]
    Get(AttendanceShiftGetArgs),
    #[command(about = "Query attendance shift by name")]
    Query(AttendanceShiftQueryArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceScheduleCommand {
    #[command(about = "Query user daily shifts")]
    Query(AttendanceScheduleQueryArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceTaskCommand {
    #[command(about = "Query user attendance results")]
    Query(AttendanceTaskQueryArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceFlowCommand {
    #[command(about = "Get one attendance flow record")]
    Get(AttendanceFlowGetArgs),
    #[command(about = "Batch query attendance flow records")]
    Query(AttendanceFlowQueryArgs),
    #[command(about = "Import attendance flow records from raw Feishu JSON")]
    Import(AttendanceFlowImportArgs),
    #[command(about = "Delete imported attendance flow records")]
    Delete(AttendanceFlowDeleteArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum AttendanceStatsCommand {
    #[command(about = "Query daily or monthly attendance statistics")]
    Query(AttendanceStatsQueryArgs),
}

#[derive(Args)]
pub(in crate::app) struct AttendancePageArgs {
    #[arg(long, default_value_t = 10, help = "Page size, max 50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceGroupGetArgs {
    #[arg(long, help = "Attendance group ID")]
    pub(in crate::app) group_id: String,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(
        long,
        default_value = "open_id",
        help = "Department ID type; Feishu currently supports open_id"
    )]
    pub(in crate::app) dept_type: String,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceShiftGetArgs {
    #[arg(long, help = "Attendance shift ID")]
    pub(in crate::app) shift_id: String,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceShiftQueryArgs {
    #[arg(long, help = "Attendance shift name")]
    pub(in crate::app) shift_name: String,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceScheduleQueryArgs {
    #[arg(
        long = "user-id",
        help = "Employee ID or employee number; can repeat, max 50"
    )]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(long = "from", help = "Start work date, yyyyMMdd")]
    pub(in crate::app) check_date_from: Option<u32>,

    #[arg(
        long = "to",
        help = "End work date, yyyyMMdd; span must not exceed 30 days"
    )]
    pub(in crate::app) check_date_to: Option<u32>,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(long, help = "Raw Feishu schedule query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu schedule query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu schedule query body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceTaskQueryArgs {
    #[arg(
        long = "user-id",
        help = "Employee ID or employee number; can repeat, max 50"
    )]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(long = "from", help = "Start work date, yyyyMMdd")]
    pub(in crate::app) check_date_from: Option<u32>,

    #[arg(long = "to", help = "End work date, yyyyMMdd")]
    pub(in crate::app) check_date_to: Option<u32>,

    #[arg(long, help = "Include overtime task-shift results")]
    pub(in crate::app) need_overtime_result: bool,

    #[arg(
        long,
        help = "Ignore invalid or unauthorized users and return valid users"
    )]
    pub(in crate::app) ignore_invalid_users: bool,

    #[arg(long, help = "Include terminated users when employee IDs were reused")]
    pub(in crate::app) include_terminated_user: bool,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(long, help = "Raw Feishu attendance task query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long,
        help = "Read raw Feishu attendance task query body JSON from file"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw Feishu attendance task query body JSON from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceFlowGetArgs {
    #[arg(long, help = "Attendance flow record ID")]
    pub(in crate::app) user_flow_id: String,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceFlowQueryArgs {
    #[arg(
        long = "user-id",
        help = "Employee ID or employee number; can repeat, max 50"
    )]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(long = "from-ts", help = "Start check time, Unix seconds")]
    pub(in crate::app) check_time_from: Option<String>,

    #[arg(long = "to-ts", help = "End check time, Unix seconds")]
    pub(in crate::app) check_time_to: Option<String>,

    #[arg(long, help = "Include terminated users when employee IDs were reused")]
    pub(in crate::app) include_terminated_user: bool,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(long, help = "Raw Feishu flow query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu flow query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu flow query body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceFlowImportArgs {
    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(
        long,
        help = "Raw Feishu import body JSON, usually {\"flow_records\":[...]}"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu import body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu import body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceFlowDeleteArgs {
    #[arg(
        long = "record-id",
        help = "Imported attendance record ID; can repeat, max 10"
    )]
    pub(in crate::app) record_ids: Vec<String>,

    #[arg(
        long,
        help = "Raw Feishu delete body JSON, usually {\"record_ids\":[...]}"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu delete body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu delete body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct AttendanceStatsQueryArgs {
    #[arg(
        long = "user-id",
        help = "Employee ID or employee number; can repeat, max 200"
    )]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(
        long = "operator-user-id",
        help = "Operator/admin user ID used to choose report fields"
    )]
    pub(in crate::app) operator_user_id: Option<String>,

    #[arg(long = "from", help = "Start date, yyyyMMdd")]
    pub(in crate::app) start_date: Option<u32>,

    #[arg(long = "to", help = "End date, yyyyMMdd; span must not exceed 31 days")]
    pub(in crate::app) end_date: Option<u32>,

    #[arg(long, default_value = "zh", help = "Locale: zh, en, or ja")]
    pub(in crate::app) locale: String,

    #[arg(long, default_value = "daily", help = "Stats type: daily or month")]
    pub(in crate::app) stats_type: String,

    #[arg(long, help = "Include history/terminated and transferred users")]
    pub(in crate::app) need_history: bool,

    #[arg(long, help = "Only show current attendance group data")]
    pub(in crate::app) current_group_only: bool,

    #[arg(long, value_enum, default_value_t = AttendanceEmployeeTypeArg::EmployeeId)]
    pub(in crate::app) employee_type: AttendanceEmployeeTypeArg,

    #[arg(long, help = "Raw Feishu stats query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu stats query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu stats query body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

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

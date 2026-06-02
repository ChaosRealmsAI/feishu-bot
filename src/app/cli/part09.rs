use super::*;

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

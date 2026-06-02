use super::*;

#[derive(Subcommand)]
#[command(after_long_help = COREHR_AFTER_HELP)]
pub(in crate::app) enum CorehrCommand {
    #[command(subcommand, about = "Search and batch-get CoreHR departments")]
    Department(CorehrDepartmentCommand),
    #[command(subcommand, about = "List, get, and batch-get CoreHR jobs")]
    Job(CorehrJobCommand),
    #[command(
        subcommand,
        name = "job-data",
        about = "Query and get CoreHR employee job data"
    )]
    JobData(CorehrJobDataCommand),
    #[command(subcommand, about = "Get CoreHR personal information")]
    Person(CorehrPersonCommand),
    #[command(subcommand, about = "List and get CoreHR process instances")]
    Process(CorehrProcessCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum CorehrDepartmentCommand {
    #[command(about = "Search departments by ID, parent, manager, name, code, or raw JSON")]
    Search(CorehrDepartmentSearchArgs),
    #[command(about = "Batch get departments by department IDs or exact names")]
    Get(CorehrDepartmentGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum CorehrJobCommand {
    #[command(about = "List CoreHR jobs")]
    List(CorehrJobListArgs),
    #[command(about = "Get one CoreHR job")]
    Get(CorehrJobGetArgs),
    #[command(name = "batch-get", about = "Batch get CoreHR jobs by IDs or codes")]
    BatchGet(CorehrJobBatchGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum CorehrJobDataCommand {
    #[command(about = "Query employee job data")]
    Query(CorehrJobDataQueryArgs),
    #[command(about = "Get one job data record")]
    Get(CorehrJobDataGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum CorehrPersonCommand {
    #[command(about = "Get one person's personal information")]
    Get(CorehrPersonGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum CorehrProcessCommand {
    #[command(about = "List process instance IDs by modify time")]
    List(CorehrProcessListArgs),
    #[command(about = "Get one process instance detail")]
    Get(CorehrProcessGetArgs),
}

#[derive(Args)]
pub(in crate::app) struct CorehrDepartmentSearchArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = CorehrDepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: CorehrDepartmentIdTypeArg,

    #[arg(
        long = "department-id",
        help = "Department ID filter; can repeat, max 100"
    )]
    pub(in crate::app) department_ids: Vec<String>,

    #[arg(long = "name", help = "Exact department name filter; can repeat")]
    pub(in crate::app) names: Vec<String>,

    #[arg(
        long = "manager-id",
        help = "Manager employment ID filter; can repeat, max 100"
    )]
    pub(in crate::app) manager_ids: Vec<String>,

    #[arg(long, help = "Parent department ID")]
    pub(in crate::app) parent_department_id: Option<String>,

    #[arg(long = "code", help = "Department code filter; can repeat")]
    pub(in crate::app) codes: Vec<String>,

    #[arg(long = "field", help = "Returned field name; can repeat")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, help = "Filter by department active state, true or false")]
    pub(in crate::app) active: Option<bool>,

    #[arg(long, help = "Return all children when parent_department_id is used")]
    pub(in crate::app) get_all_children: bool,

    #[arg(long, help = "Raw Feishu department search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu department search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu department search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct CorehrDepartmentGetArgs {
    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = CorehrDepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: CorehrDepartmentIdTypeArg,

    #[arg(long = "department-id", help = "Department ID; can repeat, max 100")]
    pub(in crate::app) department_ids: Vec<String>,

    #[arg(long = "name", help = "Exact department name; can repeat, max 100")]
    pub(in crate::app) names: Vec<String>,

    #[arg(long = "field", help = "Returned field name; can repeat, max 100")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, help = "Raw Feishu departments batch_get body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long,
        help = "Read raw Feishu departments batch_get body JSON from file"
    )]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw Feishu departments batch_get body JSON from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct CorehrJobListArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Job name filter")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Query language, for example zh-CN or en-US")]
    pub(in crate::app) query_language: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct CorehrJobGetArgs {
    #[arg(long, help = "CoreHR job ID")]
    pub(in crate::app) job_id: String,
}

#[derive(Args)]
pub(in crate::app) struct CorehrJobBatchGetArgs {
    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,

    #[arg(long = "job-id", help = "CoreHR job ID; can repeat, max 100")]
    pub(in crate::app) job_ids: Vec<String>,

    #[arg(long = "job-code", help = "CoreHR job code; can repeat, max 100")]
    pub(in crate::app) job_codes: Vec<String>,

    #[arg(long = "field", help = "Returned field name; can repeat, max 100")]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, help = "Raw Feishu jobs batch_get body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu jobs batch_get body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu jobs batch_get body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct CorehrJobDataQueryArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId)]
    pub(in crate::app) department_id_type: CorehrDepartmentIdTypeArg,

    #[arg(long = "employment-id", help = "Employment ID; can repeat, max 100")]
    pub(in crate::app) employment_ids: Vec<String>,

    #[arg(long, help = "Department ID filter")]
    pub(in crate::app) department_id: Option<String>,

    #[arg(long, help = "Data date, yyyy-MM-dd")]
    pub(in crate::app) data_date: Option<String>,

    #[arg(long, help = "Effective date range start, yyyy-MM-dd")]
    pub(in crate::app) effective_date_start: Option<String>,

    #[arg(long, help = "Effective date range end, yyyy-MM-dd")]
    pub(in crate::app) effective_date_end: Option<String>,

    #[arg(
        long,
        help = "Fetch all versions instead of only current effective records"
    )]
    pub(in crate::app) all_version: bool,

    #[arg(long, help = "Filter primary job data, true or false")]
    pub(in crate::app) primary_job_data: Option<bool>,

    #[arg(
        long = "assignment-start-reason",
        help = "Assignment start reason; can repeat"
    )]
    pub(in crate::app) assignment_start_reasons: Vec<String>,

    #[arg(long, help = "Raw Feishu job data query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw Feishu job data query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw Feishu job data query body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct CorehrJobDataGetArgs {
    #[arg(long, help = "CoreHR job data ID")]
    pub(in crate::app) job_data_id: String,

    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::PeopleCorehrId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = CorehrDepartmentIdTypeArg::PeopleCorehrDepartmentId)]
    pub(in crate::app) department_id_type: CorehrDepartmentIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CorehrPersonGetArgs {
    #[arg(long, help = "CoreHR person ID")]
    pub(in crate::app) person_id: String,

    #[arg(long, value_enum, default_value_t = CorehrPersonUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrPersonUserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CorehrProcessListArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long = "status", help = "Process status; can repeat: 1,2,4,8,9,15")]
    pub(in crate::app) statuses: Vec<u8>,

    #[arg(long, help = "Modify time from, Unix milliseconds")]
    pub(in crate::app) modify_time_from: String,

    #[arg(
        long,
        help = "Modify time to, Unix milliseconds; span must be under 31 days"
    )]
    pub(in crate::app) modify_time_to: String,

    #[arg(long, help = "Flow definition ID")]
    pub(in crate::app) flow_definition_id: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct CorehrProcessGetArgs {
    #[arg(long, help = "CoreHR process instance ID")]
    pub(in crate::app) process_id: String,

    #[arg(long, value_enum, default_value_t = CorehrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: CorehrUserIdTypeArg,
}

#[derive(Subcommand)]
#[command(after_long_help = HELPDESK_AFTER_HELP)]
pub(in crate::app) enum HelpdeskCommand {
    #[command(subcommand, about = "Read helpdesk tickets and ticket messages")]
    Ticket(HelpdeskTicketCommand),
    #[command(subcommand, about = "Create helpdesk conversations")]
    Service(HelpdeskServiceCommand),
    #[command(subcommand, about = "Send messages through the helpdesk bot")]
    Message(HelpdeskMessageCommand),
    #[command(subcommand, about = "Read helpdesk FAQ categories and articles")]
    Faq(HelpdeskFaqCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskTicketCommand {
    #[command(about = "List tickets with official filters")]
    List(HelpdeskTicketListArgs),
    #[command(about = "Get one ticket detail")]
    Get(HelpdeskTicketGetArgs),
    #[command(about = "List ticket messages")]
    Messages(HelpdeskTicketMessagesArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskServiceCommand {
    #[command(about = "Create a helpdesk conversation for a user")]
    Start(HelpdeskServiceStartArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskMessageCommand {
    #[command(about = "Send text/post/image/card through the helpdesk bot")]
    Send(HelpdeskMessageSendArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HelpdeskFaqCommand {
    #[command(about = "List FAQ categories")]
    Categories(HelpdeskFaqCategoriesArgs),
    #[command(about = "List FAQ articles")]
    List(HelpdeskFaqListArgs),
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketListArgs {
    #[arg(long, help = "Ticket ID filter")]
    pub(in crate::app) ticket_id: Option<String>,

    #[arg(long, help = "Agent open_id filter")]
    pub(in crate::app) agent_id: Option<String>,

    #[arg(long, help = "Closed-by agent open_id filter")]
    pub(in crate::app) closed_by_id: Option<String>,

    #[arg(long = "type", help = "Ticket type: 1 bot, 2 human")]
    pub(in crate::app) ticket_type: Option<u8>,

    #[arg(long, help = "Ticket channel")]
    pub(in crate::app) channel: Option<u8>,

    #[arg(long, help = "Solved filter: 1 unsolved, 2 solved")]
    pub(in crate::app) solved: Option<u8>,

    #[arg(long, help = "Score filter: 1 dissatisfied, 2 normal, 3 satisfied")]
    pub(in crate::app) score: Option<u8>,

    #[arg(long = "status", help = "Ticket status; can repeat")]
    pub(in crate::app) status_list: Vec<u8>,

    #[arg(long, help = "Guest name filter")]
    pub(in crate::app) guest_name: Option<String>,

    #[arg(long, help = "Guest open_id filter")]
    pub(in crate::app) guest_id: Option<String>,

    #[arg(long = "tag", help = "Ticket tag; can repeat")]
    pub(in crate::app) tags: Vec<String>,

    #[arg(long, default_value_t = 1, help = "Page number, starts from 1")]
    pub(in crate::app) page: u32,

    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Create time start, Unix milliseconds")]
    pub(in crate::app) create_time_start: Option<i64>,

    #[arg(long, help = "Create time end, Unix milliseconds")]
    pub(in crate::app) create_time_end: Option<i64>,

    #[arg(long, help = "Update time start, Unix milliseconds")]
    pub(in crate::app) update_time_start: Option<i64>,

    #[arg(long, help = "Update time end, Unix milliseconds")]
    pub(in crate::app) update_time_end: Option<i64>,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketGetArgs {
    #[arg(long, help = "Ticket ID")]
    pub(in crate::app) ticket_id: String,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskTicketMessagesArgs {
    #[arg(long, help = "Ticket ID")]
    pub(in crate::app) ticket_id: String,

    #[arg(long, help = "Message create time start")]
    pub(in crate::app) time_start: Option<i64>,

    #[arg(long, help = "Message create time end")]
    pub(in crate::app) time_end: Option<i64>,

    #[arg(long, default_value_t = 1, help = "Page number")]
    pub(in crate::app) page: u32,

    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskServiceStartArgs {
    #[arg(long, help = "User open_id")]
    pub(in crate::app) open_id: Option<String>,

    #[arg(long, help = "Directly enter human service")]
    pub(in crate::app) human_service: bool,

    #[arg(long = "appointed-agent", help = "Appointed agent open_id; can repeat")]
    pub(in crate::app) appointed_agents: Vec<String>,

    #[arg(long, help = "Customized source info, max 1024 chars")]
    pub(in crate::app) customized_info: Option<String>,

    #[arg(long, help = "Raw Feishu start_service body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw start_service body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw start_service body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskMessageSendArgs {
    #[arg(long, help = "Receiver user ID")]
    pub(in crate::app) receiver_id: Option<String>,

    #[arg(long, default_value = "text", help = "text, post, image, interactive")]
    pub(in crate::app) msg_type: String,

    #[arg(long, help = "Plain text content; builds the official content string")]
    pub(in crate::app) text: Option<String>,

    #[arg(long, help = "Native Feishu message content JSON object")]
    pub(in crate::app) content_json: Option<String>,

    #[arg(long, value_enum, default_value_t = HelpdeskReceiveTypeArg::Chat)]
    pub(in crate::app) receive_type: HelpdeskReceiveTypeArg,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw Feishu helpdesk message body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw helpdesk message body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw helpdesk message body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskFaqCategoriesArgs {
    #[arg(long, help = "Category language, for example zh_cn")]
    pub(in crate::app) lang: Option<String>,

    #[arg(long, help = "Sort key: 1 update time")]
    pub(in crate::app) order_by: Option<u8>,

    #[arg(long, help = "Sort ascending")]
    pub(in crate::app) asc: Option<bool>,
}

#[derive(Args)]
pub(in crate::app) struct HelpdeskFaqListArgs {
    #[arg(long, help = "FAQ category ID")]
    pub(in crate::app) category_id: Option<String>,

    #[arg(long, help = "FAQ status: 1 online, 0 deleted recoverable, 2 deleted")]
    pub(in crate::app) status: Option<String>,

    #[arg(long, help = "Search keyword")]
    pub(in crate::app) search: Option<String>,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,
}

#[derive(Subcommand)]
#[command(after_long_help = HIRE_AFTER_HELP)]
pub(in crate::app) enum HireCommand {
    #[command(subcommand, about = "Operate Hire jobs and job schemas")]
    Job(HireJobCommand),
    #[command(subcommand, about = "Operate Hire talents/candidates")]
    Talent(HireTalentCommand),
    #[command(subcommand, about = "Operate Hire applications/deliveries")]
    Application(HireApplicationCommand),
    #[command(subcommand, about = "Read Hire interviews")]
    Interview(HireInterviewCommand),
    #[command(subcommand, about = "Read Hire recruitment processes")]
    Process(HireProcessCommand),
    #[command(subcommand, about = "Read Hire recruitment requirement schemas")]
    Requirement(HireRequirementCommand),
    #[command(subcommand, about = "Read Hire metadata such as sources and job types")]
    Metadata(HireMetadataCommand),
    #[command(subcommand, about = "Read Hire attachments")]
    Attachment(HireAttachmentCommand),
    #[command(subcommand, about = "Query Hire locations")]
    Location(HireLocationCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireJobCommand {
    #[command(about = "List jobs")]
    List(HireJobListArgs),
    #[command(about = "Get one legacy job record")]
    Get(HireJobGetArgs),
    #[command(about = "Get one detailed job record")]
    Detail(HireJobGetArgs),
    #[command(about = "List job schemas/templates")]
    Schemas(HireJobSchemasArgs),
    #[command(about = "Reopen a closed job")]
    Open(HireJobOpenArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireTalentCommand {
    #[command(about = "List talents/candidates")]
    List(HireTalentListArgs),
    #[command(about = "Get one talent/candidate")]
    Get(HireTalentGetArgs),
    #[command(about = "Create one talent/candidate")]
    Create(HireTalentCreateArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireApplicationCommand {
    #[command(about = "List application IDs")]
    List(HireApplicationListArgs),
    #[command(about = "Get one legacy application")]
    Get(HireApplicationGetArgs),
    #[command(about = "Get one application detail with optional related entities")]
    Detail(HireApplicationDetailArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireInterviewCommand {
    #[command(name = "by-talent", about = "List interviews for one talent")]
    ByTalent(HireInterviewByTalentArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireProcessCommand {
    #[command(about = "List recruitment processes and stages")]
    List(HireListPageArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireRequirementCommand {
    #[command(about = "List recruitment requirement schemas/templates")]
    Schemas(HireListPageArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireMetadataCommand {
    #[command(name = "resume-sources", about = "List resume sources")]
    ResumeSources(HireListPageArgs),
    #[command(name = "job-types", about = "List job types")]
    JobTypes(HireListPageArgs),
    #[command(name = "job-functions", about = "List job functions")]
    JobFunctions(HireListPageArgs),
    #[command(about = "List recruitment subjects/projects")]
    Subjects(HireSubjectsArgs),
    #[command(about = "List recruitment websites")]
    Websites(HireListPageArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireAttachmentCommand {
    #[command(about = "Get attachment metadata and temporary download URL")]
    Get(HireAttachmentGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum HireLocationCommand {
    #[command(about = "Query countries, states, cities, or districts")]
    Query(HireLocationQueryArgs),
}

#[derive(Args)]
pub(in crate::app) struct HireListPageArgs {
    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct HireJobListArgs {
    #[arg(long, help = "Earliest update time, Unix milliseconds")]
    pub(in crate::app) update_start_time: Option<String>,

    #[arg(long, help = "Latest update time, Unix milliseconds")]
    pub(in crate::app) update_end_time: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobLevelIdTypeArg::PeopleAdminJobLevelId)]
    pub(in crate::app) job_level_id_type: HireJobLevelIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId)]
    pub(in crate::app) job_family_id_type: HireJobFamilyIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct HireJobGetArgs {
    #[arg(long, help = "Hire job ID")]
    pub(in crate::app) job_id: String,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobLevelIdTypeArg::PeopleAdminJobLevelId)]
    pub(in crate::app) job_level_id_type: HireJobLevelIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId)]
    pub(in crate::app) job_family_id_type: HireJobFamilyIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct HireJobSchemasArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Job schema scenario: 1 social, 2 campus")]
    pub(in crate::app) scenario: Option<u8>,
}

#[derive(Args)]
pub(in crate::app) struct HireJobOpenArgs {
    #[arg(long, help = "Hire job ID")]
    pub(in crate::app) job_id: String,

    #[arg(long, help = "Whether the reopened job never expires")]
    pub(in crate::app) is_never_expired: Option<bool>,

    #[arg(long, help = "Expiry timestamp in milliseconds when not never-expired")]
    pub(in crate::app) expiry_time: Option<i64>,

    #[arg(long, help = "Raw Feishu job open body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw job open body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw job open body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HireTalentListArgs {
    #[arg(long, help = "Search keyword, supports Hire boolean query syntax")]
    pub(in crate::app) keyword: Option<String>,

    #[arg(long, help = "Earliest update time, Unix milliseconds")]
    pub(in crate::app) update_start_time: Option<String>,

    #[arg(long, help = "Latest update time, Unix milliseconds")]
    pub(in crate::app) update_end_time: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(
        long,
        help = "Sort rule: 1 update desc, 2 relevance desc, 3 delivery time desc, 4 talent create time desc"
    )]
    pub(in crate::app) sort_by: Option<u8>,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::PeopleAdminId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, help = "Request option such as ignore_empty_error")]
    pub(in crate::app) query_option: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct HireTalentGetArgs {
    #[arg(long, help = "Hire talent/candidate ID")]
    pub(in crate::app) talent_id: String,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::PeopleAdminId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,
}

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

use super::*;

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

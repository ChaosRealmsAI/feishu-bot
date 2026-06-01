use super::*;
#[derive(Args)]
pub(in crate::app) struct HireTalentCreateArgs {
    #[arg(long, help = "Candidate name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Candidate email")]
    pub(in crate::app) email: Option<String>,

    #[arg(long, help = "Candidate mobile number")]
    pub(in crate::app) mobile: Option<String>,

    #[arg(long, help = "Mobile country code, for example CN_1")]
    pub(in crate::app) mobile_country_code: Option<String>,

    #[arg(long, help = "Current city code")]
    pub(in crate::app) current_city_code: Option<String>,

    #[arg(long, help = "Resume source ID")]
    pub(in crate::app) resume_source_id: Option<String>,

    #[arg(long = "folder-id", help = "Talent folder ID; can repeat")]
    pub(in crate::app) folder_ids: Vec<String>,

    #[arg(long, help = "Creator ID matching --user-id-type")]
    pub(in crate::app) creator_id: Option<String>,

    #[arg(long, help = "Creator account type: 1 employee, 3 system")]
    pub(in crate::app) creator_account_type: Option<u8>,

    #[arg(long, help = "Resume attachment ID")]
    pub(in crate::app) resume_attachment_id: Option<String>,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, help = "Raw Feishu combined_create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw combined_create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw combined_create body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct HireApplicationListArgs {
    #[arg(long, help = "Recruitment process ID")]
    pub(in crate::app) process_id: Option<String>,

    #[arg(long, help = "Recruitment stage ID")]
    pub(in crate::app) stage_id: Option<String>,

    #[arg(long, help = "Talent/candidate ID")]
    pub(in crate::app) talent_id: Option<String>,

    #[arg(long, help = "Active status: 1 active, 2 inactive, 3 all")]
    pub(in crate::app) active_status: Option<String>,

    #[arg(long, help = "Hire job ID")]
    pub(in crate::app) job_id: Option<String>,

    #[arg(long = "lock-status", help = "Lock status; can repeat")]
    pub(in crate::app) lock_status: Vec<u8>,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Earliest update time, Unix milliseconds")]
    pub(in crate::app) update_start_time: Option<String>,

    #[arg(long, help = "Latest update time, Unix milliseconds")]
    pub(in crate::app) update_end_time: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct HireApplicationGetArgs {
    #[arg(long, help = "Hire application/delivery ID")]
    pub(in crate::app) application_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long = "option", help = "Legacy application get option; can repeat")]
    pub(in crate::app) options: Vec<String>,
}

#[derive(Args)]
pub(in crate::app) struct HireApplicationDetailArgs {
    #[arg(long, help = "Hire application/delivery ID")]
    pub(in crate::app) application_id: String,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = DepartmentIdTypeArg::OpenDepartmentId)]
    pub(in crate::app) department_id_type: DepartmentIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobLevelIdTypeArg::JobLevelId)]
    pub(in crate::app) job_level_id_type: HireJobLevelIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobFamilyIdTypeArg::JobFamilyId)]
    pub(in crate::app) job_family_id_type: HireJobFamilyIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId)]
    pub(in crate::app) employee_type_id_type: HireEmployeeTypeIdTypeArg,

    #[arg(
        long = "option",
        help = "Related entity option, e.g. with_job; can repeat"
    )]
    pub(in crate::app) options: Vec<String>,
}

#[derive(Args)]
pub(in crate::app) struct HireInterviewByTalentArgs {
    #[arg(long, help = "Talent/candidate ID")]
    pub(in crate::app) talent_id: String,

    #[arg(long, value_enum, default_value_t = HireUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: HireUserIdTypeArg,

    #[arg(long, value_enum, default_value_t = HireJobLevelIdTypeArg::PeopleAdminJobLevelId)]
    pub(in crate::app) job_level_id_type: HireJobLevelIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct HireSubjectsArgs {
    #[arg(long, default_value_t = 20, help = "Page size, max 200")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct HireAttachmentGetArgs {
    #[arg(long, help = "Hire attachment ID")]
    pub(in crate::app) attachment_id: String,

    #[arg(long = "type", help = "Attachment type: 1 resume, 2 works, 3 common")]
    pub(in crate::app) attachment_type: Option<u8>,
}

#[derive(Args)]
pub(in crate::app) struct HireLocationQueryArgs {
    #[arg(
        long,
        help = "Location type: 1 country, 2 state/province, 3 city, 4 district"
    )]
    pub(in crate::app) location_type: Option<u8>,

    #[arg(long = "code", help = "Location code; can repeat")]
    pub(in crate::app) code_list: Vec<String>,

    #[arg(long, default_value_t = 100, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Raw Feishu location query body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw location query body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw location query body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = WIKI_AFTER_HELP)]
pub(in crate::app) enum WikiCommand {
    #[command(about = "Check whether the configured Wiki route is usable")]
    RouteCheck(WikiRouteCheckArgs),
    #[command(about = "Create wiki space; requires user_access_token")]
    CreateSpace(WikiCreateSpaceArgs),
    #[command(about = "List wiki spaces")]
    Spaces(WikiSpacesArgs),
    #[command(about = "Get wiki space info")]
    Space(WikiSpaceGetArgs),
    #[command(about = "List child nodes in a space")]
    Nodes(WikiNodesArgs),
    #[command(about = "Get node by token and object type")]
    Node(WikiNodeGetArgs),
    #[command(about = "Create wiki node")]
    CreateNode(WikiCreateNodeArgs),
    #[command(about = "Move wiki node inside or across spaces")]
    MoveNode(WikiMoveNodeArgs),
    #[command(about = "Copy wiki node inside or across spaces")]
    CopyNode(WikiCopyNodeArgs),
    #[command(about = "Update wiki node title")]
    UpdateTitle(WikiUpdateTitleArgs),
    #[command(about = "Move existing cloud docs into wiki")]
    MoveDocsToWiki(WikiMoveDocsToWikiArgs),
    #[command(subcommand, about = "Manage wiki space members")]
    Member(WikiMemberCommand),
    #[command(subcommand, about = "Manage wiki space settings")]
    Setting(WikiSettingCommand),
    #[command(about = "Get wiki async task result")]
    Task(WikiTaskArgs),
    #[command(about = "Search wiki nodes; requires user_access_token")]
    Search(WikiSearchArgs),
}

#[derive(Args)]
pub(in crate::app) struct WikiRouteCheckArgs {
    #[arg(
        long,
        help = "Wiki space ID to check; defaults to FEISHU_WIKI_SPACE_ID"
    )]
    pub(in crate::app) space_id: Option<String>,

    #[arg(
        long,
        help = "Parent Wiki node token for future publishing; defaults to FEISHU_WIKI_PARENT_NODE_TOKEN"
    )]
    pub(in crate::app) parent_node_token: Option<String>,

    #[arg(long, default_value_t = 1, help = "Page size for read checks")]
    pub(in crate::app) page_size: u16,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(
        long,
        help = "Create a small docx and attempt move_docs_to_wiki to prove write publishing"
    )]
    pub(in crate::app) write_probe: bool,

    #[arg(long, help = "Title for --write-probe proof document")]
    pub(in crate::app) write_probe_title: Option<String>,

    #[arg(
        long,
        help = "Ask Feishu to apply for move approval during --write-probe"
    )]
    pub(in crate::app) write_probe_apply: bool,

    #[arg(
        long,
        help = "Print the check result, then exit non-zero when route_ready is false"
    )]
    pub(in crate::app) strict: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiCreateSpaceArgs {
    #[arg(long, help = "Wiki space name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Wiki space description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Open sharing status: open or closed")]
    pub(in crate::app) open_sharing: Option<String>,

    #[arg(long, help = "Raw Feishu wiki space body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read wiki space body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read wiki space body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiSpacesArgs {
    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiSpaceGetArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Display language for My Document Library spaces")]
    pub(in crate::app) lang: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiNodesArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Parent node token")]
    pub(in crate::app) parent_node_token: Option<String>,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiNodeGetArgs {
    #[arg(long, help = "Node token or object token")]
    pub(in crate::app) token: String,

    #[arg(long, help = "Object type when token is an underlying doc token")]
    pub(in crate::app) obj_type: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiCreateNodeArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(
        long,
        default_value = "docx",
        help = "Object type: docx, sheet, bitable, mindnote, file, slides"
    )]
    pub(in crate::app) obj_type: String,

    #[arg(long, default_value = "origin", help = "Node type: origin or shortcut")]
    pub(in crate::app) node_type: String,

    #[arg(long, help = "Parent wiki node token; omit for root")]
    pub(in crate::app) parent_node_token: Option<String>,

    #[arg(long, help = "Original node token when creating a shortcut")]
    pub(in crate::app) origin_node_token: Option<String>,

    #[arg(long, help = "Wiki node title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu wiki body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read wiki body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read wiki body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiMoveNodeArgs {
    #[arg(long, help = "Source wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Wiki node token to move")]
    pub(in crate::app) node_token: String,

    #[arg(long, help = "Destination parent node token")]
    pub(in crate::app) target_parent_token: Option<String>,

    #[arg(long, help = "Destination wiki space ID")]
    pub(in crate::app) target_space_id: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu wiki move body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read wiki move body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read wiki move body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiCopyNodeArgs {
    #[arg(long, help = "Source wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Wiki node token to copy")]
    pub(in crate::app) node_token: String,

    #[arg(long, help = "Destination parent node token")]
    pub(in crate::app) target_parent_token: Option<String>,

    #[arg(long, help = "Destination wiki space ID")]
    pub(in crate::app) target_space_id: Option<String>,

    #[arg(long, help = "Copied node title; omit to keep original title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu wiki copy body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read wiki copy body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read wiki copy body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiUpdateTitleArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Wiki node token")]
    pub(in crate::app) node_token: String,

    #[arg(long, help = "New title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu title body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read title body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read title body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiMoveDocsToWikiArgs {
    #[arg(long, help = "Target wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Parent wiki node token; omit for root")]
    pub(in crate::app) parent_wiki_token: Option<String>,

    #[arg(
        long,
        help = "Object type: docx, doc, sheet, bitable, mindnote, file, slides"
    )]
    pub(in crate::app) obj_type: Option<String>,

    #[arg(long, help = "Underlying cloud document token")]
    pub(in crate::app) obj_token: Option<String>,

    #[arg(
        long,
        help = "Apply for move approval when lacking document permission"
    )]
    pub(in crate::app) apply: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu move_docs_to_wiki body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read move_docs_to_wiki body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read move_docs_to_wiki body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum WikiMemberCommand {
    #[command(about = "List wiki space members")]
    List(WikiMemberListArgs),
    #[command(about = "Add wiki space member or admin")]
    Add(WikiMemberAddArgs),
    #[command(about = "Delete wiki space member or admin")]
    Delete(WikiMemberDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct WikiMemberListArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiMemberAddArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(
        long,
        help = "Member type: openchat, userid, email, opendepartmentid, openid, unionid"
    )]
    pub(in crate::app) member_type: Option<String>,

    #[arg(long, help = "Member ID matching --member-type")]
    pub(in crate::app) member_id: Option<String>,

    #[arg(long, default_value = "member", help = "Member role: admin or member")]
    pub(in crate::app) member_role: String,

    #[arg(long, help = "Whether Feishu should notify the member")]
    pub(in crate::app) need_notification: Option<bool>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu member body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read member body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiMemberDeleteArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Member ID matching --member-type")]
    pub(in crate::app) member_id: String,

    #[arg(
        long,
        help = "Member type: openchat, userid, email, opendepartmentid, openid, unionid"
    )]
    pub(in crate::app) member_type: Option<String>,

    #[arg(long, default_value = "member", help = "Member role: admin or member")]
    pub(in crate::app) member_role: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu delete member body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read delete member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read delete member body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum WikiSettingCommand {
    #[command(about = "Update wiki space settings")]
    Update(WikiSettingUpdateArgs),
}

#[derive(Args)]
pub(in crate::app) struct WikiSettingUpdateArgs {
    #[arg(long, help = "Wiki space ID")]
    pub(in crate::app) space_id: String,

    #[arg(long, help = "Who can create root pages: admin or admin_and_member")]
    pub(in crate::app) create_setting: Option<String>,

    #[arg(
        long,
        help = "Whether readers can copy/export/print: allow or not_allow"
    )]
    pub(in crate::app) security_setting: Option<String>,

    #[arg(long, help = "Whether readers can comment: allow or not_allow")]
    pub(in crate::app) comment_setting: Option<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu setting body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read setting body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read setting body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct WikiTaskArgs {
    #[arg(long, help = "Wiki async task ID")]
    pub(in crate::app) task_id: String,

    #[arg(long, default_value = "move", help = "Task type; currently move")]
    pub(in crate::app) task_type: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct WikiSearchArgs {
    #[arg(long, help = "Search keyword, up to 50 characters")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, help = "Restrict to wiki space ID")]
    pub(in crate::app) space_id: Option<String>,

    #[arg(
        long,
        help = "Restrict to a node and its descendants; requires --space-id"
    )]
    pub(in crate::app) node_id: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, 1..50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Raw Feishu wiki search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read wiki search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read wiki search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = SHEET_AFTER_HELP)]
pub(in crate::app) enum SheetCommand {
    #[command(about = "Create a spreadsheet")]
    Create(SheetCreateArgs),
    #[command(about = "Get spreadsheet metadata")]
    Get(SheetTokenArgs),
    #[command(about = "List sheets in a spreadsheet")]
    Sheets(SheetTokenArgs),
    #[command(about = "Get one sheet tab by sheet_id")]
    GetSheet(SheetGetArgs),
    #[command(about = "Add a sheet tab to a spreadsheet")]
    AddSheet(SheetAddArgs),
    #[command(about = "Copy a sheet tab inside a spreadsheet")]
    CopySheet(SheetCopyArgs),
    #[command(about = "Delete a sheet tab")]
    DeleteSheet(SheetDeleteArgs),
    #[command(about = "Update sheet tab properties")]
    UpdateSheet(SheetUpdateArgs),
    #[command(about = "Merge a cell range")]
    Merge(SheetMergeArgs),
    #[command(about = "Unmerge a cell range")]
    Unmerge(SheetUnmergeArgs),
    #[command(about = "Batch set cell styles")]
    Style(SheetStyleArgs),
    #[command(subcommand, about = "Operate sheet cell values")]
    Values(SheetValuesCommand),
    #[command(about = "Run sheets_batch_update")]
    BatchUpdate(SheetBodyArgs),
}

#[derive(Args)]
pub(in crate::app) struct SheetCreateArgs {
    #[arg(long, help = "Spreadsheet title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "Drive folder token")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(long, help = "Raw Feishu spreadsheet create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read spreadsheet create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read spreadsheet create body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetTokenArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,
}

#[derive(Args)]
pub(in crate::app) struct SheetGetArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Sheet ID")]
    pub(in crate::app) sheet_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SheetAddArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "New sheet title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "New sheet position index")]
    pub(in crate::app) index: Option<i64>,

    #[arg(long, help = "Raw Feishu addSheet body JSON or full requests body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read addSheet body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read addSheet body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetCopyArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Source sheet_id")]
    pub(in crate::app) sheet_id: String,

    #[arg(long, help = "Copied sheet title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "Copied sheet position index")]
    pub(in crate::app) index: Option<i64>,

    #[arg(long, help = "Raw Feishu copySheet body JSON or full requests body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read copySheet body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read copySheet body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetDeleteArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Sheet ID to delete")]
    pub(in crate::app) sheet_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SheetUpdateArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Sheet ID to update")]
    pub(in crate::app) sheet_id: String,

    #[arg(long, help = "New sheet title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "Move sheet to this position index")]
    pub(in crate::app) index: Option<i64>,

    #[arg(long, help = "Hide or show the sheet")]
    pub(in crate::app) hidden: Option<bool>,

    #[arg(long, help = "Frozen row count")]
    pub(in crate::app) frozen_row_count: Option<i64>,

    #[arg(long, help = "Frozen column count")]
    pub(in crate::app) frozen_col_count: Option<i64>,

    #[arg(long, help = "Protect lock value, usually LOCK or UNLOCK")]
    pub(in crate::app) protect_lock: Option<String>,

    #[arg(long, help = "Protect lock message")]
    pub(in crate::app) lock_info: Option<String>,

    #[arg(
        long = "protect-user",
        help = "User ID allowed to edit protected sheet; can repeat"
    )]
    pub(in crate::app) protect_users: Vec<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Raw Feishu updateSheet body JSON or full requests body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read updateSheet body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read updateSheet body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetMergeArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Range such as Sheet1!A1:C1")]
    pub(in crate::app) range: Option<String>,

    #[arg(
        long,
        default_value = "MERGE_ALL",
        help = "MERGE_ALL, MERGE_ROWS, MERGE_COLUMNS, or shorthand all/rows/columns"
    )]
    pub(in crate::app) merge_type: String,

    #[arg(long, help = "Raw Feishu merge_cells body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read merge_cells body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read merge_cells body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SheetUnmergeArgs {
    #[arg(long, help = "Spreadsheet token")]
    pub(in crate::app) spreadsheet_token: String,

    #[arg(long, help = "Range such as Sheet1!A1:C1")]
    pub(in crate::app) range: Option<String>,

    #[arg(long, help = "Raw Feishu unmerge_cells body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read unmerge_cells body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read unmerge_cells body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

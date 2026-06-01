use super::*;
#[derive(Subcommand)]
pub(in crate::app) enum BaseFieldCommand {
    #[command(about = "List fields in a Base table")]
    List(BaseFieldListArgs),
    #[command(about = "Create a field in a Base table")]
    Create(BaseFieldCreateArgs),
    #[command(about = "Update a field in a Base table")]
    Update(BaseFieldUpdateArgs),
    #[command(about = "Delete a field in a Base table")]
    Delete(BaseFieldRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseFieldListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Optional view_id")]
    pub(in crate::app) view_id: Option<String>,

    #[arg(
        long,
        help = "Return field description as structured text segments instead of a plain string"
    )]
    pub(in crate::app) text_field_as_array: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseFieldCreateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Field name")]
    pub(in crate::app) name: String,

    #[arg(long = "type", help = "Feishu field type integer, for example 1=text")]
    pub(in crate::app) field_type: Option<i64>,

    #[arg(
        long,
        value_enum,
        help = "Typed field kind; avoids memorizing Feishu type integers"
    )]
    pub(in crate::app) kind: Option<BaseFieldKindArg>,

    #[arg(
        long = "option",
        help = "Single/multi-select option as name or name:color. Can repeat."
    )]
    pub(in crate::app) options: Vec<String>,

    #[arg(
        long,
        help = "Number/currency/progress/formula formatter, e.g. 0.00 or ¥0.00"
    )]
    pub(in crate::app) formatter: Option<String>,

    #[arg(long, help = "Currency code for --kind currency, e.g. CNY or USD")]
    pub(in crate::app) currency_code: Option<String>,

    #[arg(long, help = "Date formatter, e.g. yyyy/MM/dd or yyyy/MM/dd HH:mm")]
    pub(in crate::app) date_formatter: Option<String>,

    #[arg(long, help = "Date field auto_fill flag")]
    pub(in crate::app) auto_fill: Option<bool>,

    #[arg(long, help = "Allow multiple users/records/groups where supported")]
    pub(in crate::app) multiple: Option<bool>,

    #[arg(long, help = "Target table_id for link/duplex-link fields")]
    pub(in crate::app) linked_table_id: Option<String>,

    #[arg(long, help = "Formula expression for --kind formula")]
    pub(in crate::app) formula: Option<String>,

    #[arg(long, help = "Location input type: not_limit or only_mobile")]
    pub(in crate::app) location_input_type: Option<String>,

    #[arg(long, help = "Raw JSON for field.property")]
    pub(in crate::app) property_json: Option<String>,

    #[arg(long, help = "Raw JSON for field.description")]
    pub(in crate::app) description_json: Option<String>,

    #[arg(long, help = "Feishu UI type such as Text, Number, Progress, Email")]
    pub(in crate::app) ui_type: Option<String>,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) client_token: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "kebab_case")]
pub(in crate::app) enum BaseFieldKindArg {
    Text,
    Barcode,
    Email,
    Number,
    Progress,
    Currency,
    Rating,
    SingleSelect,
    MultiSelect,
    Date,
    Checkbox,
    User,
    Phone,
    Url,
    Attachment,
    Link,
    Formula,
    DuplexLink,
    Location,
    Group,
    AutoNumber,
}

#[derive(Args)]
pub(in crate::app) struct BaseFieldRefArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Base field_id")]
    pub(in crate::app) field_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseFieldUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Base field_id")]
    pub(in crate::app) field_id: String,

    #[arg(long, help = "Field name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long = "type", help = "Feishu field type integer, for example 1=text")]
    pub(in crate::app) field_type: Option<i64>,

    #[arg(
        long,
        value_enum,
        help = "Typed field kind; avoids memorizing Feishu type integers"
    )]
    pub(in crate::app) kind: Option<BaseFieldKindArg>,

    #[arg(
        long = "option",
        help = "Single/multi-select option as name or name:color. Can repeat."
    )]
    pub(in crate::app) options: Vec<String>,

    #[arg(
        long,
        help = "Number/currency/progress/formula formatter, e.g. 0.00 or ¥0.00"
    )]
    pub(in crate::app) formatter: Option<String>,

    #[arg(long, help = "Currency code for --kind currency, e.g. CNY or USD")]
    pub(in crate::app) currency_code: Option<String>,

    #[arg(long, help = "Date formatter, e.g. yyyy/MM/dd or yyyy/MM/dd HH:mm")]
    pub(in crate::app) date_formatter: Option<String>,

    #[arg(long, help = "Date field auto_fill flag")]
    pub(in crate::app) auto_fill: Option<bool>,

    #[arg(long, help = "Allow multiple users/records/groups where supported")]
    pub(in crate::app) multiple: Option<bool>,

    #[arg(long, help = "Target table_id for link/duplex-link fields")]
    pub(in crate::app) linked_table_id: Option<String>,

    #[arg(long, help = "Formula expression for --kind formula")]
    pub(in crate::app) formula: Option<String>,

    #[arg(long, help = "Location input type: not_limit or only_mobile")]
    pub(in crate::app) location_input_type: Option<String>,

    #[arg(long, help = "Raw JSON for field.property")]
    pub(in crate::app) property_json: Option<String>,

    #[arg(long, help = "Raw JSON for field.description")]
    pub(in crate::app) description_json: Option<String>,

    #[arg(long, help = "Feishu UI type such as Text, Number, Progress, Email")]
    pub(in crate::app) ui_type: Option<String>,

    #[arg(long, help = "Raw JSON object for field update")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read field update JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read field update JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseViewCommand {
    #[command(about = "List views in a Base table")]
    List(BaseViewListArgs),
    #[command(about = "Create a view in a Base table")]
    Create(BaseViewCreateArgs),
    #[command(about = "Get one view")]
    Get(BaseViewRefArgs),
    #[command(about = "Patch a view name or property")]
    Update(BaseViewUpdateArgs),
    #[command(about = "Delete one view")]
    Delete(BaseViewRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseViewListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseViewCreateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "View name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long,
        default_value = "grid",
        help = "View type: grid, kanban, gallery, gantt, form"
    )]
    pub(in crate::app) view_type: String,

    #[arg(long, help = "Raw Feishu view create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read view create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read view create body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseViewRefArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Base view_id")]
    pub(in crate::app) view_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseViewUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Base view_id")]
    pub(in crate::app) view_id: String,

    #[arg(long, help = "New view name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long = "hidden-field-id",
        help = "Hide a field in the view property.hidden_fields array. Can repeat."
    )]
    pub(in crate::app) hidden_field_ids: Vec<String>,

    #[arg(long, help = "Filter conjunction for property.filter_info: and or or")]
    pub(in crate::app) filter_conjunction: Option<String>,

    #[arg(
        long = "filter-condition",
        help = "Filter condition as field_id:field_type:operator:value. Can repeat. Prefix value with json: to compact JSON into the official string value."
    )]
    pub(in crate::app) filter_conditions: Vec<String>,

    #[arg(long, help = "Set property.filter_info.condition_omitted")]
    pub(in crate::app) filter_condition_omitted: Option<bool>,

    #[arg(
        long,
        help = "Set property.hierarchy_config.field_id for hierarchy/sub-record views"
    )]
    pub(in crate::app) hierarchy_field_id: Option<String>,

    #[arg(long, help = "Raw JSON for view.property")]
    pub(in crate::app) property_json: Option<String>,

    #[arg(long, help = "Raw Feishu view patch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read view patch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read view patch body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseRecordCommand {
    #[command(about = "List records in a Base table")]
    List(BaseRecordListArgs),
    #[command(about = "Search records in a Base table")]
    Search(BaseRecordSearchArgs),
    #[command(about = "Get one record")]
    Get(BaseRecordGetArgs),
    #[command(about = "Get multiple records by record_id")]
    BatchGet(BaseRecordBatchGetArgs),
    #[command(about = "Create one record")]
    Create(BaseRecordWriteArgs),
    #[command(about = "Create multiple records")]
    BatchCreate(BaseRecordBatchCreateArgs),
    #[command(about = "Update one record")]
    Update(BaseRecordUpdateArgs),
    #[command(about = "Update multiple records")]
    BatchUpdate(BaseRecordBatchUpdateArgs),
    #[command(about = "Delete one record")]
    Delete(BaseRecordDeleteArgs),
    #[command(about = "Delete multiple records")]
    BatchDelete(BaseRecordBatchDeleteArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseDashboardCommand {
    #[command(about = "List dashboards in a Base")]
    List(BaseDashboardListArgs),
    #[command(about = "Copy a dashboard in a Base")]
    Copy(BaseDashboardCopyArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseDashboardListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseDashboardCopyArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Dashboard block_id")]
    pub(in crate::app) block_id: String,

    #[arg(long, help = "Copied dashboard name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw JSON object for dashboard copy body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read dashboard copy body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read dashboard copy body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseWorkflowCommand {
    #[command(about = "List automation workflows in a Base")]
    List(BaseWorkflowListArgs),
    #[command(about = "List block workflows in a Base")]
    BlockList(BaseWorkflowBlockListArgs),
    #[command(about = "Enable or disable an automation workflow")]
    Update(BaseWorkflowUpdateArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseWorkflowListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseWorkflowBlockListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub(in crate::app) enum BaseWorkflowStatusArg {
    Enable,
    Disable,
}

impl BaseWorkflowStatusArg {
    pub(in crate::app) fn as_feishu(self) -> &'static str {
        match self {
            Self::Enable => "Enable",
            Self::Disable => "Disable",
        }
    }
}

#[derive(Args)]
pub(in crate::app) struct BaseWorkflowUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Workflow ID from workflow list or block-list")]
    pub(in crate::app) workflow_id: String,

    #[arg(long, value_enum, help = "Workflow status: enable or disable")]
    pub(in crate::app) status: Option<BaseWorkflowStatusArg>,

    #[arg(long, help = "Raw JSON object for workflow update body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read workflow update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read workflow update body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseFormCommand {
    #[command(about = "Get Base form metadata")]
    Get(BaseFormRefArgs),
    #[command(about = "Patch Base form metadata")]
    Update(BaseFormUpdateArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseFormRefArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Form ID, usually a form view_id")]
    pub(in crate::app) form_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseFormUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Form ID, usually a form view_id")]
    pub(in crate::app) form_id: String,

    #[arg(long, help = "Raw JSON object for form update body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read form update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read form update body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseRoleCommand {
    #[command(about = "List advanced permission roles in a Base")]
    List(BaseRoleListArgs),
    #[command(about = "Create an advanced permission role")]
    Create(BaseRoleWriteArgs),
    #[command(about = "Update an advanced permission role")]
    Update(BaseRoleUpdateArgs),
    #[command(about = "Delete an advanced permission role")]
    Delete(BaseRoleRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseRoleListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(
        long,
        value_enum,
        default_value_t = BaseRoleApiVersionArg::V1,
        help = "Role API version: v1=bitable/v1, v2=base/v2 advanced permissions 2.0"
    )]
    pub(in crate::app) api_version: BaseRoleApiVersionArg,

    #[arg(long, default_value_t = 30, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseRoleRefArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseRoleWriteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(
        long,
        value_enum,
        default_value_t = BaseRoleApiVersionArg::V1,
        help = "Role API version: v1=bitable/v1, v2=base/v2 advanced permissions 2.0"
    )]
    pub(in crate::app) api_version: BaseRoleApiVersionArg,

    #[arg(long, help = "Role name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw JSON array for role.table_roles")]
    pub(in crate::app) table_roles_json: Option<String>,

    #[arg(long, help = "Raw JSON array for role.block_roles")]
    pub(in crate::app) block_roles_json: Option<String>,

    #[arg(
        long,
        help = "Raw JSON object for role.base_rule; v2 supports base_complex_edit and copy"
    )]
    pub(in crate::app) base_rule_json: Option<String>,

    #[arg(
        long,
        help = "Advanced permissions 2.0 base_rule.base_complex_edit: allow copy/download/print Base"
    )]
    pub(in crate::app) allow_base_complex_edit: Option<bool>,

    #[arg(
        long,
        help = "Advanced permissions 2.0 base_rule.copy: allow copying Base content"
    )]
    pub(in crate::app) allow_copy: Option<bool>,

    #[arg(long, help = "Raw JSON object for role body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read role body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read role body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseRoleUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,

    #[arg(long, help = "Role name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw JSON array for role.table_roles")]
    pub(in crate::app) table_roles_json: Option<String>,

    #[arg(long, help = "Raw JSON array for role.block_roles")]
    pub(in crate::app) block_roles_json: Option<String>,

    #[arg(long, help = "Raw JSON object for role body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read role body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read role body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::app) enum BaseRoleApiVersionArg {
    V1,
    V2,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseMemberCommand {
    #[command(about = "List members of an advanced permission role")]
    List(BaseMemberListArgs),
    #[command(about = "Add one member to an advanced permission role")]
    Add(BaseMemberAddArgs),
    #[command(about = "Delete one member from an advanced permission role")]
    Delete(BaseMemberDeleteArgs),
    #[command(about = "Batch add members to an advanced permission role")]
    BatchAdd(BaseMemberBatchArgs),
    #[command(about = "Batch delete members from an advanced permission role")]
    BatchDelete(BaseMemberBatchArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseMemberListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseMemberAddArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,

    #[arg(long, help = "Member ID")]
    pub(in crate::app) member_id: Option<String>,

    #[arg(
        long,
        default_value = "open_id",
        help = "Member ID type: open_id, union_id, user_id, chat_id, department_id, open_department_id"
    )]
    pub(in crate::app) member_id_type: String,

    #[arg(long, help = "Raw JSON object for member add body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read member add body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read member add body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseMemberDeleteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,

    #[arg(long, help = "Member ID")]
    pub(in crate::app) member_id: String,

    #[arg(
        long,
        default_value = "open_id",
        help = "Member ID type: open_id, union_id, user_id, chat_id, department_id, open_department_id"
    )]
    pub(in crate::app) member_id_type: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseMemberBatchArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base advanced permission role_id")]
    pub(in crate::app) role_id: String,

    #[arg(
        long = "member",
        help = "Member as type:id, for example open_id:ou_xxx or chat_id:oc_xxx. Repeatable."
    )]
    pub(in crate::app) members: Vec<String>,

    #[arg(long, help = "Raw JSON array or object with member_list")]
    pub(in crate::app) member_list_json: Option<String>,

    #[arg(long, help = "Raw JSON object for batch member body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read batch member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read batch member body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Optional view_id")]
    pub(in crate::app) view_id: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordSearchArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Raw Feishu records/search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read records/search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read records/search body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(
        long,
        help = "Search within a Base view; ignored by Feishu when filter/sort is set"
    )]
    pub(in crate::app) view_id: Option<String>,

    #[arg(long = "field-name", help = "Field name to return. Can repeat.")]
    pub(in crate::app) field_names: Vec<String>,

    #[arg(long, help = "JSON array/object with field_names to return")]
    pub(in crate::app) field_names_json: Option<String>,

    #[arg(long, help = "Raw Feishu filter object JSON")]
    pub(in crate::app) filter_json: Option<String>,

    #[arg(long, help = "Raw Feishu sort array JSON")]
    pub(in crate::app) sort_json: Option<String>,

    #[arg(long, help = "Return created/modified time and user automatic fields")]
    pub(in crate::app) automatic_fields: bool,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordGetArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Record ID")]
    pub(in crate::app) record_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordBatchGetArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long = "record-id", help = "Record ID. Can be repeated.")]
    pub(in crate::app) record_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array/object for record_ids")]
    pub(in crate::app) record_ids_json: Option<String>,

    #[arg(long, help = "Read record_ids JSON array/object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read record_ids JSON array/object from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordWriteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Raw JSON object for record fields")]
    pub(in crate::app) fields_json: Option<String>,

    #[arg(long, help = "Read record fields JSON object from file")]
    pub(in crate::app) fields_file: Option<PathBuf>,

    #[arg(long, help = "Read record fields JSON object from stdin")]
    pub(in crate::app) fields_stdin: bool,

    #[arg(
        long = "field",
        help = "Record field as name=value. Values auto-parse JSON; use json:<value> or str:<value>. Can repeat."
    )]
    pub(in crate::app) fields: Vec<String>,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Set ignore_consistency_check=true")]
    pub(in crate::app) ignore_consistency_check: bool,
}

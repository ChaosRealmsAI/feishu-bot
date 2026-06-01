use super::*;
#[derive(Args)]
pub(in crate::app) struct TaskCommentUpdateArgs {
    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(long, help = "New comment content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Raw Feishu comment update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read comment update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read comment update body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCommentDeleteArgs {
    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskSubtaskCommand {
    #[command(about = "Create a subtask")]
    Create(TaskSubtaskCreateArgs),
    #[command(about = "List subtasks")]
    List(TaskSubtaskListArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskSubtaskCreateArgs {
    #[arg(long, help = "Parent task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long, help = "Subtask title/summary")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Subtask description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Due timestamp in milliseconds")]
    pub(in crate::app) due_ms: Option<String>,

    #[arg(
        long,
        help = "Due time as RFC3339 or local 'YYYY-MM-DD HH:MM[:SS]'; converted to milliseconds"
    )]
    pub(in crate::app) due_at: Option<String>,

    #[arg(
        long,
        help = "Due all-day date as YYYY-MM-DD; sets due.is_all_day=true"
    )]
    pub(in crate::app) due_date: Option<String>,

    #[arg(long, help = "Treat --due-ms as an all-day date timestamp")]
    pub(in crate::app) due_all_day: bool,

    #[arg(long, help = "Start timestamp in milliseconds")]
    pub(in crate::app) start_ms: Option<String>,

    #[arg(
        long,
        help = "Start time as RFC3339 or local 'YYYY-MM-DD HH:MM[:SS]'; converted to milliseconds"
    )]
    pub(in crate::app) start_at: Option<String>,

    #[arg(
        long,
        help = "Start all-day date as YYYY-MM-DD; sets start.is_all_day=true"
    )]
    pub(in crate::app) start_date: Option<String>,

    #[arg(long, help = "Treat --start-ms as an all-day date timestamp")]
    pub(in crate::app) start_all_day: bool,

    #[arg(
        long,
        help = "Completed timestamp in milliseconds; use 0 for unfinished"
    )]
    pub(in crate::app) completed_at: Option<String>,

    #[arg(long, help = "RFC5545 RRULE string, e.g. FREQ=WEEKLY;INTERVAL=1")]
    pub(in crate::app) repeat_rule: Option<String>,

    #[arg(long, help = "Raw JSON object for custom_complete")]
    pub(in crate::app) custom_complete_json: Option<String>,

    #[arg(long, help = "Raw JSON object for third-party origin; create-only")]
    pub(in crate::app) origin_json: Option<String>,

    #[arg(long, help = "Caller-defined extra string, often Base64")]
    pub(in crate::app) extra: Option<String>,

    #[arg(long, help = "Task completion mode: 1=all assignees, 2=any assignee")]
    pub(in crate::app) mode: Option<u8>,

    #[arg(long, help = "Whether this subtask is a milestone, pass true or false")]
    pub(in crate::app) is_milestone: Option<bool>,

    #[arg(long, help = "Raw JSON array for initial reminders")]
    pub(in crate::app) reminders_json: Option<String>,

    #[arg(
        long = "reminder-minute",
        help = "Initial reminder minutes before due time; requires due. 0 means at due time"
    )]
    pub(in crate::app) reminder_minute: Option<i64>,

    #[arg(long, help = "Raw JSON array for initial custom_fields")]
    pub(in crate::app) custom_fields_json: Option<String>,

    #[arg(long, help = "Raw JSON object for docx_source")]
    pub(in crate::app) docx_source_json: Option<String>,

    #[arg(long, help = "Raw Feishu subtask create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read subtask create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read subtask create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency token. Defaults to a random UUID.")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSubtaskListArgs {
    #[arg(long, help = "Parent task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskSectionCommand {
    #[command(about = "List custom sections under a tasklist or my_tasks")]
    List(TaskSectionListArgs),
    #[command(about = "Get one custom section")]
    Get(TaskSectionGetArgs),
    #[command(about = "Create a custom section")]
    Create(TaskSectionCreateArgs),
    #[command(about = "Patch section name or order")]
    Update(TaskSectionUpdateArgs),
    #[command(about = "Delete a custom section")]
    Delete(TaskSectionDeleteArgs),
    #[command(about = "List tasks in a custom section")]
    Tasks(TaskSectionTasksArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionListArgs {
    #[arg(long, default_value = "tasklist", help = "tasklist or my_tasks")]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Tasklist GUID when --resource-type tasklist")]
    pub(in crate::app) resource_id: Option<String>,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long,
        help = "Filter sections updated after this millisecond timestamp"
    )]
    pub(in crate::app) update_msec: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionGetArgs {
    #[arg(long, help = "Section GUID")]
    pub(in crate::app) section_guid: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionCreateArgs {
    #[arg(long, help = "Section name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, default_value = "tasklist", help = "tasklist or my_tasks")]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Tasklist GUID when --resource-type tasklist")]
    pub(in crate::app) resource_id: Option<String>,

    #[arg(long, help = "Insert before this section GUID")]
    pub(in crate::app) insert_before: Option<String>,

    #[arg(long, help = "Insert after this section GUID")]
    pub(in crate::app) insert_after: Option<String>,

    #[arg(long, help = "Raw Feishu section create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read section create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read section create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionUpdateArgs {
    #[arg(long, help = "Section GUID")]
    pub(in crate::app) section_guid: String,

    #[arg(long, help = "New section name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Move before this section GUID")]
    pub(in crate::app) insert_before: Option<String>,

    #[arg(long, help = "Move after this section GUID")]
    pub(in crate::app) insert_after: Option<String>,

    #[arg(long = "update-field", help = "Explicit update field. Can repeat.")]
    pub(in crate::app) update_fields: Vec<String>,

    #[arg(long, help = "Raw Feishu section patch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read section patch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read section patch body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionDeleteArgs {
    #[arg(long, help = "Section GUID")]
    pub(in crate::app) section_guid: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskSectionTasksArgs {
    #[arg(long, help = "Section GUID")]
    pub(in crate::app) section_guid: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Filter by completed=true/false")]
    pub(in crate::app) completed: Option<bool>,

    #[arg(long, help = "Created from timestamp in milliseconds")]
    pub(in crate::app) created_from: Option<String>,

    #[arg(long, help = "Created to timestamp in milliseconds")]
    pub(in crate::app) created_to: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskCustomFieldCommand {
    #[command(about = "List custom fields visible to the caller")]
    List(TaskCustomFieldListArgs),
    #[command(about = "Get one custom field")]
    Get(TaskCustomFieldGetArgs),
    #[command(about = "Create a custom field and attach it to a tasklist")]
    Create(TaskCustomFieldCreateArgs),
    #[command(about = "Patch custom field metadata")]
    Update(TaskCustomFieldUpdateArgs),
    #[command(about = "Attach an existing custom field to a tasklist")]
    Add(TaskCustomFieldResourceArgs),
    #[command(about = "Detach a custom field from a tasklist")]
    Remove(TaskCustomFieldResourceArgs),
    #[command(about = "Set a custom field value on a task")]
    SetValue(TaskCustomFieldSetValueArgs),
    #[command(subcommand, about = "Operate single/multi-select field options")]
    Option(TaskCustomFieldOptionCommand),
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub(in crate::app) enum TaskCustomFieldValueTypeArg {
    Text,
    Number,
    Datetime,
    Member,
    SingleSelect,
    MultiSelect,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldListArgs {
    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Resource type filter, currently tasklist")]
    pub(in crate::app) resource_type: Option<String>,

    #[arg(long, help = "Resource ID filter, usually tasklist GUID")]
    pub(in crate::app) resource_id: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldGetArgs {
    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldCreateArgs {
    #[arg(long, help = "Custom field name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long = "type",
        help = "number, datetime, member, single_select, multi_select, or text"
    )]
    pub(in crate::app) field_type: Option<String>,

    #[arg(
        long,
        default_value = "tasklist",
        help = "Resource type, currently tasklist"
    )]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Tasklist GUID to attach this field to")]
    pub(in crate::app) resource_id: Option<String>,

    #[arg(
        long,
        help = "Setting key, e.g. number_setting or single_select_setting"
    )]
    pub(in crate::app) setting_key: Option<String>,

    #[arg(long, help = "Setting object JSON for this field type")]
    pub(in crate::app) setting_json: Option<String>,

    #[arg(long = "option", help = "Select option name. Can repeat.")]
    pub(in crate::app) options: Vec<String>,

    #[arg(long, help = "Raw JSON array for select options")]
    pub(in crate::app) options_json: Option<String>,

    #[arg(long, help = "Raw Feishu custom field create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read custom field create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read custom field create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldUpdateArgs {
    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(long, help = "New custom field name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long,
        help = "Setting key, e.g. number_setting or single_select_setting"
    )]
    pub(in crate::app) setting_key: Option<String>,

    #[arg(long, help = "Setting object JSON")]
    pub(in crate::app) setting_json: Option<String>,

    #[arg(long = "update-field", help = "Explicit update field. Can repeat.")]
    pub(in crate::app) update_fields: Vec<String>,

    #[arg(long, help = "Raw Feishu custom field patch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read custom field patch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read custom field patch body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldResourceArgs {
    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(
        long,
        default_value = "tasklist",
        help = "Resource type, currently tasklist"
    )]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Resource ID, usually tasklist GUID")]
    pub(in crate::app) resource_id: Option<String>,

    #[arg(long, help = "Raw Feishu custom field resource body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read custom field resource body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read custom field resource body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldSetValueArgs {
    #[arg(long, help = "Task GUID to patch")]
    pub(in crate::app) task_guid: String,

    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(long = "type", value_enum, help = "Custom field value type")]
    pub(in crate::app) value_type: TaskCustomFieldValueTypeArg,

    #[arg(
        long,
        help = "Value for text, number, datetime-ms, or single_select option GUID"
    )]
    pub(in crate::app) value: Option<String>,

    #[arg(
        long = "member",
        help = "Member ID for member custom field. Can repeat."
    )]
    pub(in crate::app) members: Vec<String>,

    #[arg(
        long = "option-guid",
        help = "Option GUID for single_select or multi_select. Can repeat for multi_select."
    )]
    pub(in crate::app) option_guids: Vec<String>,

    #[arg(long, help = "Clear this custom field value")]
    pub(in crate::app) clear: bool,

    #[arg(long, default_value = "user", help = "Member type for --member values")]
    pub(in crate::app) member_type: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskCustomFieldOptionCommand {
    #[command(about = "Create an option on a single_select or multi_select field")]
    Create(TaskCustomFieldOptionCreateArgs),
    #[command(about = "Patch one custom field option")]
    Update(TaskCustomFieldOptionUpdateArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldOptionCreateArgs {
    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(long, help = "Option name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Color index 0..54")]
    pub(in crate::app) color_index: Option<i64>,

    #[arg(long, help = "Insert before this option GUID")]
    pub(in crate::app) insert_before: Option<String>,

    #[arg(long, help = "Insert after this option GUID")]
    pub(in crate::app) insert_after: Option<String>,

    #[arg(long, help = "Raw Feishu option create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read option create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read option create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCustomFieldOptionUpdateArgs {
    #[arg(long, help = "Custom field GUID")]
    pub(in crate::app) custom_field_guid: String,

    #[arg(long, help = "Option GUID")]
    pub(in crate::app) option_guid: String,

    #[arg(long, help = "New option name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Color index 0..54")]
    pub(in crate::app) color_index: Option<i64>,

    #[arg(long, help = "Hide or unhide this option")]
    pub(in crate::app) is_hidden: Option<bool>,

    #[arg(long, help = "Move before this option GUID")]
    pub(in crate::app) insert_before: Option<String>,

    #[arg(long, help = "Move after this option GUID")]
    pub(in crate::app) insert_after: Option<String>,

    #[arg(long = "update-field", help = "Explicit update field. Can repeat.")]
    pub(in crate::app) update_fields: Vec<String>,

    #[arg(long, help = "Raw Feishu option patch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read option patch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read option patch body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskAttachmentCommand {
    #[command(about = "List attachments on a task")]
    List(TaskAttachmentListArgs),
    #[command(about = "Upload one to five local files as task attachments")]
    Upload(TaskAttachmentUploadArgs),
    #[command(about = "Delete one task attachment")]
    Delete(TaskAttachmentDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskAttachmentListArgs {
    #[arg(
        long,
        default_value = "task",
        help = "Attachment resource type, currently task"
    )]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Resource ID, usually task GUID")]
    pub(in crate::app) resource_id: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskAttachmentUploadArgs {
    #[arg(
        long,
        default_value = "task",
        help = "Attachment resource type, currently task"
    )]
    pub(in crate::app) resource_type: String,

    #[arg(long, help = "Resource ID, usually task GUID")]
    pub(in crate::app) resource_id: String,

    #[arg(long = "file", help = "Local file to upload. Can repeat, max 5.")]
    pub(in crate::app) files: Vec<PathBuf>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskAttachmentDeleteArgs {
    #[arg(long, help = "Attachment GUID")]
    pub(in crate::app) attachment_guid: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TasklistMemberWriteArgs {
    #[arg(long, help = "Tasklist GUID")]
    pub(in crate::app) tasklist_guid: String,

    #[arg(long = "editor", help = "Editor user/chat ID. Can repeat.")]
    pub(in crate::app) editors: Vec<String>,

    #[arg(long = "viewer", help = "Viewer user/chat ID. Can repeat.")]
    pub(in crate::app) viewers: Vec<String>,

    #[arg(long, help = "Member type for --editor/--viewer values: user or chat")]
    pub(in crate::app) member_type: Option<String>,

    #[arg(long, help = "Raw JSON array for members")]
    pub(in crate::app) members_json: Option<String>,

    #[arg(long, help = "Raw Feishu tasklist member body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read tasklist member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read tasklist member body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveCommand {
    #[command(about = "List files in a Drive folder")]
    List(DriveListArgs),
    #[command(subcommand, about = "Operate Drive folders")]
    Folder(DriveFolderCommand),
    #[command(about = "Upload a local file to Drive, up to 20 MB")]
    Upload(DriveUploadArgs),
    #[command(about = "Multipart upload a local file to Drive for files over 20 MB")]
    UploadLarge(DriveUploadLargeArgs),
    #[command(
        subcommand,
        about = "Upload/download Drive media assets for docs, sheets, Base, and imports"
    )]
    Media(DriveMediaCommand),
    #[command(
        subcommand,
        about = "Create and poll Drive import tasks, including HTML -> online docx"
    )]
    Import(DriveImportCommand),
    #[command(subcommand, about = "Export Feishu docs/sheets/Base to local files")]
    Export(DriveExportCommand),
    #[command(subcommand, about = "Operate cloud document comments and replies")]
    Comment(DriveCommentCommand),
    #[command(subcommand, about = "Operate cloud document versions")]
    Version(DriveVersionCommand),
    #[command(subcommand, about = "Operate cloud document subscriptions")]
    Subscription(DriveSubscriptionCommand),
    #[command(about = "List Drive file view records")]
    ViewRecord(DriveViewRecordArgs),
    #[command(about = "Download a Drive resource file to local disk")]
    Download(DriveDownloadArgs),
    #[command(subcommand, about = "Operate cloud document permissions")]
    Permission(DrivePermissionCommand),
    #[command(about = "Get file statistics")]
    Stats(DriveFileRefArgs),
    #[command(about = "Copy a file")]
    Copy(DriveCopyArgs),
    #[command(about = "Move a file")]
    Move(DriveMoveArgs),
    #[command(about = "Delete a file")]
    Delete(DriveFileRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveListArgs {
    #[arg(long, help = "Folder token. Empty/missing means root")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, default_value = "EditedTime", help = "EditedTime or CreatedTime")]
    pub(in crate::app) order_by: String,

    #[arg(long, default_value = "DESC", help = "ASC or DESC")]
    pub(in crate::app) direction: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum DriveFolderCommand {
    #[command(about = "Create a Drive folder")]
    Create(DriveFolderCreateArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveFolderCreateArgs {
    #[arg(long, help = "Folder name")]
    pub(in crate::app) name: String,

    #[arg(
        long,
        default_value = "",
        help = "Parent folder token. Empty means root"
    )]
    pub(in crate::app) folder_token: String,
}

#[derive(Args)]
pub(in crate::app) struct DriveUploadArgs {
    #[arg(long, help = "Local file path to upload")]
    pub(in crate::app) file: PathBuf,

    #[arg(long, help = "Target Drive folder token")]
    pub(in crate::app) folder_token: String,

    #[arg(long, help = "Override uploaded file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, default_value = "explorer", help = "Upload parent type")]
    pub(in crate::app) parent_type: String,

    #[arg(long, help = "Optional Adler-32 checksum")]
    pub(in crate::app) checksum: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct DriveUploadLargeArgs {
    #[arg(
        long,
        help = "Local file path to upload through upload_prepare/part/finish"
    )]
    pub(in crate::app) file: PathBuf,

    #[arg(
        long,
        default_value = "",
        help = "Target Drive folder token. Empty means root"
    )]
    pub(in crate::app) folder_token: String,

    #[arg(long, help = "Override uploaded file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, default_value = "explorer", help = "Upload parent type")]
    pub(in crate::app) parent_type: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
#[command(after_long_help = DRIVE_AFTER_HELP)]
pub(in crate::app) enum DriveMediaCommand {
    #[command(about = "Upload a cloud-document media asset, up to 20 MB")]
    Upload(DriveMediaUploadArgs),
    #[command(about = "Download a cloud-document media asset")]
    Download(DriveMediaDownloadArgs),
    #[command(about = "Get temporary download URLs for up to 5 media assets")]
    TmpUrl(DriveMediaTmpUrlArgs),
}

#[derive(Args)]
pub(in crate::app) struct DriveMediaUploadArgs {
    #[arg(long, help = "Local file path to upload as a media asset")]
    pub(in crate::app) file: PathBuf,

    #[arg(long, help = "Override uploaded file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(
        long,
        default_value = "docx_file",
        help = "Upload point: docx_image, docx_file, sheet_image, sheet_file, bitable_image, bitable_file, ccm_import_open, etc."
    )]
    pub(in crate::app) parent_type: String,

    #[arg(
        long,
        help = "Upload point token. For docx_image/docx_file this is the target image/file block_id; ccm_import_open can be empty."
    )]
    pub(in crate::app) parent_node: Option<String>,

    #[arg(
        long,
        help = "Build extra as {\"drive_route_token\":\"...\"}; needed for docx/sheet/bitable assets"
    )]
    pub(in crate::app) drive_route_token: Option<String>,

    #[arg(long, help = "Raw extra string for advanced media upload/import auth")]
    pub(in crate::app) extra: Option<String>,

    #[arg(long, help = "Optional Adler-32 checksum")]
    pub(in crate::app) checksum: Option<String>,
}

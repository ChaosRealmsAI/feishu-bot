use super::*;
#[derive(Args)]
pub(in crate::app) struct BaseRecordBatchCreateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Raw JSON array for records")]
    pub(in crate::app) records_json: Option<String>,

    #[arg(long, help = "Read records JSON array from file")]
    pub(in crate::app) records_file: Option<PathBuf>,

    #[arg(long, help = "Read records JSON array from stdin")]
    pub(in crate::app) records_stdin: bool,

    #[arg(
        long = "record-field",
        help = "Batch record field as index:name=value. Index starts at 0. Values auto-parse JSON. Can repeat."
    )]
    pub(in crate::app) record_fields: Vec<String>,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Set ignore_consistency_check=true")]
    pub(in crate::app) ignore_consistency_check: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordBatchUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Raw JSON array for records")]
    pub(in crate::app) records_json: Option<String>,

    #[arg(long, help = "Read records JSON array from file")]
    pub(in crate::app) records_file: Option<PathBuf>,

    #[arg(long, help = "Read records JSON array from stdin")]
    pub(in crate::app) records_stdin: bool,

    #[arg(
        long = "record-id",
        help = "Record ID for the matching index. Repeat in the same order as indexed --record-field groups."
    )]
    pub(in crate::app) record_ids: Vec<String>,

    #[arg(
        long = "record-field",
        help = "Batch record field as index:name=value. Index starts at 0. Values auto-parse JSON. Can repeat."
    )]
    pub(in crate::app) record_fields: Vec<String>,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Set ignore_consistency_check=true")]
    pub(in crate::app) ignore_consistency_check: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Record ID")]
    pub(in crate::app) record_id: String,

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

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, help = "Set ignore_consistency_check=true")]
    pub(in crate::app) ignore_consistency_check: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordDeleteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Record ID")]
    pub(in crate::app) record_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseRecordBatchDeleteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long = "record-id", help = "Record ID to delete. Can be repeated.")]
    pub(in crate::app) record_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array/object for records")]
    pub(in crate::app) records_json: Option<String>,

    #[arg(long, help = "Read records JSON array/object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read records JSON array/object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = TASK_AFTER_HELP)]
pub(in crate::app) enum TaskCommand {
    #[command(subcommand, about = "Operate tasklists")]
    Tasklist(TasklistCommand),
    #[command(about = "Create a task")]
    Create(TaskCreateArgs),
    #[command(about = "List tasks visible to the caller")]
    List(TaskListArgs),
    #[command(about = "Get task details")]
    Get(TaskGetArgs),
    #[command(about = "Update task fields")]
    Update(TaskUpdateArgs),
    #[command(about = "Mark a task completed")]
    Complete(TaskCompleteArgs),
    #[command(about = "Reopen a completed task")]
    Reopen(TaskGetArgs),
    #[command(about = "Delete a task")]
    Delete(TaskGetArgs),
    #[command(subcommand, about = "Operate task members")]
    Member(TaskMemberCommand),
    #[command(about = "List tasklists containing a task")]
    Tasklists(TaskTasklistsArgs),
    #[command(about = "Add a task to a tasklist")]
    AddTasklist(TaskTasklistWriteArgs),
    #[command(about = "Remove a task from a tasklist")]
    RemoveTasklist(TaskTasklistWriteArgs),
    #[command(subcommand, about = "Operate task reminders")]
    Reminder(TaskReminderCommand),
    #[command(subcommand, about = "Operate task dependencies")]
    Dependency(TaskDependencyCommand),
    #[command(subcommand, about = "Operate task comments")]
    Comment(TaskCommentCommand),
    #[command(subcommand, about = "Operate subtasks")]
    Subtask(TaskSubtaskCommand),
    #[command(subcommand, about = "Operate task custom sections")]
    Section(TaskSectionCommand),
    #[command(subcommand, about = "Operate task custom fields")]
    CustomField(TaskCustomFieldCommand),
    #[command(subcommand, about = "Operate task attachments")]
    Attachment(TaskAttachmentCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum TasklistCommand {
    #[command(about = "Create a tasklist")]
    Create(TasklistCreateArgs),
    #[command(about = "List tasklists visible to the caller")]
    List(TasklistListArgs),
    #[command(about = "Get one tasklist")]
    Get(TasklistGetArgs),
    #[command(about = "Patch a tasklist name or owner")]
    Update(TasklistUpdateArgs),
    #[command(about = "Delete one tasklist")]
    Delete(TasklistGetArgs),
    #[command(about = "List tasks in a tasklist")]
    Tasks(TasklistTasksArgs),
    #[command(about = "Add tasklist collaborators")]
    AddMember(TasklistMemberWriteArgs),
    #[command(about = "Remove tasklist collaborators")]
    RemoveMember(TasklistMemberWriteArgs),
}

#[derive(Args)]
pub(in crate::app) struct TasklistCreateArgs {
    #[arg(long, help = "Tasklist name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long = "member", help = "Tasklist member ID. Can be repeated.")]
    pub(in crate::app) members: Vec<String>,

    #[arg(
        long,
        default_value = "editor",
        help = "Role for --member values: editor/viewer"
    )]
    pub(in crate::app) member_role: String,

    #[arg(long, help = "Raw Feishu tasklist create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read tasklist create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read tasklist create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TasklistListArgs {
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
pub(in crate::app) struct TasklistGetArgs {
    #[arg(long, help = "Tasklist GUID")]
    pub(in crate::app) tasklist_guid: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TasklistUpdateArgs {
    #[arg(long, help = "Tasklist GUID")]
    pub(in crate::app) tasklist_guid: String,

    #[arg(long, help = "New tasklist name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw JSON object for new tasklist owner member")]
    pub(in crate::app) owner_json: Option<String>,

    #[arg(
        long,
        default_value = "none",
        help = "Role for previous owner when owner changes: editor, viewer, none"
    )]
    pub(in crate::app) origin_owner_to_role: String,

    #[arg(long, help = "Raw Feishu tasklist patch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read tasklist patch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read tasklist patch body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TasklistTasksArgs {
    #[arg(long, help = "Tasklist GUID")]
    pub(in crate::app) tasklist_guid: String,

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

#[derive(Args)]
pub(in crate::app) struct TaskCreateArgs {
    #[arg(long, help = "Task title/summary")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Task description")]
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

    #[arg(long, help = "Whether this task is a milestone, pass true or false")]
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

    #[arg(long = "assignee", help = "User ID to add as assignee. Can repeat.")]
    pub(in crate::app) assignees: Vec<String>,

    #[arg(long = "follower", help = "User ID to add as follower. Can repeat.")]
    pub(in crate::app) followers: Vec<String>,

    #[arg(
        long = "tasklist-guid",
        help = "Tasklist GUID to add task into. Can repeat."
    )]
    pub(in crate::app) tasklist_guids: Vec<String>,

    #[arg(long, help = "Raw Feishu task create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read task create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read task create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency token. Defaults to a random UUID.")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskListArgs {
    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Filter by completed=true/false")]
    pub(in crate::app) completed: Option<bool>,

    #[arg(
        long = "type",
        default_value = "my_tasks",
        help = "Task list type; Feishu currently supports my_tasks"
    )]
    pub(in crate::app) list_type: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::User,
        help = "Access token type; Feishu task list is user-token first"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskGetArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) guid: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskUpdateArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) guid: String,

    #[arg(long, help = "New task summary")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "New task description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Clear task description")]
    pub(in crate::app) clear_description: bool,

    #[arg(long, help = "New due timestamp in milliseconds")]
    pub(in crate::app) due_ms: Option<String>,

    #[arg(
        long,
        help = "New due time as RFC3339 or local 'YYYY-MM-DD HH:MM[:SS]'; converted to milliseconds"
    )]
    pub(in crate::app) due_at: Option<String>,

    #[arg(
        long,
        help = "New due all-day date as YYYY-MM-DD; sets due.is_all_day=true"
    )]
    pub(in crate::app) due_date: Option<String>,

    #[arg(long, help = "Treat --due-ms as an all-day date timestamp")]
    pub(in crate::app) due_all_day: bool,

    #[arg(long, help = "Clear task due time")]
    pub(in crate::app) clear_due: bool,

    #[arg(long, help = "New start timestamp in milliseconds")]
    pub(in crate::app) start_ms: Option<String>,

    #[arg(
        long,
        help = "New start time as RFC3339 or local 'YYYY-MM-DD HH:MM[:SS]'; converted to milliseconds"
    )]
    pub(in crate::app) start_at: Option<String>,

    #[arg(
        long,
        help = "New start all-day date as YYYY-MM-DD; sets start.is_all_day=true"
    )]
    pub(in crate::app) start_date: Option<String>,

    #[arg(long, help = "Treat --start-ms as an all-day date timestamp")]
    pub(in crate::app) start_all_day: bool,

    #[arg(long, help = "Clear task start time")]
    pub(in crate::app) clear_start: bool,

    #[arg(
        long,
        help = "New completed_at timestamp in milliseconds; use 0 to reopen"
    )]
    pub(in crate::app) completed_at: Option<String>,

    #[arg(long, help = "New RFC5545 RRULE string, e.g. FREQ=WEEKLY;INTERVAL=1")]
    pub(in crate::app) repeat_rule: Option<String>,

    #[arg(long, help = "Clear repeat_rule")]
    pub(in crate::app) clear_repeat_rule: bool,

    #[arg(long, help = "Raw JSON object for custom_complete")]
    pub(in crate::app) custom_complete_json: Option<String>,

    #[arg(long, help = "Clear custom_complete")]
    pub(in crate::app) clear_custom_complete: bool,

    #[arg(long, help = "Caller-defined extra string, often Base64")]
    pub(in crate::app) extra: Option<String>,

    #[arg(long, help = "Clear extra")]
    pub(in crate::app) clear_extra: bool,

    #[arg(long, help = "Task completion mode: 1=all assignees, 2=any assignee")]
    pub(in crate::app) mode: Option<u8>,

    #[arg(long, help = "Whether this task is a milestone, pass true or false")]
    pub(in crate::app) is_milestone: Option<bool>,

    #[arg(long, help = "Raw JSON array for custom_fields values")]
    pub(in crate::app) custom_fields_json: Option<String>,

    #[arg(long, help = "Raw Feishu task update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read task update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read task update body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCompleteArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) guid: String,

    #[arg(long, help = "completed_at timestamp in milliseconds; defaults to now")]
    pub(in crate::app) completed_at: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskMemberCommand {
    #[command(about = "Add assignees/followers to a task")]
    Add(TaskMemberWriteArgs),
    #[command(about = "Remove assignees/followers from a task")]
    Remove(TaskMemberWriteArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskMemberWriteArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long = "assignee", help = "Assignee user/app ID. Can repeat.")]
    pub(in crate::app) assignees: Vec<String>,

    #[arg(long = "follower", help = "Follower user/app ID. Can repeat.")]
    pub(in crate::app) followers: Vec<String>,

    #[arg(
        long,
        help = "Member type for --assignee/--follower values: user or app"
    )]
    pub(in crate::app) member_type: Option<String>,

    #[arg(long, help = "Raw JSON array for members")]
    pub(in crate::app) members_json: Option<String>,

    #[arg(long, help = "Raw Feishu member body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read member body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read member body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency token for add-members")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskTasklistsArgs {
    #[arg(long, help = "Task GUID")]
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

#[derive(Args)]
pub(in crate::app) struct TaskTasklistWriteArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long, help = "Tasklist GUID")]
    pub(in crate::app) tasklist_guid: Option<String>,

    #[arg(
        long,
        help = "Optional section GUID inside the tasklist; only used by add-tasklist"
    )]
    pub(in crate::app) section_guid: Option<String>,

    #[arg(long, help = "Raw Feishu tasklist relation body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read tasklist relation body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read tasklist relation body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskReminderCommand {
    #[command(about = "Add a task reminder")]
    Add(TaskReminderAddArgs),
    #[command(about = "Remove task reminders")]
    Remove(TaskReminderRemoveArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskReminderAddArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(
        long,
        visible_alias = "reminder-minute",
        help = "Reminder minutes before due time, e.g. 30; 0 means at due time"
    )]
    pub(in crate::app) relative_fire_minute: Option<i64>,

    #[arg(long, help = "Raw JSON array for reminders")]
    pub(in crate::app) reminders_json: Option<String>,

    #[arg(long, help = "Raw Feishu reminder add body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read reminder add body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reminder add body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskReminderRemoveArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long = "reminder-id", help = "Reminder ID. Can repeat.")]
    pub(in crate::app) reminder_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array for reminder_ids")]
    pub(in crate::app) reminder_ids_json: Option<String>,

    #[arg(long, help = "Raw Feishu reminder remove body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read reminder remove body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reminder remove body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskDependencyCommand {
    #[command(about = "Add dependencies to a task")]
    Add(TaskDependencyAddArgs),
    #[command(about = "Remove dependencies from a task")]
    Remove(TaskDependencyRemoveArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskDependencyAddArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(
        long = "dependency-task-guid",
        help = "Dependent task GUID. Can repeat."
    )]
    pub(in crate::app) dependency_task_guids: Vec<String>,

    #[arg(
        long = "type",
        default_value = "next",
        help = "Dependency type, usually next"
    )]
    pub(in crate::app) dependency_type: String,

    #[arg(long, help = "Raw JSON array for dependencies")]
    pub(in crate::app) dependencies_json: Option<String>,

    #[arg(long, help = "Raw Feishu dependency add body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read dependency add body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read dependency add body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskDependencyRemoveArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(
        long = "dependency-task-guid",
        help = "Dependent task GUID. Can repeat."
    )]
    pub(in crate::app) dependency_task_guids: Vec<String>,

    #[arg(long, help = "Raw JSON array for dependencies")]
    pub(in crate::app) dependencies_json: Option<String>,

    #[arg(long, help = "Raw Feishu dependency remove body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read dependency remove body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read dependency remove body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum TaskCommentCommand {
    #[command(about = "List comments on a task")]
    List(TaskCommentListArgs),
    #[command(about = "Get one task comment")]
    Get(TaskCommentGetArgs),
    #[command(about = "Create a comment on a task")]
    Create(TaskCommentCreateArgs),
    #[command(about = "Update one task comment")]
    Update(TaskCommentUpdateArgs),
    #[command(about = "Delete one task comment")]
    Delete(TaskCommentDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct TaskCommentListArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, default_value = "asc", help = "asc or desc")]
    pub(in crate::app) direction: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCommentCreateArgs {
    #[arg(long, help = "Task GUID")]
    pub(in crate::app) task_guid: String,

    #[arg(long, help = "Comment content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Reply to this comment ID")]
    pub(in crate::app) reply_to_comment_id: Option<String>,

    #[arg(long, help = "Raw Feishu comment create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read comment create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read comment create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct TaskCommentGetArgs {
    #[arg(long, help = "Comment ID")]
    pub(in crate::app) comment_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

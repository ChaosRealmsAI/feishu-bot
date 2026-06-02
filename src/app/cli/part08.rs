use super::*;

#[derive(Subcommand)]
#[command(after_long_help = CALENDAR_AFTER_HELP)]
pub(in crate::app) enum CalendarCommand {
    #[command(about = "Get primary calendar")]
    Primary,
    #[command(about = "List calendars")]
    List(CalendarListArgs),
    #[command(about = "Create shared calendar")]
    Create(CalendarCreateArgs),
    #[command(subcommand, about = "Operate calendar events")]
    Event(CalendarEventCommand),
    #[command(subcommand, about = "Operate calendar event attendees")]
    Attendee(CalendarAttendeeCommand),
    #[command(subcommand, about = "Query user or room free/busy")]
    Freebusy(CalendarFreebusyCommand),
}

#[derive(Args)]
pub(in crate::app) struct CalendarListArgs {
    #[arg(
        long,
        default_value_t = 100,
        value_parser = clap::value_parser!(u16).range(50..=100),
        help = "Page size; Feishu calendar list accepts 50..=100"
    )]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct CalendarCreateArgs {
    #[arg(long, help = "Calendar summary/name")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Calendar description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Raw Feishu calendar create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read calendar create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read calendar create body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum CalendarEventCommand {
    #[command(about = "List events")]
    List(CalendarEventListArgs),
    #[command(about = "Get one event")]
    Get(CalendarEventGetArgs),
    #[command(about = "Create event")]
    Create(CalendarEventCreateArgs),
    #[command(about = "Patch event")]
    Update(CalendarEventUpdateArgs),
    #[command(about = "Delete event")]
    Delete(CalendarEventGetArgs),
}

#[derive(Args)]
pub(in crate::app) struct CalendarEventListArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, default_value_t = 50, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Start time lower bound as Unix seconds")]
    pub(in crate::app) start_ts: Option<String>,

    #[arg(long, help = "End time upper bound as Unix seconds")]
    pub(in crate::app) end_ts: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarEventGetArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarEventCreateArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event summary/title")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Event description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Start time as Unix seconds")]
    pub(in crate::app) start_ts: Option<String>,

    #[arg(long, help = "End time as Unix seconds")]
    pub(in crate::app) end_ts: Option<String>,

    #[arg(long, default_value = "Asia/Shanghai", help = "Event timezone")]
    pub(in crate::app) time_zone: String,

    #[arg(long, help = "Idempotency UUID")]
    pub(in crate::app) idempotency_key: Option<String>,

    #[arg(long, help = "Raw Feishu event create body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read event create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read event create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarEventUpdateArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(long, help = "Event summary/title")]
    pub(in crate::app) summary: Option<String>,

    #[arg(long, help = "Event description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Raw Feishu event update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read event update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read event update body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum CalendarAttendeeCommand {
    #[command(about = "List event attendees")]
    List(CalendarAttendeeListArgs),
    #[command(about = "Add event attendees")]
    Add(CalendarAttendeeAddArgs),
    #[command(about = "Delete event attendees")]
    Delete(CalendarAttendeeDeleteArgs),
    #[command(about = "List members of a chat attendee")]
    ChatMembers(CalendarAttendeeChatMembersArgs),
}

#[derive(Args)]
pub(in crate::app) struct CalendarAttendeeListArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarAttendeeAddArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(long = "user", help = "User ID to add as attendee. Can repeat.")]
    pub(in crate::app) users: Vec<String>,

    #[arg(long = "chat", help = "Chat ID to add as attendee. Can repeat.")]
    pub(in crate::app) chats: Vec<String>,

    #[arg(long, help = "Mark added attendees optional")]
    pub(in crate::app) optional: bool,

    #[arg(long, help = "Raw JSON array for attendees")]
    pub(in crate::app) attendees_json: Option<String>,

    #[arg(long, help = "Raw Feishu attendee add body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read attendee add body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read attendee add body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarAttendeeDeleteArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(
        long = "attendee-id",
        help = "Attendee ID returned by attendee list/add. Can repeat."
    )]
    pub(in crate::app) attendee_ids: Vec<String>,

    #[arg(
        long = "delete-id",
        help = "User/chat/resource ID fallback. Can repeat."
    )]
    pub(in crate::app) delete_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array for attendee_ids")]
    pub(in crate::app) attendee_ids_json: Option<String>,

    #[arg(long, help = "Raw JSON array for delete_ids")]
    pub(in crate::app) delete_ids_json: Option<String>,

    #[arg(long, help = "Raw Feishu attendee delete body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read attendee delete body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read attendee delete body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarAttendeeChatMembersArgs {
    #[arg(long, help = "Calendar ID")]
    pub(in crate::app) calendar_id: String,

    #[arg(long, help = "Event ID")]
    pub(in crate::app) event_id: String,

    #[arg(long, help = "Chat attendee ID")]
    pub(in crate::app) attendee_id: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum CalendarFreebusyCommand {
    #[command(about = "Query one user's primary calendar or one room's free/busy")]
    List(CalendarFreebusyListArgs),
    #[command(about = "Batch query users' primary-calendar free/busy")]
    Batch(CalendarFreebusyBatchArgs),
}

#[derive(Args)]
pub(in crate::app) struct CalendarFreebusyListArgs {
    #[arg(
        long,
        help = "Start RFC3339 datetime, for example 2026-06-01T09:00:00+08:00"
    )]
    pub(in crate::app) time_min: String,

    #[arg(
        long,
        help = "End RFC3339 datetime, for example 2026-06-01T18:00:00+08:00"
    )]
    pub(in crate::app) time_max: String,

    #[arg(
        long,
        help = "User ID matching --user-id-type. Mutually exclusive with --room-id"
    )]
    pub(in crate::app) user_id: Option<String>,

    #[arg(long, help = "Meeting room ID. Mutually exclusive with --user-id")]
    pub(in crate::app) room_id: Option<String>,

    #[arg(long, help = "Include third-party/external calendars")]
    pub(in crate::app) include_external_calendar: Option<bool>,

    #[arg(long, help = "Only return busy items")]
    pub(in crate::app) only_busy: Option<bool>,

    #[arg(long, help = "Raw Feishu freebusy/list body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read freebusy/list body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read freebusy/list body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct CalendarFreebusyBatchArgs {
    #[arg(
        long,
        help = "Start RFC3339 datetime, for example 2026-06-01T09:00:00+08:00"
    )]
    pub(in crate::app) time_min: String,

    #[arg(
        long,
        help = "End RFC3339 datetime, for example 2026-06-01T18:00:00+08:00"
    )]
    pub(in crate::app) time_max: String,

    #[arg(
        long = "user-id",
        help = "User ID matching --user-id-type. Can repeat."
    )]
    pub(in crate::app) user_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array or object with user_ids")]
    pub(in crate::app) user_ids_json: Option<String>,

    #[arg(long, help = "Raw Feishu freebusy/batch body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read freebusy/batch body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read freebusy/batch body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

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

#[derive(Subcommand)]
#[command(after_long_help = VC_AFTER_HELP)]
pub(in crate::app) enum VcCommand {
    #[command(subcommand, about = "Create/read/update/delete video meeting reserves")]
    Reserve(VcReserveCommand),
    #[command(subcommand, about = "Read video meetings")]
    Meeting(VcMeetingCommand),
    #[command(subcommand, about = "Read meeting recordings")]
    Recording(VcRecordingCommand),
    #[command(subcommand, about = "Read meeting reports")]
    Report(VcReportCommand),
    #[command(subcommand, about = "Read meeting rooms")]
    Room(VcRoomCommand),
    #[command(subcommand, about = "Read meeting room levels")]
    RoomLevel(VcRoomLevelCommand),
}

#[derive(Subcommand)]
pub(in crate::app) enum VcReserveCommand {
    #[command(about = "Reserve a video meeting")]
    Apply(VcReserveApplyArgs),
    #[command(about = "Get one meeting reserve")]
    Get(VcReserveGetArgs),
    #[command(about = "Update one meeting reserve")]
    Update(VcReserveUpdateArgs),
    #[command(about = "Delete one meeting reserve")]
    Delete(VcReserveDeleteArgs),
    #[command(about = "Get the active meeting for one reserve")]
    ActiveMeeting(VcReserveActiveMeetingArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum VcMeetingCommand {
    #[command(about = "Get meeting details")]
    Get(VcMeetingGetArgs),
    #[command(about = "List meetings related to a meeting number")]
    ListByNo(VcMeetingListByNoArgs),
    #[command(about = "Invite participants into an active meeting")]
    Invite(VcMeetingInviteArgs),
    #[command(about = "Set the active meeting host")]
    SetHost(VcMeetingSetHostArgs),
    #[command(about = "End an active meeting")]
    End(VcMeetingEndArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcReserveApplyArgs {
    #[arg(long, help = "Reserve expiration Unix timestamp in seconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, help = "Meeting owner ID; required when using tenant token")]
    pub(in crate::app) owner_id: Option<String>,

    #[arg(long, help = "Meeting topic")]
    pub(in crate::app) topic: Option<String>,

    #[arg(long, help = "Automatically record the meeting")]
    pub(in crate::app) auto_record: Option<bool>,

    #[arg(
        long = "assign-host",
        help = "Host user ID to assign; can repeat, max 10"
    )]
    pub(in crate::app) assign_hosts: Vec<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu reserve body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read reserve body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reserve body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct VcReserveGetArgs {
    #[arg(long, help = "Reserve ID")]
    pub(in crate::app) reserve_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct VcReserveUpdateArgs {
    #[arg(long, help = "Reserve ID")]
    pub(in crate::app) reserve_id: String,

    #[arg(long, help = "Reserve expiration Unix timestamp in seconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, help = "Meeting topic")]
    pub(in crate::app) topic: Option<String>,

    #[arg(long, help = "Automatically record the meeting")]
    pub(in crate::app) auto_record: Option<bool>,

    #[arg(
        long = "assign-host",
        help = "Host user ID to assign; can repeat, max 10"
    )]
    pub(in crate::app) assign_hosts: Vec<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw Feishu reserve update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read reserve update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read reserve update body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct VcReserveDeleteArgs {
    #[arg(long, help = "Reserve ID")]
    pub(in crate::app) reserve_id: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct VcReserveActiveMeetingArgs {
    #[arg(long, help = "Reserve ID")]
    pub(in crate::app) reserve_id: String,

    #[arg(long, help = "Return participant list")]
    pub(in crate::app) with_participants: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct VcMeetingGetArgs {
    #[arg(long, help = "Meeting ID generated after a meeting starts")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct VcMeetingListByNoArgs {
    #[arg(long, help = "9-digit meeting number")]
    pub(in crate::app) meeting_no: String,

    #[arg(long, help = "Start Unix timestamp in seconds")]
    pub(in crate::app) start_time: String,

    #[arg(long, help = "End Unix timestamp in seconds")]
    pub(in crate::app) end_time: String,

    #[arg(long, default_value_t = 20, help = "Page size, max 50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct VcMeetingInviteArgs {
    #[arg(long, help = "Active meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(long = "user", help = "User ID to invite; can repeat, max 10")]
    pub(in crate::app) users: Vec<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw invite body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read invite body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read invite body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct VcMeetingSetHostArgs {
    #[arg(long, help = "Active meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, help = "Host user ID")]
    pub(in crate::app) user_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw set-host body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read set-host body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read set-host body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct VcMeetingEndArgs {
    #[arg(long, help = "Active meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum VcRecordingCommand {
    #[command(about = "Get meeting recording metadata")]
    Get(VcRecordingGetArgs),
    #[command(about = "Start recording an active meeting")]
    Start(VcRecordingStartArgs),
    #[command(about = "Stop recording an active meeting")]
    Stop(VcRecordingStopArgs),
    #[command(about = "Set recording file permissions")]
    SetPermission(VcRecordingSetPermissionArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcRecordingGetArgs {
    #[arg(long, help = "Meeting ID generated after a meeting starts")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct VcRecordingStartArgs {
    #[arg(long, help = "Active meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, help = "Recording timezone, -12..12")]
    pub(in crate::app) timezone: Option<i64>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw recording start body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read recording start body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read recording start body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct VcRecordingStopArgs {
    #[arg(long, help = "Active meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct VcRecordingSetPermissionArgs {
    #[arg(long, help = "Meeting ID")]
    pub(in crate::app) meeting_id: String,

    #[arg(
        long = "user",
        help = "Grant recording view permission to a user ID; can repeat"
    )]
    pub(in crate::app) users: Vec<String>,

    #[arg(
        long = "chat",
        help = "Grant recording view permission to a chat ID; can repeat"
    )]
    pub(in crate::app) chats: Vec<String>,

    #[arg(long, help = "Grant recording view permission to the whole tenant")]
    pub(in crate::app) tenant: bool,

    #[arg(long, help = "Grant public recording view permission")]
    pub(in crate::app) public: bool,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User, help = "Access token type")]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, help = "Raw recording permission body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read recording permission body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read recording permission body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum VcReportCommand {
    #[command(about = "Get daily meeting report")]
    Daily(VcReportDailyArgs),
    #[command(about = "Get top meeting users")]
    TopUser(VcReportTopUserArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcReportDailyArgs {
    #[arg(long, help = "Start Unix timestamp in seconds")]
    pub(in crate::app) start_time: String,

    #[arg(long, help = "End Unix timestamp in seconds")]
    pub(in crate::app) end_time: String,
}

#[derive(Args)]
pub(in crate::app) struct VcReportTopUserArgs {
    #[arg(long, help = "Start Unix timestamp in seconds")]
    pub(in crate::app) start_time: String,

    #[arg(long, help = "End Unix timestamp in seconds")]
    pub(in crate::app) end_time: String,

    #[arg(long, default_value_t = 10, help = "Top N users, max 100")]
    pub(in crate::app) limit: u16,

    #[arg(long, default_value_t = 1, help = "1 by meeting count, 2 by duration")]
    pub(in crate::app) order_by: u8,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum VcRoomCommand {
    #[command(about = "List meeting rooms")]
    List(VcRoomListArgs),
    #[command(about = "Get one meeting room")]
    Get(VcRoomGetArgs),
    #[command(about = "Batch get meeting rooms")]
    Mget(VcRoomMgetArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcRoomListArgs {
    #[arg(long, help = "Room level ID. Omit for tenant root when API allows it.")]
    pub(in crate::app) room_level_id: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

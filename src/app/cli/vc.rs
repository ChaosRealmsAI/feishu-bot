use super::*;

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

#[derive(Args)]
pub(in crate::app) struct VcRoomGetArgs {
    #[arg(long, help = "Meeting room ID")]
    pub(in crate::app) room_id: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct VcRoomMgetArgs {
    #[arg(long = "room-id", help = "Meeting room ID. Can repeat.")]
    pub(in crate::app) room_ids: Vec<String>,

    #[arg(long, help = "Raw Feishu mget body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read mget body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read mget body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum VcRoomLevelCommand {
    #[command(about = "List child meeting room levels")]
    List(VcRoomLevelListArgs),
}

#[derive(Args)]
pub(in crate::app) struct VcRoomLevelListArgs {
    #[arg(long, help = "Room level ID. Omit for tenant root when API allows it.")]
    pub(in crate::app) room_level_id: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

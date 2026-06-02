use super::*;

#[derive(Subcommand)]
#[command(after_long_help = MINUTES_AFTER_HELP)]
pub(in crate::app) enum MinutesCommand {
    #[command(about = "Search minutes by keyword and native filters")]
    Search(MinutesSearchArgs),
    #[command(about = "Get minutes metadata")]
    Get(MinutesGetArgs),
    #[command(about = "Get minutes AI artifacts such as summary, actions, and chapters")]
    Artifacts(MinutesTokenArgs),
    #[command(about = "Get minutes audio/video download URL")]
    Media(MinutesTokenArgs),
    #[command(about = "Export minutes transcript to a local file")]
    Transcript(MinutesTranscriptArgs),
}

#[derive(Args)]
pub(in crate::app) struct MinutesSearchArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, help = "Native Feishu minutes filter JSON object")]
    pub(in crate::app) filter_json: Option<String>,

    #[arg(long, help = "Sorter such as create_time_desc")]
    pub(in crate::app) sorter: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long, help = "Raw Feishu search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read search body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MinutesGetArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct MinutesTokenArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,
}

#[derive(Args)]
pub(in crate::app) struct MinutesTranscriptArgs {
    #[arg(long, help = "Minute token, or a full Feishu/Lark minutes URL")]
    pub(in crate::app) minute_token: String,

    #[arg(long, help = "Include speaker names")]
    pub(in crate::app) need_speaker: bool,

    #[arg(long, help = "Include timestamps")]
    pub(in crate::app) need_timestamp: bool,

    #[arg(long, help = "Export format, usually txt or srt")]
    pub(in crate::app) file_format: Option<String>,

    #[arg(long, help = "Output file path, or - for stdout")]
    pub(in crate::app) output: PathBuf,
}

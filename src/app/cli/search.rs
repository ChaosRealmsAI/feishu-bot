use super::*;

#[derive(Subcommand)]
#[command(after_long_help = SEARCH_AFTER_HELP)]
pub(in crate::app) enum SearchCommand {
    #[command(about = "Search current user's visible Feishu docs and wiki")]
    Docs(SearchDocsArgs),
    #[command(about = "Search current user's visible messages")]
    Message(SearchMessageArgs),
    #[command(subcommand, about = "Manage custom search data sources")]
    Source(SearchSourceCommand),
    #[command(subcommand, about = "Manage custom search schemas")]
    Schema(SearchSchemaCommand),
    #[command(subcommand, about = "Index or inspect custom search data items")]
    Item(SearchItemCommand),
}

#[derive(Args)]
pub(in crate::app) struct SearchDocsArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, default_value_t = 10, help = "Page size, max 20")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(
        long = "doc-type",
        help = "DOC, SHEET, BITABLE, MINDNOTE, FILE, WIKI, DOCX, FOLDER, CATALOG, SLIDES, SHORTCUT"
    )]
    pub(in crate::app) doc_types: Vec<String>,

    #[arg(
        long = "folder-token",
        help = "Restrict document search to Drive folder token; can repeat"
    )]
    pub(in crate::app) folder_tokens: Vec<String>,

    #[arg(
        long = "space-id",
        help = "Restrict wiki search to space ID; can repeat"
    )]
    pub(in crate::app) space_ids: Vec<String>,

    #[arg(long, help = "Only search titles")]
    pub(in crate::app) only_title: bool,

    #[arg(
        long,
        help = "DEFAULT_TYPE, OPEN_TIME, EDIT_TIME, EDIT_TIME_ASC, CREATE_TIME"
    )]
    pub(in crate::app) sort_type: Option<String>,

    #[arg(long, help = "Create time range start, Unix seconds")]
    pub(in crate::app) create_start: Option<i64>,

    #[arg(long, help = "Create time range end, Unix seconds")]
    pub(in crate::app) create_end: Option<i64>,

    #[arg(long, help = "Open time range start, Unix seconds")]
    pub(in crate::app) open_start: Option<i64>,

    #[arg(long, help = "Open time range end, Unix seconds")]
    pub(in crate::app) open_end: Option<i64>,

    #[arg(long, help = "Raw Feishu docs search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read docs search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read docs search body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchMessageArgs {
    #[arg(long, help = "Search keyword")]
    pub(in crate::app) query: Option<String>,

    #[arg(long, default_value_t = 20, help = "Page size, max 100")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,

    #[arg(long = "from-id", help = "Sender user/open ID; can repeat")]
    pub(in crate::app) from_ids: Vec<String>,

    #[arg(long = "chat-id", help = "Chat ID; can repeat")]
    pub(in crate::app) chat_ids: Vec<String>,

    #[arg(long = "at-chatter-id", help = "Mentioned user/open ID; can repeat")]
    pub(in crate::app) at_chatter_ids: Vec<String>,

    #[arg(long, help = "Message type: file, image, or media")]
    pub(in crate::app) message_type: Option<String>,

    #[arg(long, help = "Sender type: bot or user")]
    pub(in crate::app) from_type: Option<String>,

    #[arg(long, help = "Chat type: group_chat or p2p_chat")]
    pub(in crate::app) chat_type: Option<String>,

    #[arg(long, help = "Message send start time, Unix seconds")]
    pub(in crate::app) start_time: Option<String>,

    #[arg(long, help = "Message send end time, Unix seconds")]
    pub(in crate::app) end_time: Option<String>,

    #[arg(long, help = "Raw Feishu message search body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read message search body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read message search body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchSourceCommand {
    #[command(about = "List custom search data sources")]
    List(SearchSourceListArgs),
    #[command(about = "Get one custom search data source")]
    Get(SearchSourceRefArgs),
    #[command(about = "Create a custom search data source")]
    Create(SearchSourceWriteArgs),
    #[command(about = "Update a custom search data source")]
    Update(SearchSourceUpdateArgs),
    #[command(about = "Delete a custom search data source")]
    Delete(SearchSourceRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceListArgs {
    #[arg(long, help = "View format: 0 full, 1 summary")]
    pub(in crate::app) view: Option<u8>,

    #[arg(long, default_value_t = 20, help = "Page size, max 50")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceRefArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceWriteArgs {
    #[arg(long, help = "Data source display name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Data source description")]
    pub(in crate::app) description: Option<String>,

    #[arg(long, help = "Icon URL")]
    pub(in crate::app) icon_url: Option<String>,

    #[arg(long, help = "Associated schema ID")]
    pub(in crate::app) schema_id: Option<String>,

    #[arg(long, help = "Display template, usually search_common_card")]
    pub(in crate::app) template: Option<String>,

    #[arg(long, help = "Data source state: 0 online, 1 offline")]
    pub(in crate::app) state: Option<i64>,

    #[arg(long, help = "Raw Feishu data source JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read data source JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read data source JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchSourceUpdateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[command(flatten)]
    pub(in crate::app) body: SearchSourceWriteArgs,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchSchemaCommand {
    #[command(about = "Get a custom search schema")]
    Get(SearchSchemaRefArgs),
    #[command(about = "Create a custom search schema")]
    Create(SearchSchemaCreateArgs),
    #[command(about = "Update a custom search schema display config")]
    Update(SearchSchemaUpdateArgs),
    #[command(about = "Delete a custom search schema")]
    Delete(SearchSchemaRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaRefArgs {
    #[arg(long, help = "Schema ID")]
    pub(in crate::app) schema_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaCreateArgs {
    #[arg(long, help = "Raw Feishu schema JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read schema JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read schema JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Only validate schema without creating it")]
    pub(in crate::app) validate_only: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchSchemaUpdateArgs {
    #[arg(long, help = "Schema ID")]
    pub(in crate::app) schema_id: String,

    #[arg(long, help = "Raw Feishu schema update JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read schema update JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read schema update JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum SearchItemCommand {
    #[command(about = "Get one indexed item")]
    Get(SearchItemRefArgs),
    #[command(about = "Create/index one item")]
    Create(SearchItemCreateArgs),
    #[command(about = "Batch create/index items")]
    BatchCreate(SearchItemBatchCreateArgs),
    #[command(about = "Delete one indexed item")]
    Delete(SearchItemRefArgs),
}

#[derive(Args)]
pub(in crate::app) struct SearchItemRefArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Item ID")]
    pub(in crate::app) item_id: String,
}

#[derive(Args)]
pub(in crate::app) struct SearchItemCreateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Item ID")]
    pub(in crate::app) id: Option<String>,

    #[arg(long, help = "Item title")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, help = "Desktop source URL")]
    pub(in crate::app) url: Option<String>,

    #[arg(long, help = "Mobile source URL")]
    pub(in crate::app) mobile_url: Option<String>,

    #[arg(
        long,
        help = "Structured data JSON object; encoded as string for Feishu"
    )]
    pub(in crate::app) structured_json: Option<String>,

    #[arg(long, help = "Full text content for recall")]
    pub(in crate::app) text: Option<String>,

    #[arg(
        long,
        default_value = "plaintext",
        help = "Content format: plaintext or html"
    )]
    pub(in crate::app) content_format: String,

    #[arg(long, help = "ACL JSON array. Defaults to allow everyone.")]
    pub(in crate::app) acl_json: Option<String>,

    #[arg(long, help = "Raw Feishu item JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read item JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read item JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct SearchItemBatchCreateArgs {
    #[arg(long, help = "Data source ID")]
    pub(in crate::app) data_source_id: String,

    #[arg(long, help = "Raw Feishu batch item JSON, usually {\"items\":[...]}")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read batch item JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read batch item JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

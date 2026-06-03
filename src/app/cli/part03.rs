use super::*;
#[derive(Args)]
pub(in crate::app) struct ChatTabDeleteArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(long = "tab-id", help = "Tab ID to delete. Repeat for multiple tabs")]
    pub(in crate::app) tab_ids: Vec<String>,

    #[arg(
        long = "body-json",
        help = "Raw official delete body JSON object or tab_ids array"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official delete body JSON object or tab_ids array from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official delete body JSON object or tab_ids array from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ChatTabSortArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(
        long = "tab-id",
        help = "Tab ID in desired left-to-right order. Repeat for all tabs"
    )]
    pub(in crate::app) tab_ids: Vec<String>,

    #[arg(
        long = "body-json",
        help = "Raw official sort body JSON object or tab_ids array"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official sort body JSON object or tab_ids array from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official sort body JSON object or tab_ids array from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
pub(in crate::app) enum ChatMenuCommand {
    #[command(about = "Get chat menu tree")]
    Get(ChatMenuGetArgs),
    #[command(about = "Add chat menu tree; pass official JSON")]
    Add(ChatMenuBodyArgs),
    #[command(about = "Delete top-level chat menu entries")]
    Delete(ChatMenuIdsArgs),
    #[command(about = "Sort top-level chat menu entries")]
    Sort(ChatMenuIdsArgs),
}

#[derive(Args)]
pub(in crate::app) struct ChatMenuGetArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,
}

#[derive(Args)]
pub(in crate::app) struct ChatMenuBodyArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(
        long = "body-json",
        help = "Raw official body JSON object with menu_tree"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official body JSON object from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(long, help = "Read raw official body JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct ChatMenuIdsArgs {
    #[arg(long, help = "Chat ID")]
    pub(in crate::app) chat_id: String,

    #[arg(
        long = "id",
        help = "Top-level chat menu ID. Repeat in delete/sort order"
    )]
    pub(in crate::app) ids: Vec<String>,

    #[arg(
        long = "body-json",
        help = "Raw official body JSON object or chat_menu_top_level_ids array"
    )]
    pub(in crate::app) body_json: Option<String>,

    #[arg(
        long = "body-file",
        help = "Read raw official body JSON object or IDs array from file"
    )]
    pub(in crate::app) body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read raw official body JSON object or IDs array from stdin"
    )]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = DOC_AFTER_HELP)]
pub(in crate::app) enum DocCommand {
    #[command(about = "Print AI writing capabilities and format boundaries")]
    Capabilities,
    #[command(about = "Print AI-ready raw docx block JSON templates")]
    #[command(after_long_help = DOC_TEMPLATE_AFTER_HELP)]
    Template(DocTemplateArgs),
    #[command(about = "Preview generated native docx blocks without calling Feishu")]
    #[command(after_long_help = DOC_PREVIEW_AFTER_HELP)]
    Preview(DocPreviewArgs),
    #[command(about = "Call Feishu's official Markdown/HTML -> docx block converter")]
    #[command(after_long_help = DOC_CONVERT_AFTER_HELP)]
    Convert(DocConvertArgs),
    #[command(about = "Create a docx document, optionally with Markdown-ish content")]
    #[command(after_long_help = DOC_CREATE_AFTER_HELP)]
    Create(DocCreateArgs),
    #[command(about = "Append Markdown-ish content to an existing docx document")]
    Append(DocAppendArgs),
    #[command(about = "Append raw JSON block children under a parent block")]
    #[command(after_long_help = DOC_RAW_BLOCK_AFTER_HELP)]
    AppendJson(DocAppendJsonArgs),
    #[command(about = "Append raw nested descendant blocks")]
    #[command(after_long_help = DOC_RAW_BLOCK_AFTER_HELP)]
    AppendDescendant(DocAppendDescendantArgs),
    #[command(about = "Insert and upload an image or file block into a docx document")]
    #[command(after_long_help = DOC_MEDIA_AFTER_HELP)]
    InsertMedia(DocInsertMediaArgs),
    #[command(about = "Get document metadata")]
    Get(DocGetArgs),
    #[command(about = "List document blocks for format verification")]
    Blocks(DocBlocksArgs),
    #[command(about = "Get plain text content")]
    Raw(DocGetArgs),
    #[command(about = "Send a document link to a user or chat")]
    SendLink(DocSendLinkArgs),
}

#[derive(Args)]
#[command(after_long_help = DOC_PREVIEW_AFTER_HELP)]
pub(in crate::app) struct DocPreviewArgs {
    #[arg(long, help = "Markdown-ish content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read Markdown-ish content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Markdown-ish content from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_CONVERT_AFTER_HELP)]
pub(in crate::app) struct DocConvertArgs {
    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(long, help = "Markdown or HTML content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read Markdown/HTML content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Markdown/HTML content from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_CREATE_AFTER_HELP)]
pub(in crate::app) struct DocCreateArgs {
    #[arg(long, help = "Document title")]
    pub(in crate::app) title: String,

    #[arg(long, help = "Optional Drive folder token for document placement")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for docx create/write calls"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, value_enum, default_value_t = WriterArg::Local)]
    pub(in crate::app) writer: WriterArg,

    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(long, help = "Markdown-ish, Markdown, or HTML content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read Markdown-ish content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Markdown-ish content from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Send the created document link to this receiver")]
    pub(in crate::app) send_to: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type for --send-to"
    )]
    pub(in crate::app) send_to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        help = "After --send-to, read back message/chat/member proof for the sent doc link"
    )]
    pub(in crate::app) send_loop_check: bool,

    #[arg(
        long,
        help = "Move the created docx into Wiki after writing; uses --wiki-space-id or FEISHU_WIKI_SPACE_ID"
    )]
    pub(in crate::app) wiki: bool,

    #[arg(
        long,
        help = "Do not publish into Wiki even when FEISHU_DOC_CREATE_WIKI_DEFAULT=true"
    )]
    pub(in crate::app) no_wiki: bool,

    #[arg(long, help = "Target Wiki space ID for automatic publishing")]
    pub(in crate::app) wiki_space_id: Option<String>,

    #[arg(
        long,
        help = "Target parent Wiki node token; falls back to FEISHU_WIKI_PARENT_NODE_TOKEN"
    )]
    pub(in crate::app) wiki_parent_token: Option<String>,

    #[arg(
        long,
        help = "Ask Feishu to apply for move approval if document permissions are insufficient"
    )]
    pub(in crate::app) wiki_apply: bool,

    #[arg(
        long,
        help = "Keep the created docx as a fallback and return success if Wiki move fails"
    )]
    pub(in crate::app) wiki_fallback_ok: bool,

    #[arg(
        long,
        help = "Return an error if the default Wiki move fails; by default FEISHU_DOC_CREATE_WIKI_DEFAULT keeps a fallback docx"
    )]
    pub(in crate::app) wiki_strict: bool,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for automatic Wiki move"
    )]
    pub(in crate::app) wiki_auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DocAppendArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(
        long,
        help = "Parent block ID. Defaults to document_id/root page block."
    )]
    pub(in crate::app) block_id: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for the docx write call"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, value_enum, default_value_t = WriterArg::Local)]
    pub(in crate::app) writer: WriterArg,

    #[arg(long, value_enum, default_value_t = ContentTypeArg::Markdown)]
    pub(in crate::app) content_type: ContentTypeArg,

    #[arg(long, help = "Markdown-ish, Markdown, or HTML content")]
    pub(in crate::app) content: Option<String>,

    #[arg(long, help = "Read Markdown-ish content from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Markdown-ish content from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_RAW_BLOCK_AFTER_HELP)]
pub(in crate::app) struct DocAppendJsonArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(
        long,
        help = "Parent block ID. Defaults to document_id/root page block."
    )]
    pub(in crate::app) block_id: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for the docx write call"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(
        long = "raw-json",
        help = "Raw JSON array or object with a children array"
    )]
    pub(in crate::app) raw_json: Option<String>,

    #[arg(long, help = "Read raw JSON from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_RAW_BLOCK_AFTER_HELP)]
pub(in crate::app) struct DocAppendDescendantArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(
        long,
        help = "Parent block ID. Defaults to document_id/root page block."
    )]
    pub(in crate::app) block_id: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for the docx write call"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long = "raw-json", help = "Raw Feishu descendant request body JSON")]
    pub(in crate::app) raw_json: Option<String>,

    #[arg(long, help = "Read raw JSON from this file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_MEDIA_AFTER_HELP)]
pub(in crate::app) struct DocInsertMediaArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(
        long,
        help = "Parent block ID for the new media block. Defaults to document_id/root page block."
    )]
    pub(in crate::app) block_id: Option<String>,

    #[arg(long, value_enum, default_value_t = DocMediaKindArg::Image)]
    pub(in crate::app) kind: DocMediaKindArg,

    #[arg(long, help = "Local image/file path to upload")]
    pub(in crate::app) file: PathBuf,

    #[arg(long, help = "Override uploaded file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, default_value_t = -1, help = "Insert index under the parent block")]
    pub(in crate::app) index: i64,

    #[arg(long, help = "Optional checksum for Drive media upload")]
    pub(in crate::app) checksum: Option<String>,

    #[arg(long, help = "Image display width")]
    pub(in crate::app) width: Option<i64>,

    #[arg(long, help = "Image display height")]
    pub(in crate::app) height: Option<i64>,

    #[arg(long, help = "Image alignment: 1 left, 2 center, 3 right")]
    pub(in crate::app) align: Option<i64>,

    #[arg(long, help = "File block view type")]
    pub(in crate::app) view_type: Option<i64>,
}

#[derive(Args)]
pub(in crate::app) struct DocGetArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for the docx read call"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DocBlocksArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(long, default_value_t = 500, help = "Maximum block count to fetch")]
    pub(in crate::app) page_size: u16,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for the docx read call"
    )]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
pub(in crate::app) struct DocSendLinkArgs {
    #[arg(long, help = "Target docx document_id")]
    pub(in crate::app) document_id: String,

    #[arg(long, help = "Human-readable title included in the sent message")]
    pub(in crate::app) title: Option<String>,

    #[arg(long, short = 't', help = "Receiver ID for the document link")]
    pub(in crate::app) to: String,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type"
    )]
    pub(in crate::app) to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        default_value = "飞书文档",
        help = "Prefix text before the title and URL"
    )]
    pub(in crate::app) text: String,

    #[arg(
        long,
        help = "After sending, read back message/chat/member proof for this link"
    )]
    pub(in crate::app) send_loop_check: bool,
}

#[derive(Args)]
#[command(after_long_help = DOC_TEMPLATE_AFTER_HELP)]
pub(in crate::app) struct DocTemplateArgs {
    #[arg(long, value_enum, default_value_t = DocTemplateKind::All)]
    pub(in crate::app) kind: DocTemplateKind,
}

#[derive(Subcommand)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) enum BoardCommand {
    #[command(about = "Import Mermaid or PlantUML syntax into an existing whiteboard")]
    Import(BoardImportArgs),
    #[command(about = "Create raw whiteboard nodes")]
    NodeCreate(BoardNodeCreateArgs),
    #[command(about = "Print an AI-ready native-shape SVG starter for a Feishu whiteboard")]
    Template(BoardTemplateArgs),
    #[command(about = "Check an SVG against Feishu editable-whiteboard constraints")]
    CheckSvg(BoardSvgCheckArgs),
    #[command(about = "Convert an SVG to Feishu Board nodes and optionally write them")]
    Svg(BoardSvgArgs),
    #[command(about = "Create a docx document with a whiteboard block, optionally from SVG")]
    Create(BoardCreateArgs),
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardImportArgs {
    #[arg(long, help = "Whiteboard ID from a docx board block token")]
    pub(in crate::app) whiteboard_id: String,

    #[arg(long, value_enum, default_value_t = BoardSyntaxArg::Mermaid)]
    pub(in crate::app) syntax: BoardSyntaxArg,

    #[arg(long, help = "Mermaid or PlantUML source code")]
    pub(in crate::app) code: Option<String>,

    #[arg(long, help = "Read Mermaid or PlantUML source from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Mermaid or PlantUML source from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, default_value_t = 1, help = "Board parser style_type")]
    pub(in crate::app) style_type: u8,

    #[arg(long, default_value_t = 0, help = "Board parser diagram_type")]
    pub(in crate::app) diagram_type: u8,

    #[arg(long, help = "Idempotency token")]
    pub(in crate::app) client_token: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardNodeCreateArgs {
    #[arg(long, help = "Whiteboard ID from a docx board block token")]
    pub(in crate::app) whiteboard_id: String,

    #[arg(long, help = "Raw JSON object with nodes array, or a nodes array")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw node JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw node JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Idempotency token")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardTemplateArgs {
    #[arg(
        long,
        default_value = "项目画板",
        help = "Board title rendered in the SVG"
    )]
    pub(in crate::app) title: String,

    #[arg(long, value_enum, default_value_t = BoardSvgStyleArg::BrutalNote)]
    pub(in crate::app) style: BoardSvgStyleArg,
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardSvgCheckArgs {
    #[arg(long, help = "SVG source")]
    pub(in crate::app) svg: Option<String>,

    #[arg(long, help = "Read SVG source from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read SVG source from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(
        long,
        help = "Also run @larksuite/whiteboard-cli --check if npx is available"
    )]
    pub(in crate::app) external: bool,

    #[arg(
        long,
        default_value = "@larksuite/whiteboard-cli@^0.2.11",
        help = "NPM package spec for the optional external checker"
    )]
    pub(in crate::app) package: String,
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardSvgArgs {
    #[arg(
        long,
        help = "Whiteboard ID from a docx board block token; omit with --print-nodes"
    )]
    pub(in crate::app) whiteboard_id: Option<String>,

    #[arg(long, help = "SVG source")]
    pub(in crate::app) svg: Option<String>,

    #[arg(long, help = "Read SVG source from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read SVG source from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(
        long,
        help = "Print converted Board node JSON instead of writing to Feishu"
    )]
    pub(in crate::app) print_nodes: bool,

    #[arg(long, help = "Run local SVG medium checks before conversion")]
    pub(in crate::app) check: bool,

    #[arg(long, help = "Run @larksuite/whiteboard-cli --check before conversion")]
    pub(in crate::app) external_check: bool,

    #[arg(long, help = "Render a PNG preview to this path with whiteboard-cli")]
    pub(in crate::app) render_output: Option<PathBuf>,

    #[arg(
        long,
        default_value = "@larksuite/whiteboard-cli@^0.2.11",
        help = "NPM package spec for SVG conversion/render/check"
    )]
    pub(in crate::app) package: String,

    #[arg(long, help = "Idempotency token")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
#[command(after_long_help = BOARD_AFTER_HELP)]
pub(in crate::app) struct BoardCreateArgs {
    #[arg(long, help = "Document title")]
    pub(in crate::app) title: String,

    #[arg(long, help = "Optional Drive folder token for document placement")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ApiAuthArg::Tenant,
        help = "Access token type for docx create/write calls"
    )]
    pub(in crate::app) auth: ApiAuthArg,

    #[arg(long, default_value_t = 1200, help = "Whiteboard block width")]
    pub(in crate::app) width: i64,

    #[arg(long, default_value_t = 720, help = "Whiteboard block height")]
    pub(in crate::app) height: i64,

    #[arg(
        long,
        default_value_t = 1,
        help = "Whiteboard block align: 1 left, 2 center, 3 right"
    )]
    pub(in crate::app) align: i64,

    #[arg(long, help = "SVG source to convert and write into the new board")]
    pub(in crate::app) svg: Option<String>,

    #[arg(long, help = "Read SVG source from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read SVG source from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, help = "Run local SVG medium checks before conversion")]
    pub(in crate::app) check: bool,

    #[arg(long, help = "Run @larksuite/whiteboard-cli --check before conversion")]
    pub(in crate::app) external_check: bool,

    #[arg(long, help = "Render a PNG preview to this path with whiteboard-cli")]
    pub(in crate::app) render_output: Option<PathBuf>,

    #[arg(
        long,
        default_value = "@larksuite/whiteboard-cli@^0.2.11",
        help = "NPM package spec for SVG conversion/render/check"
    )]
    pub(in crate::app) package: String,

    #[arg(long, help = "Send the created document link to this receiver")]
    pub(in crate::app) send_to: Option<String>,

    #[arg(
        long,
        value_enum,
        default_value_t = ReceiveIdTypeArg::Auto,
        help = "Receiver ID type for --send-to"
    )]
    pub(in crate::app) send_to_type: ReceiveIdTypeArg,

    #[arg(
        long,
        help = "After --send-to, read back message/chat/member proof for this link"
    )]
    pub(in crate::app) send_loop_check: bool,

    #[arg(long, help = "Idempotency token for Board node create")]
    pub(in crate::app) client_token: Option<String>,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum BoardSvgStyleArg {
    BrutalNote,
    CalmMap,
    BrightSystem,
}

#[derive(Subcommand)]
#[command(after_long_help = BASE_AFTER_HELP)]
pub(in crate::app) enum BaseCommand {
    #[command(about = "Parse app/table/view tokens from a Feishu Base URL")]
    ParseUrl(BaseParseUrlArgs),
    #[command(about = "Create a Base/Bitable app")]
    Create(BaseCreateArgs),
    #[command(about = "Get Base/Bitable app metadata")]
    Get(BaseAppArgs),
    #[command(about = "Update Base/Bitable app metadata")]
    Update(BaseAppUpdateArgs),
    #[command(about = "Copy a Base/Bitable app")]
    Copy(BaseCopyArgs),
    #[command(subcommand, about = "Operate Base tables")]
    Table(BaseTableCommand),
    #[command(subcommand, about = "Operate Base fields")]
    Field(BaseFieldCommand),
    #[command(subcommand, about = "Operate Base views")]
    View(BaseViewCommand),
    #[command(subcommand, about = "Operate Base records")]
    Record(BaseRecordCommand),
    #[command(subcommand, about = "Upload/download Base attachment media")]
    Media(BaseMediaCommand),
    #[command(subcommand, about = "Operate Base dashboards")]
    Dashboard(BaseDashboardCommand),
    #[command(subcommand, about = "Operate Base automation workflows")]
    Workflow(BaseWorkflowCommand),
    #[command(subcommand, about = "Operate Base forms")]
    Form(BaseFormCommand),
    #[command(subcommand, about = "Operate Base advanced permission roles")]
    Role(BaseRoleCommand),
    #[command(subcommand, about = "Operate Base advanced permission role members")]
    Member(BaseMemberCommand),
}

#[derive(Args)]
pub(in crate::app) struct BaseParseUrlArgs {
    #[arg(help = "Feishu/Lark Base URL, Wiki URL, or raw Base app_token")]
    pub(in crate::app) url: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseCreateArgs {
    #[arg(long, help = "Base name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Optional Drive folder token")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(long, help = "Document time zone, for example Asia/Shanghai")]
    pub(in crate::app) time_zone: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseAppArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseAppUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Enable or disable Base advanced permissions")]
    pub(in crate::app) is_advanced: Option<bool>,

    #[arg(long, help = "Raw JSON object for app update")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read app update JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read app update JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseCopyArgs {
    #[arg(long, help = "Source Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Copied Base name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Target Drive folder token")]
    pub(in crate::app) folder_token: Option<String>,

    #[arg(long, help = "Copy only structure, without records/content")]
    pub(in crate::app) without_content: Option<bool>,

    #[arg(long, help = "Document time zone, for example Asia/Shanghai")]
    pub(in crate::app) time_zone: Option<String>,

    #[arg(long, help = "Raw JSON object for Base copy")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read Base copy JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read Base copy JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Subcommand)]
#[command(after_long_help = BASE_AFTER_HELP)]
pub(in crate::app) enum BaseMediaCommand {
    #[command(about = "Upload an image/file/video asset into a Base")]
    Upload(BaseMediaUploadArgs),
    #[command(about = "Download a Base media asset")]
    Download(BaseMediaDownloadArgs),
    #[command(about = "Get temporary download URLs for Base media assets")]
    TmpUrl(BaseMediaTmpUrlArgs),
    #[command(about = "Build a Base attachment field JSON value from file_tokens")]
    FieldValue(BaseMediaFieldValueArgs),
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub(in crate::app) enum BaseMediaKindArg {
    Image,
    File,
}

impl BaseMediaKindArg {
    pub(in crate::app) fn parent_type(self) -> &'static str {
        match self {
            Self::Image => "bitable_image",
            Self::File => "bitable_file",
        }
    }
}

#[derive(Args)]
pub(in crate::app) struct BaseMediaUploadArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Local file path to upload into this Base")]
    pub(in crate::app) file: PathBuf,

    #[arg(long, help = "Override uploaded file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, value_enum, default_value_t = BaseMediaKindArg::File, help = "Base media kind; file also covers videos and generic attachments")]
    pub(in crate::app) kind: BaseMediaKindArg,

    #[arg(long, help = "Optional Adler-32 checksum")]
    pub(in crate::app) checksum: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseMediaDownloadArgs {
    #[arg(long, help = "Base media file_token from an attachment field")]
    pub(in crate::app) file_token: String,

    #[arg(long, help = "Local output path")]
    pub(in crate::app) output: PathBuf,

    #[arg(long, help = "Optional HTTP Range header, e.g. bytes=0-1023")]
    pub(in crate::app) range: Option<String>,

    #[command(flatten)]
    pub(in crate::app) perm: BaseMediaPermArgs,
}

#[derive(Args)]
pub(in crate::app) struct BaseMediaTmpUrlArgs {
    #[arg(
        long = "file-token",
        help = "Base media file_token; repeat up to 5 times"
    )]
    pub(in crate::app) file_tokens: Vec<String>,

    #[command(flatten)]
    pub(in crate::app) perm: BaseMediaPermArgs,
}

#[derive(Args)]
pub(in crate::app) struct BaseMediaFieldValueArgs {
    #[arg(
        long = "file-token",
        help = "Uploaded file_token; repeat for many files"
    )]
    pub(in crate::app) file_tokens: Vec<String>,

    #[arg(long, help = "Optional attachment field name or field_id")]
    pub(in crate::app) field: Option<String>,
}

#[derive(Args, Clone)]
pub(in crate::app) struct BaseMediaPermArgs {
    #[arg(long, help = "Raw extra JSON string for advanced Base media download")]
    pub(in crate::app) extra: Option<String>,

    #[arg(long, help = "Table ID for advanced-permission Base media extra")]
    pub(in crate::app) table_id: Option<String>,

    #[arg(
        long,
        help = "Attachment field ID for advanced-permission Base media extra"
    )]
    pub(in crate::app) field_id: Option<String>,

    #[arg(long, help = "Record ID for advanced-permission Base media extra")]
    pub(in crate::app) record_id: Option<String>,
}

#[derive(Subcommand)]
pub(in crate::app) enum BaseTableCommand {
    #[command(about = "List tables in a Base")]
    List(BaseTableListArgs),
    #[command(about = "Create a table in a Base")]
    Create(BaseTableCreateArgs),
    #[command(about = "Create multiple tables in a Base")]
    BatchCreate(BaseTableBatchCreateArgs),
    #[command(about = "Update a table in a Base")]
    Update(BaseTableUpdateArgs),
    #[command(about = "Delete a table in a Base")]
    Delete(BaseTableRefArgs),
    #[command(about = "Delete multiple tables in a Base")]
    BatchDelete(BaseTableBatchDeleteArgs),
}

#[derive(Args)]
pub(in crate::app) struct BaseTableListArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, default_value_t = 100, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct BaseTableCreateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Table name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Default view name")]
    pub(in crate::app) default_view_name: Option<String>,

    #[arg(
        long = "field",
        help = "Typed field spec name:kind[:config]. Can repeat, e.g. \"状态:single-select:待处理:0|完成:1\""
    )]
    pub(in crate::app) field_specs: Vec<String>,

    #[arg(long, help = "Raw JSON array for table.fields")]
    pub(in crate::app) fields_json: Option<String>,

    #[arg(long, help = "Read table.fields JSON array from file")]
    pub(in crate::app) fields_file: Option<PathBuf>,

    #[arg(long, help = "Read table.fields JSON array from stdin")]
    pub(in crate::app) fields_stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseTableBatchCreateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Table name; repeat to create multiple tables")]
    pub(in crate::app) name: Vec<String>,

    #[arg(long, help = "Raw JSON array for request tables")]
    pub(in crate::app) tables_json: Option<String>,

    #[arg(long, help = "Raw JSON object for table batch_create body")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read table batch_create body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read table batch_create body JSON from stdin")]
    pub(in crate::app) stdin: bool,

    #[arg(long, value_enum, default_value_t = UserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: UserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct BaseTableRefArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,
}

#[derive(Args)]
pub(in crate::app) struct BaseTableUpdateArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(long, help = "Base table_id")]
    pub(in crate::app) table_id: String,

    #[arg(long, help = "Table name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw JSON object for table update")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read table update JSON object from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read table update JSON object from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct BaseTableBatchDeleteArgs {
    #[arg(long, help = "Base app_token")]
    pub(in crate::app) app_token: String,

    #[arg(
        long = "table-id",
        help = "Base table_id; repeat to delete multiple tables"
    )]
    pub(in crate::app) table_ids: Vec<String>,

    #[arg(long, help = "Raw JSON array or object with table_ids")]
    pub(in crate::app) table_ids_json: Option<String>,

    #[arg(long, help = "Read table_ids JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read table_ids JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

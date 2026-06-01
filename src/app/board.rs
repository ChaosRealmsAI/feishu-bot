use super::*;

pub(super) async fn run_board_command(
    api: &mut FeishuClient,
    command: BoardCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        BoardCommand::Import(args) => {
            let code = read_content(args.code, args.file, args.stdin)?;
            api.import_board_syntax(
                &args.whiteboard_id,
                args.syntax,
                &code,
                args.style_type,
                args.diagram_type,
                args.client_token,
            )
            .await?
        }
        BoardCommand::NodeCreate(args) => {
            let body = read_board_nodes_json(args.body_json, args.file, args.stdin)?;
            api.create_board_nodes(
                &args.whiteboard_id,
                body,
                args.user_id_type,
                args.client_token,
            )
            .await?
        }
    };
    print_response(raw_json, "board operation completed", data)
}

pub(super) fn read_board_nodes_json(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let value = read_json_value(text, file, stdin)?;
    if value.get("nodes").is_some() {
        return ensure_json_object(value, "board node body");
    }
    if value.is_array() {
        return Ok(json!({ "nodes": value }));
    }
    bail!("board node JSON must be an object with nodes array or a nodes array")
}

impl BoardSyntaxArg {
    pub(super) fn as_api_value(self) -> i64 {
        match self {
            BoardSyntaxArg::Plantuml => 1,
            BoardSyntaxArg::Mermaid => 2,
        }
    }
}

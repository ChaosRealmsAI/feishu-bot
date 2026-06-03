use super::*;

pub(super) async fn run_board_command(
    api: &mut FeishuClient,
    command: BoardCommand,
    raw_json: bool,
) -> Result<()> {
    match command {
        BoardCommand::Import(args) => {
            let code = read_content(args.code, args.file, args.stdin)?;
            let data = api
                .import_board_syntax(
                    &args.whiteboard_id,
                    args.syntax,
                    &code,
                    args.style_type,
                    args.diagram_type,
                    args.client_token,
                )
                .await?;
            print_response(raw_json, "board operation completed", data)
        }
        BoardCommand::NodeCreate(args) => {
            let body = read_board_nodes_json(args.body_json, args.file, args.stdin)?;
            let data = api
                .create_board_nodes(
                    &args.whiteboard_id,
                    body,
                    args.user_id_type,
                    args.client_token,
                )
                .await?;
            print_response(raw_json, "board operation completed", data)
        }
        BoardCommand::Template(args) => run_board_template_command(args, raw_json),
        BoardCommand::CheckSvg(args) => run_board_svg_check_command(args, raw_json),
        BoardCommand::Svg(args) => {
            let input = read_svg_input(args.svg, args.file, args.stdin)?;
            if args.check {
                let local = check_svg_medium(&input.svg);
                if local_has_errors(&local) {
                    if raw_json {
                        println!("{}", serde_json::to_string_pretty(&local)?);
                    }
                    bail!("local SVG check failed: {}", summarize_svg_check(&local));
                }
            }
            let external_check = if args.external_check {
                Some(run_whiteboard_cli_check(&args.package, input.path())?)
            } else {
                None
            };
            let render = if let Some(output) = args.render_output {
                Some(run_whiteboard_cli_render(
                    &args.package,
                    input.path(),
                    &output,
                )?)
            } else {
                None
            };
            let nodes = convert_svg_to_board_nodes(&args.package, input.path())?;
            if args.print_nodes {
                let mut output = json!({
                    "code": 0,
                    "msg": "success",
                    "data": {
                        "nodes": nodes,
                        "external_check": external_check,
                        "render": render,
                    }
                });
                if let Some(whiteboard_id) = args.whiteboard_id {
                    output["data"]["whiteboard_id"] = Value::String(whiteboard_id);
                }
                println!("{}", serde_json::to_string_pretty(&output)?);
                return Ok(());
            }
            let whiteboard_id = args
                .whiteboard_id
                .ok_or_else(|| anyhow!("board svg needs --whiteboard-id or --print-nodes"))?;
            let imported = api
                .create_board_nodes(&whiteboard_id, nodes, args.user_id_type, args.client_token)
                .await?;
            let output = json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "whiteboard_id": whiteboard_id,
                    "imported": imported,
                    "external_check": external_check,
                    "render": render,
                }
            });
            print_response(raw_json, "SVG whiteboard written", output)
        }
        BoardCommand::Create(args) => run_board_create_command(api, args, raw_json).await,
    }
}

pub(super) fn board_command_can_run_without_api(command: &BoardCommand) -> bool {
    matches!(
        command,
        BoardCommand::Template(_)
            | BoardCommand::CheckSvg(_)
            | BoardCommand::Svg(BoardSvgArgs {
                print_nodes: true,
                whiteboard_id: None,
                ..
            })
    )
}

pub(super) fn run_board_local_command(command: BoardCommand, raw_json: bool) -> Result<()> {
    match command {
        BoardCommand::Template(args) => run_board_template_command(args, raw_json),
        BoardCommand::CheckSvg(args) => run_board_svg_check_command(args, raw_json),
        BoardCommand::Svg(args)
            if args.print_nodes && args.whiteboard_id.as_deref().unwrap_or("").is_empty() =>
        {
            run_board_svg_print_nodes_command(args, raw_json)
        }
        _ => unreachable!("API board command routed to local board runner"),
    }
}

fn run_board_template_command(args: BoardTemplateArgs, raw_json: bool) -> Result<()> {
    let svg = board_svg_template(args.style, &args.title);
    if raw_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "style": args.style.as_slug(),
                "title": args.title,
                "svg": svg,
            }))?
        );
    } else {
        print!("{svg}");
    }
    Ok(())
}

fn run_board_svg_check_command(args: BoardSvgCheckArgs, raw_json: bool) -> Result<()> {
    let input = read_svg_input(args.svg, args.file, args.stdin)?;
    let local = check_svg_medium(&input.svg);
    let external = if args.external {
        Some(run_whiteboard_cli_check(&args.package, input.path())?)
    } else {
        None
    };
    let output = json!({
        "code": if local_has_errors(&local) { 1 } else { 0 },
        "msg": if local_has_errors(&local) { "local SVG check failed" } else { "success" },
        "data": {
            "local": local,
            "external": external,
        }
    });
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        print_svg_check_summary(&output);
    }
    if local_has_errors(&output["data"]["local"]) {
        bail!("local SVG check failed");
    }
    Ok(())
}

fn run_board_svg_print_nodes_command(args: BoardSvgArgs, _raw_json: bool) -> Result<()> {
    let input = read_svg_input(args.svg, args.file, args.stdin)?;
    if args.check {
        let local = check_svg_medium(&input.svg);
        if local_has_errors(&local) {
            println!("{}", serde_json::to_string_pretty(&local)?);
            bail!("local SVG check failed: {}", summarize_svg_check(&local));
        }
    }
    let external_check = if args.external_check {
        Some(run_whiteboard_cli_check(&args.package, input.path())?)
    } else {
        None
    };
    let render = if let Some(output) = args.render_output {
        Some(run_whiteboard_cli_render(
            &args.package,
            input.path(),
            &output,
        )?)
    } else {
        None
    };
    let nodes = convert_svg_to_board_nodes(&args.package, input.path())?;
    let output = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "nodes": nodes,
            "external_check": external_check,
            "render": render,
        }
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
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

async fn run_board_create_command(
    api: &mut FeishuClient,
    args: BoardCreateArgs,
    raw_json: bool,
) -> Result<()> {
    let created = api
        .create_document_with_auth(&args.title, args.folder_token.as_deref(), args.auth)
        .await?;
    let document_id = get_string(&created, &["data", "document", "document_id"])
        .or_else(|| get_string(&created, &["data", "document_id"]))
        .ok_or_else(|| {
            anyhow!("create document response did not include document_id: {created}")
        })?;

    let append_response = api
        .append_raw_children_at_with_auth(
            &document_id,
            &document_id,
            -1,
            vec![json!({
                "block_type": 43,
                "board": {
                    "align": args.align,
                    "width": args.width,
                    "height": args.height
                }
            })],
            args.auth,
        )
        .await
        .with_context(|| {
            format!("created document {document_id}, but failed to append whiteboard block")
        })?;
    let blocks = api
        .get_document_blocks_with_auth(&document_id, 500, args.auth)
        .await
        .with_context(|| {
            format!(
                "appended whiteboard block in document {document_id}, but failed to read blocks"
            )
        })?;
    let whiteboard_id = first_board_token(&append_response)
        .or_else(|| first_board_token(&blocks))
        .ok_or_else(|| anyhow!("document {document_id} board block did not expose board.token"))?;

    let mut imported = None;
    let mut external_check = None;
    let mut render = None;
    if args.svg.is_some() || args.file.is_some() || args.stdin {
        let input = read_svg_input(args.svg, args.file, args.stdin)?;
        if args.check {
            let local = check_svg_medium(&input.svg);
            if local_has_errors(&local) {
                bail!("local SVG check failed: {}", summarize_svg_check(&local));
            }
        }
        if args.external_check {
            external_check = Some(run_whiteboard_cli_check(&args.package, input.path())?);
        }
        if let Some(output) = args.render_output {
            render = Some(run_whiteboard_cli_render(
                &args.package,
                input.path(),
                &output,
            )?);
        }
        let nodes = convert_svg_to_board_nodes(&args.package, input.path())?;
        imported = Some(
            api.create_board_nodes(
                &whiteboard_id,
                nodes,
                args.user_id_type,
                args.client_token,
            )
            .await
            .with_context(|| {
                format!("created document {document_id} and board {whiteboard_id}, but SVG node import failed")
            })?,
        );
    }

    let url = api.document_url(&document_id);
    let sent_delivery = if let Some(to) = args.send_to {
        let msg = format!("{}: {}\n{}", args.title, url, document_id);
        let sent_message = api
            .send_text(&to, args.send_to_type.resolve(&to), &msg, None)
            .await?;
        let proof = if args.send_loop_check {
            Some(probe_sent_text_message(api, &to, &sent_message, &msg).await?)
        } else {
            None
        };
        Some((sent_message, proof))
    } else {
        None
    };

    let mut output = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document_id": document_id,
            "url": url,
            "whiteboard_id": whiteboard_id,
            "created": created,
            "append_response": append_response,
            "blocks": blocks,
            "imported": imported,
            "external_check": external_check,
            "render": render,
        }
    });
    if let Some((sent_message, proof)) = sent_delivery {
        output["data"]["sent_message"] = sent_message;
        if let Some(proof) = proof {
            output["data"]["send_loop_check"] = proof;
        }
    }
    print_response(raw_json, "whiteboard document created", output)
}

fn first_board_token(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => {
            if let Some(token) = value.pointer("/board/token").and_then(Value::as_str) {
                return Some(token.to_string());
            }
            map.values().find_map(first_board_token)
        }
        Value::Array(items) => items.iter().find_map(first_board_token),
        _ => None,
    }
}

struct SvgInput {
    svg: String,
    source_path: PathBuf,
    temp_path: Option<PathBuf>,
}

impl SvgInput {
    fn path(&self) -> &Path {
        &self.source_path
    }
}

impl Drop for SvgInput {
    fn drop(&mut self) {
        if let Some(path) = &self.temp_path {
            let _ = fs::remove_file(path);
        }
    }
}

fn read_svg_input(svg: Option<String>, file: Option<PathBuf>, stdin: bool) -> Result<SvgInput> {
    if let Some(path) = file {
        let svg = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        return Ok(SvgInput {
            svg,
            source_path: path,
            temp_path: None,
        });
    }
    let svg = read_content(svg, None, stdin)?;
    let path = std::env::temp_dir().join(format!("feishu-bot-board-{}.svg", random_uuid()));
    fs::write(&path, &svg).with_context(|| format!("write {}", path.display()))?;
    Ok(SvgInput {
        svg,
        source_path: path.clone(),
        temp_path: Some(path),
    })
}

fn convert_svg_to_board_nodes(package: &str, input: &Path) -> Result<Value> {
    let output = run_whiteboard_cli_json(
        package,
        input,
        &["--to", "openapi", "--format", "json", "-f", "svg"],
    )?;
    normalize_whiteboard_cli_nodes(output)
}

pub(super) fn normalize_whiteboard_cli_nodes(value: Value) -> Result<Value> {
    if value.get("nodes").is_some() {
        return ensure_json_object(value, "whiteboard node body");
    }
    if let Some(result) = value.pointer("/data/result") {
        if result.get("nodes").is_some() {
            return ensure_json_object(result.clone(), "whiteboard-cli data.result");
        }
    }
    if let Some(result) = value.get("result") {
        if result.get("nodes").is_some() {
            return ensure_json_object(result.clone(), "whiteboard-cli result");
        }
    }
    bail!("whiteboard-cli output did not contain nodes: {value}")
}

fn run_whiteboard_cli_check(package: &str, input: &Path) -> Result<Value> {
    run_whiteboard_cli_json(package, input, &["-f", "svg", "--check"])
}

fn run_whiteboard_cli_render(package: &str, input: &Path, output: &Path) -> Result<Value> {
    run_whiteboard_cli(
        package,
        input,
        &[
            "-o",
            output.to_str().ok_or_else(|| {
                anyhow!(
                    "render output path is not valid UTF-8: {}",
                    output.display()
                )
            })?,
            "-f",
            "svg",
        ],
    )?;
    Ok(json!({
        "output": output.display().to_string(),
    }))
}

fn run_whiteboard_cli_json(package: &str, input: &Path, args: &[&str]) -> Result<Value> {
    let stdout = run_whiteboard_cli(package, input, args)?;
    serde_json::from_str(&stdout)
        .with_context(|| format!("parse whiteboard-cli JSON output for {}", input.display()))
}

fn run_whiteboard_cli(package: &str, input: &Path, args: &[&str]) -> Result<String> {
    let output = ProcessCommand::new("npx")
        .arg("-y")
        .arg(package)
        .arg("-i")
        .arg(input)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "run npx -y {package}; install Node/npm and ensure npx can reach @larksuite/whiteboard-cli"
            )
        })?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        bail!(
            "whiteboard-cli failed with status {}:\nstdout:\n{}\nstderr:\n{}",
            output.status,
            stdout,
            stderr
        );
    }
    Ok(stdout)
}

pub(super) fn check_svg_medium(svg: &str) -> Value {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let lower = svg.to_ascii_lowercase();
    if !lower.contains("<svg") {
        errors.push("missing <svg> root".to_string());
    }

    let forbidden_tags = [
        "filter",
        "lineargradient",
        "radialgradient",
        "pattern",
        "clippath",
        "mask",
        "image",
        "foreignobject",
    ];
    for tag in forbidden_tags {
        if contains_svg_tag(&lower, tag) {
            errors.push(format!(
                "unsupported <{tag}>; Feishu editable boards flatten or reject it"
            ));
        }
    }

    if contains_non_marker_path(&lower) {
        warnings.push(
            "avoid structural <path>; use rect/circle/ellipse/line/polyline/text, except marker path for arrowheads"
                .to_string(),
        );
    }
    if contains_svg_tag(&lower, "polygon") {
        warnings.push(
            "avoid structural <polygon>; use rect/circle/ellipse/line/polyline/text, except tiny decorative accents"
                .to_string(),
        );
    }

    let forbidden_attrs = [
        "font-family",
        " opacity=",
        "\topacity=",
        "fill-opacity",
        "stroke-opacity",
        "filter=",
        "clip-path",
        "mask=",
    ];
    for attr in forbidden_attrs {
        if lower.contains(attr) {
            errors.push(format!("unsupported SVG attribute `{}`", attr.trim()));
        }
    }

    let allowed = [
        "svg", "g", "defs", "marker", "rect", "circle", "ellipse", "line", "polyline", "text",
        "tspan", "title", "desc", "path",
    ];
    for tag in svg_tag_names(&lower) {
        if !allowed.contains(&tag.as_str()) {
            warnings.push(format!(
                "tag <{tag}> may not convert to editable Feishu shapes; prefer native shape tags"
            ));
        }
    }

    if lower.contains("lineargradient") || lower.contains("radialgradient") {
        errors.push("gradients are unsupported; use solid hex fills".to_string());
    }
    if !lower.contains("<text") {
        warnings.push("no <text> elements found; labels should be editable text".to_string());
    }

    json!({
        "errors": errors.len(),
        "warnings": warnings.len(),
        "issues": errors.into_iter().map(|message| json!({"severity": "error", "message": message}))
            .chain(warnings.into_iter().map(|message| json!({"severity": "warning", "message": message})))
            .collect::<Vec<_>>(),
    })
}

fn local_has_errors(value: &Value) -> bool {
    value
        .get("errors")
        .and_then(Value::as_u64)
        .is_some_and(|count| count > 0)
}

fn summarize_svg_check(value: &Value) -> String {
    let errors = value.get("errors").and_then(Value::as_u64).unwrap_or(0);
    let warnings = value.get("warnings").and_then(Value::as_u64).unwrap_or(0);
    format!("{errors} errors, {warnings} warnings")
}

fn print_svg_check_summary(value: &Value) {
    let local = &value["data"]["local"];
    println!("local_check={}", summarize_svg_check(local));
    if let Some(issues) = local.get("issues").and_then(Value::as_array) {
        for issue in issues {
            let severity = issue
                .get("severity")
                .and_then(Value::as_str)
                .unwrap_or("issue");
            let message = issue
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("<missing message>");
            println!("{severity}: {message}");
        }
    }
    if !value["data"]["external"].is_null() {
        println!("external_check={}", value["data"]["external"]);
    }
}

fn contains_svg_tag(svg: &str, tag: &str) -> bool {
    svg.contains(&format!("<{tag} "))
        || svg.contains(&format!("<{tag}>"))
        || svg.contains(&format!("</{tag}>"))
}

fn contains_non_marker_path(svg: &str) -> bool {
    let mut rest = svg;
    let mut offset = 0;
    while let Some(relative) = rest.find("<path") {
        let index = offset + relative;
        let before = &svg[..index];
        let open_marker = before.rfind("<marker");
        let close_marker = before.rfind("</marker");
        if !matches!((open_marker, close_marker), (Some(open), close) if close.is_none_or(|close| open > close))
        {
            return true;
        }
        let next = relative + "<path".len();
        rest = &rest[next..];
        offset += next;
    }
    false
}

fn svg_tag_names(svg: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut rest = svg;
    while let Some(start) = rest.find('<') {
        rest = &rest[start + 1..];
        let trimmed = rest.trim_start_matches('/');
        if trimmed.starts_with('!') || trimmed.starts_with('?') {
            continue;
        }
        let name = trimmed
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == ':')
            .collect::<String>();
        if !name.is_empty() {
            tags.push(name);
        }
    }
    tags.sort();
    tags.dedup();
    tags
}

pub(super) fn board_svg_template(style: BoardSvgStyleArg, title: &str) -> String {
    match style {
        BoardSvgStyleArg::BrutalNote => brutal_note_template(title),
        BoardSvgStyleArg::CalmMap => calm_map_template(title),
        BoardSvgStyleArg::BrightSystem => bright_system_template(title),
    }
}

impl BoardSvgStyleArg {
    fn as_slug(self) -> &'static str {
        match self {
            BoardSvgStyleArg::BrutalNote => "brutal-note",
            BoardSvgStyleArg::CalmMap => "calm-map",
            BoardSvgStyleArg::BrightSystem => "bright-system",
        }
    }
}

fn xml_escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn brutal_note_template(title: &str) -> String {
    let title = xml_escape_text(title);
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="920" viewBox="0 0 1600 920">
  <defs>
    <marker id="arrow" markerWidth="12" markerHeight="12" refX="9" refY="4" orient="auto" markerUnits="strokeWidth">
      <path d="M0 0 L10 4 L0 8 z"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1600" height="920" fill="#EFE9D9"/>
  <rect x="90" y="84" width="1480" height="120" fill="#F5C518" stroke="#0F0F0F" stroke-width="4"/>
  <text x="130" y="158" font-size="50" font-weight="700" fill="#0F0F0F">{title}</text>
  <rect x="122" y="278" width="360" height="220" fill="#1F8A4C" stroke="#0F0F0F" stroke-width="4"/>
  <text x="154" y="346" font-size="30" font-weight="700" fill="#0F0F0F">01  输入</text>
  <text x="154" y="402" font-size="24" fill="#0F0F0F">写这里：目标、约束、素材</text>
  <rect x="542" y="278" width="360" height="220" fill="#F06CA8" stroke="#0F0F0F" stroke-width="4"/>
  <text x="574" y="346" font-size="30" font-weight="700" fill="#0F0F0F">02  判断</text>
  <text x="574" y="402" font-size="24" fill="#0F0F0F">写这里：分支、取舍、风险</text>
  <rect x="962" y="278" width="360" height="220" fill="#E85A1F" stroke="#0F0F0F" stroke-width="4"/>
  <text x="994" y="346" font-size="30" font-weight="700" fill="#0F0F0F">03  输出</text>
  <text x="994" y="402" font-size="24" fill="#0F0F0F">写这里：结论、交付、链接</text>
  <line x1="496" y1="388" x2="526" y2="388" stroke="#0F0F0F" stroke-width="4" marker-end="url(#arrow)"/>
  <line x1="916" y1="388" x2="946" y2="388" stroke="#0F0F0F" stroke-width="4" marker-end="url(#arrow)"/>
  <rect x="122" y="594" width="1200" height="170" fill="#E4DCC4" stroke="#0F0F0F" stroke-width="4"/>
  <text x="154" y="660" font-size="30" font-weight="700" fill="#0F0F0F">备注</text>
  <text x="154" y="716" font-size="24" fill="#0F0F0F">替换模板文字；保持 rect / circle / ellipse / line / polyline / text 这些原生形状。</text>
</svg>
"##
    )
}

fn calm_map_template(title: &str) -> String {
    let title = xml_escape_text(title);
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="900" viewBox="0 0 1600 900">
  <defs>
    <marker id="arrow" markerWidth="12" markerHeight="12" refX="9" refY="4" orient="auto" markerUnits="strokeWidth">
      <path d="M0 0 L10 4 L0 8 z"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1600" height="900" fill="#F6F1E7"/>
  <text x="110" y="135" font-size="54" font-weight="700" fill="#183B32">{title}</text>
  <line x1="110" y1="174" x2="1490" y2="174" stroke="#183B32" stroke-width="3"/>
  <ellipse cx="800" cy="438" rx="190" ry="116" fill="#D8E7DD" stroke="#183B32" stroke-width="4"/>
  <text x="710" y="426" font-size="30" font-weight="700" fill="#183B32">核心议题</text>
  <text x="684" y="474" font-size="24" fill="#183B32">一句话放这里</text>
  <rect x="110" y="302" width="340" height="170" rx="18" fill="#FFFFFF" stroke="#183B32" stroke-width="4"/>
  <text x="142" y="364" font-size="28" font-weight="700" fill="#183B32">背景</text>
  <text x="142" y="416" font-size="22" fill="#183B32">用户、场景、上下文</text>
  <rect x="1150" y="302" width="340" height="170" rx="18" fill="#FFFFFF" stroke="#183B32" stroke-width="4"/>
  <text x="1182" y="364" font-size="28" font-weight="700" fill="#183B32">结果</text>
  <text x="1182" y="416" font-size="22" fill="#183B32">交付、验收、后续</text>
  <rect x="324" y="650" width="340" height="150" rx="18" fill="#EEDCC7" stroke="#183B32" stroke-width="4"/>
  <text x="356" y="712" font-size="28" font-weight="700" fill="#183B32">风险</text>
  <text x="356" y="760" font-size="22" fill="#183B32">需要提前处理</text>
  <rect x="936" y="650" width="340" height="150" rx="18" fill="#C8D7EA" stroke="#183B32" stroke-width="4"/>
  <text x="968" y="712" font-size="28" font-weight="700" fill="#183B32">决策</text>
  <text x="968" y="760" font-size="22" fill="#183B32">已确认或待确认</text>
  <line x1="454" y1="386" x2="612" y2="416" stroke="#183B32" stroke-width="3" marker-end="url(#arrow)"/>
  <line x1="988" y1="416" x2="1140" y2="386" stroke="#183B32" stroke-width="3" marker-end="url(#arrow)"/>
  <line x1="688" y1="536" x2="560" y2="640" stroke="#183B32" stroke-width="3" marker-end="url(#arrow)"/>
  <line x1="912" y1="536" x2="1040" y2="640" stroke="#183B32" stroke-width="3" marker-end="url(#arrow)"/>
</svg>
"##
    )
}

fn bright_system_template(title: &str) -> String {
    let title = xml_escape_text(title);
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="900" viewBox="0 0 1600 900">
  <defs>
    <marker id="arrow" markerWidth="12" markerHeight="12" refX="9" refY="4" orient="auto" markerUnits="strokeWidth">
      <path d="M0 0 L10 4 L0 8 z"/>
    </marker>
  </defs>
  <rect x="0" y="0" width="1600" height="900" fill="#FFFFFF"/>
  <rect x="80" y="70" width="420" height="120" rx="24" fill="#2F6BFF" stroke="#111111" stroke-width="4"/>
  <text x="112" y="142" font-size="42" font-weight="700" fill="#FFFFFF">{title}</text>
  <rect x="96" y="286" width="320" height="190" rx="20" fill="#B7F04A" stroke="#111111" stroke-width="4"/>
  <text x="126" y="354" font-size="30" font-weight="700" fill="#111111">触发</text>
  <text x="126" y="408" font-size="23" fill="#111111">评论 / 指令 / 状态变化</text>
  <rect x="560" y="286" width="320" height="190" rx="20" fill="#FFD447" stroke="#111111" stroke-width="4"/>
  <text x="590" y="354" font-size="30" font-weight="700" fill="#111111">执行</text>
  <text x="590" y="408" font-size="23" fill="#111111">agent 计划、修改、验证</text>
  <rect x="1024" y="286" width="320" height="190" rx="20" fill="#FF7AC8" stroke="#111111" stroke-width="4"/>
  <text x="1054" y="354" font-size="30" font-weight="700" fill="#111111">回传</text>
  <text x="1054" y="408" font-size="23" fill="#111111">回复批注、更新文档</text>
  <line x1="426" y1="381" x2="548" y2="381" stroke="#111111" stroke-width="4" marker-end="url(#arrow)"/>
  <line x1="890" y1="381" x2="1012" y2="381" stroke="#111111" stroke-width="4" marker-end="url(#arrow)"/>
  <rect x="96" y="600" width="1344" height="150" rx="20" fill="#F4F4F4" stroke="#111111" stroke-width="4"/>
  <text x="126" y="662" font-size="30" font-weight="700" fill="#111111">运行记录</text>
  <text x="126" y="714" font-size="23" fill="#111111">把关键命令、产物链接、验证结果放在这里，便于远程协作追踪。</text>
</svg>
"##
    )
}

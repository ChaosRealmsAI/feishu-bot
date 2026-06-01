use super::*;

pub(super) fn preview_doc(args: DocPreviewArgs, raw_json: bool) -> Result<()> {
    let content = read_content(args.content, args.file, args.stdin)?;
    let blocks = markdown_to_blocks(&content);
    print_generated_blocks(raw_json, &blocks)
}

pub(super) async fn run_doc_command(
    api: &mut FeishuClient,
    command: DocCommand,
    raw_json: bool,
) -> Result<()> {
    match command {
        DocCommand::Capabilities | DocCommand::Template(_) | DocCommand::Preview(_) => {
            unreachable!("non-API doc commands are handled before config loading")
        }
        DocCommand::Convert(args) => {
            let content = read_content(args.content, args.file, args.stdin)?;
            let data = api.convert_content(args.content_type, &content).await?;
            print_convert_response(raw_json, data)
        }
        DocCommand::Create(args) => {
            if args.no_wiki
                && (args.wiki || args.wiki_space_id.is_some() || args.wiki_parent_token.is_some())
            {
                bail!("doc create cannot combine --no-wiki with --wiki, --wiki-space-id, or --wiki-parent-token");
            }
            let allow_wiki_fallback =
                doc_create_allows_wiki_fallback(&args, api.config.default_doc_create_wiki);
            let wants_wiki = !args.no_wiki
                && (api.config.default_doc_create_wiki
                    || args.wiki
                    || args.wiki_space_id.is_some()
                    || args.wiki_parent_token.is_some());
            let wiki_target = if wants_wiki {
                let space_id = args
                    .wiki_space_id
                    .clone()
                    .or_else(|| api.config.default_wiki_space_id.clone())
                    .ok_or_else(|| {
                        anyhow!(
                            "Wiki publishing requires --wiki-space-id or FEISHU_WIKI_SPACE_ID before creating a document"
                        )
                    })?;
                let parent_node_token = args
                    .wiki_parent_token
                    .clone()
                    .or_else(|| api.config.default_wiki_parent_node_token.clone());
                Some((space_id, parent_node_token, args.wiki_apply, args.wiki_auth))
            } else {
                None
            };
            let doc = api
                .create_document_with_auth(&args.title, args.folder_token.as_deref(), args.auth)
                .await?;
            let document_id = get_string(&doc, &["data", "document", "document_id"])
                .or_else(|| get_string(&doc, &["data", "document_id"]))
                .ok_or_else(|| {
                    anyhow!("create document response did not include document_id: {doc}")
                })?;

            let content = read_optional_content(args.content, args.file, args.stdin)?;
            if let Some(content) = content {
                match args.writer {
                    WriterArg::Local => {
                        api.append_document_with_auth(
                            &document_id,
                            &document_id,
                            &content,
                            args.auth,
                        )
                        .await?;
                    }
                    WriterArg::Official => {
                        api.append_converted_content_with_auth(
                            &document_id,
                            &document_id,
                            args.content_type,
                            &content,
                            args.auth,
                        )
                        .await?;
                    }
                }
            }

            let url = api.document_url(&document_id);
            let mut wiki_move_error = None;
            let wiki_move = if let Some((space_id, parent_node_token, apply, auth)) = wiki_target {
                let path = format!(
                    "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
                    encode_path_segment(&space_id)
                );
                let body = build_doc_create_wiki_move_body(&document_id, parent_node_token, apply);
                match wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await {
                    Ok(data) => Some(data),
                    Err(error) if allow_wiki_fallback => {
                        wiki_move_error = Some(format!(
                            "created document {document_id} ({url}), but failed to move it into Wiki space {space_id}: {error:#}"
                        ));
                        None
                    }
                    Err(error) => {
                        return Err(error).with_context(|| {
                            format!(
                                "created document {document_id} ({url}), but failed to move it into Wiki space {space_id}"
                            )
                        });
                    }
                }
            } else {
                None
            };

            let sent_delivery = if let Some(to) = args.send_to {
                let msg = if wiki_move_error.is_some() {
                    format!(
                        "{}: {}\n{}\nWiki move failed; this is the fallback docx.",
                        args.title, url, document_id
                    )
                } else {
                    format!("{}: {}\n{}", args.title, url, document_id)
                };
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

            if raw_json {
                let mut output = doc;
                output["url"] = Value::String(url);
                if let Some(wiki_move) = wiki_move {
                    output["wiki_move"] = wiki_move;
                }
                if let Some(error) = wiki_move_error {
                    output["wiki_move_error"] = Value::String(error);
                }
                if let Some((sent_message, send_loop_check)) = sent_delivery {
                    output["sent_message"] = sent_message;
                    if let Some(send_loop_check) = send_loop_check {
                        output["send_loop_check"] = send_loop_check;
                    }
                }
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("document created");
                println!("document_id={document_id}");
                println!("url={url}");
                if let Some(wiki_move) = wiki_move {
                    println!("wiki_move={}", serde_json::to_string_pretty(&wiki_move)?);
                }
                if let Some(error) = wiki_move_error {
                    println!("wiki_move_error={error}");
                }
                if let Some((sent_message, send_loop_check)) = sent_delivery {
                    println!(
                        "sent_message={}",
                        serde_json::to_string_pretty(&sent_message)?
                    );
                    if let Some(send_loop_check) = send_loop_check {
                        println!(
                            "send_loop_check={}",
                            serde_json::to_string_pretty(&send_loop_check)?
                        );
                    }
                }
            }
            Ok(())
        }
        DocCommand::Append(args) => {
            let content = read_content(args.content, args.file, args.stdin)?;
            let block_id = args.block_id.as_deref().unwrap_or(&args.document_id);
            let data = match args.writer {
                WriterArg::Local => {
                    api.append_document_with_auth(&args.document_id, block_id, &content, args.auth)
                        .await?
                }
                WriterArg::Official => {
                    api.append_converted_content_with_auth(
                        &args.document_id,
                        block_id,
                        args.content_type,
                        &content,
                        args.auth,
                    )
                    .await?
                }
            };
            print_response(raw_json, "document appended", data)
        }
        DocCommand::AppendJson(args) => {
            let text = read_content(args.raw_json, args.file, args.stdin)?;
            let block_id = args.block_id.as_deref().unwrap_or(&args.document_id);
            let data = api
                .append_raw_children_with_auth(
                    &args.document_id,
                    block_id,
                    parse_raw_children(&text)?,
                    args.auth,
                )
                .await?;
            print_response(raw_json, "raw children appended", data)
        }
        DocCommand::AppendDescendant(args) => {
            let text = read_content(args.raw_json, args.file, args.stdin)?;
            let block_id = args.block_id.as_deref().unwrap_or(&args.document_id);
            let body: Value = serde_json::from_str(&text).context("parse descendant JSON body")?;
            let data = api
                .append_descendant_body_with_auth(&args.document_id, block_id, body, args.auth)
                .await?;
            print_response(raw_json, "descendant blocks appended", data)
        }
        DocCommand::InsertMedia(args) => {
            let data = insert_doc_media(api, args).await?;
            print_response(raw_json, "document media inserted", data)
        }
        DocCommand::Get(args) => {
            let data = api
                .get_document_with_auth(&args.document_id, args.auth)
                .await?;
            print_response(raw_json, "document metadata", data)
        }
        DocCommand::Blocks(args) => {
            let data = api
                .get_document_blocks_with_auth(&args.document_id, args.page_size, args.auth)
                .await?;
            print_blocks_response(raw_json, data)
        }
        DocCommand::Raw(args) => {
            let data = api
                .raw_document_with_auth(&args.document_id, args.auth)
                .await?;
            if raw_json {
                println!("{}", serde_json::to_string_pretty(&data)?);
            } else if let Some(content) = get_string(&data, &["data", "content"]) {
                println!("{content}");
            } else {
                println!("{}", serde_json::to_string_pretty(&data)?);
            }
            Ok(())
        }
        DocCommand::SendLink(args) => {
            let title = args.title.unwrap_or_else(|| args.document_id.clone());
            let url = api.document_url(&args.document_id);
            let msg = format!("{}: {}\n{}", args.text, title, url);
            let sent_message = api
                .send_text(&args.to, args.to_type.resolve(&args.to), &msg, None)
                .await?;
            let send_loop_check = if args.send_loop_check {
                Some(probe_sent_text_message(api, &args.to, &sent_message, &msg).await?)
            } else {
                None
            };
            let mut output = sent_message;
            output["url"] = Value::String(url);
            output["title"] = Value::String(title);
            if let Some(send_loop_check) = send_loop_check {
                output["send_loop_check"] = send_loop_check;
            }
            print_response(raw_json, "document link sent", output)
        }
    }
}

pub(super) fn doc_create_allows_wiki_fallback(
    args: &DocCreateArgs,
    default_doc_create_wiki: bool,
) -> bool {
    args.wiki_fallback_ok || (default_doc_create_wiki && !args.wiki_strict)
}

pub(super) fn markdown_to_blocks(content: &str) -> Vec<Value> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut in_code = false;
    let mut code = Vec::new();
    let mut code_language: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim_end();
        let stripped = trimmed.trim_start();
        if stripped.starts_with("```") {
            if in_code {
                if !code.is_empty() {
                    blocks.push(code_block(&code.join("\n"), code_language.as_deref()));
                    code.clear();
                }
                code_language = None;
                in_code = false;
            } else {
                flush_paragraph(&mut blocks, &mut paragraph);
                code_language = parse_code_fence_language(stripped);
                in_code = true;
            }
            continue;
        }

        if in_code {
            code.push(trimmed.to_string());
            continue;
        }

        if trimmed.trim().is_empty() {
            flush_paragraph(&mut blocks, &mut paragraph);
            continue;
        }

        if is_divider(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(divider_block());
        } else if let Some((level, heading)) = parse_heading(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            let level = level.clamp(1, 9);
            let block_type = 2 + level as i64;
            let field = format!("heading{level}");
            blocks.push(text_block(block_type, &field, heading.trim()));
        } else if let Some((done, item)) = parse_todo(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(todo_block(item.trim(), done));
        } else if let Some(quote) = parse_quote(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(15, "quote", quote.trim()));
        } else if let Some(item) = parse_unordered(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(12, "bullet", item.trim()));
        } else if let Some(item) = parse_ordered(stripped) {
            flush_paragraph(&mut blocks, &mut paragraph);
            blocks.push(text_block(13, "ordered", item.trim()));
        } else {
            paragraph.push(stripped.to_string());
        }
    }

    if in_code && !code.is_empty() {
        blocks.push(code_block(&code.join("\n"), code_language.as_deref()));
    }
    flush_paragraph(&mut blocks, &mut paragraph);
    blocks
}

fn flush_paragraph(blocks: &mut Vec<Value>, paragraph: &mut Vec<String>) {
    if paragraph.is_empty() {
        return;
    }
    blocks.push(text_block(2, "text", &paragraph.join("\n")));
    paragraph.clear();
}

fn text_block(block_type: i64, field: &str, content: &str) -> Value {
    text_block_with_style(block_type, field, content, json!({}))
}

fn text_block_with_style(block_type: i64, field: &str, content: &str, style: Value) -> Value {
    let mut block = Map::new();
    block.insert("block_type".to_string(), Value::Number(block_type.into()));
    block.insert(
        field.to_string(),
        json!({
            "elements": [{
                "text_run": {
                    "content": content,
                    "text_element_style": {}
                }
            }],
            "style": style
        }),
    );
    Value::Object(block)
}

fn code_block(content: &str, language: Option<&str>) -> Value {
    let mut style = Map::new();
    style.insert("wrap".to_string(), Value::Bool(true));
    if let Some(language) = language.and_then(code_language_enum) {
        style.insert("language".to_string(), Value::Number(language.into()));
    }
    text_block_with_style(14, "code", content, Value::Object(style))
}

fn todo_block(content: &str, done: bool) -> Value {
    text_block_with_style(17, "todo", content, json!({ "done": done }))
}

fn divider_block() -> Value {
    json!({
        "block_type": 22,
        "divider": {}
    })
}

fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let count = line.chars().take_while(|c| *c == '#').count();
    if count == 0 || count > 9 {
        return None;
    }
    let rest = &line[count..];
    if rest.starts_with(' ') {
        Some((count, rest))
    } else {
        None
    }
}

fn parse_quote(line: &str) -> Option<&str> {
    line.strip_prefix("> ")
        .or_else(|| line.strip_prefix(">"))
        .map(str::trim_start)
}

fn parse_todo(line: &str) -> Option<(bool, &str)> {
    for prefix in ["- [ ] ", "* [ ] ", "+ [ ] "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((false, rest));
        }
    }
    for prefix in ["- [x] ", "- [X] ", "* [x] ", "* [X] ", "+ [x] ", "+ [X] "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((true, rest));
        }
    }
    None
}

fn is_divider(line: &str) -> bool {
    matches!(line, "---" | "***" | "___")
}

fn parse_unordered(line: &str) -> Option<&str> {
    for prefix in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest);
        }
    }
    None
}

fn parse_ordered(line: &str) -> Option<&str> {
    let dot = line.find(". ")?;
    if dot == 0 || !line[..dot].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(&line[dot + 2..])
}

fn parse_code_fence_language(line: &str) -> Option<String> {
    let rest = line.trim_start_matches('`').trim();
    let language = rest
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|c: char| c == '{' || c == '}' || c == '.')
        .trim();
    if language.is_empty() {
        None
    } else {
        Some(language.to_ascii_lowercase())
    }
}

fn code_language_enum(language: &str) -> Option<i64> {
    let normalized = language
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase()
        .replace(['_', '-'], "");
    match normalized.as_str() {
        "text" | "txt" | "plain" | "plaintext" | "mermaid" | "mmd" => Some(1),
        "abap" => Some(2),
        "ada" => Some(3),
        "apache" => Some(4),
        "apex" => Some(5),
        "asm" | "assembly" => Some(6),
        "bash" => Some(7),
        "csharp" | "cs" => Some(8),
        "cpp" | "cplusplus" | "c++" | "cc" | "cxx" | "hpp" => Some(9),
        "c" | "h" => Some(10),
        "cobol" => Some(11),
        "css" => Some(12),
        "coffeescript" | "coffee" => Some(13),
        "d" => Some(14),
        "dart" => Some(15),
        "delphi" | "pascal" => Some(16),
        "django" => Some(17),
        "dockerfile" | "docker" => Some(18),
        "erlang" | "erl" => Some(19),
        "fortran" => Some(20),
        "foxpro" => Some(21),
        "go" | "golang" => Some(22),
        "groovy" => Some(23),
        "html" | "htm" => Some(24),
        "htmlbars" | "handlebars" | "hbs" => Some(25),
        "http" => Some(26),
        "haskell" | "hs" => Some(27),
        "json" | "jsonc" => Some(28),
        "java" => Some(29),
        "javascript" | "js" | "jsx" | "node" => Some(30),
        "julia" | "jl" => Some(31),
        "kotlin" | "kt" | "kts" => Some(32),
        "latex" | "tex" => Some(33),
        "lisp" | "clisp" | "commonlisp" | "elisp" => Some(34),
        "logo" => Some(35),
        "lua" => Some(36),
        "matlab" => Some(37),
        "makefile" | "make" => Some(38),
        "markdown" | "md" | "mdx" => Some(39),
        "nginx" => Some(40),
        "objectivec" | "objc" => Some(41),
        "openedgeabl" | "abl" => Some(42),
        "php" => Some(43),
        "perl" | "pl" => Some(44),
        "postscript" => Some(45),
        "powershell" | "pwsh" | "ps1" => Some(46),
        "prolog" => Some(47),
        "protobuf" | "proto" => Some(48),
        "python" | "py" | "python3" => Some(49),
        "r" => Some(50),
        "rpg" => Some(51),
        "ruby" | "rb" => Some(52),
        "rust" | "rs" => Some(53),
        "sas" => Some(54),
        "scss" => Some(55),
        "sql" => Some(56),
        "scala" => Some(57),
        "scheme" => Some(58),
        "scratch" => Some(59),
        "shell" | "sh" | "zsh" | "fish" => Some(60),
        "swift" => Some(61),
        "thrift" => Some(62),
        "typescript" | "ts" | "tsx" => Some(63),
        "vbscript" | "vbs" => Some(64),
        "visual" => Some(65),
        "xml" => Some(66),
        "yaml" | "yml" => Some(67),
        "cmake" => Some(68),
        "diff" | "patch" => Some(69),
        "gherkin" | "feature" | "cucumber" => Some(70),
        "graphql" | "gql" => Some(71),
        "glsl" | "opengl" => Some(72),
        "properties" | "ini" | "conf" => Some(73),
        "solidity" | "sol" => Some(74),
        "toml" => Some(75),
        _ => None,
    }
}

pub(super) fn print_doc_template(kind: DocTemplateKind) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&doc_template(kind))?);
    Ok(())
}

async fn insert_doc_media(api: &mut FeishuClient, args: DocInsertMediaArgs) -> Result<Value> {
    let parent_block_id = args
        .block_id
        .clone()
        .unwrap_or_else(|| args.document_id.clone());
    let placeholder = build_doc_media_placeholder(args.kind);
    let append_response = api
        .append_raw_children_at(
            &args.document_id,
            &parent_block_id,
            args.index,
            vec![placeholder],
        )
        .await?;
    let media_block_id = first_appended_block_id(&append_response).ok_or_else(|| {
        anyhow!(
            "doc insert-media append response did not include a child block_id: {append_response}"
        )
    })?;

    let file_name = drive_upload_file_name(&args.file, args.name)?;
    let parent_type = match args.kind {
        DocMediaKindArg::Image => "docx_image",
        DocMediaKindArg::File => "docx_file",
    };
    let extra = build_drive_media_extra(None, Some(args.document_id.clone()))?;
    let upload_response = api
        .upload_drive_media(
            &args.file,
            file_name.clone(),
            parent_type.to_string(),
            media_block_id.clone(),
            args.checksum,
            extra,
        )
        .await
        .with_context(|| {
            format!(
                "created {} placeholder block {media_block_id} in document {}, but media upload failed",
                doc_media_kind_label(args.kind),
                args.document_id
            )
        })?;
    let file_token = get_string(&upload_response, &["data", "file_token"]).ok_or_else(|| {
        anyhow!("doc insert-media upload response missing file_token: {upload_response}")
    })?;
    let patch_body = build_doc_media_replace_body(
        args.kind,
        &file_token,
        &file_name,
        args.width,
        args.height,
        args.align,
        args.view_type,
    );
    let patch_response = api
        .patch_document_block(&args.document_id, &media_block_id, patch_body.clone())
        .await
        .with_context(|| {
            format!(
                "uploaded media token {file_token} for block {media_block_id}, but document block patch failed"
            )
        })?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document_id": args.document_id,
            "parent_block_id": parent_block_id,
            "media_block_id": media_block_id,
            "kind": doc_media_kind_label(args.kind),
            "parent_type": parent_type,
            "file_name": file_name,
            "file_token": file_token,
            "append_response": append_response,
            "upload_response": upload_response,
            "patch_body": patch_body,
            "patch_response": patch_response
        }
    }))
}

pub(super) fn build_doc_media_placeholder(kind: DocMediaKindArg) -> Value {
    match kind {
        DocMediaKindArg::Image => json!({
            "block_type": 27,
            "image": {}
        }),
        DocMediaKindArg::File => json!({
            "block_type": 23,
            "file": {}
        }),
    }
}

pub(super) fn build_doc_media_replace_body(
    kind: DocMediaKindArg,
    file_token: &str,
    file_name: &str,
    width: Option<i64>,
    height: Option<i64>,
    align: Option<i64>,
    view_type: Option<i64>,
) -> Value {
    match kind {
        DocMediaKindArg::Image => {
            let mut body = Map::new();
            body.insert("token".to_string(), Value::String(file_token.to_string()));
            insert_opt_i64(&mut body, "width", width);
            insert_opt_i64(&mut body, "height", height);
            insert_opt_i64(&mut body, "align", align);
            json!({ "replace_image": Value::Object(body) })
        }
        DocMediaKindArg::File => {
            let mut body = Map::new();
            body.insert("token".to_string(), Value::String(file_token.to_string()));
            body.insert("name".to_string(), Value::String(file_name.to_string()));
            insert_opt_i64(&mut body, "view_type", view_type);
            json!({ "replace_file": Value::Object(body) })
        }
    }
}

pub(super) fn first_appended_block_id(value: &Value) -> Option<String> {
    value
        .get("data")
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
        .and_then(|children| children.first())
        .and_then(|child| child.get("block_id"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn doc_media_kind_label(kind: DocMediaKindArg) -> &'static str {
    match kind {
        DocMediaKindArg::Image => "image",
        DocMediaKindArg::File => "file",
    }
}

pub(super) fn doc_template(kind: DocTemplateKind) -> Value {
    match kind {
        DocTemplateKind::All => json!({
            "support-matrix": doc_template(DocTemplateKind::SupportMatrix),
            "text-child": doc_template(DocTemplateKind::TextChild),
            "heading-child": doc_template(DocTemplateKind::HeadingChild),
            "bullet-child": doc_template(DocTemplateKind::BulletChild),
            "ordered-child": doc_template(DocTemplateKind::OrderedChild),
            "todo-child": doc_template(DocTemplateKind::TodoChild),
            "quote-child": doc_template(DocTemplateKind::QuoteChild),
            "code-child": doc_template(DocTemplateKind::CodeChild),
            "mermaid-code-child": doc_template(DocTemplateKind::MermaidCodeChild),
            "divider-child": doc_template(DocTemplateKind::DividerChild),
            "image-child": doc_template(DocTemplateKind::ImageChild),
            "file-child": doc_template(DocTemplateKind::FileChild),
            "sheet-child": doc_template(DocTemplateKind::SheetChild),
            "bitable-child": doc_template(DocTemplateKind::BitableChild),
            "iframe-child": doc_template(DocTemplateKind::IframeChild),
            "chat-card-child": doc_template(DocTemplateKind::ChatCardChild),
            "isv-child": doc_template(DocTemplateKind::IsvChild),
            "add-ons-child": doc_template(DocTemplateKind::AddOnsChild),
            "jira-issue-child": doc_template(DocTemplateKind::JiraIssueChild),
            "board-child": doc_template(DocTemplateKind::BoardChild),
            "link-preview-child": doc_template(DocTemplateKind::LinkPreviewChild),
            "sub-page-list-child": doc_template(DocTemplateKind::SubPageListChild),
            "wiki-catalog-child": doc_template(DocTemplateKind::WikiCatalogChild),
            "table-descendant": doc_template(DocTemplateKind::TableDescendant),
            "grid-descendant": doc_template(DocTemplateKind::GridDescendant),
            "callout-descendant": doc_template(DocTemplateKind::CalloutDescendant),
            "quote-container-descendant": doc_template(DocTemplateKind::QuoteContainerDescendant),
            "agenda-descendant": doc_template(DocTemplateKind::AgendaDescendant),
        }),
        DocTemplateKind::SupportMatrix => doc_support_matrix(),
        DocTemplateKind::TextChild => child_body(vec![text_block(
            2,
            "text",
            "普通文本。可在 elements 内拆成多个 text_run 来做混合样式、链接、@用户、公式。",
        )]),
        DocTemplateKind::HeadingChild => child_body(vec![text_block(
            3,
            "heading1",
            "一级标题。heading1..heading9 对应 block_type 3..11。",
        )]),
        DocTemplateKind::BulletChild => child_body(vec![text_block(12, "bullet", "无序列表项")]),
        DocTemplateKind::OrderedChild => child_body(vec![text_block(13, "ordered", "有序列表项")]),
        DocTemplateKind::TodoChild => child_body(vec![todo_block("待办事项", false)]),
        DocTemplateKind::QuoteChild => child_body(vec![text_block(15, "quote", "引用内容")]),
        DocTemplateKind::CodeChild => child_body(vec![code_block(
            "fn main() {\n    println!(\"hello feishu\");\n}",
            Some("rust"),
        )]),
        DocTemplateKind::MermaidCodeChild => child_body(vec![code_block(
            "flowchart TD\n  A[AI] --> B[feishu-bot]\n  B --> C[Feishu docx code block]",
            Some("mermaid"),
        )]),
        DocTemplateKind::DividerChild => child_body(vec![divider_block()]),
        DocTemplateKind::ImageChild => child_body(vec![json!({
            "block_type": 27,
            "image": {
                "token": "<image_token_from_docx_upload>",
                "width": 640,
                "height": 360,
                "align": 2,
                "caption": {
                    "content": "图片说明"
                }
            }
        })]),
        DocTemplateKind::FileChild => child_body(vec![json!({
            "block_type": 23,
            "file": {
                "token": "<file_token_from_docx_upload>",
                "name": "example.pdf",
                "view_type": 1
            }
        })]),
        DocTemplateKind::SheetChild => child_body(vec![json!({
            "block_type": 30,
            "sheet": {
                "row_size": 5,
                "column_size": 3
            }
        })]),
        DocTemplateKind::BitableChild => child_body(vec![json!({
            "block_type": 18,
            "bitable": {
                "view_type": 1
            }
        })]),
        DocTemplateKind::BoardChild => child_body(vec![json!({
            "block_type": 43,
            "board": {
                "align": 1,
                "width": 900,
                "height": 500
            }
        })]),
        DocTemplateKind::IframeChild => child_body(vec![json!({
            "block_type": 26,
            "iframe": {
                "component": {
                    "type": 11,
                    "url": "https%3A%2F%2Fcodepen.io%2F"
                }
            }
        })]),
        DocTemplateKind::ChatCardChild => child_body(vec![json!({
            "block_type": 20,
            "chat_card": {
                "chat_id": "oc_xxx",
                "align": 1
            }
        })]),
        DocTemplateKind::IsvChild => child_body(vec![json!({
            "block_type": 28,
            "isv": {
                "component_id": "<component_id>",
                "component_type_id": "<component_type_id>"
            }
        })]),
        DocTemplateKind::AddOnsChild => child_body(vec![json!({
            "block_type": 40,
            "add_ons": {
                "component_type_id": "<component_type_id>",
                "record": "{\"key\":\"value\"}"
            }
        })]),
        DocTemplateKind::JiraIssueChild => child_body(vec![json!({
            "block_type": 41,
            "jira_issue": {
                "id": "<jira_issue_id>",
                "key": "PROJ-123"
            }
        })]),
        DocTemplateKind::LinkPreviewChild => child_body(vec![json!({
            "block_type": 48,
            "link_preview": {
                "url": "<message_link_url_encoded>",
                "url_type": "MessageLink"
            }
        })]),
        DocTemplateKind::SubPageListChild => child_body(vec![json!({
            "block_type": 51,
            "sub_page_list": {
                "wiki_token": "<current_wiki_node_token>"
            }
        })]),
        DocTemplateKind::WikiCatalogChild => child_body(vec![json!({
            "block_type": 42,
            "wiki_catalog": {
                "wiki_token": "<wiki_space_or_node_token>"
            }
        })]),
        DocTemplateKind::TableDescendant => json!({
            "index": -1,
            "children_id": ["heading_1", "table_1"],
            "descendants": [
                descendant_text_block("heading_1", 3, "heading1", "简单表格", vec![]),
                {
                    "block_id": "table_1",
                    "block_type": 31,
                    "table": {
                        "property": {
                            "row_size": 1,
                            "column_size": 2
                        }
                    },
                    "children": ["table_cell_1", "table_cell_2"]
                },
                {
                    "block_id": "table_cell_1",
                    "block_type": 32,
                    "table_cell": {},
                    "children": ["table_cell_1_text"]
                },
                {
                    "block_id": "table_cell_2",
                    "block_type": 32,
                    "table_cell": {},
                    "children": ["table_cell_2_text"]
                },
                descendant_text_block("table_cell_1_text", 2, "text", "左侧单元格", vec![]),
                descendant_text_block("table_cell_2_text", 2, "text", "右侧单元格", vec![])
            ]
        }),
        DocTemplateKind::GridDescendant => json!({
            "index": -1,
            "children_id": ["grid_1"],
            "descendants": [
                {
                    "block_id": "grid_1",
                    "block_type": 24,
                    "grid": {
                        "column_size": 2
                    },
                    "children": ["grid_col_1", "grid_col_2"]
                },
                {
                    "block_id": "grid_col_1",
                    "block_type": 25,
                    "grid_column": {
                        "width_ratio": 50
                    },
                    "children": ["grid_col_1_text"]
                },
                {
                    "block_id": "grid_col_2",
                    "block_type": 25,
                    "grid_column": {
                        "width_ratio": 50
                    },
                    "children": ["grid_col_2_text"]
                },
                descendant_text_block("grid_col_1_text", 2, "text", "左栏内容", vec![]),
                descendant_text_block("grid_col_2_text", 2, "text", "右栏内容", vec![])
            ]
        }),
        DocTemplateKind::CalloutDescendant => json!({
            "index": -1,
            "children_id": ["callout_1"],
            "descendants": [
                {
                    "block_id": "callout_1",
                    "block_type": 19,
                    "callout": {
                        "background_color": 5,
                        "border_color": 5,
                        "text_color": 7,
                        "emoji_id": "bulb"
                    },
                    "children": ["callout_1_text"]
                },
                descendant_text_block("callout_1_text", 2, "text", "高亮块内容", vec![])
            ]
        }),
        DocTemplateKind::QuoteContainerDescendant => json!({
            "index": -1,
            "children_id": ["quote_container_1"],
            "descendants": [
                {
                    "block_id": "quote_container_1",
                    "block_type": 34,
                    "quote_container": {},
                    "children": ["quote_container_text_1"]
                },
                descendant_text_block(
                    "quote_container_text_1",
                    2,
                    "text",
                    "引用容器内的内容",
                    vec![]
                )
            ]
        }),
        DocTemplateKind::AgendaDescendant => json!({
            "index": -1,
            "children_id": ["agenda_1"],
            "descendants": [
                {
                    "block_id": "agenda_1",
                    "block_type": 44,
                    "agenda": {},
                    "children": ["agenda_item_1"]
                },
                {
                    "block_id": "agenda_item_1",
                    "block_type": 45,
                    "agenda_item": {},
                    "children": ["agenda_item_title_1", "agenda_item_content_1"]
                },
                {
                    "block_id": "agenda_item_title_1",
                    "block_type": 46,
                    "agenda_item_title": {
                        "align": 1,
                        "elements": [{
                            "text_run": {
                                "content": "议题一",
                                "text_element_style": {}
                            }
                        }]
                    },
                    "children": []
                },
                {
                    "block_id": "agenda_item_content_1",
                    "block_type": 47,
                    "agenda_item_content": {},
                    "children": ["agenda_item_content_text_1"]
                },
                descendant_text_block(
                    "agenda_item_content_text_1",
                    2,
                    "text",
                    "议题内容和结论",
                    vec![]
                )
            ]
        }),
    }
}

fn doc_support_matrix() -> Value {
    json!({
        "mermaid": {
            "preserve_source": "doc template --kind mermaid-code-child, then doc append-json",
            "rendered_diagram": "doc template --kind board-child, doc append-json, doc blocks, then board import --syntax mermaid",
            "not_direct_docx": "diagram block has diagram_type but no Mermaid source field and is not writable through public docx OpenAPI"
        },
        "local_writer": [
            "heading1..heading9",
            "text",
            "bullet",
            "ordered",
            "quote",
            "todo",
            "divider",
            "code"
        ],
        "raw_child_templates": [
            "text-child",
            "heading-child",
            "bullet-child",
            "ordered-child",
            "todo-child",
            "quote-child",
            "code-child",
            "mermaid-code-child",
            "divider-child",
            "image-child",
            "file-child",
            "sheet-child",
            "bitable-child",
            "iframe-child",
            "chat-card-child",
            "isv-child",
            "add-ons-child",
            "jira-issue-child",
            "board-child",
            "link-preview-child",
            "sub-page-list-child",
            "wiki-catalog-child"
        ],
        "raw_descendant_templates": [
            "table-descendant",
            "grid-descendant",
            "callout-descendant",
            "quote-container-descendant",
            "agenda-descendant"
        ],
        "token_or_context_required": {
            "image": "requires an uploaded image token",
            "file": "requires an uploaded file token",
            "chat_card": "requires an oc_ chat_id and permissions",
            "isv/add_ons": "requires configured Feishu document component IDs",
            "link_preview": "currently only supports message links",
            "sub_page_list/wiki_catalog": "requires wiki context token",
            "board": "token is generated after insertion; inspect with doc blocks"
        },
        "not_writable_by_public_docx_openapi": {
            "page": "root block only",
            "diagram": "rendered flowchart/UML/Mermaid cannot be created by docx create-block API",
            "mindnote": "read placeholder only",
            "task": "read task_id only; use feishu-bot task commands to create tasks",
            "source_synced": "read-only",
            "reference_synced": "read-only",
            "ai_template": "read-only",
            "undefined": "read-only placeholder"
        },
        "requires_user_access_token_or_external_product": {
            "okr": "docx OKR insertion requires user_access_token; this CLI currently uses tenant_access_token",
            "okr_objective/okr_key_result/okr_progress": "children of an OKR block, not standalone AI-created blocks"
        }
    })
}

fn child_body(children: Vec<Value>) -> Value {
    json!({
        "index": -1,
        "children": children,
    })
}

fn descendant_text_block(
    block_id: &str,
    block_type: i64,
    field: &str,
    content: &str,
    children: Vec<&str>,
) -> Value {
    let mut block = text_block(block_type, field, content);
    if let Some(object) = block.as_object_mut() {
        object.insert("block_id".to_string(), Value::String(block_id.to_string()));
        object.insert(
            "children".to_string(),
            Value::Array(
                children
                    .into_iter()
                    .map(|child| Value::String(child.to_string()))
                    .collect(),
            ),
        );
    }
    block
}

fn parse_raw_children(text: &str) -> Result<Vec<Value>> {
    let value: Value = serde_json::from_str(text).context("parse raw children JSON")?;
    if let Some(children) = value.as_array() {
        return Ok(children.clone());
    }
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        return Ok(children.clone());
    }
    bail!("raw children JSON must be an array or an object with a children array")
}

pub(super) fn converted_to_descendant_body(converted: Value) -> Result<Value> {
    let data = converted
        .get("data")
        .ok_or_else(|| anyhow!("convert response missing data: {converted}"))?;
    if let Some(images) = data
        .get("block_id_to_image_urls")
        .and_then(Value::as_array)
        .filter(|items| !items.is_empty())
    {
        bail!(
            "official converter returned image URL mappings that this CLI cannot upload yet: {}",
            serde_json::to_string(images)?
        );
    }
    let children_id = data
        .get("first_level_block_ids")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| anyhow!("convert response missing first_level_block_ids"))?;
    let mut descendants = data
        .get("blocks")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| anyhow!("convert response missing blocks"))?;
    for block in &mut descendants {
        sanitize_descendant_block(block);
    }
    Ok(json!({
        "index": -1,
        "children_id": children_id,
        "descendants": descendants,
    }))
}

pub(super) fn ensure_descendant_defaults(body: &mut Value) -> Result<()> {
    let object = body
        .as_object_mut()
        .ok_or_else(|| anyhow!("descendant body must be a JSON object"))?;
    object
        .entry("index".to_string())
        .or_insert_with(|| Value::Number((-1).into()));
    let needs_children_id = object
        .get("children_id")
        .and_then(Value::as_array)
        .is_none_or(|children| children.is_empty());
    let descendants = object
        .get_mut("descendants")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("descendant body must contain descendants array"))?;
    let inferred_children_id = if needs_children_id {
        let ids = descendants
            .iter()
            .filter_map(|block| block.get("block_id").and_then(Value::as_str))
            .map(|id| Value::String(id.to_string()))
            .collect::<Vec<_>>();
        Some(ids)
    } else {
        None
    };
    for block in descendants {
        sanitize_descendant_block(block);
    }
    if let Some(ids) = inferred_children_id {
        object.insert("children_id".to_string(), Value::Array(ids));
    }
    Ok(())
}

fn sanitize_descendant_block(block: &mut Value) {
    if let Some(object) = block.as_object_mut() {
        object.remove("parent_id");
        object.remove("comment_ids");
        object
            .entry("children".to_string())
            .or_insert_with(|| Value::Array(Vec::new()));
    }
    remove_unsupported_descendant_fields(block);
}

fn remove_unsupported_descendant_fields(value: &mut Value) {
    match value {
        Value::Object(object) => {
            object.remove("merge_info");
            for child in object.values_mut() {
                remove_unsupported_descendant_fields(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                remove_unsupported_descendant_fields(item);
            }
        }
        _ => {}
    }
}

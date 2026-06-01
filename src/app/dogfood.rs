#![allow(clippy::too_many_arguments)]

use super::*;

mod probes;
mod read;

pub(super) use probes::*;
use read::verify_dogfood;

pub(super) async fn run_dogfood_command(
    api: &mut FeishuClient,
    command: DogfoodCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        DogfoodCommand::Publish(args) => publish_dogfood(api, args).await?,
        DogfoodCommand::Verify(args) => verify_dogfood(api, args).await?,
    };
    print_response(raw_json, "dogfood completed", data)
}

async fn publish_dogfood(api: &mut FeishuClient, args: DogfoodPublishArgs) -> Result<Value> {
    if args.no_wiki
        && (args.wiki || args.wiki_space_id.is_some() || args.wiki_parent_token.is_some())
    {
        bail!("dogfood publish cannot combine --no-wiki with --wiki, --wiki-space-id, or --wiki-parent-token");
    }
    let receiver = resolve_dogfood_receiver(args.to, api.config.default_user_id.as_deref())?;
    let receiver_type = args.to_type.resolve(&receiver).to_string();
    let content = read_content(args.content, args.file, args.stdin)?;

    let create_response = api
        .create_document(&args.title, args.folder_token.as_deref())
        .await?;
    let document_id = get_string(&create_response, &["data", "document", "document_id"])
        .or_else(|| get_string(&create_response, &["data", "document_id"]))
        .ok_or_else(|| {
            anyhow!("create document response did not include document_id: {create_response}")
        })?;
    let append_response = match args.writer {
        WriterArg::Local => {
            api.append_document(&document_id, &document_id, &content)
                .await?
        }
        WriterArg::Official => {
            api.append_converted_content(&document_id, &document_id, args.content_type, &content)
                .await?
        }
    };
    let url = api.document_url(&document_id);
    let raw_readback = probe_value(api.raw_document(&document_id).await);
    let raw_readback_markers = dogfood_readback_markers(&args.title, &content);
    let raw_contains_title = response_contains(&raw_readback, &args.title);
    let raw_contains_content = raw_readback_markers
        .iter()
        .all(|marker| response_contains(&raw_readback, marker));

    let wiki_target = dogfood_wiki_target(
        args.no_wiki,
        args.wiki,
        api.config.default_doc_create_wiki,
        args.wiki_space_id,
        api.config.default_wiki_space_id.clone(),
        args.wiki_parent_token,
        api.config.default_wiki_parent_node_token.clone(),
    )?;
    let mut wiki_move_error = None;
    let wiki_move = if let Some((space_id, parent_node_token)) = wiki_target {
        let path = format!(
            "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
            encode_path_segment(&space_id)
        );
        let body =
            build_doc_create_wiki_move_body(&document_id, parent_node_token, args.wiki_apply);
        match wiki_request_json(api, Method::POST, &path, &[], Some(body), args.wiki_auth).await {
            Ok(response) => Some(response),
            Err(error) => {
                wiki_move_error = Some(format!(
                    "created document {document_id} ({url}), but failed to move it into Wiki space {space_id}: {error:#}"
                ));
                None
            }
        }
    } else {
        None
    };

    let wiki_status = if wiki_move.is_some() {
        "Wiki move succeeded."
    } else if wiki_move_error.is_some() {
        "Wiki move failed; this is the fallback docx."
    } else {
        "Wiki move was not requested."
    };
    let message = format!("{}: {}\n{}\n{}", args.title, url, document_id, wiki_status);
    let sent_message = api
        .send_text(&receiver, &receiver_type, &message, None)
        .await?;
    let send_loop_check = probe_sent_text_message(api, &receiver, &sent_message, &message).await?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document": {
                "document_id": document_id,
                "title": args.title,
                "url": url,
            },
            "receiver": {
                "id": receiver,
                "id_type": receiver_type,
            },
            "closed_loop": {
                "document_created": true,
                "append_ok": true,
                "raw_readback_ok": raw_readback.get("ok").and_then(Value::as_bool).unwrap_or(false),
                "raw_contains_title": raw_contains_title,
                "raw_contains_content": raw_contains_content,
                "raw_readback_markers": raw_readback_markers,
                "send_loop": send_loop_check.get("closed_loop").cloned().unwrap_or(Value::Null),
            },
            "create_response": create_response,
            "append_response": append_response,
            "raw_readback": raw_readback,
            "wiki_move": wiki_move,
            "wiki_move_error": wiki_move_error,
            "sent_message": sent_message,
            "send_loop_check": send_loop_check,
        }
    }))
}

pub(super) fn resolve_dogfood_receiver(
    explicit: Option<String>,
    default_user_id: Option<&str>,
) -> Result<String> {
    explicit
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            default_user_id
                .filter(|value| !value.trim().is_empty())
                .map(ToString::to_string)
        })
        .ok_or_else(|| anyhow!("dogfood publish requires --to or FEISHU_USER_ID"))
}

pub(super) fn dogfood_readback_markers(title: &str, content: &str) -> Vec<String> {
    let mut markers = Vec::new();
    if !title.trim().is_empty() {
        markers.push(title.trim().to_string());
    }
    for line in content.lines() {
        let marker = normalize_dogfood_marker(line);
        if marker.chars().count() >= 6 && !markers.iter().any(|existing| existing == &marker) {
            markers.push(marker);
        }
        if markers.len() >= 4 {
            break;
        }
    }
    markers
}

fn normalize_dogfood_marker(line: &str) -> String {
    line.trim()
        .trim_start_matches('#')
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim_start_matches("> ")
        .trim_matches('`')
        .replace('`', "")
        .trim()
        .to_string()
}

pub(super) fn dogfood_wiki_target(
    no_wiki: bool,
    explicit_wiki: bool,
    default_doc_create_wiki: bool,
    explicit_space_id: Option<String>,
    default_space_id: Option<String>,
    explicit_parent_token: Option<String>,
    default_parent_token: Option<String>,
) -> Result<Option<(String, Option<String>)>> {
    if no_wiki {
        return Ok(None);
    }
    let space_id = explicit_space_id.or(default_space_id);
    let wants_wiki = explicit_wiki || default_doc_create_wiki || space_id.is_some();
    if !wants_wiki {
        return Ok(None);
    }
    let space_id = space_id.ok_or_else(|| {
        anyhow!("dogfood publish Wiki move requires --wiki-space-id or FEISHU_WIKI_SPACE_ID")
    })?;
    Ok(Some((
        space_id,
        explicit_parent_token.or(default_parent_token),
    )))
}

async fn run_dogfood_message_loop_probe(
    api: &mut FeishuClient,
    to: Option<String>,
    to_type: ReceiveIdTypeArg,
    include_response: bool,
) -> Value {
    let result = async {
        let receiver = resolve_dogfood_receiver(to, api.config.default_user_id.as_deref())?;
        let receiver_type = to_type.resolve(&receiver).to_string();
        let text = format!(
            "飞书Bot dogfood verify 消息闭环 {}",
            Local::now().to_rfc3339()
        );
        let sent = api
            .send_text(&receiver, &receiver_type, &text, None)
            .await?;
        probe_sent_text_message(api, &receiver, &sent, &text).await
    }
    .await;
    dogfood_probe_from_result(
        "message",
        "message.loop_check",
        "feishu-bot --json dogfood verify --send-loop-check",
        "POST /im/v1/messages + GET message/chat/readback",
        "im",
        probe_value(result),
        include_response,
        &api.config.app_id,
    )
}

async fn run_dogfood_write_probes(api: &mut FeishuClient, args: &DogfoodVerifyArgs) -> Vec<Value> {
    let mut probes = Vec::new();
    let stamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if dogfood_module_selected(&args.module, "doc", "doc.create_write_readback") {
        let result = async {
            let title = format!("飞书Bot verify doc {stamp}");
            let content = format!("# {title}\n\n- dogfood verify write probe\n");
            let created = api.create_document(&title, None).await?;
            let document_id = get_string(&created, &["data", "document", "document_id"])
                .or_else(|| get_string(&created, &["data", "document_id"]))
                .ok_or_else(|| {
                    anyhow!("create document response missing document_id: {created}")
                })?;
            let appended = api
                .append_converted_content(
                    &document_id,
                    &document_id,
                    ContentTypeArg::Markdown,
                    &content,
                )
                .await?;
            let readback = api.raw_document(&document_id).await?;
            Ok(json!({
                "created": created,
                "appended": appended,
                "document_id": document_id,
                "url": api.document_url(&document_id),
                "raw_contains_title": response_contains(&readback, &title),
                "raw_contains_content": response_contains(&readback, "dogfood verify write probe"),
                "readback": readback,
            }))
        }
        .await;
        probes.push(dogfood_probe_from_result(
            "doc",
            "doc.create_write_readback",
            "feishu-bot --json dogfood verify --write --module doc",
            "POST /docx/v1/documents + document block convert/write/read",
            "doc",
            probe_value(result),
            args.include_response,
            &api.config.app_id,
        ));
    }

    if dogfood_module_selected(&args.module, "base", "base.create") {
        let result = api
            .post_json(
                "/bitable/v1/apps",
                &[],
                json!({ "name": format!("飞书Bot verify base {stamp}") }),
            )
            .await;
        probes.push(dogfood_probe_from_result(
            "base",
            "base.create",
            "feishu-bot --json dogfood verify --write --module base",
            "POST /bitable/v1/apps",
            "base",
            probe_value(result),
            args.include_response,
            &api.config.app_id,
        ));
    }

    if dogfood_module_selected(&args.module, "board", "board.mermaid_import") {
        let result = async {
            let title = format!("飞书Bot verify board Mermaid {stamp}");
            let created = api.create_document(&title, None).await?;
            let document_id = get_string(&created, &["data", "document", "document_id"])
                .or_else(|| get_string(&created, &["data", "document_id"]))
                .ok_or_else(|| {
                    anyhow!("create document response missing document_id: {created}")
                })?;
            let append_response = api
                .append_raw_children_at(
                    &document_id,
                    &document_id,
                    -1,
                    vec![json!({
                        "block_type": 43,
                        "board": {
                            "align": 1,
                            "height": 500,
                            "width": 900
                        }
                    })],
                )
                .await
                .with_context(|| {
                    format!("created document {document_id}, but failed to append board block")
                })?;
            let blocks = api
                .get_document_blocks(&document_id, 500)
                .await
                .with_context(|| {
                    format!("appended board block in document {document_id}, but failed to read blocks")
                })?;
            let whiteboard_id = first_board_token(&append_response)
                .or_else(|| first_board_token(&blocks))
                .ok_or_else(|| {
                    anyhow!("document {document_id} board block did not expose board.token")
                })?;
            let mermaid = "flowchart TD\n  A[dogfood verify] --> B[Feishu Board]\n  B --> C[Rendered Mermaid]";
            let imported = api
                .import_board_syntax(
                    &whiteboard_id,
                    BoardSyntaxArg::Mermaid,
                    mermaid,
                    1,
                    0,
                    Some(Uuid::new_v4().to_string()),
                )
                .await
                .with_context(|| {
                    format!(
                        "created document {document_id} and board {whiteboard_id}, but Mermaid import failed"
                    )
                })?;
            Ok(json!({
                "created": created,
                "append_response": append_response,
                "blocks": blocks,
                "document_id": document_id,
                "url": api.document_url(&document_id),
                "whiteboard_id": whiteboard_id,
                "mermaid": mermaid,
                "imported": imported,
            }))
        }
        .await;
        probes.push(dogfood_probe_from_result(
            "board",
            "board.mermaid_import",
            "feishu-bot --json dogfood verify --write --module board --include-response",
            "POST /docx/v1/documents + POST /board/v1/whiteboards/:whiteboard_id/nodes/plantuml",
            "board",
            probe_value(result),
            args.include_response,
            &api.config.app_id,
        ));
    }

    if dogfood_module_selected(&args.module, "task", "task.create") {
        let result = api
            .post_json(
                "/task/v2/tasks",
                &[("user_id_type".to_string(), "open_id".to_string())],
                json!({ "summary": format!("飞书Bot verify task {stamp}") }),
            )
            .await;
        probes.push(dogfood_probe_from_result(
            "task",
            "task.create",
            "feishu-bot --json dogfood verify --write --module task",
            "POST /task/v2/tasks",
            "task",
            probe_value(result),
            args.include_response,
            &api.config.app_id,
        ));
    }

    if dogfood_module_selected(&args.module, "sheet", "sheet.create") {
        let result = api
            .post_json(
                "/sheets/v3/spreadsheets",
                &[],
                json!({ "title": format!("飞书Bot verify sheet {stamp}") }),
            )
            .await;
        probes.push(dogfood_probe_from_result(
            "sheet",
            "sheet.create",
            "feishu-bot --json dogfood verify --write --module sheet",
            "POST /sheets/v3/spreadsheets",
            "sheet",
            probe_value(result),
            args.include_response,
            &api.config.app_id,
        ));
    }

    probes
}

fn first_board_token(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => {
            if let Some(board) = map.get("board").and_then(Value::as_object) {
                if let Some(token) = board.get("token").and_then(Value::as_str) {
                    return Some(token.to_string());
                }
            }
            map.values().find_map(first_board_token)
        }
        Value::Array(items) => items.iter().find_map(first_board_token),
        _ => None,
    }
}

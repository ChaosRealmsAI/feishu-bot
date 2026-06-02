#![allow(clippy::too_many_arguments)]

use super::*;

mod probes;
mod publish;
mod read;
mod summary;

pub(super) use probes::*;
pub(super) use publish::*;
use read::verify_dogfood;
pub(super) use summary::*;

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

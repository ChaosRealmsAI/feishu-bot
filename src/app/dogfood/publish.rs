use super::*;

pub(in crate::app) async fn publish_dogfood(
    api: &mut FeishuClient,
    args: DogfoodPublishArgs,
) -> Result<Value> {
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

pub(in crate::app) fn resolve_dogfood_receiver(
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

pub(in crate::app) fn dogfood_readback_markers(title: &str, content: &str) -> Vec<String> {
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

pub(in crate::app) fn dogfood_wiki_target(
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

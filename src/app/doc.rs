use super::*;

mod descendants;
mod markdown;
mod media;
mod templates;

pub(in crate::app) use descendants::*;
pub(super) use markdown::*;
pub(super) use media::*;
pub(super) use templates::doc_template;

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

pub(super) fn print_doc_template(kind: DocTemplateKind) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&doc_template(kind))?);
    Ok(())
}

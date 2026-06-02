use super::*;

pub(in crate::app) fn drive_comment_ref_query(
    file_type: &str,
    user_id_type: UserIdTypeArg,
) -> Vec<(String, String)> {
    vec![
        ("file_type".to_string(), file_type.to_string()),
        (
            "user_id_type".to_string(),
            user_id_type.resolve(None).to_string(),
        ),
    ]
}

pub(in crate::app) fn drive_comment_list_query(
    args: &DriveCommentListArgs,
) -> Result<Vec<(String, String)>> {
    if args.page_size > 100 {
        bail!("drive comment list page_size cannot exceed 100");
    }
    let mut query = drive_comment_ref_query(&args.file_type, args.user_id_type);
    query.push(("page_size".to_string(), args.page_size.to_string()));
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    if let Some(value) = args.is_whole {
        query.push(("is_whole".to_string(), value.to_string()));
    }
    if let Some(value) = args.is_solved {
        query.push(("is_solved".to_string(), value.to_string()));
    }
    if args.need_reaction {
        query.push(("need_reaction".to_string(), "true".to_string()));
    }
    Ok(query)
}

pub(in crate::app) fn build_drive_comment_elements(
    text: Option<String>,
    docs_links: Vec<String>,
    mention_users: Vec<String>,
) -> Result<Vec<Value>> {
    let mut elements = Vec::new();
    if let Some(text) = text.filter(|value| !value.trim().is_empty()) {
        elements.push(json!({
            "type": "text_run",
            "text_run": { "text": text },
        }));
    }
    for url in docs_links
        .into_iter()
        .filter(|value| !value.trim().is_empty())
    {
        elements.push(json!({
            "type": "docs_link",
            "docs_link": { "url": url },
        }));
    }
    for user_id in mention_users
        .into_iter()
        .filter(|value| !value.trim().is_empty())
    {
        elements.push(json!({
            "type": "person",
            "person": { "user_id": user_id },
        }));
    }
    if elements.is_empty() {
        bail!("provide --text, --docs-link, --mention-user, or raw JSON body");
    }
    Ok(elements)
}

pub(in crate::app) fn build_drive_comment_content_body(
    text: Option<String>,
    docs_links: Vec<String>,
    mention_users: Vec<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    label: &str,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(read_json_value(body_json, file, stdin)?, label);
    }
    Ok(json!({
        "content": {
            "elements": build_drive_comment_elements(text, docs_links, mention_users)?,
        }
    }))
}

pub(in crate::app) fn build_drive_comment_create_body(
    args: DriveCommentCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "drive comment create body",
        );
    }
    Ok(json!({
        "reply_list": {
            "replies": [
                {
                    "content": {
                        "elements": build_drive_comment_elements(
                            args.text,
                            args.docs_links,
                            args.mention_users,
                        )?,
                    }
                }
            ]
        }
    }))
}

pub(in crate::app) fn build_drive_comment_reply_body(args: DriveCommentReplyArgs) -> Result<Value> {
    build_drive_comment_content_body(
        args.text,
        args.docs_links,
        args.mention_users,
        args.body_json,
        args.file,
        args.stdin,
        "drive comment reply body",
    )
}

pub(in crate::app) fn build_drive_comment_update_reply_body(
    args: DriveCommentUpdateReplyArgs,
) -> Result<Value> {
    build_drive_comment_content_body(
        args.text,
        args.docs_links,
        args.mention_users,
        args.body_json,
        args.file,
        args.stdin,
        "drive comment update-reply body",
    )
}

pub(in crate::app) fn build_drive_comment_batch_body(
    args: DriveCommentBatchGetArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "drive comment batch-query body",
        );
    }
    if args.comment_ids.is_empty() || args.comment_ids.len() > 100 {
        bail!("drive comment batch-get needs 1..=100 --comment-id values");
    }
    Ok(json!({
        "comment_ids": args.comment_ids,
        "need_reaction": args.need_reaction,
    }))
}

use super::*;

pub(in crate::app) fn drive_upload_file_name(
    path: &Path,
    override_name: Option<String>,
) -> Result<String> {
    if let Some(name) = override_name.filter(|value| !value.trim().is_empty()) {
        return Ok(name);
    }
    path.file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
        .ok_or_else(|| anyhow!("cannot infer upload file name from {}", path.display()))
}

pub(in crate::app) fn validate_drive_upload_size(size: u64) -> Result<()> {
    const MAX_UPLOAD_ALL_BYTES: u64 = 20 * 1024 * 1024;
    if size == 0 {
        bail!("drive upload file cannot be empty");
    }
    if size > MAX_UPLOAD_ALL_BYTES {
        bail!(
            "drive upload uses upload_all and supports files up to 20 MB; got {size} bytes. Use `feishu-bot drive upload-large` for larger files"
        );
    }
    Ok(())
}

pub(in crate::app) fn validate_drive_large_upload_size(size: u64) -> Result<()> {
    if size == 0 {
        bail!("drive upload-large file cannot be empty");
    }
    Ok(())
}

pub(in crate::app) fn build_drive_upload_prepare_body(
    file_name: String,
    parent_type: String,
    parent_node: String,
    size: u64,
) -> Result<Value> {
    validate_drive_large_upload_size(size)?;
    Ok(json!({
        "file_name": file_name,
        "parent_type": parent_type,
        "parent_node": parent_node,
        "size": size,
    }))
}

pub(in crate::app) async fn upload_large_drive_file(
    api: &mut FeishuClient,
    args: DriveUploadLargeArgs,
) -> Result<Value> {
    let metadata =
        fs::metadata(&args.file).with_context(|| format!("stat {}", args.file.display()))?;
    let size = metadata.len();
    let file_name = drive_upload_file_name(&args.file, args.name)?;
    let prepare_body = build_drive_upload_prepare_body(
        file_name.clone(),
        args.parent_type,
        args.folder_token,
        size,
    )?;
    let prepare = api
        .request_json_with_auth(
            Method::POST,
            "/drive/v1/files/upload_prepare",
            &[],
            Some(prepare_body),
            args.auth,
            &[],
        )
        .await?;
    let upload_id = get_string(&prepare, &["data", "upload_id"])
        .ok_or_else(|| anyhow!("drive upload_prepare response missing upload_id: {prepare}"))?;
    let block_size = get_i64(&prepare, &["data", "block_size"])
        .ok_or_else(|| anyhow!("drive upload_prepare response missing block_size: {prepare}"))?;
    let block_num = get_i64(&prepare, &["data", "block_num"])
        .ok_or_else(|| anyhow!("drive upload_prepare response missing block_num: {prepare}"))?;
    if block_size <= 0 || block_num <= 0 {
        bail!("drive upload_prepare returned invalid block_size/block_num: {prepare}");
    }

    let mut file =
        fs::File::open(&args.file).with_context(|| format!("open {}", args.file.display()))?;
    let mut parts_uploaded = 0_i64;
    let mut buffer = vec![0_u8; block_size as usize];
    for seq in 0..block_num {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("read part {seq} from {}", args.file.display()))?;
        if read == 0 {
            bail!("drive upload-large ended before uploading declared block {seq}/{block_num}");
        }
        api.upload_drive_file_part(&upload_id, seq, &buffer[..read], &file_name, args.auth)
            .await
            .with_context(|| format!("upload drive file part {seq}/{block_num}"))?;
        parts_uploaded += 1;
    }

    let finish = api
        .request_json_with_auth(
            Method::POST,
            "/drive/v1/files/upload_finish",
            &[],
            Some(json!({
                "upload_id": upload_id,
                "block_num": block_num,
            })),
            args.auth,
            &[],
        )
        .await?;
    let file_token = get_string(&finish, &["data", "file_token"]);
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "file_name": file_name,
            "size": size,
            "upload_id": upload_id,
            "block_size": block_size,
            "block_num": block_num,
            "parts_uploaded": parts_uploaded,
            "file_token": file_token,
            "prepare": prepare.get("data").cloned().unwrap_or(Value::Null),
            "finish": finish.get("data").cloned().unwrap_or(Value::Null),
        }
    }))
}

pub(in crate::app) fn validate_upload_size(size: u64, max_bytes: u64, label: &str) -> Result<()> {
    if size == 0 {
        bail!("{label} file cannot be empty");
    }
    if size > max_bytes {
        bail!(
            "{label} supports non-empty files up to {} MB; got {size} bytes",
            max_bytes / 1024 / 1024
        );
    }
    Ok(())
}

pub(in crate::app) fn build_drive_media_extra(
    raw_extra: Option<String>,
    drive_route_token: Option<String>,
) -> Result<Option<String>> {
    match (
        raw_extra.filter(|value| !value.trim().is_empty()),
        drive_route_token.filter(|value| !value.trim().is_empty()),
    ) {
        (Some(_), Some(_)) => bail!("use either --extra or --drive-route-token, not both"),
        (Some(extra), None) => Ok(Some(extra)),
        (None, Some(token)) => Ok(Some(json!({ "drive_route_token": token }).to_string())),
        (None, None) => Ok(None),
    }
}

pub(in crate::app) fn build_drive_import_task_body(
    file_token: String,
    file_extension: String,
    target_type: String,
    title: Option<String>,
    folder_token: String,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(
            read_json_value(body_json, file, stdin)?,
            "drive import create request body",
        );
    }
    let mut body = Map::new();
    body.insert("file_extension".to_string(), Value::String(file_extension));
    body.insert("file_token".to_string(), Value::String(file_token));
    body.insert("type".to_string(), Value::String(target_type));
    if let Some(title) = title.filter(|value| !value.trim().is_empty()) {
        body.insert("file_name".to_string(), Value::String(title));
    }
    body.insert(
        "point".to_string(),
        json!({
            "mount_type": 1,
            "mount_key": folder_token,
        }),
    );
    Ok(Value::Object(body))
}

pub(in crate::app) fn infer_upload_extension(
    path: &Path,
    uploaded_name: &str,
    override_extension: Option<String>,
) -> Result<String> {
    if let Some(extension) = override_extension.filter(|value| !value.trim().is_empty()) {
        return Ok(extension.trim().trim_start_matches('.').to_string());
    }
    Path::new(uploaded_name)
        .extension()
        .or_else(|| path.extension())
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().trim_start_matches('.').to_string())
        .ok_or_else(|| {
            anyhow!(
                "cannot infer file extension from {}; pass --file-extension",
                path.display()
            )
        })
}

pub(in crate::app) async fn wait_drive_import_task(
    api: &mut FeishuClient,
    ticket: &str,
    polls: u16,
    poll_interval_ms: u64,
) -> Result<Value> {
    let path = format!("/drive/v1/import_tasks/{ticket}");
    let mut last = api.get_json(&path, &[]).await?;
    for _ in 0..polls {
        let status = last
            .pointer("/data/result/job_status")
            .and_then(Value::as_i64);
        if !matches!(status, Some(1 | 2)) {
            return Ok(last);
        }
        std::thread::sleep(std::time::Duration::from_millis(poll_interval_ms));
        last = api.get_json(&path, &[]).await?;
    }
    Ok(last)
}

pub(in crate::app) fn build_drive_export_task_body(
    token: String,
    file_type: String,
    file_extension: String,
    sub_id: Option<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(
            read_json_value(body_json, file, stdin)?,
            "drive export create request body",
        );
    }
    let mut body = Map::new();
    body.insert("token".to_string(), Value::String(token));
    body.insert("type".to_string(), Value::String(file_type));
    body.insert("file_extension".to_string(), Value::String(file_extension));
    insert_opt_string(&mut body, "sub_id", sub_id);
    Ok(Value::Object(body))
}

pub(in crate::app) async fn wait_drive_export_task(
    api: &mut FeishuClient,
    ticket: &str,
    token: &str,
    auth: ApiAuthArg,
    polls: u16,
    poll_interval_ms: u64,
) -> Result<Value> {
    let path = format!("/drive/v1/export_tasks/{ticket}");
    let query = vec![("token".to_string(), token.to_string())];
    let mut last = api
        .request_json_with_auth(Method::GET, &path, &query, None, auth, &[])
        .await?;
    for _ in 0..polls {
        let status = last
            .pointer("/data/result/job_status")
            .and_then(Value::as_i64);
        if !matches!(status, Some(1 | 2)) {
            return Ok(last);
        }
        std::thread::sleep(std::time::Duration::from_millis(poll_interval_ms));
        last = api
            .request_json_with_auth(Method::GET, &path, &query, None, auth, &[])
            .await?;
    }
    Ok(last)
}

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

pub(in crate::app) fn drive_version_query(
    obj_type: &str,
    user_id_type: UserIdTypeArg,
    include_obj_type: bool,
) -> Result<Vec<(String, String)>> {
    if !matches!(obj_type, "docx" | "sheet") {
        bail!("drive version obj-type must be docx or sheet");
    }
    let mut query = vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )];
    if include_obj_type {
        query.push(("obj_type".to_string(), obj_type.to_string()));
    }
    Ok(query)
}

pub(in crate::app) fn build_drive_version_create_body(
    args: DriveVersionCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "drive version create body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("drive version create needs --name or raw JSON body"))?;
    if !matches!(args.obj_type.as_str(), "docx" | "sheet") {
        bail!("drive version obj-type must be docx or sheet");
    }
    Ok(json!({
        "name": name,
        "obj_type": args.obj_type,
    }))
}

pub(in crate::app) fn build_drive_subscription_create_body(
    args: DriveSubscriptionCreateArgs,
) -> Value {
    let mut body = Map::new();
    body.insert("file_type".to_string(), Value::String(args.file_type));
    body.insert(
        "subscription_type".to_string(),
        Value::String(args.subscription_type),
    );
    insert_opt_string(&mut body, "subscription_id", args.subscription_id);
    if let Some(value) = args.is_subscribe {
        body.insert("is_subcribe".to_string(), Value::Bool(value));
    }
    Value::Object(body)
}

pub(in crate::app) fn drive_view_record_query(
    args: DriveViewRecordArgs,
) -> Result<(Vec<(String, String)>, ApiAuthArg)> {
    if !(1..=50).contains(&args.page_size) {
        bail!("drive view-record page_size must be 1..=50");
    }
    let mut query = vec![
        ("file_type".to_string(), args.file_type),
        ("page_size".to_string(), args.page_size.to_string()),
        (
            "viewer_id_type".to_string(),
            args.viewer_id_type.resolve(None).to_string(),
        ),
    ];
    push_query_opt(&mut query, "page_token", args.page_token);
    Ok((query, args.auth))
}

pub(in crate::app) fn write_output_file(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub(in crate::app) fn drive_permission_member_query(
    file_type: &str,
    need_notification: bool,
    member_type: Option<&str>,
) -> Vec<(String, String)> {
    let mut query = vec![("type".to_string(), file_type.to_string())];
    if need_notification {
        query.push(("need_notification".to_string(), "true".to_string()));
    }
    if let Some(member_type) = member_type.filter(|value| !value.trim().is_empty()) {
        query.push(("member_type".to_string(), member_type.to_string()));
    }
    query
}

pub(in crate::app) fn drive_permission_member_list_query(
    args: &DrivePermissionMemberListArgs,
) -> Result<Vec<(String, String)>> {
    if args.page_size == 0 || args.page_size > 200 {
        bail!("drive permission member-list page_size must be between 1 and 200");
    }
    let mut query = vec![
        ("type".to_string(), args.file_type.clone()),
        ("page_size".to_string(), args.page_size.to_string()),
    ];
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    push_query_opt(&mut query, "member_type", args.member_type.clone());
    Ok(query)
}

pub(in crate::app) fn build_drive_public_update_body(
    args: DrivePermissionPublicUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "public permission body",
        );
    }
    let mut body = Map::new();
    if let Some(value) = args.external_access {
        body.insert("external_access".to_string(), Value::Bool(value));
    }
    if let Some(value) = args.invite_external {
        body.insert("invite_external".to_string(), Value::Bool(value));
    }
    insert_opt_string(&mut body, "security_entity", args.security_entity);
    insert_opt_string(&mut body, "comment_entity", args.comment_entity);
    insert_opt_string(&mut body, "share_entity", args.share_entity);
    insert_opt_string(&mut body, "link_share_entity", args.link_share_entity);
    if body.is_empty() {
        bail!("provide public permission fields or raw JSON via --body-json/--file/--stdin");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_drive_member_add_body(
    args: DrivePermissionMemberAddArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member add body",
        );
    }
    Ok(json!({
        "member_type": args.member_type,
        "member_id": args.member_id,
        "perm": args.perm,
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

pub(in crate::app) fn build_drive_member_update_body(
    args: DrivePermissionMemberUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member update body",
        );
    }
    Ok(json!({
        "member_type": args.member_type,
        "perm": args.perm,
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

pub(in crate::app) fn build_drive_member_delete_body(
    args: DrivePermissionMemberDeleteArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member delete body",
        );
    }
    Ok(json!({
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

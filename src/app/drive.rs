#![allow(clippy::too_many_arguments)]

use super::*;
pub(super) async fn run_drive_command(
    api: &mut FeishuClient,
    command: DriveCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        DriveCommand::List(args) => {
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                ("order_by".to_string(), args.order_by),
                ("direction".to_string(), args.direction),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "folder_token", args.folder_token);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/drive/v1/files", &query).await?
        }
        DriveCommand::Folder(DriveFolderCommand::Create(args)) => {
            api.post_json(
                "/drive/v1/files/create_folder",
                &[],
                json!({
                    "name": args.name,
                    "folder_token": args.folder_token,
                }),
            )
            .await?
        }
        DriveCommand::Upload(args) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            api.upload_drive_file(
                &args.file,
                file_name,
                args.parent_type,
                args.folder_token,
                args.checksum,
            )
            .await?
        }
        DriveCommand::UploadLarge(args) => upload_large_drive_file(api, args).await?,
        DriveCommand::Media(DriveMediaCommand::Upload(args)) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            let parent_node = args.parent_node.unwrap_or_default();
            if args.parent_type != "ccm_import_open" && parent_node.trim().is_empty() {
                bail!(
                    "drive media upload needs --parent-node for {}; docx_image/docx_file use the target image/file block_id",
                    args.parent_type
                );
            }
            let extra = build_drive_media_extra(args.extra, args.drive_route_token)?;
            api.upload_drive_media(
                &args.file,
                file_name,
                args.parent_type,
                parent_node,
                args.checksum,
                extra,
            )
            .await?
        }
        DriveCommand::Media(DriveMediaCommand::Download(args)) => {
            let bytes = api
                .download_drive_media(
                    &args.file_token,
                    args.range.as_deref(),
                    args.extra.as_deref(),
                )
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len()
                }
            })
        }
        DriveCommand::Media(DriveMediaCommand::TmpUrl(args)) => {
            if args.file_tokens.is_empty() || args.file_tokens.len() > 5 {
                bail!("drive media tmp-url needs 1..=5 --file-token values");
            }
            let mut query = args
                .file_tokens
                .into_iter()
                .map(|token| ("file_tokens".to_string(), token))
                .collect::<Vec<_>>();
            push_query_opt(&mut query, "extra", args.extra);
            api.get_json("/drive/v1/medias/batch_get_tmp_download_url", &query)
                .await?
        }
        DriveCommand::Import(DriveImportCommand::Create(args)) => {
            let body = build_drive_import_task_body(
                args.file_token,
                args.file_extension,
                args.target_type,
                args.title,
                args.folder_token,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.post_json("/drive/v1/import_tasks", &[], body).await?
        }
        DriveCommand::Import(DriveImportCommand::Get(args)) => {
            let path = format!("/drive/v1/import_tasks/{}", args.ticket);
            api.get_json(&path, &[]).await?
        }
        DriveCommand::Import(DriveImportCommand::File(args)) => {
            let uploaded_name = drive_upload_file_name(&args.file, args.name)?;
            let file_extension =
                infer_upload_extension(&args.file, &uploaded_name, args.file_extension)?;
            let upload_extra = json!({
                "obj_type": args.target_type,
                "file_extension": file_extension,
            })
            .to_string();
            let uploaded = api
                .upload_drive_media(
                    &args.file,
                    uploaded_name.clone(),
                    "ccm_import_open".to_string(),
                    String::new(),
                    None,
                    Some(upload_extra),
                )
                .await?;
            let file_token = get_string(&uploaded, &["data", "file_token"]).ok_or_else(|| {
                anyhow!("drive media upload response missing file_token: {uploaded}")
            })?;
            let import_title = args.title.or(Some(uploaded_name));
            let task_body = build_drive_import_task_body(
                file_token.clone(),
                file_extension,
                args.target_type,
                import_title,
                args.folder_token,
                None,
                None,
                false,
            )?;
            let task = api
                .post_json("/drive/v1/import_tasks", &[], task_body)
                .await?;
            let ticket = get_string(&task, &["data", "ticket"])
                .ok_or_else(|| anyhow!("drive import create response missing ticket: {task}"))?;
            let result = if args.polls > 0 {
                Some(wait_drive_import_task(api, &ticket, args.polls, args.poll_interval_ms).await?)
            } else {
                None
            };
            let mut data = Map::new();
            data.insert("source_file_token".to_string(), Value::String(file_token));
            data.insert("ticket".to_string(), Value::String(ticket));
            data.insert(
                "upload".to_string(),
                uploaded.get("data").cloned().unwrap_or(Value::Null),
            );
            data.insert(
                "task".to_string(),
                task.get("data").cloned().unwrap_or(Value::Null),
            );
            if let Some(result) = result {
                data.insert(
                    "result".to_string(),
                    result
                        .pointer("/data/result")
                        .cloned()
                        .unwrap_or_else(|| result.get("data").cloned().unwrap_or(result)),
                );
            }
            json!({ "code": 0, "msg": "success", "data": data })
        }
        DriveCommand::Export(DriveExportCommand::Create(args)) => {
            let auth = args.auth;
            let body = build_drive_export_task_body(
                args.token,
                args.file_type,
                args.file_extension,
                args.sub_id,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.request_json_with_auth(
                Method::POST,
                "/drive/v1/export_tasks",
                &[],
                Some(body),
                auth,
                &[],
            )
            .await?
        }
        DriveCommand::Export(DriveExportCommand::Get(args)) => {
            let path = format!("/drive/v1/export_tasks/{}", args.ticket);
            let query = vec![("token".to_string(), args.token)];
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Export(DriveExportCommand::Download(args)) => {
            let path = format!("/drive/v1/export_tasks/file/{}/download", args.file_token);
            let bytes = api
                .request_binary_with_auth(Method::GET, &path, &[], args.auth, &[], None)
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len()
                }
            })
        }
        DriveCommand::Export(DriveExportCommand::File(args)) => {
            let body = build_drive_export_task_body(
                args.token.clone(),
                args.file_type,
                args.file_extension,
                args.sub_id,
                None,
                None,
                false,
            )?;
            let task = api
                .request_json_with_auth(
                    Method::POST,
                    "/drive/v1/export_tasks",
                    &[],
                    Some(body),
                    args.auth,
                    &[],
                )
                .await?;
            let ticket = get_string(&task, &["data", "ticket"])
                .ok_or_else(|| anyhow!("drive export create response missing ticket: {task}"))?;
            let result = wait_drive_export_task(
                api,
                &ticket,
                &args.token,
                args.auth,
                args.polls,
                args.poll_interval_ms,
            )
            .await?;
            let file_token = get_string(&result, &["data", "result", "file_token"])
                .ok_or_else(|| anyhow!("drive export result missing file_token: {result}"))?;
            let path = format!("/drive/v1/export_tasks/file/{file_token}/download");
            let bytes = api
                .request_binary_with_auth(Method::GET, &path, &[], args.auth, &[], None)
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ticket": ticket,
                    "file_token": file_token,
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len(),
                    "result": result.pointer("/data/result").cloned().unwrap_or(Value::Null)
                }
            })
        }
        DriveCommand::Comment(DriveCommentCommand::List(args)) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_list_query(&args)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::BatchGet(args)) => {
            let path = format!("/drive/v1/files/{}/comments/batch_query", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_batch_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Reply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_reply_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::UpdateReply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_update_reply_body(args)?;
            api.request_json_with_auth(Method::PUT, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::DeleteReply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = vec![("file_type".to_string(), args.file_type)];
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Resolve(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}",
                args.file_token, args.comment_id
            );
            let query = vec![("file_type".to_string(), args.file_type)];
            api.request_json_with_auth(
                Method::PATCH,
                &path,
                &query,
                Some(json!({ "is_solved": args.is_solved })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::Version(DriveVersionCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let query = drive_version_query(&args.obj_type, args.user_id_type, false)?;
            let auth = args.auth;
            let body = build_drive_version_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::List(args)) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let mut query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::Delete(args)) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/subscriptions", args.file_token);
            let auth = args.auth;
            let body = build_drive_subscription_create_body(args);
            api.request_json_with_auth(Method::POST, &path, &[], Some(body), auth, &[])
                .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/subscriptions/{}",
                args.file_token, args.subscription_id
            );
            api.request_json_with_auth(
                Method::GET,
                &path,
                &[],
                Some(json!({ "file_type": args.file_type })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Update(args)) => {
            let path = format!(
                "/drive/v1/files/{}/subscriptions/{}",
                args.file_token, args.subscription_id
            );
            api.request_json_with_auth(
                Method::PATCH,
                &path,
                &[],
                Some(json!({
                    "file_type": args.file_type,
                    "is_subscribe": args.is_subscribe,
                })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::ViewRecord(args) => {
            let path = format!("/drive/v1/files/{}/view_records", args.file_token);
            let query = drive_view_record_query(args)?;
            let auth = query.1;
            api.request_json_with_auth(Method::GET, &path, &query.0, None, auth, &[])
                .await?
        }
        DriveCommand::Download(args) => {
            let bytes = api
                .download_drive_file(&args.file_token, args.range.as_deref())
                .await?;
            write_output_file(&args.output, &bytes)?;
            json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "output": args.output.display().to_string(),
                    "bytes": bytes.len()
                }
            })
        }
        DriveCommand::Permission(DrivePermissionCommand::PublicGet(args)) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            api.get_json(&path, &[("type".to_string(), args.file_type)])
                .await?
        }
        DriveCommand::Permission(DrivePermissionCommand::PublicUpdate(args)) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            let query = vec![("type".to_string(), args.file_type.clone())];
            let body = build_drive_public_update_body(args)?;
            api.patch_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::PublicPasswordOff(args)) => {
            let path = format!("/drive/v1/permissions/{}/public/password", args.token);
            api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
                .await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberList(args)) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query = drive_permission_member_list_query(&args)?;
            api.get_json(&path, &query).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberAdd(args)) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_add_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberUpdate(args)) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_update_body(args)?;
            api.put_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberDelete(args)) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query = vec![
                ("type".to_string(), args.file_type.clone()),
                ("member_type".to_string(), args.member_type.clone()),
            ];
            let body = Some(build_drive_member_delete_body(args)?);
            api.delete_json(&path, &query, body).await?
        }
        DriveCommand::Stats(args) => {
            let path = format!("/drive/v1/files/{}/statistics", args.file_token);
            api.get_json(&path, &[("type".to_string(), args.file_type)])
                .await?
        }
        DriveCommand::Copy(args) => {
            let path = format!("/drive/v1/files/{}/copy", args.file_token);
            let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
                read_json_value(args.body_json, args.file, args.stdin)?
            } else {
                let mut body = Map::new();
                body.insert("type".to_string(), Value::String(args.file_type));
                body.insert("folder_token".to_string(), Value::String(args.folder_token));
                if let Some(name) = args.name {
                    body.insert("name".to_string(), Value::String(name));
                }
                Value::Object(body)
            };
            api.post_json(&path, &[], body).await?
        }
        DriveCommand::Move(args) => {
            let path = format!("/drive/v1/files/{}/move", args.file_token);
            let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
                read_json_value(args.body_json, args.file, args.stdin)?
            } else {
                json!({
                    "type": args.file_type,
                    "folder_token": args.folder_token,
                })
            };
            api.post_json(&path, &[], body).await?
        }
        DriveCommand::Delete(args) => {
            let path = format!("/drive/v1/files/{}", args.file_token);
            api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
                .await?
        }
    };
    print_response(raw_json, "drive operation completed", data)
}

pub(super) fn drive_upload_file_name(path: &Path, override_name: Option<String>) -> Result<String> {
    if let Some(name) = override_name.filter(|value| !value.trim().is_empty()) {
        return Ok(name);
    }
    path.file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
        .ok_or_else(|| anyhow!("cannot infer upload file name from {}", path.display()))
}

pub(super) fn validate_drive_upload_size(size: u64) -> Result<()> {
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

pub(super) fn validate_drive_large_upload_size(size: u64) -> Result<()> {
    if size == 0 {
        bail!("drive upload-large file cannot be empty");
    }
    Ok(())
}

pub(super) fn build_drive_upload_prepare_body(
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

pub(super) async fn upload_large_drive_file(
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

pub(super) fn validate_upload_size(size: u64, max_bytes: u64, label: &str) -> Result<()> {
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

pub(super) fn build_drive_media_extra(
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

pub(super) fn build_drive_import_task_body(
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

pub(super) fn infer_upload_extension(
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

pub(super) async fn wait_drive_import_task(
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

pub(super) fn build_drive_export_task_body(
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

pub(super) async fn wait_drive_export_task(
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

pub(super) fn drive_comment_ref_query(
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

pub(super) fn drive_comment_list_query(
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

pub(super) fn build_drive_comment_elements(
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

pub(super) fn build_drive_comment_content_body(
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

pub(super) fn build_drive_comment_create_body(args: DriveCommentCreateArgs) -> Result<Value> {
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

pub(super) fn build_drive_comment_reply_body(args: DriveCommentReplyArgs) -> Result<Value> {
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

pub(super) fn build_drive_comment_update_reply_body(
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

pub(super) fn build_drive_comment_batch_body(args: DriveCommentBatchGetArgs) -> Result<Value> {
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

pub(super) fn drive_version_query(
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

pub(super) fn build_drive_version_create_body(args: DriveVersionCreateArgs) -> Result<Value> {
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

pub(super) fn build_drive_subscription_create_body(args: DriveSubscriptionCreateArgs) -> Value {
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

pub(super) fn drive_view_record_query(
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

pub(super) fn write_output_file(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub(super) fn drive_permission_member_query(
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

pub(super) fn drive_permission_member_list_query(
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

pub(super) fn build_drive_public_update_body(
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

pub(super) fn build_drive_member_add_body(args: DrivePermissionMemberAddArgs) -> Result<Value> {
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

pub(super) fn build_drive_member_update_body(
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

pub(super) fn build_drive_member_delete_body(
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

use super::*;

pub(super) async fn run_drive_list_command(
    api: &mut FeishuClient,
    args: DriveListArgs,
) -> Result<Value> {
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
    api.get_json("/drive/v1/files", &query).await
}

pub(super) async fn run_drive_folder_command(
    api: &mut FeishuClient,
    command: DriveFolderCommand,
) -> Result<Value> {
    match command {
        DriveFolderCommand::Create(args) => {
            api.post_json(
                "/drive/v1/files/create_folder",
                &[],
                json!({
                    "name": args.name,
                    "folder_token": args.folder_token,
                }),
            )
            .await
        }
    }
}

pub(super) async fn run_drive_upload_command(
    api: &mut FeishuClient,
    args: DriveUploadArgs,
) -> Result<Value> {
    let file_name = drive_upload_file_name(&args.file, args.name)?;
    api.upload_drive_file(
        &args.file,
        file_name,
        args.parent_type,
        args.folder_token,
        args.checksum,
    )
    .await
}

pub(super) async fn run_drive_view_record_command(
    api: &mut FeishuClient,
    args: DriveViewRecordArgs,
) -> Result<Value> {
    let path = format!("/drive/v1/files/{}/view_records", args.file_token);
    let query = drive_view_record_query(args)?;
    let auth = query.1;
    api.request_json_with_auth(Method::GET, &path, &query.0, None, auth, &[])
        .await
}

pub(super) async fn run_drive_download_command(
    api: &mut FeishuClient,
    args: DriveDownloadArgs,
) -> Result<Value> {
    let bytes = api
        .download_drive_file(&args.file_token, args.range.as_deref())
        .await?;
    write_output_file(&args.output, &bytes)?;
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "output": args.output.display().to_string(),
            "bytes": bytes.len()
        }
    }))
}

pub(super) async fn run_drive_stats_command(
    api: &mut FeishuClient,
    args: DriveFileRefArgs,
) -> Result<Value> {
    let path = format!("/drive/v1/files/{}/statistics", args.file_token);
    api.get_json(&path, &[("type".to_string(), args.file_type)])
        .await
}

pub(super) async fn run_drive_copy_command(
    api: &mut FeishuClient,
    args: DriveCopyArgs,
) -> Result<Value> {
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
    api.post_json(&path, &[], body).await
}

pub(super) async fn run_drive_move_command(
    api: &mut FeishuClient,
    args: DriveMoveArgs,
) -> Result<Value> {
    let path = format!("/drive/v1/files/{}/move", args.file_token);
    let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
        read_json_value(args.body_json, args.file, args.stdin)?
    } else {
        json!({
            "type": args.file_type,
            "folder_token": args.folder_token,
        })
    };
    api.post_json(&path, &[], body).await
}

pub(super) async fn run_drive_delete_command(
    api: &mut FeishuClient,
    args: DriveFileRefArgs,
) -> Result<Value> {
    let path = format!("/drive/v1/files/{}", args.file_token);
    api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
        .await
}

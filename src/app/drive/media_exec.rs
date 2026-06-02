use super::*;

pub(super) async fn run_drive_media_command(
    api: &mut FeishuClient,
    command: DriveMediaCommand,
) -> Result<Value> {
    match command {
        DriveMediaCommand::Upload(args) => run_drive_media_upload(api, args).await,
        DriveMediaCommand::Download(args) => run_drive_media_download(api, args).await,
        DriveMediaCommand::TmpUrl(args) => run_drive_media_tmp_url(api, args).await,
    }
}

async fn run_drive_media_upload(
    api: &mut FeishuClient,
    args: DriveMediaUploadArgs,
) -> Result<Value> {
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
    .await
}

async fn run_drive_media_download(
    api: &mut FeishuClient,
    args: DriveMediaDownloadArgs,
) -> Result<Value> {
    let bytes = api
        .download_drive_media(
            &args.file_token,
            args.range.as_deref(),
            args.extra.as_deref(),
        )
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

async fn run_drive_media_tmp_url(
    api: &mut FeishuClient,
    args: DriveMediaTmpUrlArgs,
) -> Result<Value> {
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
        .await
}

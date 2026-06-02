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

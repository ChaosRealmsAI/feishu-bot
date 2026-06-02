use super::*;

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

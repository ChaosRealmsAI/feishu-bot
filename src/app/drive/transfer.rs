use super::*;

pub(super) async fn run_drive_import_command(
    api: &mut FeishuClient,
    command: DriveImportCommand,
) -> Result<Value> {
    match command {
        DriveImportCommand::Create(args) => {
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
            api.post_json("/drive/v1/import_tasks", &[], body).await
        }
        DriveImportCommand::Get(args) => {
            let path = format!("/drive/v1/import_tasks/{}", args.ticket);
            api.get_json(&path, &[]).await
        }
        DriveImportCommand::File(args) => import_drive_file(api, args).await,
    }
}

pub(super) async fn run_drive_export_command(
    api: &mut FeishuClient,
    command: DriveExportCommand,
) -> Result<Value> {
    match command {
        DriveExportCommand::Create(args) => {
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
            .await
        }
        DriveExportCommand::Get(args) => {
            let path = format!("/drive/v1/export_tasks/{}", args.ticket);
            let query = vec![("token".to_string(), args.token)];
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await
        }
        DriveExportCommand::Download(args) => {
            let path = format!("/drive/v1/export_tasks/file/{}/download", args.file_token);
            let bytes = api
                .request_binary_with_auth(Method::GET, &path, &[], args.auth, &[], None)
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
        DriveExportCommand::File(args) => export_drive_file(api, args).await,
    }
}

async fn import_drive_file(api: &mut FeishuClient, args: DriveImportFileArgs) -> Result<Value> {
    let uploaded_name = drive_upload_file_name(&args.file, args.name)?;
    let file_extension = infer_upload_extension(&args.file, &uploaded_name, args.file_extension)?;
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
    let file_token = get_string(&uploaded, &["data", "file_token"])
        .ok_or_else(|| anyhow!("drive media upload response missing file_token: {uploaded}"))?;
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
    Ok(json!({ "code": 0, "msg": "success", "data": data }))
}

async fn export_drive_file(api: &mut FeishuClient, args: DriveExportFileArgs) -> Result<Value> {
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
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "ticket": ticket,
            "file_token": file_token,
            "output": args.output.display().to_string(),
            "bytes": bytes.len(),
            "result": result.pointer("/data/result").cloned().unwrap_or(Value::Null)
        }
    }))
}

use super::*;

pub(super) async fn run_office_report(
    api: &mut FeishuClient,
    args: OfficeReportArgs,
) -> Result<Value> {
    if args.dry_run {
        return run_office_report_dry_run(args);
    }
    let content = read_content(args.content, args.file, args.stdin)?;
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let mut project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let mut next_actions = Vec::new();

    let report = if !args.no_wiki {
        if let Some(space_id) = project.wiki_space_id.as_deref() {
            let parent = project
                .wiki_index_node_token
                .clone()
                .or_else(|| project.wiki_parent_node_token.clone());
            match create_wiki_doc(
                api,
                space_id,
                parent.as_deref(),
                &args.title,
                args.content_type,
                &content,
                args.auth,
            )
            .await
            {
                Ok(created) => json!({
                    "route": "wiki",
                    "document_id": created.document_id,
                    "node_token": created.node_token,
                    "url": created.url,
                    "create_response": created.create_response,
                    "append_response": created.append_response,
                }),
                Err(error) => {
                    next_actions.push(format!(
                        "Wiki report creation failed; created a fallback standalone docx instead: {error:#}"
                    ));
                    let mut fallback = create_standalone_report_doc(
                        api,
                        &args.title,
                        args.content_type,
                        &content,
                        args.auth,
                    )
                    .await?;
                    fallback["route"] = Value::String("docx_fallback".to_string());
                    fallback["wiki_error"] = Value::String(format!("{error:#}"));
                    fallback
                }
            }
        } else {
            next_actions.push(
                "Project has no Wiki space. Run office bootstrap --space-id <space_id>, then rerun report without --no-wiki."
                    .to_string(),
            );
            create_standalone_report_doc(api, &args.title, args.content_type, &content, args.auth)
                .await?
        }
    } else {
        create_standalone_report_doc(api, &args.title, args.content_type, &content, args.auth)
            .await?
    };

    let url = report
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or("<no url returned>");
    let route = report
        .get("route")
        .and_then(Value::as_str)
        .unwrap_or("docx");
    let message_text = format!(
        "{}\n\n状态：报告已写入 {}\n链接：{}",
        args.title, route, url
    );
    let sent = api
        .send_text(chat_id, "chat_id", &message_text, None)
        .await
        .with_context(|| format!("send report notification to project chat {chat_id}"))?;
    let message_id = extract_message_id(&sent);
    let pin = if args.pin {
        pin_message(api, message_id.as_deref()).await
    } else {
        json!({ "status": "skipped" })
    };
    if args.pin {
        project.pinned_summary_message_id = message_id.clone();
    }
    let message_get = readback_message(api, message_id.as_deref()).await;
    let base_record = if args.base_record {
        append_project_base_record(
            api,
            &project,
            "report",
            &args.title,
            "done",
            url,
            truncate_chars(&content, 500).as_str(),
        )
        .await
    } else {
        json!({ "status": "skipped" })
    };
    if args.base_record
        && !base_record
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    {
        next_actions.push(
            "Base record write failed or is not configured. Check project base_app_token/base_table_id with office status --check."
                .to_string(),
        );
    }
    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "message_id": message_id,
            "document_id": report.get("document_id").cloned(),
            "url": url,
            "route": route,
            "report": report,
            "sent_message": sent,
            "pin": pin,
            "message_get": message_get,
            "base_record": base_record,
            "next_actions": next_actions,
        }
    }))
}

pub(super) async fn run_office_progress(
    api: &mut FeishuClient,
    args: OfficeProgressArgs,
) -> Result<Value> {
    let detail_content =
        read_optional_content(args.content, args.file, args.stdin)?.unwrap_or_default();
    let summary = args
        .summary
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            let fallback = truncate_chars(detail_content.trim(), 500);
            if fallback.is_empty() {
                args.status.clone()
            } else {
                fallback
            }
        });
    let mut registry = read_office_registry()?;
    let project_key = office_project_key(&args.project)?;
    let mut project = get_office_project(&registry, &project_key)?;
    let chat_id = required_project_field(project.chat_id.as_deref(), &project_key, "chat_id")?;
    let mut next_actions = Vec::new();

    let report = if args.wiki_report {
        let report_content = if detail_content.trim().is_empty() {
            format!(
                "# {}\n\n- 状态：{}\n- 摘要：{}\n",
                args.title, args.status, summary
            )
        } else {
            detail_content.clone()
        };
        if let Some(space_id) = project.wiki_space_id.as_deref() {
            let parent = project
                .wiki_index_node_token
                .clone()
                .or_else(|| project.wiki_parent_node_token.clone());
            match create_wiki_doc(
                api,
                space_id,
                parent.as_deref(),
                &args.title,
                args.content_type,
                &report_content,
                args.auth,
            )
            .await
            {
                Ok(created) => json!({
                    "route": "wiki",
                    "document_id": created.document_id,
                    "node_token": created.node_token,
                    "url": created.url,
                    "create_response": created.create_response,
                    "append_response": created.append_response,
                }),
                Err(error) => {
                    next_actions.push(format!(
                        "Wiki progress report creation failed; created a fallback standalone docx instead: {error:#}"
                    ));
                    let mut fallback = create_standalone_report_doc(
                        api,
                        &args.title,
                        args.content_type,
                        &report_content,
                        args.auth,
                    )
                    .await?;
                    fallback["route"] = Value::String("docx_fallback".to_string());
                    fallback["wiki_error"] = Value::String(format!("{error:#}"));
                    fallback
                }
            }
        } else {
            next_actions.push(
                "Project has no Wiki space. Progress message was sent, and the optional detail report used standalone docx fallback."
                    .to_string(),
            );
            create_standalone_report_doc(
                api,
                &args.title,
                args.content_type,
                &report_content,
                args.auth,
            )
            .await?
        }
    } else {
        json!({ "status": "skipped" })
    };

    let report_url = report.get("url").and_then(Value::as_str);
    let message_text = office_progress_message(&args.title, &args.status, &summary, report_url);
    let sent = api
        .send_text(chat_id, "chat_id", &message_text, None)
        .await
        .with_context(|| format!("send progress update to project chat {chat_id}"))?;
    let message_id = extract_message_id(&sent);
    let pin = if args.pin {
        pin_message(api, message_id.as_deref()).await
    } else {
        json!({ "status": "skipped" })
    };
    if args.pin {
        project.pinned_summary_message_id = message_id.clone();
    }
    let message_get = readback_message(api, message_id.as_deref()).await;
    let record_url = report_url
        .map(str::to_string)
        .or_else(|| message_id.as_ref().map(|id| format!("message:{id}")))
        .unwrap_or_default();
    let base_record = if args.no_base_record {
        json!({ "status": "skipped" })
    } else {
        append_project_base_record(
            api,
            &project,
            "progress",
            &args.title,
            &args.status,
            &record_url,
            &summary,
        )
        .await
    };
    if !args.no_base_record
        && !base_record
            .get("ok")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    {
        next_actions.push(
            "Base progress record write failed or is not configured. Check office status --check."
                .to_string(),
        );
    }
    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "chat_id": chat_id,
            "title": args.title,
            "status": args.status,
            "summary": summary,
            "message_id": message_id,
            "sent_message": sent,
            "pin": pin,
            "message_get": message_get,
            "report": report,
            "base_record": base_record,
            "next_actions": next_actions,
        }
    }))
}

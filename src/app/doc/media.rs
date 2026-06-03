use super::*;

pub(in crate::app) async fn insert_doc_media(
    api: &mut FeishuClient,
    args: DocInsertMediaArgs,
) -> Result<Value> {
    let parent_block_id = args
        .block_id
        .clone()
        .unwrap_or_else(|| args.document_id.clone());
    let placeholder = build_doc_media_placeholder(args.kind, args.view_type);
    let append_response = api
        .append_raw_children_at(
            &args.document_id,
            &parent_block_id,
            args.index,
            vec![placeholder],
        )
        .await?;
    let placeholder_block_id = first_appended_block_id(&append_response).ok_or_else(|| {
        anyhow!(
            "doc insert-media append response did not include a child block_id: {append_response}"
        )
    })?;
    let media_block_id = appended_media_target_block_id(args.kind, &append_response).ok_or_else(|| {
        anyhow!(
            "doc insert-media append response did not include a target {} block_id: {append_response}",
            doc_media_kind_label(args.kind)
        )
    })?;

    let file_name = drive_upload_file_name(&args.file, args.name)?;
    let parent_type = match args.kind {
        DocMediaKindArg::Image => "docx_image",
        DocMediaKindArg::File => "docx_file",
    };
    let extra = build_drive_media_extra(None, Some(args.document_id.clone()))?;
    let upload_response = api
        .upload_drive_media(
            &args.file,
            file_name.clone(),
            parent_type.to_string(),
            media_block_id.clone(),
            args.checksum,
            extra,
        )
        .await
        .with_context(|| {
            format!(
                "created {} placeholder block {placeholder_block_id} in document {}, but media upload failed for target block {media_block_id}",
                doc_media_kind_label(args.kind),
                args.document_id
            )
        })?;
    let file_token = get_string(&upload_response, &["data", "file_token"]).ok_or_else(|| {
        anyhow!("doc insert-media upload response missing file_token: {upload_response}")
    })?;
    let patch_body = build_doc_media_replace_body(
        args.kind,
        &file_token,
        &file_name,
        args.width,
        args.height,
        args.align,
        args.view_type,
    );
    let patch_response = api
        .patch_document_block(&args.document_id, &media_block_id, patch_body.clone())
        .await
        .with_context(|| {
            format!(
                "uploaded media token {file_token} for block {media_block_id}, but document block patch failed"
            )
        })?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "document_id": args.document_id,
            "parent_block_id": parent_block_id,
            "placeholder_block_id": placeholder_block_id,
            "media_block_id": media_block_id,
            "kind": doc_media_kind_label(args.kind),
            "parent_type": parent_type,
            "file_name": file_name,
            "file_token": file_token,
            "append_response": append_response,
            "upload_response": upload_response,
            "patch_body": patch_body,
            "patch_response": patch_response
        }
    }))
}

pub(in crate::app) fn build_doc_media_placeholder(
    kind: DocMediaKindArg,
    view_type: Option<i64>,
) -> Value {
    match kind {
        DocMediaKindArg::Image => json!({
            "block_type": 27,
            "image": {}
        }),
        DocMediaKindArg::File => {
            let mut file = Map::new();
            insert_opt_i64(&mut file, "view_type", view_type);
            json!({
                "block_type": 23,
                "file": file
            })
        }
    }
}

pub(in crate::app) fn build_doc_media_replace_body(
    kind: DocMediaKindArg,
    file_token: &str,
    file_name: &str,
    width: Option<i64>,
    height: Option<i64>,
    align: Option<i64>,
    view_type: Option<i64>,
) -> Value {
    match kind {
        DocMediaKindArg::Image => {
            let mut body = Map::new();
            body.insert("token".to_string(), Value::String(file_token.to_string()));
            insert_opt_i64(&mut body, "width", width);
            insert_opt_i64(&mut body, "height", height);
            insert_opt_i64(&mut body, "align", align);
            json!({ "replace_image": Value::Object(body) })
        }
        DocMediaKindArg::File => {
            let mut body = Map::new();
            body.insert("token".to_string(), Value::String(file_token.to_string()));
            body.insert("name".to_string(), Value::String(file_name.to_string()));
            insert_opt_i64(&mut body, "view_type", view_type);
            json!({ "replace_file": Value::Object(body) })
        }
    }
}

pub(in crate::app) fn first_appended_block_id(value: &Value) -> Option<String> {
    value
        .get("data")
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
        .and_then(|children| children.first())
        .and_then(|child| child.get("block_id"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

pub(in crate::app) fn appended_media_target_block_id(
    kind: DocMediaKindArg,
    value: &Value,
) -> Option<String> {
    let first_child = value
        .get("data")
        .and_then(|data| data.get("children"))
        .and_then(Value::as_array)
        .and_then(|children| children.first())?;
    match kind {
        DocMediaKindArg::Image => first_child
            .get("block_id")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        DocMediaKindArg::File => first_child
            .get("children")
            .and_then(Value::as_array)
            .and_then(|children| children.first())
            .and_then(Value::as_str)
            .or_else(|| first_child.get("block_id").and_then(Value::as_str))
            .map(ToString::to_string),
    }
}

fn doc_media_kind_label(kind: DocMediaKindArg) -> &'static str {
    match kind {
        DocMediaKindArg::Image => "image",
        DocMediaKindArg::File => "file",
    }
}

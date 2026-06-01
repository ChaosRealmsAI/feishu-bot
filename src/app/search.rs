use super::*;

pub(super) async fn run_search_command(
    api: &mut FeishuClient,
    command: SearchCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        SearchCommand::Docs(args) => {
            let body = build_search_docs_body(args)?;
            api.post_json_user("/search/v2/doc_wiki/search", &[], body)
                .await?
        }
        SearchCommand::Message(args) => {
            if args.page_size > 100 {
                bail!("message search page_size cannot exceed 100");
            }
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token.clone());
            let body = build_search_message_body(args)?;
            api.post_json_user("/search/v2/message", &query, body)
                .await?
        }
        SearchCommand::Source(SearchSourceCommand::List(args)) => {
            if args.page_size > 50 {
                bail!("search source list page_size cannot exceed 50");
            }
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            if let Some(view) = args.view {
                query.push(("view".to_string(), view.to_string()));
            }
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/search/v2/data_sources", &query).await?
        }
        SearchCommand::Source(SearchSourceCommand::Get(args)) => {
            let path = format!("/search/v2/data_sources/{}", args.data_source_id);
            api.get_json(&path, &[]).await?
        }
        SearchCommand::Source(SearchSourceCommand::Create(args)) => {
            let body = build_search_source_body(args, true)?;
            api.post_json("/search/v2/data_sources", &[], body).await?
        }
        SearchCommand::Source(SearchSourceCommand::Update(args)) => {
            let path = format!("/search/v2/data_sources/{}", args.data_source_id);
            let body = build_search_source_body(args.body, false)?;
            api.patch_json(&path, &[], body).await?
        }
        SearchCommand::Source(SearchSourceCommand::Delete(args)) => {
            let path = format!("/search/v2/data_sources/{}", args.data_source_id);
            api.delete_json(&path, &[], None).await?
        }
        SearchCommand::Schema(SearchSchemaCommand::Get(args)) => {
            let path = format!("/search/v2/schemas/{}", args.schema_id);
            api.get_json(&path, &[]).await?
        }
        SearchCommand::Schema(SearchSchemaCommand::Create(args)) => {
            let mut query = Vec::new();
            if args.validate_only {
                query.push(("validate_only".to_string(), "true".to_string()));
            }
            let body = ensure_json_object(
                read_json_value(args.body_json, args.file, args.stdin)?,
                "search schema body",
            )?;
            api.post_json("/search/v2/schemas", &query, body).await?
        }
        SearchCommand::Schema(SearchSchemaCommand::Update(args)) => {
            let path = format!("/search/v2/schemas/{}", args.schema_id);
            let body = ensure_json_object(
                read_json_value(args.body_json, args.file, args.stdin)?,
                "search schema update body",
            )?;
            api.patch_json(&path, &[], body).await?
        }
        SearchCommand::Schema(SearchSchemaCommand::Delete(args)) => {
            let path = format!("/search/v2/schemas/{}", args.schema_id);
            api.delete_json(&path, &[], None).await?
        }
        SearchCommand::Item(SearchItemCommand::Get(args)) => {
            let path = format!(
                "/search/v2/data_sources/{}/items/{}",
                args.data_source_id, args.item_id
            );
            api.get_json(&path, &[]).await?
        }
        SearchCommand::Item(SearchItemCommand::Create(args)) => {
            let path = format!("/search/v2/data_sources/{}/items", args.data_source_id);
            let body = build_search_item_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SearchCommand::Item(SearchItemCommand::BatchCreate(args)) => {
            let path = format!(
                "/search/v2/data_sources/{}/items/batch_create",
                args.data_source_id
            );
            let body = ensure_json_object(
                read_json_value(args.body_json, args.file, args.stdin)?,
                "search item batch body",
            )?;
            api.post_json(&path, &[], body).await?
        }
        SearchCommand::Item(SearchItemCommand::Delete(args)) => {
            let path = format!(
                "/search/v2/data_sources/{}/items/{}",
                args.data_source_id, args.item_id
            );
            api.delete_json(&path, &[], None).await?
        }
    };
    print_response(raw_json, "search operation completed", data)
}

fn build_time_range(start: Option<i64>, end: Option<i64>) -> Option<Value> {
    let mut object = Map::new();
    insert_opt_i64(&mut object, "start", start);
    insert_opt_i64(&mut object, "end", end);
    (!object.is_empty()).then_some(Value::Object(object))
}

pub(super) fn build_search_docs_body(args: SearchDocsArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "docs search body",
        );
    }
    if args.page_size > 20 {
        bail!("docs search page_size cannot exceed 20");
    }
    let query = args
        .query
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("docs search needs --query or raw JSON body"))?;

    let mut doc_filter = Map::new();
    insert_string_array(&mut doc_filter, "creator_ids", Vec::new());
    insert_string_array(&mut doc_filter, "doc_types", args.doc_types.clone());
    insert_string_array(&mut doc_filter, "folder_tokens", args.folder_tokens);
    if args.only_title {
        doc_filter.insert("only_title".to_string(), Value::Bool(true));
    }
    if let Some(sort_type) = args
        .sort_type
        .clone()
        .filter(|value| !value.trim().is_empty())
    {
        doc_filter.insert("sort_type".to_string(), Value::String(sort_type));
    }
    if let Some(range) = build_time_range(args.open_start, args.open_end) {
        doc_filter.insert("open_time".to_string(), range);
    }
    if let Some(range) = build_time_range(args.create_start, args.create_end) {
        doc_filter.insert("create_time".to_string(), range);
    }

    let mut wiki_filter = Map::new();
    insert_string_array(&mut wiki_filter, "doc_types", args.doc_types);
    insert_string_array(&mut wiki_filter, "space_ids", args.space_ids);
    if args.only_title {
        wiki_filter.insert("only_title".to_string(), Value::Bool(true));
    }
    if let Some(sort_type) = args.sort_type.filter(|value| !value.trim().is_empty()) {
        wiki_filter.insert("sort_type".to_string(), Value::String(sort_type));
    }
    if let Some(range) = build_time_range(args.open_start, args.open_end) {
        wiki_filter.insert("open_time".to_string(), range);
    }
    if let Some(range) = build_time_range(args.create_start, args.create_end) {
        wiki_filter.insert("create_time".to_string(), range);
    }

    let mut body = Map::new();
    body.insert("query".to_string(), Value::String(query));
    body.insert(
        "page_size".to_string(),
        Value::Number(args.page_size.into()),
    );
    insert_opt_string(&mut body, "page_token", args.page_token);
    if doc_filter.is_empty() && wiki_filter.is_empty() {
        body.insert("doc_filter".to_string(), Value::Object(Map::new()));
        body.insert("wiki_filter".to_string(), Value::Object(Map::new()));
    } else {
        body.insert("doc_filter".to_string(), Value::Object(doc_filter));
        if !wiki_filter.is_empty() {
            body.insert("wiki_filter".to_string(), Value::Object(wiki_filter));
        }
    }
    Ok(Value::Object(body))
}

pub(super) fn build_search_message_body(args: SearchMessageArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "message search body",
        );
    }
    let query = args
        .query
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("message search needs --query or raw JSON body"))?;
    let mut body = Map::new();
    body.insert("query".to_string(), Value::String(query));
    insert_string_array(&mut body, "from_ids", args.from_ids);
    insert_string_array(&mut body, "chat_ids", args.chat_ids);
    insert_string_array(&mut body, "at_chatter_ids", args.at_chatter_ids);
    insert_opt_string(&mut body, "message_type", args.message_type);
    insert_opt_string(&mut body, "from_type", args.from_type);
    insert_opt_string(&mut body, "chat_type", args.chat_type);
    insert_opt_string(&mut body, "start_time", args.start_time);
    insert_opt_string(&mut body, "end_time", args.end_time);
    Ok(Value::Object(body))
}

pub(super) fn build_search_source_body(
    args: SearchSourceWriteArgs,
    require_name: bool,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "search data source body",
        );
    }
    if require_name
        && args
            .name
            .as_ref()
            .filter(|value| !value.trim().is_empty())
            .is_none()
    {
        bail!("search source create needs --name or raw JSON body");
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "description", args.description);
    insert_opt_string(&mut body, "icon_url", args.icon_url);
    insert_opt_string(&mut body, "schema_id", args.schema_id);
    insert_opt_string(&mut body, "template", args.template);
    insert_opt_i64(&mut body, "state", args.state);
    Ok(Value::Object(body))
}

pub(super) fn build_search_item_body(args: SearchItemCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "search item body",
        );
    }
    let id = args
        .id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("search item create needs --id or raw JSON body"))?;
    let title = args
        .title
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("search item create needs --title or raw JSON body"))?;
    let url = args
        .url
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("search item create needs --url or raw JSON body"))?;

    let acl = if let Some(acl_json) = args.acl_json {
        ensure_json_array(
            parse_json_value(&acl_json, "search item ACL JSON")?,
            "search item ACL",
        )?
    } else {
        json!([{ "access": "allow", "value": "everyone", "type": "user" }])
    };
    let structured_data = if let Some(structured_json) = args.structured_json {
        let value = parse_json_value(&structured_json, "search item structured JSON")?;
        serde_json::to_string(&value).context("serialize structured_data")?
    } else {
        "{}".to_string()
    };

    let mut metadata = Map::new();
    metadata.insert("title".to_string(), Value::String(title));
    metadata.insert("source_url".to_string(), Value::String(url));
    insert_opt_string(&mut metadata, "source_url_mobile", args.mobile_url);

    let mut body = Map::new();
    body.insert("id".to_string(), Value::String(id));
    body.insert("acl".to_string(), acl);
    body.insert("metadata".to_string(), Value::Object(metadata));
    body.insert(
        "structured_data".to_string(),
        Value::String(structured_data),
    );
    if let Some(text) = args.text.filter(|value| !value.trim().is_empty()) {
        body.insert(
            "content".to_string(),
            json!({
                "format": args.content_format,
                "content_data": text
            }),
        );
    }
    Ok(Value::Object(body))
}

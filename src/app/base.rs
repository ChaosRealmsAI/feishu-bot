#![allow(clippy::too_many_arguments)]

use super::*;

mod media;
mod records;
mod reference;
mod schema;

pub(super) use media::*;
pub(super) use records::*;
pub(super) use reference::*;
pub(super) use schema::*;

pub(super) async fn run_base_command(
    api: &mut FeishuClient,
    command: BaseCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        BaseCommand::ParseUrl(args) => parse_base_reference(&args.url)?,
        BaseCommand::Create(args) => {
            let mut body = Map::new();
            if let Some(name) = args.name {
                body.insert("name".to_string(), Value::String(name));
            }
            if let Some(folder_token) = args.folder_token {
                body.insert("folder_token".to_string(), Value::String(folder_token));
            }
            if let Some(time_zone) = args.time_zone {
                body.insert("time_zone".to_string(), Value::String(time_zone));
            }
            api.post_json("/bitable/v1/apps", &[], Value::Object(body))
                .await?
        }
        BaseCommand::Get(args) => {
            let path = format!("/bitable/v1/apps/{}", args.app_token);
            api.get_json(&path, &[]).await?
        }
        BaseCommand::Update(args) => {
            let path = format!("/bitable/v1/apps/{}", args.app_token);
            let body = build_base_app_update_body(args)?;
            api.put_json(&path, &[], body).await?
        }
        BaseCommand::Copy(args) => {
            let path = format!("/bitable/v1/apps/{}/copy", args.app_token);
            let body = build_base_copy_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::Media(BaseMediaCommand::Upload(args)) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            api.upload_drive_media(
                &args.file,
                file_name,
                args.kind.parent_type().to_string(),
                args.app_token.clone(),
                args.checksum,
                build_drive_media_extra(None, Some(args.app_token))?,
            )
            .await?
        }
        BaseCommand::Media(BaseMediaCommand::Download(args)) => {
            let extra = build_base_media_extra(
                args.perm.extra,
                args.perm.table_id,
                args.perm.field_id,
                args.perm.record_id,
                std::slice::from_ref(&args.file_token),
            )?;
            let bytes = api
                .download_drive_media(&args.file_token, args.range.as_deref(), extra.as_deref())
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
        BaseCommand::Media(BaseMediaCommand::TmpUrl(args)) => {
            if args.file_tokens.is_empty() || args.file_tokens.len() > 5 {
                bail!("base media tmp-url needs 1..=5 --file-token values");
            }
            let extra = build_base_media_extra(
                args.perm.extra,
                args.perm.table_id,
                args.perm.field_id,
                args.perm.record_id,
                &args.file_tokens,
            )?;
            let mut query = args
                .file_tokens
                .into_iter()
                .map(|token| ("file_tokens".to_string(), token))
                .collect::<Vec<_>>();
            push_query_opt(&mut query, "extra", extra);
            api.get_json("/drive/v1/medias/batch_get_tmp_download_url", &query)
                .await?
        }
        BaseCommand::Media(BaseMediaCommand::FieldValue(args)) => {
            build_base_media_field_value(args.file_tokens, args.field)?
        }
        BaseCommand::Table(BaseTableCommand::List(args)) => {
            let path = format!("/bitable/v1/apps/{}/tables", args.app_token);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Table(BaseTableCommand::Create(args)) => {
            let path = format!("/bitable/v1/apps/{}/tables", args.app_token);
            let body = build_base_table_create_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::Table(BaseTableCommand::BatchCreate(args)) => {
            let path = format!("/bitable/v1/apps/{}/tables/batch_create", args.app_token);
            let user_id_type = args.user_id_type.resolve(None).to_string();
            let body = build_base_table_batch_create_body(args)?;
            api.post_json(&path, &[("user_id_type".to_string(), user_id_type)], body)
                .await?
        }
        BaseCommand::Table(BaseTableCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}",
                args.app_token, args.table_id
            );
            let body = build_base_table_update_body(args)?;
            api.patch_json(&path, &[], body).await?
        }
        BaseCommand::Table(BaseTableCommand::Delete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}",
                args.app_token, args.table_id
            );
            api.delete_json(&path, &[], None).await?
        }
        BaseCommand::Table(BaseTableCommand::BatchDelete(args)) => {
            let path = format!("/bitable/v1/apps/{}/tables/batch_delete", args.app_token);
            let table_ids =
                read_table_ids_json(args.table_ids, args.table_ids_json, args.file, args.stdin)?;
            api.post_json(&path, &[], json!({ "table_ids": table_ids }))
                .await?
        }
        BaseCommand::Field(BaseFieldCommand::List(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields",
                args.app_token, args.table_id
            );
            let query = build_base_field_list_query(&args);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Field(BaseFieldCommand::Create(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields",
                args.app_token, args.table_id
            );
            let mut query = Vec::new();
            push_query_opt(&mut query, "client_token", args.client_token.clone());
            let body = build_base_field_create_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        BaseCommand::Field(BaseFieldCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields/{}",
                args.app_token, args.table_id, args.field_id
            );
            let body = build_base_field_update_body(args)?;
            api.put_json(&path, &[], body).await?
        }
        BaseCommand::Field(BaseFieldCommand::Delete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields/{}",
                args.app_token, args.table_id, args.field_id
            );
            api.delete_json(&path, &[], None).await?
        }
        BaseCommand::View(BaseViewCommand::List(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views",
                args.app_token, args.table_id
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::View(BaseViewCommand::Create(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views",
                args.app_token, args.table_id
            );
            let body = build_base_view_create_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::View(BaseViewCommand::Get(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            api.get_json(&path, &[]).await?
        }
        BaseCommand::View(BaseViewCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            let body = build_base_view_update_body(args)?;
            api.patch_json(&path, &[], body).await?
        }
        BaseCommand::View(BaseViewCommand::Delete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            api.delete_json(&path, &[], None).await?
        }
        BaseCommand::Record(BaseRecordCommand::List(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records",
                args.app_token, args.table_id
            );
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "view_id", args.view_id);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Record(BaseRecordCommand::Search(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/search",
                args.app_token, args.table_id
            );
            let body = build_base_record_search_body(&args)?;
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.post_json(&path, &query, body).await?
        }
        BaseCommand::Record(BaseRecordCommand::Get(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/{}",
                args.app_token, args.table_id, args.record_id
            );
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        BaseCommand::Record(BaseRecordCommand::BatchGet(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/batch_get",
                args.app_token, args.table_id
            );
            let record_ids =
                read_record_ids_json(args.record_ids, args.record_ids_json, args.file, args.stdin)?;
            api.post_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
                json!({ "record_ids": record_ids }),
            )
            .await?
        }
        BaseCommand::Record(BaseRecordCommand::Create(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records",
                args.app_token, args.table_id
            );
            let mut fields = read_base_record_fields(
                args.fields,
                args.fields_json,
                args.fields_file,
                args.fields_stdin,
            )?;
            normalize_base_record_write_fields(api, &args.app_token, &args.table_id, &mut fields)
                .await?;
            let query = base_record_write_query(
                args.client_token,
                args.user_id_type,
                args.ignore_consistency_check,
            );
            api.post_json(&path, &query, json!({ "fields": fields }))
                .await?
        }
        BaseCommand::Record(BaseRecordCommand::BatchCreate(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/batch_create",
                args.app_token, args.table_id
            );
            let mut records = read_base_record_batch_records(
                args.record_fields,
                Vec::new(),
                args.records_json,
                args.records_file,
                args.records_stdin,
                false,
            )?;
            normalize_base_record_write_records(api, &args.app_token, &args.table_id, &mut records)
                .await?;
            let query = base_record_write_query(
                args.client_token,
                args.user_id_type,
                args.ignore_consistency_check,
            );
            api.post_json(&path, &query, json!({ "records": records }))
                .await?
        }
        BaseCommand::Record(BaseRecordCommand::BatchUpdate(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/batch_update",
                args.app_token, args.table_id
            );
            let mut records = read_base_record_batch_records(
                args.record_fields,
                args.record_ids,
                args.records_json,
                args.records_file,
                args.records_stdin,
                true,
            )?;
            normalize_base_record_write_records(api, &args.app_token, &args.table_id, &mut records)
                .await?;
            let query = base_record_write_query(
                args.client_token,
                args.user_id_type,
                args.ignore_consistency_check,
            );
            api.post_json(&path, &query, json!({ "records": records }))
                .await?
        }
        BaseCommand::Record(BaseRecordCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/{}",
                args.app_token, args.table_id, args.record_id
            );
            let mut fields = read_base_record_fields(
                args.fields,
                args.fields_json,
                args.fields_file,
                args.fields_stdin,
            )?;
            normalize_base_record_write_fields(api, &args.app_token, &args.table_id, &mut fields)
                .await?;
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            if args.ignore_consistency_check {
                query.push(("ignore_consistency_check".to_string(), "true".to_string()));
            }
            api.put_json(&path, &query, json!({ "fields": fields }))
                .await?
        }
        BaseCommand::Record(BaseRecordCommand::Delete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/{}",
                args.app_token, args.table_id, args.record_id
            );
            api.delete_json(&path, &[], None).await?
        }
        BaseCommand::Record(BaseRecordCommand::BatchDelete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/batch_delete",
                args.app_token, args.table_id
            );
            let records =
                read_record_ids_json(args.record_ids, args.records_json, args.file, args.stdin)?;
            api.post_json(&path, &[], json!({ "records": records }))
                .await?
        }
        BaseCommand::Dashboard(BaseDashboardCommand::List(args)) => {
            let path = format!("/bitable/v1/apps/{}/dashboards", args.app_token);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Dashboard(BaseDashboardCommand::Copy(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/dashboards/{}/copy",
                args.app_token, args.block_id
            );
            let body = build_base_dashboard_copy_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::Workflow(BaseWorkflowCommand::List(args)) => {
            let path = format!("/bitable/v1/apps/{}/workflows", args.app_token);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Workflow(BaseWorkflowCommand::BlockList(args)) => {
            let path = format!("/bitable/v1/apps/{}/block_workflows", args.app_token);
            api.get_json(&path, &[]).await?
        }
        BaseCommand::Workflow(BaseWorkflowCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/workflows/{}",
                args.app_token, args.workflow_id
            );
            let body = build_base_workflow_update_body(args)?;
            api.put_json(&path, &[], body).await?
        }
        BaseCommand::Form(BaseFormCommand::Get(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/forms/{}",
                args.app_token, args.table_id, args.form_id
            );
            api.get_json(&path, &[]).await?
        }
        BaseCommand::Form(BaseFormCommand::Update(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/forms/{}",
                args.app_token, args.table_id, args.form_id
            );
            let body = ensure_json_object(
                read_json_value(args.body_json, args.file, args.stdin)?,
                "form update body",
            )?;
            api.patch_json(&path, &[], body).await?
        }
        BaseCommand::Role(BaseRoleCommand::List(args)) => {
            let path = base_role_path(args.api_version, &args.app_token, None);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Role(BaseRoleCommand::Create(args)) => {
            let path = base_role_path(args.api_version, &args.app_token, None);
            let body = build_base_role_write_body(
                args.name,
                args.table_roles_json,
                args.block_roles_json,
                args.base_rule_json,
                args.allow_base_complex_edit,
                args.allow_copy,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::Role(BaseRoleCommand::Update(args)) => {
            let path = format!("/bitable/v1/apps/{}/roles/{}", args.app_token, args.role_id);
            let body = build_base_role_write_body(
                args.name,
                args.table_roles_json,
                args.block_roles_json,
                None,
                None,
                None,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.put_json(&path, &[], body).await?
        }
        BaseCommand::Role(BaseRoleCommand::Delete(args)) => {
            let path = format!("/bitable/v1/apps/{}/roles/{}", args.app_token, args.role_id);
            api.delete_json(&path, &[], None).await?
        }
        BaseCommand::Member(BaseMemberCommand::List(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members",
                args.app_token, args.role_id
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        BaseCommand::Member(BaseMemberCommand::Add(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members",
                args.app_token, args.role_id
            );
            let body =
                build_base_member_add_body(args.member_id, args.body_json, args.file, args.stdin)?;
            api.post_json(
                &path,
                &[("member_id_type".to_string(), args.member_id_type)],
                body,
            )
            .await?
        }
        BaseCommand::Member(BaseMemberCommand::Delete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members/{}",
                args.app_token, args.role_id, args.member_id
            );
            api.delete_json(
                &path,
                &[("member_id_type".to_string(), args.member_id_type)],
                None,
            )
            .await?
        }
        BaseCommand::Member(BaseMemberCommand::BatchAdd(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members/batch_create",
                args.app_token, args.role_id
            );
            let body = build_base_member_batch_body(
                args.members,
                args.member_list_json,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.post_json(&path, &[], body).await?
        }
        BaseCommand::Member(BaseMemberCommand::BatchDelete(args)) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members/batch_delete",
                args.app_token, args.role_id
            );
            let body = build_base_member_batch_body(
                args.members,
                args.member_list_json,
                args.body_json,
                args.file,
                args.stdin,
            )?;
            api.delete_json(&path, &[], Some(body)).await?
        }
    };
    print_response(raw_json, "base operation completed", data)
}

fn base_role_path(
    api_version: BaseRoleApiVersionArg,
    app_token: &str,
    role_id: Option<&str>,
) -> String {
    let base = match api_version {
        BaseRoleApiVersionArg::V1 => format!("/bitable/v1/apps/{app_token}/roles"),
        BaseRoleApiVersionArg::V2 => format!("/base/v2/apps/{app_token}/roles"),
    };
    if let Some(role_id) = role_id {
        format!("{base}/{role_id}")
    } else {
        base
    }
}

pub(super) fn build_base_field_list_query(args: &BaseFieldListArgs) -> Vec<(String, String)> {
    let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    push_query_opt(&mut query, "view_id", args.view_id.clone());
    if args.text_field_as_array {
        query.push(("text_field_as_array".to_string(), "true".to_string()));
    }
    query
}

pub(super) fn build_base_record_search_body(args: &BaseRecordSearchArgs) -> Result<Value> {
    let has_raw = has_json_input(&args.body_json, &args.file, args.stdin);
    let has_typed = args.view_id.is_some()
        || !args.field_names.is_empty()
        || args.field_names_json.is_some()
        || args.filter_json.is_some()
        || args.sort_json.is_some()
        || args.automatic_fields;
    if has_raw {
        if has_typed {
            bail!("base record search cannot combine raw body input with typed search flags");
        }
        return ensure_json_object(
            read_json_value(args.body_json.clone(), args.file.clone(), args.stdin)?,
            "record search body",
        );
    }

    let mut body = Map::new();
    insert_opt_string(&mut body, "view_id", args.view_id.clone());
    let field_names = base_record_search_field_names(args)?;
    insert_string_array(&mut body, "field_names", field_names);
    if let Some(filter_json) = args.filter_json.as_ref() {
        let filter = ensure_json_object(parse_json_value(filter_json, "filter-json")?, "filter")?;
        body.insert("filter".to_string(), filter);
    }
    if let Some(sort_json) = args.sort_json.as_ref() {
        let sort = ensure_json_array(parse_json_value(sort_json, "sort-json")?, "sort")?;
        body.insert("sort".to_string(), sort);
    }
    if args.automatic_fields {
        body.insert("automatic_fields".to_string(), Value::Bool(true));
    }
    Ok(Value::Object(body))
}

fn base_record_search_field_names(args: &BaseRecordSearchArgs) -> Result<Vec<String>> {
    let mut names = clean_string_values(args.field_names.clone());
    if let Some(field_names_json) = args.field_names_json.as_ref() {
        let value = parse_json_value(field_names_json, "field-names-json")?;
        let field_names = if let Some(field_names) = value.get("field_names") {
            field_names.clone()
        } else {
            value
        };
        let field_names = ensure_json_array(field_names, "field_names")?;
        let Some(items) = field_names.as_array() else {
            unreachable!("ensure_json_array returned a non-array");
        };
        for item in items {
            let name = item
                .as_str()
                .ok_or_else(|| anyhow!("field_names must contain only strings"))?;
            if !name.trim().is_empty() {
                names.push(name.to_string());
            }
        }
    }
    Ok(names)
}

pub(super) fn build_base_view_create_body(args: BaseViewCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "view create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("base view create needs --name or raw body"))?;
    Ok(json!({
        "view_name": name,
        "view_type": args.view_type,
    }))
}

pub(super) fn build_base_view_update_body(args: BaseViewUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "view update body",
        );
    }
    let mut body = Map::new();
    if let Some(name) = args.name {
        body.insert("view_name".to_string(), Value::String(name));
    }
    let property = build_base_view_property(
        args.property_json,
        args.hidden_field_ids,
        args.filter_conjunction,
        args.filter_conditions,
        args.filter_condition_omitted,
        args.hierarchy_field_id,
    )?;
    if let Some(property) = property {
        body.insert("property".to_string(), property);
    }
    if body.is_empty() {
        bail!("base view update needs --name, typed property flags, --property-json, or raw body");
    }
    Ok(Value::Object(body))
}

fn build_base_view_property(
    property_json: Option<String>,
    hidden_field_ids: Vec<String>,
    filter_conjunction: Option<String>,
    filter_conditions: Vec<String>,
    filter_condition_omitted: Option<bool>,
    hierarchy_field_id: Option<String>,
) -> Result<Option<Value>> {
    let mut property = match property_json {
        Some(property) => match ensure_json_object(
            parse_json_value(&property, "property-json")?,
            "view.property",
        )? {
            Value::Object(map) => map,
            _ => unreachable!("ensure_json_object returned a non-object"),
        },
        None => Map::new(),
    };

    let hidden_field_ids = clean_string_values(hidden_field_ids);
    if !hidden_field_ids.is_empty() {
        let mut hidden_fields = property
            .remove("hidden_fields")
            .map(|value| ensure_json_array(value, "view.property.hidden_fields"))
            .transpose()?
            .and_then(|value| value.as_array().cloned())
            .unwrap_or_default();
        for field_id in hidden_field_ids {
            if hidden_fields
                .iter()
                .any(|value| value.as_str() == Some(&field_id))
            {
                continue;
            }
            hidden_fields.push(Value::String(field_id));
        }
        property.insert("hidden_fields".to_string(), Value::Array(hidden_fields));
    }

    let has_filter = filter_conjunction.is_some()
        || !filter_conditions.is_empty()
        || filter_condition_omitted.is_some();
    if has_filter {
        let mut filter_info = property
            .remove("filter_info")
            .map(|value| ensure_json_object(value, "view.property.filter_info"))
            .transpose()?
            .and_then(|value| value.as_object().cloned())
            .unwrap_or_default();
        if let Some(conjunction) = filter_conjunction.filter(|value| !value.trim().is_empty()) {
            filter_info.insert("conjunction".to_string(), Value::String(conjunction));
        }
        if !filter_conditions.is_empty() {
            filter_info.insert(
                "conditions".to_string(),
                Value::Array(
                    filter_conditions
                        .into_iter()
                        .map(parse_base_view_filter_condition)
                        .collect::<Result<Vec<_>>>()?,
                ),
            );
        }
        if let Some(condition_omitted) = filter_condition_omitted {
            filter_info.insert(
                "condition_omitted".to_string(),
                Value::Bool(condition_omitted),
            );
        }
        property.insert("filter_info".to_string(), Value::Object(filter_info));
    }

    if let Some(field_id) = hierarchy_field_id.filter(|value| !value.trim().is_empty()) {
        let mut hierarchy_config = property
            .remove("hierarchy_config")
            .map(|value| ensure_json_object(value, "view.property.hierarchy_config"))
            .transpose()?
            .and_then(|value| value.as_object().cloned())
            .unwrap_or_default();
        hierarchy_config.insert("field_id".to_string(), Value::String(field_id));
        property.insert(
            "hierarchy_config".to_string(),
            Value::Object(hierarchy_config),
        );
    }

    if property.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Object(property)))
    }
}

fn parse_base_view_filter_condition(value: String) -> Result<Value> {
    let mut parts = value.splitn(4, ':');
    let field_id = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs field_id"))?;
    let field_type = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs field_type"))?;
    let operator = parts
        .next()
        .filter(|part| !part.trim().is_empty())
        .ok_or_else(|| anyhow!("base view --filter-condition needs operator"))?;
    let raw_filter_value = parts.next().ok_or_else(|| {
        anyhow!("base view --filter-condition must be field_id:field_type:operator:value")
    })?;
    let filter_value = if let Some(json_value) = raw_filter_value.strip_prefix("json:") {
        serde_json::to_string(&parse_json_value(
            json_value,
            "filter condition JSON value",
        )?)?
    } else {
        raw_filter_value.to_string()
    };
    Ok(json!({
        "field_id": field_id,
        "field_type": field_type,
        "operator": operator,
        "value": filter_value,
    }))
}

pub(super) fn build_base_role_write_body(
    name: Option<String>,
    table_roles_json: Option<String>,
    block_roles_json: Option<String>,
    base_rule_json: Option<String>,
    allow_base_complex_edit: Option<bool>,
    allow_copy: Option<bool>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(read_json_value(body_json, file, stdin)?, "base role body");
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "role_name", name);
    if let Some(table_roles) = table_roles_json {
        body.insert(
            "table_roles".to_string(),
            ensure_json_array(
                parse_json_value(&table_roles, "table-roles-json")?,
                "table_roles",
            )?,
        );
    }
    if let Some(block_roles) = block_roles_json {
        body.insert(
            "block_roles".to_string(),
            ensure_json_array(
                parse_json_value(&block_roles, "block-roles-json")?,
                "block_roles",
            )?,
        );
    }
    let base_rule = build_base_rule_body(base_rule_json, allow_base_complex_edit, allow_copy)?;
    if let Some(base_rule) = base_rule {
        body.insert("base_rule".to_string(), base_rule);
    }
    if body.is_empty() {
        bail!("base role write needs --name, --table-roles-json, --block-roles-json, --base-rule-json, --allow-base-complex-edit, --allow-copy, or raw body");
    }
    Ok(Value::Object(body))
}

fn build_base_rule_body(
    base_rule_json: Option<String>,
    allow_base_complex_edit: Option<bool>,
    allow_copy: Option<bool>,
) -> Result<Option<Value>> {
    if base_rule_json.is_none() && allow_base_complex_edit.is_none() && allow_copy.is_none() {
        return Ok(None);
    }
    let mut rule = match base_rule_json {
        Some(raw) => {
            match ensure_json_object(parse_json_value(&raw, "base-rule-json")?, "base_rule")? {
                Value::Object(map) => map,
                _ => unreachable!("ensure_json_object returned a non-object"),
            }
        }
        None => Map::new(),
    };
    if let Some(allow) = allow_base_complex_edit {
        rule.insert(
            "base_complex_edit".to_string(),
            json!(if allow { 1 } else { 0 }),
        );
    }
    if let Some(allow) = allow_copy {
        rule.insert("copy".to_string(), json!(if allow { 1 } else { 0 }));
    }
    Ok(Some(Value::Object(rule)))
}

pub(super) fn build_base_member_add_body(
    member_id: Option<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(read_json_value(body_json, file, stdin)?, "base member body");
    }
    let member_id = member_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("base member add needs --member-id or raw body"))?;
    Ok(json!({ "member_id": member_id }))
}

pub(super) fn build_base_member_batch_body(
    mut members: Vec<String>,
    member_list_json: Option<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(
            read_json_value(body_json, file, stdin)?,
            "base member batch body",
        );
    }
    if let Some(member_list_json) = member_list_json {
        let value = parse_json_value(&member_list_json, "member-list-json")?;
        if value.get("member_list").is_some() {
            return ensure_json_object(value, "base member batch body");
        }
        return Ok(json!({ "member_list": ensure_json_array(value, "member_list")? }));
    }
    members.retain(|member| !member.trim().is_empty());
    if members.is_empty() {
        bail!("base member batch needs --member type:id, --member-list-json, or raw body");
    }
    let member_list = members
        .into_iter()
        .map(|member| {
            let (member_type, member_id) = member
                .split_once(':')
                .ok_or_else(|| anyhow!("--member must use type:id, for example open_id:ou_xxx"))?;
            let member_type = member_type.trim();
            let member_id = member_id.trim();
            if member_type.is_empty() || member_id.is_empty() {
                bail!("--member must use non-empty type:id");
            }
            Ok(json!({ "type": member_type, "id": member_id }))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({ "member_list": member_list }))
}

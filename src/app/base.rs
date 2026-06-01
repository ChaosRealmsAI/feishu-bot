#![allow(clippy::too_many_arguments)]

use super::*;

mod media;
mod reference;

pub(super) use media::*;
pub(super) use reference::*;

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

fn base_record_write_query(
    client_token: Option<String>,
    user_id_type: UserIdTypeArg,
    ignore_consistency_check: bool,
) -> Vec<(String, String)> {
    let mut query = vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )];
    push_query_opt(&mut query, "client_token", client_token);
    if ignore_consistency_check {
        query.push(("ignore_consistency_check".to_string(), "true".to_string()));
    }
    query
}

pub(super) fn read_base_record_fields(
    fields: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    let mut output = match read_optional_json_value(text, file, stdin)? {
        Some(value) => match read_base_record_fields_value(value)? {
            Value::Object(map) => map,
            _ => unreachable!("read_base_record_fields_value returned a non-object"),
        },
        None => Map::new(),
    };
    for field in fields {
        let (key, value) = parse_base_record_field_pair(field)?;
        output.insert(key, value);
    }
    if output.is_empty() {
        bail!("base record needs --field, --fields-json, --fields-file, or --fields-stdin");
    }
    Ok(Value::Object(output))
}

fn read_base_record_fields_value(value: Value) -> Result<Value> {
    if let Some(fields) = value.get("fields") {
        return ensure_json_object(fields.clone(), "fields");
    }
    ensure_json_object(value, "fields")
}

fn parse_base_record_field_pair(value: String) -> Result<(String, Value)> {
    let (key, raw_value) = value
        .split_once('=')
        .ok_or_else(|| anyhow!("base record --field must be name=value, got {value}"))?;
    let key = key.trim();
    if key.is_empty() {
        bail!("base record --field key cannot be empty");
    }
    Ok((key.to_string(), parse_base_record_field_value(raw_value)?))
}

fn parse_base_record_field_value(value: &str) -> Result<Value> {
    if let Some(json_value) = value.strip_prefix("json:") {
        return parse_json_value(json_value, "base record field json value");
    }
    if let Some(string_value) = value.strip_prefix("str:") {
        return Ok(Value::String(string_value.to_string()));
    }
    if let Some(date_value) = value.strip_prefix("date:") {
        return Ok(json!(parse_base_record_date_millis(date_value)?));
    }
    if let Some(datetime_value) = value.strip_prefix("datetime:") {
        return Ok(json!(parse_base_record_datetime_millis(datetime_value)?));
    }
    match serde_json::from_str(value) {
        Ok(value) => Ok(value),
        Err(_) => Ok(Value::String(value.to_string())),
    }
}

async fn normalize_base_record_write_fields(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
    fields: &mut Value,
) -> Result<()> {
    if !base_fields_contain_date_like_string(fields) {
        return Ok(());
    }

    let Ok(date_fields) = load_base_date_field_names(api, app_token, table_id).await else {
        return Ok(());
    };
    let Some(fields) = fields.as_object_mut() else {
        return Ok(());
    };
    for (field_name, value) in fields {
        if !date_fields.contains(field_name) {
            continue;
        }
        let Some(text) = value.as_str() else {
            continue;
        };
        if let Some(timestamp) = maybe_parse_base_record_date_millis(text)? {
            *value = json!(timestamp);
        }
    }
    Ok(())
}

async fn normalize_base_record_write_records(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
    records: &mut Value,
) -> Result<()> {
    if !base_fields_contain_date_like_string(records) {
        return Ok(());
    }

    let Ok(date_fields) = load_base_date_field_names(api, app_token, table_id).await else {
        return Ok(());
    };
    let Some(records) = records.as_array_mut() else {
        return Ok(());
    };
    for record in records {
        let Some(fields) = record.get_mut("fields").and_then(Value::as_object_mut) else {
            continue;
        };
        for (field_name, value) in fields {
            if !date_fields.contains(field_name) {
                continue;
            }
            let Some(text) = value.as_str() else {
                continue;
            };
            if let Some(timestamp) = maybe_parse_base_record_date_millis(text)? {
                *value = json!(timestamp);
            }
        }
    }
    Ok(())
}

async fn load_base_date_field_names(
    api: &mut FeishuClient,
    app_token: &str,
    table_id: &str,
) -> Result<Vec<String>> {
    let path = format!("/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
    let mut page_token = None;
    let mut names = Vec::new();

    loop {
        let mut query = vec![("page_size".to_string(), "100".to_string())];
        push_query_opt(&mut query, "page_token", page_token);
        let response = api.get_json(&path, &query).await?;
        let data = response.get("data").unwrap_or(&Value::Null);
        if let Some(items) = data.get("items").and_then(Value::as_array) {
            for item in items {
                if !base_field_item_is_date(item) {
                    continue;
                }
                if let Some(name) = item.get("field_name").and_then(Value::as_str) {
                    names.push(name.to_string());
                }
            }
        }

        if !data
            .get("has_more")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            break;
        }
        let next = data
            .get("page_token")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if next.is_empty() {
            break;
        }
        page_token = Some(next);
    }

    Ok(names)
}

fn base_field_item_is_date(item: &Value) -> bool {
    item.get("type").and_then(Value::as_i64) == Some(5)
        || item
            .get("ui_type")
            .and_then(Value::as_str)
            .is_some_and(|ui_type| ui_type.eq_ignore_ascii_case("DateTime"))
}

fn base_fields_contain_date_like_string(value: &Value) -> bool {
    match value {
        Value::String(text) => maybe_parse_base_record_date_millis(text)
            .ok()
            .flatten()
            .is_some(),
        Value::Array(values) => values.iter().any(base_fields_contain_date_like_string),
        Value::Object(map) => map.values().any(base_fields_contain_date_like_string),
        _ => false,
    }
}

fn maybe_parse_base_record_date_millis(value: &str) -> Result<Option<i64>> {
    let value = value.trim();
    if value.is_empty() || value.chars().all(|char| char.is_ascii_digit()) {
        return Ok(None);
    }
    parse_base_record_datetime_millis(value)
        .map(Some)
        .or_else(|_| {
            parse_base_record_date_millis(value)
                .map(Some)
                .or_else(|_| Ok(None))
        })
}

fn parse_base_record_datetime_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    if value.chars().all(|char| char.is_ascii_digit()) {
        return value
            .parse::<i64>()
            .with_context(|| format!("parse base datetime milliseconds: {value}"));
    }
    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.timestamp_millis());
    }
    for format in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base datetime is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base datetime must be milliseconds, RFC3339, or local 'YYYY-MM-DD HH:MM[:SS]': {value}");
}

fn parse_base_record_date_millis(value: &str) -> Result<i64> {
    let value = value.trim();
    for format in ["%Y-%m-%d", "%Y/%m/%d"] {
        if let Ok(date) = NaiveDate::parse_from_str(value, format) {
            let naive = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("invalid base date: {value}"))?;
            let datetime = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| anyhow!("base date is ambiguous in local timezone: {value}"))?;
            return Ok(datetime.timestamp_millis());
        }
    }
    bail!("base date must be YYYY-MM-DD or YYYY/MM/DD: {value}");
}

pub(super) fn read_base_record_batch_records(
    record_fields: Vec<String>,
    record_ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    require_record_ids: bool,
) -> Result<Value> {
    let mut records = match read_optional_json_value(text, file, stdin)? {
        Some(value) => match read_records_value(value)? {
            Value::Array(values) => values,
            _ => unreachable!("read_records_value returned a non-array"),
        },
        None => Vec::new(),
    };

    for (index, record_id) in clean_string_values(record_ids).into_iter().enumerate() {
        ensure_record_object(&mut records, index)?;
        set_record_id(&mut records[index], record_id)?;
    }

    for field in record_fields {
        let (index, key, value) = parse_base_record_indexed_field_pair(field)?;
        ensure_record_object(&mut records, index)?;
        let fields = ensure_record_fields_object(&mut records[index])?;
        fields.insert(key, value);
    }

    if records.is_empty() {
        bail!("base record batch needs --record-field, --records-json, --records-file, or --records-stdin");
    }
    if require_record_ids {
        for (index, record) in records.iter().enumerate() {
            let Some(record_id) = record.get("record_id").and_then(Value::as_str) else {
                bail!("base record batch-update needs --record-id for record index {index}, or records_json entries with record_id");
            };
            if record_id.trim().is_empty() {
                bail!("base record batch-update record_id cannot be empty at index {index}");
            }
        }
    }
    Ok(Value::Array(records))
}

fn read_records_value(value: Value) -> Result<Value> {
    let records = if let Some(records) = value.get("records") {
        records.clone()
    } else {
        value
    };
    let array = records
        .as_array()
        .ok_or_else(|| anyhow!("records must be a JSON array or object with records array"))?;
    let normalized = array
        .iter()
        .map(|record| {
            if record.get("fields").is_some() {
                Ok(record.clone())
            } else {
                Ok(json!({ "fields": ensure_json_object(record.clone(), "record fields")? }))
            }
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Value::Array(normalized))
}

fn parse_base_record_indexed_field_pair(value: String) -> Result<(usize, String, Value)> {
    let (index, pair) = value.split_once(':').ok_or_else(|| {
        anyhow!("base record --record-field must be index:name=value, got {value}")
    })?;
    let index = index
        .trim()
        .parse::<usize>()
        .with_context(|| format!("parse record-field index in {value}"))?;
    let (key, field_value) = parse_base_record_field_pair(pair.to_string())?;
    Ok((index, key, field_value))
}

fn ensure_record_object(records: &mut Vec<Value>, index: usize) -> Result<()> {
    while records.len() <= index {
        records.push(json!({ "fields": {} }));
    }
    if !records[index].is_object() {
        bail!("record index {index} must be an object");
    }
    Ok(())
}

fn ensure_record_fields_object(record: &mut Value) -> Result<&mut Map<String, Value>> {
    let record = record
        .as_object_mut()
        .ok_or_else(|| anyhow!("record must be an object"))?;
    if record.get("fields").is_none() {
        record.insert("fields".to_string(), json!({}));
    }
    record
        .get_mut("fields")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| anyhow!("record.fields must be an object"))
}

fn set_record_id(record: &mut Value, record_id: String) -> Result<()> {
    let record = record
        .as_object_mut()
        .ok_or_else(|| anyhow!("record must be an object"))?;
    record.insert("record_id".to_string(), Value::String(record_id));
    Ok(())
}

pub(super) fn build_base_app_update_body(args: BaseAppUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base app update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if let Some(is_advanced) = args.is_advanced {
        body.insert("is_advanced".to_string(), Value::Bool(is_advanced));
    }
    if body.is_empty() {
        bail!("base update needs --name, --is-advanced, or raw body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_base_copy_body(args: BaseCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base copy body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "folder_token", args.folder_token);
    insert_opt_string(&mut body, "time_zone", args.time_zone);
    if let Some(without_content) = args.without_content {
        body.insert("without_content".to_string(), Value::Bool(without_content));
    }
    if body.is_empty() {
        bail!(
            "base copy needs --name, --folder-token, --without-content, --time-zone, or raw body"
        );
    }
    Ok(Value::Object(body))
}

pub(super) fn build_base_table_batch_create_body(args: BaseTableBatchCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "table batch_create body",
        );
    }
    if let Some(tables_json) = args.tables_json {
        let value = parse_json_value(&tables_json, "tables-json")?;
        if value.get("tables").is_some() {
            return ensure_json_object(value, "table batch_create body");
        }
        return Ok(json!({ "tables": ensure_json_array(value, "tables")? }));
    }
    let tables = args
        .name
        .into_iter()
        .filter(|name| !name.trim().is_empty())
        .map(|name| json!({ "name": name }))
        .collect::<Vec<_>>();
    if tables.is_empty() {
        bail!("table batch-create needs --name, --tables-json, or raw body");
    }
    Ok(json!({ "tables": tables }))
}

pub(super) fn build_base_table_create_body(args: BaseTableCreateArgs) -> Result<Value> {
    let mut table = Map::new();
    insert_opt_string(&mut table, "name", args.name);
    insert_opt_string(&mut table, "default_view_name", args.default_view_name);

    let mut fields = Vec::new();
    if let Some(value) =
        read_optional_json_value(args.fields_json, args.fields_file, args.fields_stdin)?
    {
        match ensure_json_array(value, "table.fields")? {
            Value::Array(items) => fields.extend(items),
            _ => unreachable!("ensure_json_array only returns arrays"),
        }
    }
    for spec in args.field_specs {
        fields.push(parse_base_table_field_spec(&spec)?);
    }
    if !fields.is_empty() {
        table.insert("fields".to_string(), Value::Array(fields));
    }

    Ok(json!({ "table": Value::Object(table) }))
}

pub(super) fn build_base_table_update_body(args: BaseTableUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base table update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    if body.is_empty() {
        bail!("base table update needs --name or raw body");
    }
    Ok(Value::Object(body))
}

pub(super) fn parse_base_table_field_spec(spec: &str) -> Result<Value> {
    let mut parts = spec.splitn(3, ':');
    let name = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("base table --field must be name:kind[:config]"))?;
    let kind_text = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("base table --field must be name:kind[:config]"))?;
    let config = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let kind = parse_base_field_kind(kind_text)?;

    let mut input = BaseFieldBuildInput {
        name: Some(name.to_string()),
        field_type: None,
        kind: Some(kind),
        property_json: None,
        description_json: None,
        ui_type: None,
        options: Vec::new(),
        formatter: None,
        currency_code: None,
        date_formatter: None,
        auto_fill: None,
        multiple: None,
        linked_table_id: None,
        formula: None,
        location_input_type: None,
        require_name_and_type: true,
    };

    if let Some(config) = config {
        if let Some(json) = config.strip_prefix("json:").or_else(|| {
            config
                .strip_prefix("property=")
                .or_else(|| config.strip_prefix("property-json="))
        }) {
            input.property_json = Some(json.to_string());
        } else if config.starts_with('{') {
            input.property_json = Some(config.to_string());
        } else {
            apply_base_table_field_config(&mut input, kind, config)?;
        }
    }

    build_base_field_body(input)
}

fn parse_base_field_kind(value: &str) -> Result<BaseFieldKindArg> {
    let normalized = value.trim().to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "text" => Ok(BaseFieldKindArg::Text),
        "barcode" => Ok(BaseFieldKindArg::Barcode),
        "email" => Ok(BaseFieldKindArg::Email),
        "number" => Ok(BaseFieldKindArg::Number),
        "progress" => Ok(BaseFieldKindArg::Progress),
        "currency" => Ok(BaseFieldKindArg::Currency),
        "rating" => Ok(BaseFieldKindArg::Rating),
        "single-select" | "select" => Ok(BaseFieldKindArg::SingleSelect),
        "multi-select" | "multiple-select" => Ok(BaseFieldKindArg::MultiSelect),
        "date" => Ok(BaseFieldKindArg::Date),
        "checkbox" => Ok(BaseFieldKindArg::Checkbox),
        "user" => Ok(BaseFieldKindArg::User),
        "phone" => Ok(BaseFieldKindArg::Phone),
        "url" => Ok(BaseFieldKindArg::Url),
        "attachment" | "file" => Ok(BaseFieldKindArg::Attachment),
        "link" => Ok(BaseFieldKindArg::Link),
        "formula" => Ok(BaseFieldKindArg::Formula),
        "duplex-link" => Ok(BaseFieldKindArg::DuplexLink),
        "location" => Ok(BaseFieldKindArg::Location),
        "group" => Ok(BaseFieldKindArg::Group),
        "auto-number" | "autonumber" => Ok(BaseFieldKindArg::AutoNumber),
        _ => bail!("unknown base table --field kind: {value}"),
    }
}

fn apply_base_table_field_config(
    input: &mut BaseFieldBuildInput,
    kind: BaseFieldKindArg,
    config: &str,
) -> Result<()> {
    match kind {
        BaseFieldKindArg::SingleSelect | BaseFieldKindArg::MultiSelect => {
            input.options = split_base_table_field_config(config)
                .into_iter()
                .map(str::to_string)
                .collect();
        }
        BaseFieldKindArg::Currency => {
            for (index, token) in split_base_table_field_config(config)
                .into_iter()
                .enumerate()
            {
                if let Some((key, value)) = token.split_once('=') {
                    match normalize_base_table_config_key(key).as_str() {
                        "formatter" | "format" => input.formatter = Some(value.to_string()),
                        "currency" | "currency-code" | "currencycode" => {
                            input.currency_code = Some(value.to_string())
                        }
                        _ => bail!("unknown currency --field config key: {key}"),
                    }
                } else if index == 0 {
                    input.formatter = Some(token.to_string());
                } else if index == 1 {
                    input.currency_code = Some(token.to_string());
                } else {
                    bail!("currency --field config supports formatter and currency_code only");
                }
            }
        }
        BaseFieldKindArg::Date => {
            for token in split_base_table_field_config(config) {
                if let Some((key, value)) = token.split_once('=') {
                    match normalize_base_table_config_key(key).as_str() {
                        "formatter" | "date-formatter" | "dateformatter" => {
                            input.date_formatter = Some(value.to_string())
                        }
                        "auto-fill" | "autofill" => input.auto_fill = Some(parse_boolish(value)?),
                        _ => bail!("unknown date --field config key: {key}"),
                    }
                } else {
                    input.date_formatter = Some(token.to_string());
                }
            }
        }
        BaseFieldKindArg::Number | BaseFieldKindArg::Progress | BaseFieldKindArg::Rating => {
            input.formatter = Some(config.to_string());
        }
        BaseFieldKindArg::Formula => {
            input.formula = Some(config.to_string());
        }
        BaseFieldKindArg::Link | BaseFieldKindArg::DuplexLink => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.linked_table_id = Some(value.to_string());
        }
        BaseFieldKindArg::User | BaseFieldKindArg::Group => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.multiple = Some(value.eq_ignore_ascii_case("multiple") || parse_boolish(value)?);
        }
        BaseFieldKindArg::Location => {
            let value = config
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(config)
                .trim();
            input.location_input_type = Some(value.to_string());
        }
        _ => {
            bail!("--field config is not supported for kind {kind:?}; use json:{{...}} for raw property")
        }
    }
    Ok(())
}

fn split_base_table_field_config(config: &str) -> Vec<&str> {
    config
        .split('|')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect()
}

fn normalize_base_table_config_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

fn parse_boolish(value: &str) -> Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Ok(true),
        "false" | "0" | "no" | "n" | "off" => Ok(false),
        other => bail!("expected boolean config value, got {other}"),
    }
}

pub(super) fn build_base_field_create_body(args: BaseFieldCreateArgs) -> Result<Value> {
    build_base_field_body(BaseFieldBuildInput {
        name: Some(args.name),
        field_type: args.field_type,
        kind: args.kind,
        property_json: args.property_json,
        description_json: args.description_json,
        ui_type: args.ui_type,
        options: args.options,
        formatter: args.formatter,
        currency_code: args.currency_code,
        date_formatter: args.date_formatter,
        auto_fill: args.auto_fill,
        multiple: args.multiple,
        linked_table_id: args.linked_table_id,
        formula: args.formula,
        location_input_type: args.location_input_type,
        require_name_and_type: true,
    })
}

pub(super) fn build_base_field_update_body(args: BaseFieldUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "base field update body",
        );
    }
    build_base_field_body(BaseFieldBuildInput {
        name: args.name,
        field_type: args.field_type,
        kind: args.kind,
        property_json: args.property_json,
        description_json: args.description_json,
        ui_type: args.ui_type,
        options: args.options,
        formatter: args.formatter,
        currency_code: args.currency_code,
        date_formatter: args.date_formatter,
        auto_fill: args.auto_fill,
        multiple: args.multiple,
        linked_table_id: args.linked_table_id,
        formula: args.formula,
        location_input_type: args.location_input_type,
        require_name_and_type: true,
    })
}

struct BaseFieldBuildInput {
    name: Option<String>,
    field_type: Option<i64>,
    kind: Option<BaseFieldKindArg>,
    property_json: Option<String>,
    description_json: Option<String>,
    ui_type: Option<String>,
    options: Vec<String>,
    formatter: Option<String>,
    currency_code: Option<String>,
    date_formatter: Option<String>,
    auto_fill: Option<bool>,
    multiple: Option<bool>,
    linked_table_id: Option<String>,
    formula: Option<String>,
    location_input_type: Option<String>,
    require_name_and_type: bool,
}

fn build_base_field_body(input: BaseFieldBuildInput) -> Result<Value> {
    let mut body = Map::new();
    let name = input
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("base field needs --name and --type/--kind, or raw body"))?;
    let (kind_type, kind_ui_type) = input.kind.map(base_field_kind_parts).unwrap_or((0, None));
    let field_type = match (input.field_type, input.kind) {
        (Some(field_type), Some(kind)) => {
            let (expected, _) = base_field_kind_parts(kind);
            if field_type != expected {
                bail!(
                    "base field --type {field_type} does not match --kind {kind:?} type {expected}"
                );
            }
            field_type
        }
        (Some(field_type), None) => field_type,
        (None, Some(_)) => kind_type,
        (None, None) if input.require_name_and_type => {
            bail!("base field needs --type or --kind unless raw body is used")
        }
        (None, None) => 0,
    };
    body.insert("field_name".to_string(), Value::String(name));
    body.insert("type".to_string(), Value::Number(field_type.into()));

    let property_value = input
        .property_json
        .map(|text| parse_json_value(&text, "property-json"))
        .transpose()?;
    let has_typed_property = !input.options.is_empty()
        || input.formatter.is_some()
        || input.currency_code.is_some()
        || input.date_formatter.is_some()
        || input.auto_fill.is_some()
        || input.multiple.is_some()
        || input.linked_table_id.is_some()
        || input.formula.is_some()
        || input.location_input_type.is_some();
    if has_typed_property {
        let mut property = match property_value {
            Some(value) => match ensure_json_object(value, "field.property")? {
                Value::Object(map) => map,
                _ => Map::new(),
            },
            None => Map::new(),
        };
        if !input.options.is_empty() {
            property.insert(
                "options".to_string(),
                Value::Array(
                    input
                        .options
                        .into_iter()
                        .map(base_field_option)
                        .collect::<Result<Vec<_>>>()?,
                ),
            );
        }
        insert_opt_string(&mut property, "formatter", input.formatter);
        insert_opt_string(&mut property, "currency_code", input.currency_code);
        insert_opt_string(&mut property, "date_formatter", input.date_formatter);
        insert_opt_string(&mut property, "table_id", input.linked_table_id);
        insert_opt_string(&mut property, "formula_expression", input.formula);
        if let Some(auto_fill) = input.auto_fill {
            property.insert("auto_fill".to_string(), Value::Bool(auto_fill));
        }
        if let Some(multiple) = input.multiple {
            property.insert("multiple".to_string(), Value::Bool(multiple));
        }
        if let Some(location_input_type) = input.location_input_type {
            property.insert(
                "location".to_string(),
                json!({ "input_type": location_input_type }),
            );
        }
        body.insert("property".to_string(), Value::Object(property));
    } else if let Some(property) = property_value {
        body.insert("property".to_string(), property);
    }
    if let Some(description) = input.description_json {
        body.insert(
            "description".to_string(),
            parse_json_value(&description, "description-json")?,
        );
    }
    let ui_type = input.ui_type.or_else(|| kind_ui_type.map(str::to_string));
    insert_opt_string(&mut body, "ui_type", ui_type);
    Ok(Value::Object(body))
}

fn base_field_kind_parts(kind: BaseFieldKindArg) -> (i64, Option<&'static str>) {
    match kind {
        BaseFieldKindArg::Text => (1, None),
        BaseFieldKindArg::Barcode => (1, Some("Barcode")),
        BaseFieldKindArg::Email => (1, Some("Email")),
        BaseFieldKindArg::Number => (2, None),
        BaseFieldKindArg::Progress => (2, Some("Progress")),
        BaseFieldKindArg::Currency => (2, Some("Currency")),
        BaseFieldKindArg::Rating => (2, Some("Rating")),
        BaseFieldKindArg::SingleSelect => (3, None),
        BaseFieldKindArg::MultiSelect => (4, None),
        BaseFieldKindArg::Date => (5, None),
        BaseFieldKindArg::Checkbox => (7, None),
        BaseFieldKindArg::User => (11, None),
        BaseFieldKindArg::Phone => (13, None),
        BaseFieldKindArg::Url => (15, None),
        BaseFieldKindArg::Attachment => (17, None),
        BaseFieldKindArg::Link => (18, None),
        BaseFieldKindArg::Formula => (20, None),
        BaseFieldKindArg::DuplexLink => (21, None),
        BaseFieldKindArg::Location => (22, None),
        BaseFieldKindArg::Group => (23, None),
        BaseFieldKindArg::AutoNumber => (1005, None),
    }
}

fn base_field_option(value: String) -> Result<Value> {
    let mut option = Map::new();
    if let Some((name, color)) = value.rsplit_once(':') {
        if let Ok(color) = color.parse::<i64>() {
            option.insert("name".to_string(), Value::String(name.to_string()));
            option.insert("color".to_string(), Value::Number(color.into()));
            return Ok(Value::Object(option));
        }
    }
    option.insert("name".to_string(), Value::String(value));
    Ok(Value::Object(option))
}

pub(super) fn build_base_dashboard_copy_body(args: BaseDashboardCopyArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "dashboard copy body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("dashboard copy needs --name or raw body"))?;
    Ok(json!({ "name": name }))
}

pub(super) fn build_base_workflow_update_body(args: BaseWorkflowUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "workflow update body",
        );
    }
    let status = args
        .status
        .ok_or_else(|| anyhow!("workflow update needs --status or raw body"))?;
    Ok(json!({ "status": status.as_feishu() }))
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

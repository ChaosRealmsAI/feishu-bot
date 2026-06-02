#![allow(clippy::too_many_arguments)]

use super::*;

mod helpers;
mod media;
mod records;
mod reference;
mod schema;
mod schema_exec;

use helpers::base_role_path;
pub(super) use helpers::{
    build_base_field_list_query, build_base_member_add_body, build_base_member_batch_body,
    build_base_record_search_body, build_base_role_write_body, build_base_view_create_body,
    build_base_view_update_body,
};
pub(super) use media::*;
pub(super) use records::*;
pub(super) use reference::*;
pub(super) use schema::*;
use schema_exec::{run_base_field_command, run_base_table_command, run_base_view_command};

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
        BaseCommand::Table(command) => run_base_table_command(api, command).await?,
        BaseCommand::Field(command) => run_base_field_command(api, command).await?,
        BaseCommand::View(command) => run_base_view_command(api, command).await?,
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

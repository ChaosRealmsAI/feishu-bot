#![allow(clippy::too_many_arguments)]

use super::*;

mod field_schema;
mod helpers;
mod media;
mod permission_exec;
mod record_exec;
mod records;
mod reference;
mod schema;
mod schema_exec;

pub(super) use field_schema::*;
pub(super) use helpers::{
    build_base_field_list_query, build_base_member_add_body, build_base_member_batch_body,
    build_base_record_search_body, build_base_role_write_body, build_base_view_create_body,
    build_base_view_update_body,
};
pub(super) use media::*;
use permission_exec::{run_base_member_command, run_base_role_command};
use record_exec::run_base_record_command;
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
        BaseCommand::Record(command) => run_base_record_command(api, command).await?,
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
        BaseCommand::Role(command) => run_base_role_command(api, command).await?,
        BaseCommand::Member(command) => run_base_member_command(api, command).await?,
    };
    print_response(raw_json, "base operation completed", data)
}

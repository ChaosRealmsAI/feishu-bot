#![allow(clippy::too_many_arguments)]

use super::*;

mod comment;
mod helpers;
mod permissions;
mod transfer;

pub(super) use comment::*;
pub(super) use helpers::*;
pub(super) use permissions::*;
use transfer::{run_drive_export_command, run_drive_import_command};
pub(super) async fn run_drive_command(
    api: &mut FeishuClient,
    command: DriveCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        DriveCommand::List(args) => {
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                ("order_by".to_string(), args.order_by),
                ("direction".to_string(), args.direction),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "folder_token", args.folder_token);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/drive/v1/files", &query).await?
        }
        DriveCommand::Folder(DriveFolderCommand::Create(args)) => {
            api.post_json(
                "/drive/v1/files/create_folder",
                &[],
                json!({
                    "name": args.name,
                    "folder_token": args.folder_token,
                }),
            )
            .await?
        }
        DriveCommand::Upload(args) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            api.upload_drive_file(
                &args.file,
                file_name,
                args.parent_type,
                args.folder_token,
                args.checksum,
            )
            .await?
        }
        DriveCommand::UploadLarge(args) => upload_large_drive_file(api, args).await?,
        DriveCommand::Media(DriveMediaCommand::Upload(args)) => {
            let file_name = drive_upload_file_name(&args.file, args.name)?;
            let parent_node = args.parent_node.unwrap_or_default();
            if args.parent_type != "ccm_import_open" && parent_node.trim().is_empty() {
                bail!(
                    "drive media upload needs --parent-node for {}; docx_image/docx_file use the target image/file block_id",
                    args.parent_type
                );
            }
            let extra = build_drive_media_extra(args.extra, args.drive_route_token)?;
            api.upload_drive_media(
                &args.file,
                file_name,
                args.parent_type,
                parent_node,
                args.checksum,
                extra,
            )
            .await?
        }
        DriveCommand::Media(DriveMediaCommand::Download(args)) => {
            let bytes = api
                .download_drive_media(
                    &args.file_token,
                    args.range.as_deref(),
                    args.extra.as_deref(),
                )
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
        DriveCommand::Media(DriveMediaCommand::TmpUrl(args)) => {
            if args.file_tokens.is_empty() || args.file_tokens.len() > 5 {
                bail!("drive media tmp-url needs 1..=5 --file-token values");
            }
            let mut query = args
                .file_tokens
                .into_iter()
                .map(|token| ("file_tokens".to_string(), token))
                .collect::<Vec<_>>();
            push_query_opt(&mut query, "extra", args.extra);
            api.get_json("/drive/v1/medias/batch_get_tmp_download_url", &query)
                .await?
        }
        DriveCommand::Import(command) => run_drive_import_command(api, command).await?,
        DriveCommand::Export(command) => run_drive_export_command(api, command).await?,
        DriveCommand::Comment(DriveCommentCommand::List(args)) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_list_query(&args)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::BatchGet(args)) => {
            let path = format!("/drive/v1/files/{}/comments/batch_query", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_batch_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Reply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_reply_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::UpdateReply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_update_reply_body(args)?;
            api.request_json_with_auth(Method::PUT, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::DeleteReply(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = vec![("file_type".to_string(), args.file_type)];
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Comment(DriveCommentCommand::Resolve(args)) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}",
                args.file_token, args.comment_id
            );
            let query = vec![("file_type".to_string(), args.file_type)];
            api.request_json_with_auth(
                Method::PATCH,
                &path,
                &query,
                Some(json!({ "is_solved": args.is_solved })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::Version(DriveVersionCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let query = drive_version_query(&args.obj_type, args.user_id_type, false)?;
            let auth = args.auth;
            let body = build_drive_version_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::List(args)) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let mut query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Version(DriveVersionCommand::Delete(args)) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Create(args)) => {
            let path = format!("/drive/v1/files/{}/subscriptions", args.file_token);
            let auth = args.auth;
            let body = build_drive_subscription_create_body(args);
            api.request_json_with_auth(Method::POST, &path, &[], Some(body), auth, &[])
                .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Get(args)) => {
            let path = format!(
                "/drive/v1/files/{}/subscriptions/{}",
                args.file_token, args.subscription_id
            );
            api.request_json_with_auth(
                Method::GET,
                &path,
                &[],
                Some(json!({ "file_type": args.file_type })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::Subscription(DriveSubscriptionCommand::Update(args)) => {
            let path = format!(
                "/drive/v1/files/{}/subscriptions/{}",
                args.file_token, args.subscription_id
            );
            api.request_json_with_auth(
                Method::PATCH,
                &path,
                &[],
                Some(json!({
                    "file_type": args.file_type,
                    "is_subscribe": args.is_subscribe,
                })),
                args.auth,
                &[],
            )
            .await?
        }
        DriveCommand::ViewRecord(args) => {
            let path = format!("/drive/v1/files/{}/view_records", args.file_token);
            let query = drive_view_record_query(args)?;
            let auth = query.1;
            api.request_json_with_auth(Method::GET, &path, &query.0, None, auth, &[])
                .await?
        }
        DriveCommand::Download(args) => {
            let bytes = api
                .download_drive_file(&args.file_token, args.range.as_deref())
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
        DriveCommand::Permission(DrivePermissionCommand::PublicGet(args)) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            api.get_json(&path, &[("type".to_string(), args.file_type)])
                .await?
        }
        DriveCommand::Permission(DrivePermissionCommand::PublicUpdate(args)) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            let query = vec![("type".to_string(), args.file_type.clone())];
            let body = build_drive_public_update_body(args)?;
            api.patch_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::PublicPasswordOff(args)) => {
            let path = format!("/drive/v1/permissions/{}/public/password", args.token);
            api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
                .await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberList(args)) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query = drive_permission_member_list_query(&args)?;
            api.get_json(&path, &query).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberAdd(args)) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_add_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberUpdate(args)) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_update_body(args)?;
            api.put_json(&path, &query, body).await?
        }
        DriveCommand::Permission(DrivePermissionCommand::MemberDelete(args)) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query = vec![
                ("type".to_string(), args.file_type.clone()),
                ("member_type".to_string(), args.member_type.clone()),
            ];
            let body = Some(build_drive_member_delete_body(args)?);
            api.delete_json(&path, &query, body).await?
        }
        DriveCommand::Stats(args) => {
            let path = format!("/drive/v1/files/{}/statistics", args.file_token);
            api.get_json(&path, &[("type".to_string(), args.file_type)])
                .await?
        }
        DriveCommand::Copy(args) => {
            let path = format!("/drive/v1/files/{}/copy", args.file_token);
            let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
                read_json_value(args.body_json, args.file, args.stdin)?
            } else {
                let mut body = Map::new();
                body.insert("type".to_string(), Value::String(args.file_type));
                body.insert("folder_token".to_string(), Value::String(args.folder_token));
                if let Some(name) = args.name {
                    body.insert("name".to_string(), Value::String(name));
                }
                Value::Object(body)
            };
            api.post_json(&path, &[], body).await?
        }
        DriveCommand::Move(args) => {
            let path = format!("/drive/v1/files/{}/move", args.file_token);
            let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
                read_json_value(args.body_json, args.file, args.stdin)?
            } else {
                json!({
                    "type": args.file_type,
                    "folder_token": args.folder_token,
                })
            };
            api.post_json(&path, &[], body).await?
        }
        DriveCommand::Delete(args) => {
            let path = format!("/drive/v1/files/{}", args.file_token);
            api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
                .await?
        }
    };
    print_response(raw_json, "drive operation completed", data)
}

use super::*;

pub(super) async fn run_drive_comment_command(
    api: &mut FeishuClient,
    command: DriveCommentCommand,
) -> Result<Value> {
    match command {
        DriveCommentCommand::List(args) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_list_query(&args)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await
        }
        DriveCommentCommand::Get(args) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await
        }
        DriveCommentCommand::BatchGet(args) => {
            let path = format!("/drive/v1/files/{}/comments/batch_query", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_batch_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await
        }
        DriveCommentCommand::Create(args) => {
            let path = format!("/drive/v1/files/{}/comments", args.file_token);
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await
        }
        DriveCommentCommand::Reply(args) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies",
                args.file_token, args.comment_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_reply_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await
        }
        DriveCommentCommand::UpdateReply(args) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = drive_comment_ref_query(&args.file_type, args.user_id_type);
            let auth = args.auth;
            let body = build_drive_comment_update_reply_body(args)?;
            api.request_json_with_auth(Method::PUT, &path, &query, Some(body), auth, &[])
                .await
        }
        DriveCommentCommand::DeleteReply(args) => {
            let path = format!(
                "/drive/v1/files/{}/comments/{}/replies/{}",
                args.file_token, args.comment_id, args.reply_id
            );
            let query = vec![("file_type".to_string(), args.file_type)];
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await
        }
        DriveCommentCommand::Resolve(args) => {
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
            .await
        }
    }
}

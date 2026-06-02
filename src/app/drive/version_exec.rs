use super::*;

pub(super) async fn run_drive_version_command(
    api: &mut FeishuClient,
    command: DriveVersionCommand,
) -> Result<Value> {
    match command {
        DriveVersionCommand::Create(args) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let query = drive_version_query(&args.obj_type, args.user_id_type, false)?;
            let auth = args.auth;
            let body = build_drive_version_create_body(args)?;
            api.request_json_with_auth(Method::POST, &path, &query, Some(body), auth, &[])
                .await
        }
        DriveVersionCommand::List(args) => {
            let path = format!("/drive/v1/files/{}/versions", args.file_token);
            let mut query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await
        }
        DriveVersionCommand::Get(args) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await
        }
        DriveVersionCommand::Delete(args) => {
            let path = format!(
                "/drive/v1/files/{}/versions/{}",
                args.file_token, args.version_id
            );
            let query = drive_version_query(&args.obj_type, args.user_id_type, true)?;
            api.request_json_with_auth(Method::DELETE, &path, &query, None, args.auth, &[])
                .await
        }
    }
}

pub(super) async fn run_drive_subscription_command(
    api: &mut FeishuClient,
    command: DriveSubscriptionCommand,
) -> Result<Value> {
    match command {
        DriveSubscriptionCommand::Create(args) => {
            let path = format!("/drive/v1/files/{}/subscriptions", args.file_token);
            let auth = args.auth;
            let body = build_drive_subscription_create_body(args);
            api.request_json_with_auth(Method::POST, &path, &[], Some(body), auth, &[])
                .await
        }
        DriveSubscriptionCommand::Get(args) => {
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
            .await
        }
        DriveSubscriptionCommand::Update(args) => {
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
            .await
        }
    }
}

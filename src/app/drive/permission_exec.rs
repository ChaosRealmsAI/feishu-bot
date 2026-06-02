use super::*;

pub(super) async fn run_drive_permission_command(
    api: &mut FeishuClient,
    command: DrivePermissionCommand,
) -> Result<Value> {
    match command {
        DrivePermissionCommand::PublicGet(args) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            api.get_json(&path, &[("type".to_string(), args.file_type)])
                .await
        }
        DrivePermissionCommand::PublicUpdate(args) => {
            let path = format!("/drive/v1/permissions/{}/public", args.token);
            let query = vec![("type".to_string(), args.file_type.clone())];
            let body = build_drive_public_update_body(args)?;
            api.patch_json(&path, &query, body).await
        }
        DrivePermissionCommand::PublicPasswordOff(args) => {
            let path = format!("/drive/v1/permissions/{}/public/password", args.token);
            api.delete_json(&path, &[("type".to_string(), args.file_type)], None)
                .await
        }
        DrivePermissionCommand::MemberList(args) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query = drive_permission_member_list_query(&args)?;
            api.get_json(&path, &query).await
        }
        DrivePermissionCommand::MemberAdd(args) => {
            let path = format!("/drive/v1/permissions/{}/members", args.token);
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_add_body(args)?;
            api.post_json(&path, &query, body).await
        }
        DrivePermissionCommand::MemberUpdate(args) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query =
                drive_permission_member_query(&args.file_type, args.need_notification, None);
            let body = build_drive_member_update_body(args)?;
            api.put_json(&path, &query, body).await
        }
        DrivePermissionCommand::MemberDelete(args) => {
            let path = format!(
                "/drive/v1/permissions/{}/members/{}",
                args.token, args.member_id
            );
            let query = vec![
                ("type".to_string(), args.file_type.clone()),
                ("member_type".to_string(), args.member_type.clone()),
            ];
            let body = Some(build_drive_member_delete_body(args)?);
            api.delete_json(&path, &query, body).await
        }
    }
}

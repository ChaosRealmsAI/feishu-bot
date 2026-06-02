use super::helpers::base_role_path;
use super::*;

pub(super) async fn run_base_role_command(
    api: &mut FeishuClient,
    command: BaseRoleCommand,
) -> Result<Value> {
    match command {
        BaseRoleCommand::List(args) => {
            let path = base_role_path(args.api_version, &args.app_token, None);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await
        }
        BaseRoleCommand::Create(args) => {
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
            api.post_json(&path, &[], body).await
        }
        BaseRoleCommand::Update(args) => {
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
            api.put_json(&path, &[], body).await
        }
        BaseRoleCommand::Delete(args) => {
            let path = format!("/bitable/v1/apps/{}/roles/{}", args.app_token, args.role_id);
            api.delete_json(&path, &[], None).await
        }
    }
}

pub(super) async fn run_base_member_command(
    api: &mut FeishuClient,
    command: BaseMemberCommand,
) -> Result<Value> {
    match command {
        BaseMemberCommand::List(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members",
                args.app_token, args.role_id
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await
        }
        BaseMemberCommand::Add(args) => {
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
            .await
        }
        BaseMemberCommand::Delete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/roles/{}/members/{}",
                args.app_token, args.role_id, args.member_id
            );
            api.delete_json(
                &path,
                &[("member_id_type".to_string(), args.member_id_type)],
                None,
            )
            .await
        }
        BaseMemberCommand::BatchAdd(args) => {
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
            api.post_json(&path, &[], body).await
        }
        BaseMemberCommand::BatchDelete(args) => {
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
            api.delete_json(&path, &[], Some(body)).await
        }
    }
}

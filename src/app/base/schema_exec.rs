use super::*;

pub(super) async fn run_base_table_command(
    api: &mut FeishuClient,
    command: BaseTableCommand,
) -> Result<Value> {
    match command {
        BaseTableCommand::List(args) => {
            let path = format!("/bitable/v1/apps/{}/tables", args.app_token);
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await
        }
        BaseTableCommand::Create(args) => {
            let path = format!("/bitable/v1/apps/{}/tables", args.app_token);
            let body = build_base_table_create_body(args)?;
            api.post_json(&path, &[], body).await
        }
        BaseTableCommand::BatchCreate(args) => {
            let path = format!("/bitable/v1/apps/{}/tables/batch_create", args.app_token);
            let user_id_type = args.user_id_type.resolve(None).to_string();
            let body = build_base_table_batch_create_body(args)?;
            api.post_json(&path, &[("user_id_type".to_string(), user_id_type)], body)
                .await
        }
        BaseTableCommand::Update(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}",
                args.app_token, args.table_id
            );
            let body = build_base_table_update_body(args)?;
            api.patch_json(&path, &[], body).await
        }
        BaseTableCommand::Delete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}",
                args.app_token, args.table_id
            );
            api.delete_json(&path, &[], None).await
        }
        BaseTableCommand::BatchDelete(args) => {
            let path = format!("/bitable/v1/apps/{}/tables/batch_delete", args.app_token);
            let table_ids =
                read_table_ids_json(args.table_ids, args.table_ids_json, args.file, args.stdin)?;
            api.post_json(&path, &[], json!({ "table_ids": table_ids }))
                .await
        }
    }
}

pub(super) async fn run_base_field_command(
    api: &mut FeishuClient,
    command: BaseFieldCommand,
) -> Result<Value> {
    match command {
        BaseFieldCommand::List(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields",
                args.app_token, args.table_id
            );
            let query = build_base_field_list_query(&args);
            api.get_json(&path, &query).await
        }
        BaseFieldCommand::Create(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields",
                args.app_token, args.table_id
            );
            let mut query = Vec::new();
            push_query_opt(&mut query, "client_token", args.client_token.clone());
            let body = build_base_field_create_body(args)?;
            api.post_json(&path, &query, body).await
        }
        BaseFieldCommand::Update(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields/{}",
                args.app_token, args.table_id, args.field_id
            );
            let body = build_base_field_update_body(args)?;
            api.put_json(&path, &[], body).await
        }
        BaseFieldCommand::Delete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/fields/{}",
                args.app_token, args.table_id, args.field_id
            );
            api.delete_json(&path, &[], None).await
        }
    }
}

pub(super) async fn run_base_view_command(
    api: &mut FeishuClient,
    command: BaseViewCommand,
) -> Result<Value> {
    match command {
        BaseViewCommand::List(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views",
                args.app_token, args.table_id
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await
        }
        BaseViewCommand::Create(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views",
                args.app_token, args.table_id
            );
            let body = build_base_view_create_body(args)?;
            api.post_json(&path, &[], body).await
        }
        BaseViewCommand::Get(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            api.get_json(&path, &[]).await
        }
        BaseViewCommand::Update(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            let body = build_base_view_update_body(args)?;
            api.patch_json(&path, &[], body).await
        }
        BaseViewCommand::Delete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/views/{}",
                args.app_token, args.table_id, args.view_id
            );
            api.delete_json(&path, &[], None).await
        }
    }
}

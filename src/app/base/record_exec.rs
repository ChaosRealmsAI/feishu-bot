use super::*;

pub(super) async fn run_base_record_command(
    api: &mut FeishuClient,
    command: BaseRecordCommand,
) -> Result<Value> {
    match command {
        BaseRecordCommand::List(args) => {
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
            api.get_json(&path, &query).await
        }
        BaseRecordCommand::Search(args) => {
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
            api.post_json(&path, &query, body).await
        }
        BaseRecordCommand::Get(args) => {
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
            .await
        }
        BaseRecordCommand::BatchGet(args) => {
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
            .await
        }
        BaseRecordCommand::Create(args) => {
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
                .await
        }
        BaseRecordCommand::BatchCreate(args) => {
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
                .await
        }
        BaseRecordCommand::BatchUpdate(args) => {
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
                .await
        }
        BaseRecordCommand::Update(args) => {
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
                .await
        }
        BaseRecordCommand::Delete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/{}",
                args.app_token, args.table_id, args.record_id
            );
            api.delete_json(&path, &[], None).await
        }
        BaseRecordCommand::BatchDelete(args) => {
            let path = format!(
                "/bitable/v1/apps/{}/tables/{}/records/batch_delete",
                args.app_token, args.table_id
            );
            let records =
                read_record_ids_json(args.record_ids, args.records_json, args.file, args.stdin)?;
            api.post_json(&path, &[], json!({ "records": records }))
                .await
        }
    }
}

use super::*;

mod bodies;

pub(super) use bodies::*;

pub(super) async fn run_sheet_command(
    api: &mut FeishuClient,
    command: SheetCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        SheetCommand::Create(args) => {
            let body = build_sheet_create_body(args)?;
            api.post_json("/sheets/v3/spreadsheets", &[], body).await?
        }
        SheetCommand::Get(args) => {
            let path = format!("/sheets/v3/spreadsheets/{}", args.spreadsheet_token);
            api.get_json(&path, &[]).await?
        }
        SheetCommand::Sheets(args) => {
            let path = format!(
                "/sheets/v3/spreadsheets/{}/sheets/query",
                args.spreadsheet_token
            );
            api.get_json(&path, &[]).await?
        }
        SheetCommand::GetSheet(args) => {
            let path = format!(
                "/sheets/v3/spreadsheets/{}/sheets/{}",
                args.spreadsheet_token, args.sheet_id
            );
            api.get_json(&path, &[]).await?
        }
        SheetCommand::AddSheet(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/sheets_batch_update",
                args.spreadsheet_token
            );
            let body = build_sheet_add_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::CopySheet(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/sheets_batch_update",
                args.spreadsheet_token
            );
            let body = build_sheet_copy_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::DeleteSheet(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/sheets_batch_update",
                args.spreadsheet_token
            );
            let body = build_sheet_delete_body(args);
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::UpdateSheet(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/sheets_batch_update",
                args.spreadsheet_token
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_sheet_update_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        SheetCommand::Merge(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/merge_cells",
                args.spreadsheet_token
            );
            let body = build_sheet_merge_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::Unmerge(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/unmerge_cells",
                args.spreadsheet_token
            );
            let body = build_sheet_unmerge_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::Style(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/styles_batch_update",
                args.spreadsheet_token
            );
            let body = build_sheet_style_body(args)?;
            api.put_json(&path, &[], body).await?
        }
        SheetCommand::Values(SheetValuesCommand::Get(args)) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/values/{}",
                args.spreadsheet_token, args.range
            );
            api.get_json(&path, &[]).await?
        }
        SheetCommand::Values(SheetValuesCommand::BatchGet(args)) => {
            if args.ranges.is_empty() {
                bail!("sheet values batch-get needs at least one --range");
            }
            let path = format!(
                "/sheets/v2/spreadsheets/{}/values_batch_get",
                args.spreadsheet_token
            );
            let query = args
                .ranges
                .into_iter()
                .map(|range| ("ranges".to_string(), range))
                .collect::<Vec<_>>();
            api.get_json(&path, &query).await?
        }
        SheetCommand::Values(SheetValuesCommand::Update(args)) => {
            let path = format!("/sheets/v2/spreadsheets/{}/values", args.spreadsheet_token);
            let body = build_sheet_values_body(args)?;
            api.put_json(&path, &[], body).await?
        }
        SheetCommand::Values(SheetValuesCommand::Append(args)) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/values_append",
                args.spreadsheet_token
            );
            let body = build_sheet_values_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::Values(SheetValuesCommand::Prepend(args)) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/values_prepend",
                args.spreadsheet_token
            );
            let body = build_sheet_values_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        SheetCommand::BatchUpdate(args) => {
            let path = format!(
                "/sheets/v2/spreadsheets/{}/sheets_batch_update",
                args.spreadsheet_token
            );
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json(&path, &[], body).await?
        }
    };
    print_response(raw_json, "sheet operation completed", data)
}

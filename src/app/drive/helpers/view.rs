use super::*;

pub(in crate::app) fn drive_view_record_query(
    args: DriveViewRecordArgs,
) -> Result<(Vec<(String, String)>, ApiAuthArg)> {
    if !(1..=50).contains(&args.page_size) {
        bail!("drive view-record page_size must be 1..=50");
    }
    let mut query = vec![
        ("file_type".to_string(), args.file_type),
        ("page_size".to_string(), args.page_size.to_string()),
        (
            "viewer_id_type".to_string(),
            args.viewer_id_type.resolve(None).to_string(),
        ),
    ];
    push_query_opt(&mut query, "page_token", args.page_token);
    Ok((query, args.auth))
}

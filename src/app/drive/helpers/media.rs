use super::*;

pub(in crate::app) fn build_drive_media_extra(
    raw_extra: Option<String>,
    drive_route_token: Option<String>,
) -> Result<Option<String>> {
    match (
        raw_extra.filter(|value| !value.trim().is_empty()),
        drive_route_token.filter(|value| !value.trim().is_empty()),
    ) {
        (Some(_), Some(_)) => bail!("use either --extra or --drive-route-token, not both"),
        (Some(extra), None) => Ok(Some(extra)),
        (None, Some(token)) => Ok(Some(json!({ "drive_route_token": token }).to_string())),
        (None, None) => Ok(None),
    }
}

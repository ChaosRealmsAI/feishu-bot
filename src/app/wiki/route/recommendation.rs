use super::*;

pub(in crate::app) fn wiki_route_check_strict_error(data: &Value) -> String {
    let recommendation = get_string(data, &["data", "recommendation"])
        .unwrap_or_else(|| "inspect the route-check JSON for details".to_string());
    let first_failed = data
        .get("data")
        .and_then(|data| data.get("checks"))
        .and_then(Value::as_array)
        .and_then(|checks| {
            checks
                .iter()
                .find(|check| check.get("ok").and_then(Value::as_bool) != Some(true))
        });
    let failed_name = first_failed
        .and_then(|check| get_string(check, &["name"]))
        .unwrap_or_else(|| "write_probe".to_string());
    let failed_error = first_failed
        .and_then(|check| get_string(check, &["error"]))
        .or_else(|| get_string(data, &["data", "write_probe", "move_error"]))
        .unwrap_or_default();
    if failed_error.is_empty() {
        format!("wiki route is not ready after {failed_name}: {recommendation}")
    } else {
        format!("wiki route is not ready after {failed_name}: {recommendation}; {failed_error}")
    }
}

pub(in crate::app) fn wiki_route_recommendation(
    default_doc_create_wiki: bool,
    has_target_space: bool,
    all_api_checks_ok: bool,
    write_probe_requested: bool,
    write_probe_ok: Option<bool>,
) -> &'static str {
    if !has_target_space {
        "Set FEISHU_WIKI_SPACE_ID or pass --space-id before using Wiki as the default route."
    } else if !default_doc_create_wiki {
        "Set FEISHU_DOC_CREATE_WIKI_DEFAULT=true so plain `feishu-bot doc create` attempts Wiki publishing."
    } else if !all_api_checks_ok {
        "Wiki route is configured, but OpenAPI checks failed. Grant Wiki scopes and add the app or bot to the target Wiki space, then rerun route-check."
    } else if write_probe_requested && write_probe_ok != Some(true) {
        "Wiki read route is configured, but the write probe did not prove publishing. Fix the write_probe error before claiming future reports can all go through Wiki."
    } else if write_probe_requested {
        "Wiki write route is ready. Future AI reports can use plain `feishu-bot doc create --wiki-fallback-ok` and verify wiki_move or wiki node readback."
    } else {
        "Wiki read route is ready. Run `feishu-bot wiki route-check --write-probe` once before claiming future reports can all go through Wiki."
    }
}

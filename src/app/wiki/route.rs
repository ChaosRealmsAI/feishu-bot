use super::*;

mod checks;
mod recommendation;
mod request;
mod write_probe;

use checks::wiki_route_check_call;
pub(in crate::app) use recommendation::{wiki_route_check_strict_error, wiki_route_recommendation};
pub(in crate::app) use request::wiki_request_json;
use write_probe::run_wiki_write_probe;

pub(super) async fn run_wiki_route_check(
    api: &mut FeishuClient,
    args: WikiRouteCheckArgs,
) -> Result<Value> {
    if args.page_size == 0 || args.page_size > 50 {
        bail!("wiki route-check page_size must be between 1 and 50");
    }

    let target_space_id = args
        .space_id
        .clone()
        .or_else(|| api.config.default_wiki_space_id.clone());
    let target_parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| api.config.default_wiki_parent_node_token.clone());

    let mut checks = Vec::new();
    let page_size = args.page_size.to_string();
    checks.push(
        wiki_route_check_call(
            api,
            "list_spaces",
            Method::GET,
            "/wiki/v2/spaces".to_string(),
            vec![("page_size".to_string(), page_size.clone())],
            args.auth,
            &["wiki:wiki", "wiki:wiki:readonly", "wiki:space:retrieve"],
        )
        .await,
    );

    if let Some(space_id) = target_space_id.as_deref() {
        let encoded_space_id = encode_path_segment(space_id);
        checks.push(
            wiki_route_check_call(
                api,
                "get_target_space",
                Method::GET,
                format!("/wiki/v2/spaces/{encoded_space_id}"),
                Vec::new(),
                args.auth,
                &["wiki:wiki", "wiki:wiki:readonly", "wiki:space:read"],
            )
            .await,
        );
        checks.push(
            wiki_route_check_call(
                api,
                "list_target_nodes",
                Method::GET,
                format!("/wiki/v2/spaces/{encoded_space_id}/nodes"),
                vec![("page_size".to_string(), page_size)],
                args.auth,
                &["wiki:wiki", "wiki:wiki:readonly", "wiki:node:retrieve"],
            )
            .await,
        );
    }

    let all_api_checks_ok = checks
        .iter()
        .all(|check| check.get("ok").and_then(Value::as_bool).unwrap_or(false));
    let read_route_ready =
        api.config.default_doc_create_wiki && target_space_id.is_some() && all_api_checks_ok;
    let write_probe = if args.write_probe {
        Some(
            run_wiki_write_probe(
                api,
                target_space_id.clone(),
                target_parent_node_token.clone(),
                args.auth,
                args.write_probe_title.clone(),
                args.write_probe_apply,
            )
            .await,
        )
    } else {
        None
    };
    let write_probe_ok = write_probe
        .as_ref()
        .and_then(|probe| probe.get("ok"))
        .and_then(Value::as_bool);
    let route_ready = read_route_ready && (!args.write_probe || write_probe_ok == Some(true));
    let recommendation = wiki_route_recommendation(
        api.config.default_doc_create_wiki,
        target_space_id.is_some(),
        all_api_checks_ok,
        args.write_probe,
        write_probe_ok,
    );

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "route_ready": route_ready,
            "read_route_ready": read_route_ready,
            "write_probe_ran": args.write_probe,
            "default_doc_create_wiki": api.config.default_doc_create_wiki,
            "target_space_id": target_space_id,
            "target_parent_node_token": target_parent_node_token,
            "auth": format!("{:?}", args.auth).to_lowercase(),
            "has_user_access_token": api.config.user_access_token.as_ref().is_some_and(|token| !token.trim().is_empty()),
            "checks": checks,
            "write_probe": write_probe,
            "recommendation": recommendation
        }
    }))
}

use super::*;

mod bodies;
mod route;

pub(super) use bodies::*;
use route::run_wiki_route_check;
#[cfg(test)]
pub(in crate::app) use route::wiki_route_recommendation;
pub(in crate::app) use route::{wiki_request_json, wiki_route_check_strict_error};

pub(super) async fn run_wiki_command(
    api: &mut FeishuClient,
    command: WikiCommand,
    raw_json: bool,
) -> Result<()> {
    let command = match command {
        WikiCommand::RouteCheck(args) => {
            let strict = args.strict;
            let data = run_wiki_route_check(api, args).await?;
            let route_ready = data
                .get("data")
                .and_then(|data| data.get("route_ready"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
            print_response(raw_json, "wiki operation completed", data.clone())?;
            if strict && !route_ready {
                bail!("{}", wiki_route_check_strict_error(&data));
            }
            return Ok(());
        }
        other => other,
    };

    let data = match command {
        WikiCommand::RouteCheck(_) => unreachable!("route-check is handled before dispatch"),
        WikiCommand::CreateSpace(args) => {
            let body = build_wiki_create_space_body(args)?;
            wiki_request_json(
                api,
                Method::POST,
                "/wiki/v2/spaces",
                &[],
                Some(body),
                ApiAuthArg::User,
            )
            .await?
        }
        WikiCommand::Spaces(args) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            wiki_request_json(api, Method::GET, "/wiki/v2/spaces", &query, None, args.auth).await?
        }
        WikiCommand::Space(args) => {
            let path = format!("/wiki/v2/spaces/{}", encode_path_segment(&args.space_id));
            let mut query = Vec::new();
            push_query_opt(&mut query, "lang", args.lang);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Nodes(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes",
                encode_path_segment(&args.space_id)
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "parent_node_token", args.parent_node_token);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Node(args) => {
            let mut query = vec![("token".to_string(), args.token)];
            push_query_opt(&mut query, "obj_type", args.obj_type);
            wiki_request_json(
                api,
                Method::GET,
                "/wiki/v2/spaces/get_node",
                &query,
                None,
                args.auth,
            )
            .await?
        }
        WikiCommand::CreateNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_create_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::MoveNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/move",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_move_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::CopyNode(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/copy",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_copy_node_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::UpdateTitle(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/{}/update_title",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.node_token)
            );
            let auth = args.auth;
            let body = build_wiki_update_title_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::MoveDocsToWiki(args) => {
            let path = format!(
                "/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_move_docs_to_wiki_body(args)?;
            wiki_request_json(api, Method::POST, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::List(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members",
                encode_path_segment(&args.space_id)
            );
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::Add(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let mut query = Vec::new();
            if let Some(value) = args.need_notification {
                query.push(("need_notification".to_string(), value.to_string()));
            }
            let body = build_wiki_member_add_body(args)?;
            wiki_request_json(api, Method::POST, &path, &query, Some(body), auth).await?
        }
        WikiCommand::Member(WikiMemberCommand::Delete(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/members/{}",
                encode_path_segment(&args.space_id),
                encode_path_segment(&args.member_id)
            );
            let auth = args.auth;
            let body = build_wiki_member_delete_body(args)?;
            wiki_request_json(api, Method::DELETE, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Setting(WikiSettingCommand::Update(args)) => {
            let path = format!(
                "/wiki/v2/spaces/{}/setting",
                encode_path_segment(&args.space_id)
            );
            let auth = args.auth;
            let body = build_wiki_setting_update_body(args)?;
            wiki_request_json(api, Method::PUT, &path, &[], Some(body), auth).await?
        }
        WikiCommand::Task(args) => {
            let path = format!("/wiki/v2/tasks/{}", encode_path_segment(&args.task_id));
            let query = vec![("task_type".to_string(), args.task_type)];
            wiki_request_json(api, Method::GET, &path, &query, None, args.auth).await?
        }
        WikiCommand::Search(args) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token.clone());
            let body = build_wiki_search_body(args)?;
            wiki_request_json(
                api,
                Method::POST,
                "/wiki/v2/nodes/search",
                &query,
                Some(body),
                ApiAuthArg::User,
            )
            .await?
        }
    };
    print_response(raw_json, "wiki operation completed", data)
}

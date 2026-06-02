use super::*;

mod bodies;
mod queries;

use bodies::{
    build_approval_external_task_list_body, build_approval_instance_cancel_body,
    build_approval_task_transfer_body,
};
pub(super) use bodies::{
    build_approval_search_body, build_approval_task_action_body, build_approval_task_add_sign_body,
    build_approval_task_rollback_body,
};
use queries::{approval_id_query, approval_search_query, approval_task_user_query};

pub(super) async fn run_approval_command(
    api: &mut FeishuClient,
    command: ApprovalCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        ApprovalCommand::Definition(ApprovalDefinitionCommand::Get(args)) => {
            let path = format!(
                "/approval/v4/approvals/{}",
                encode_path_segment(&args.approval_code)
            );
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_opt(&mut query, "locale", args.locale);
            if args.with_admin_id {
                query.push(("with_admin_id".to_string(), "true".to_string()));
            }
            api.get_json(&path, &query).await?
        }
        ApprovalCommand::Definition(ApprovalDefinitionCommand::Create(args)) => {
            let query = approval_id_query(args.user_id_type, args.department_id_type);
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/approvals", &query, body)
                .await?
        }
        ApprovalCommand::Definition(ApprovalDefinitionCommand::Subscribe(args)) => {
            let path = format!(
                "/approval/v4/approvals/{}/subscribe",
                encode_path_segment(&args.approval_code)
            );
            api.post_json(&path, &[], json!({})).await?
        }
        ApprovalCommand::Definition(ApprovalDefinitionCommand::Unsubscribe(args)) => {
            let path = format!(
                "/approval/v4/approvals/{}/unsubscribe",
                encode_path_segment(&args.approval_code)
            );
            api.post_json(&path, &[], json!({})).await?
        }
        ApprovalCommand::Instance(ApprovalInstanceCommand::List(args)) => {
            let mut query = vec![
                ("approval_code".to_string(), args.approval_code),
                ("start_time".to_string(), args.start_time),
                ("end_time".to_string(), args.end_time),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/approval/v4/instances", &query).await?
        }
        ApprovalCommand::Instance(ApprovalInstanceCommand::Query(args)) => {
            let query = approval_search_query(&args);
            let body = build_approval_search_body(args, "approval instance query body")?;
            api.post_json("/approval/v4/instances/query", &query, body)
                .await?
        }
        ApprovalCommand::Instance(ApprovalInstanceCommand::Get(args)) => {
            let path = format!("/approval/v4/instances/{}", args.instance_code);
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_opt(&mut query, "locale", args.locale);
            api.get_json(&path, &query).await?
        }
        ApprovalCommand::Instance(ApprovalInstanceCommand::Create(args)) => {
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/instances", &[], body).await?
        }
        ApprovalCommand::Instance(ApprovalInstanceCommand::Cancel(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(Some(&args.user_id)).to_string(),
            )];
            let body = build_approval_instance_cancel_body(args)?;
            api.post_json("/approval/v4/instances/cancel", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::Search(args)) => {
            let query = approval_search_query(&args);
            let body = build_approval_search_body(args, "approval task search body")?;
            api.post_json("/approval/v4/tasks/search", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::Approve(args)) => {
            let query = approval_task_user_query(args.user_id_type, &args.user_id);
            let body = build_approval_task_action_body(args)?;
            api.post_json("/approval/v4/tasks/approve", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::Reject(args)) => {
            let query = approval_task_user_query(args.user_id_type, &args.user_id);
            let body = build_approval_task_action_body(args)?;
            api.post_json("/approval/v4/tasks/reject", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::Transfer(args)) => {
            let query = approval_task_user_query(args.user_id_type, &args.user_id);
            let body = build_approval_task_transfer_body(args)?;
            api.post_json("/approval/v4/tasks/transfer", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::AddSign(args)) => {
            let query = approval_task_user_query(args.user_id_type, &args.user_id);
            let body = build_approval_task_add_sign_body(args)?;
            api.post_json("/approval/v4/instances/add_sign", &query, body)
                .await?
        }
        ApprovalCommand::Task(ApprovalTaskCommand::Rollback(args)) => {
            let query = approval_task_user_query(args.user_id_type, &args.user_id);
            let body = build_approval_task_rollback_body(args)?;
            api.post_json("/approval/v4/instances/specified_rollback", &query, body)
                .await?
        }
        ApprovalCommand::External(ApprovalExternalCommand::DefinitionGet(args)) => {
            let path = format!(
                "/approval/v4/external_approvals/{}",
                encode_path_segment(&args.approval_code)
            );
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_opt(&mut query, "locale", args.locale);
            if args.with_admin_id {
                query.push(("with_admin_id".to_string(), "true".to_string()));
            }
            api.get_json(&path, &query).await?
        }
        ApprovalCommand::External(ApprovalExternalCommand::DefinitionCreate(args)) => {
            let query = approval_id_query(args.user_id_type, args.department_id_type);
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/external_approvals", &query, body)
                .await?
        }
        ApprovalCommand::External(ApprovalExternalCommand::InstanceSync(args)) => {
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/external_instances", &[], body)
                .await?
        }
        ApprovalCommand::External(ApprovalExternalCommand::InstanceCheck(args)) => {
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/external_instances/check", &[], body)
                .await?
        }
        ApprovalCommand::External(ApprovalExternalCommand::TaskList(args)) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token.clone());
            let body = build_approval_external_task_list_body(args)?;
            api.request_json(
                Method::GET,
                "/approval/v4/external_tasks",
                &query,
                Some(body),
            )
            .await?
        }
        ApprovalCommand::CreateDefinition(args) => {
            let body = read_json_value(args.body_json, args.file, args.stdin)?;
            api.post_json("/approval/v4/approvals", &[], body).await?
        }
    };
    print_response(raw_json, "approval operation completed", data)
}

use super::*;

mod docs;
mod formatting;
mod interactions;
mod links;
mod local;
mod readback;
mod report;
mod resources;
mod state;

use docs::*;
use formatting::*;
use interactions::*;
use links::*;
pub(super) use local::{office_command_can_run_without_api, run_office_local_command};
use local::{run_office_bootstrap_dry_run, run_office_list, run_office_report_dry_run};
use readback::*;
use report::{run_office_progress, run_office_report};
use resources::*;
pub(super) use state::*;

pub(super) async fn run_office_command(
    api: &mut FeishuClient,
    command: OfficeCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OfficeCommand::List(args) => run_office_list(args)?,
        OfficeCommand::Bootstrap(args) => run_office_bootstrap(api, args).await?,
        OfficeCommand::Report(args) => run_office_report(api, args).await?,
        OfficeCommand::Progress(args) => run_office_progress(api, args).await?,
        OfficeCommand::VoiceReport(args) => run_office_voice_report(api, args).await?,
        OfficeCommand::Inbox(args) => run_office_inbox(api, args).await?,
        OfficeCommand::Poll(args) => run_office_poll(api, args).await?,
        OfficeCommand::Status(args) => run_office_status(api, args).await?,
        OfficeCommand::Search(args) => run_office_search(api, args).await?,
        OfficeCommand::Cleanup(args) => run_office_cleanup(api, args).await?,
    };
    print_response(raw_json, "office workflow completed", data)
}

async fn run_office_bootstrap(api: &mut FeishuClient, args: OfficeBootstrapArgs) -> Result<Value> {
    if args.dry_run {
        return run_office_bootstrap_dry_run(args);
    }
    let project_key = office_project_key(&args.project)?;
    let state_path = office_registry_path()?;
    let mut registry = read_office_registry()?;
    let now = office_now();
    let mut project = registry
        .projects
        .get(&project_key)
        .cloned()
        .unwrap_or_else(|| OfficeProject {
            project: project_key.clone(),
            name: args.project.trim().to_string(),
            created_at: Some(now.clone()),
            ..OfficeProject::default()
        });

    let mut next_actions = Vec::new();
    let chat = ensure_office_chat(api, &args, &mut project).await?;
    let wiki = if args.skip_wiki {
        json!({ "status": "skipped" })
    } else {
        ensure_office_wiki_index(api, &args, &mut project, &mut next_actions).await?
    };
    let base = if args.skip_base {
        json!({ "status": "skipped" })
    } else {
        match ensure_office_base(api, &args, &mut project, &mut next_actions).await {
            Ok(value) => value,
            Err(error) => {
                next_actions.push(format!(
                    "Base setup failed after chat setup; grant Base/Wiki permissions or rerun with --skip-base: {error:#}"
                ));
                json!({
                    "status": "error",
                    "error": format!("{error:#}"),
                })
            }
        }
    };
    let tabs = if args.skip_tabs {
        json!({ "status": "skipped" })
    } else {
        add_office_tabs(api, &project).await
    };
    let summary_message = if args.send_summary {
        Some(send_office_summary(api, &mut project).await?)
    } else {
        None
    };

    project.updated_at = Some(office_now());
    registry
        .projects
        .insert(project_key.clone(), project.clone());
    write_office_registry(&registry)?;
    sync_legacy_project_chat(&project)?;

    if project.wiki_space_id.is_none() {
        next_actions.push(
            "Set FEISHU_WIKI_SPACE_ID or rerun bootstrap with --space-id to make Wiki the default report route."
                .to_string(),
        );
    }
    if project.base_app_token.is_none() {
        next_actions.push(
            "Rerun without --skip-base after Base/Wiki scopes are granted to enable project log records."
                .to_string(),
        );
    }
    if !args.send_summary {
        next_actions.push(
            "Rerun bootstrap with --send-summary or use office report to send the first visible project update."
                .to_string(),
        );
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "project": project_key,
            "state_file": state_path,
            "chat_id": project.chat_id,
            "wiki_space_id": project.wiki_space_id,
            "wiki_index_node_token": project.wiki_index_node_token,
            "wiki_index_obj_token": project.wiki_index_obj_token,
            "base_node_token": project.base_node_token,
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
            "message_id": project.pinned_summary_message_id,
            "project_state": project,
            "chat": chat,
            "wiki": wiki,
            "base": base,
            "tabs": tabs,
            "summary_message": summary_message,
            "readback": {
                "chat_get": readback_chat(api, project.chat_id.as_deref()).await,
                "wiki_index": readback_wiki_node(api, project.wiki_index_node_token.as_deref(), args.auth).await,
                "base": readback_base(api, project.base_app_token.as_deref(), project.base_table_id.as_deref()).await,
            },
            "next_actions": next_actions,
        }
    }))
}

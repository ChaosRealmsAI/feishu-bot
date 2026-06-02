use super::*;

mod browser;
mod flows;
mod plan;
mod probes;
mod wiki;

use flows::{run_setup_auto, run_setup_open_scopes, run_setup_plan, run_setup_quickstart};
use wiki::run_setup_wiki_bot;

pub(super) async fn run_setup_command(
    command: SetupCommand,
    use_lark: bool,
    base_url_override: Option<String>,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        SetupCommand::Plan(args) => run_setup_plan(args)?,
        SetupCommand::OpenScopes(args) => run_setup_open_scopes(args)?,
        SetupCommand::WikiBot(args) => {
            let config = Config::load(use_lark, base_url_override)?;
            let mut api = FeishuClient::new(config);
            run_setup_wiki_bot(
                &mut api,
                args.space_id,
                args.member_role,
                args.need_notification,
                args.auth,
            )
            .await?
        }
        SetupCommand::Quickstart(args) => {
            run_setup_quickstart(args, use_lark, base_url_override).await?
        }
        SetupCommand::Auto(args) => run_setup_auto(args, use_lark, base_url_override).await?,
    };
    print_response(raw_json, "setup operation completed", data)
}

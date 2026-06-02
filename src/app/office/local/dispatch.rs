use super::*;

pub(in crate::app) fn office_command_can_run_without_api(command: &OfficeCommand) -> bool {
    match command {
        OfficeCommand::List(_) => true,
        OfficeCommand::Bootstrap(args) => args.dry_run,
        OfficeCommand::Report(args) => args.dry_run,
        OfficeCommand::Status(args) => !args.check,
        OfficeCommand::Cleanup(args) => args.dry_run || !args.confirm || args.local_only,
        OfficeCommand::Progress(_)
        | OfficeCommand::VoiceReport(_)
        | OfficeCommand::Inbox(_)
        | OfficeCommand::Poll(_)
        | OfficeCommand::Search(_) => false,
    }
}

pub(in crate::app) fn run_office_local_command(
    command: OfficeCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OfficeCommand::List(args) => run_office_list(args)?,
        OfficeCommand::Bootstrap(args) if args.dry_run => run_office_bootstrap_dry_run(args)?,
        OfficeCommand::Report(args) if args.dry_run => run_office_report_dry_run(args)?,
        OfficeCommand::Status(args) if !args.check => run_office_status_local(args)?,
        OfficeCommand::Cleanup(args) if args.dry_run || !args.confirm || args.local_only => {
            run_office_cleanup_local(args)?
        }
        _ => bail!("office command requires Feishu API credentials"),
    };
    print_response(raw_json, "office workflow completed", data)
}

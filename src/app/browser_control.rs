use super::*;

pub(super) fn run_browser_command(command: BrowserCommand) -> Result<()> {
    match command {
        BrowserCommand::Ensure => {
            let script = std::env::var("FEISHU_PLAYWRIGHT_ENSURE")
                .unwrap_or_else(|_| "ensure-extension-mcp.sh".to_string());
            run_status(ProcessCommand::new("bash").arg(script).arg("--background"))
        }
        BrowserCommand::Tabs => run_mcpc(&[
            "tools-call",
            "browser_tabs",
            "action:=list",
            "--timeout",
            "20",
        ]),
        BrowserCommand::Open(args) => {
            let url_arg = format!("url:={}", args.url);
            run_mcpc(&[
                "tools-call",
                "browser_navigate",
                &url_arg,
                "--timeout",
                "30",
            ])
        }
        BrowserCommand::Drive => run_mcpc(&[
            "tools-call",
            "browser_navigate",
            "url:=https://my.feishu.cn/drive/home/",
            "--timeout",
            "30",
        ]),
    }
}

pub(super) fn run_mcpc(args: &[&str]) -> Result<()> {
    let mut command = ProcessCommand::new("npx");
    command.arg("--yes").arg("@apify/mcpc").arg("@browser");
    for arg in args {
        command.arg(arg);
    }
    run_status(&mut command)
}

pub(super) fn run_status(command: &mut ProcessCommand) -> Result<()> {
    let status = command.status().context("run command")?;
    if !status.success() {
        bail!("command exited with status {status}");
    }
    Ok(())
}

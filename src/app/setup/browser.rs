use super::*;

pub(super) fn setup_grant_open_probe(
    grant: &Result<Value>,
    should_open: bool,
    open: fn(&str) -> Result<()>,
) -> Value {
    if !should_open {
        return json!({ "status": "skipped" });
    }
    match grant.as_ref() {
        Ok(value) => match value.get("grant_url").and_then(Value::as_str) {
            Some(url) => probe_value(open(url).map(|_| json!({ "opened": true }))),
            None => json!({ "ok": false, "error": "setup grant response missing grant_url" }),
        },
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

pub(super) fn run_setup_browser_open(url: &str) -> Result<()> {
    run_browser_command(BrowserCommand::Open(BrowserOpenArgs {
        url: url.to_string(),
    }))
}

pub(super) fn run_system_browser_open(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        run_status(ProcessCommand::new("open").arg(url))
    }
    #[cfg(target_os = "windows")]
    {
        run_status(ProcessCommand::new("cmd").args(["/C", "start", "", url]))
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        run_status(ProcessCommand::new("xdg-open").arg(url))
    }
}

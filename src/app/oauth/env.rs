use super::*;

pub(super) fn oauth_env_lines(access_token: Option<&str>, refresh_token: Option<&str>) -> Value {
    let mut lines = Vec::new();
    if let Some(token) = access_token {
        lines.push(format!("export FEISHU_USER_ACCESS_TOKEN={token}"));
    }
    if let Some(token) = refresh_token {
        lines.push(format!("export FEISHU_REFRESH_TOKEN={token}"));
    }
    Value::Array(lines.into_iter().map(Value::String).collect())
}

pub(super) fn save_oauth_tokens_to_env(
    path: &Path,
    access_token: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<()> {
    let mut updates = Vec::new();
    if let Some(token) = access_token {
        updates.push(("FEISHU_USER_ACCESS_TOKEN", token.to_string()));
    }
    if let Some(token) = refresh_token {
        updates.push(("FEISHU_REFRESH_TOKEN", token.to_string()));
    }
    if updates.is_empty() {
        bail!("OAuth response did not include access_token or refresh_token");
    }
    let existing = if path.exists() {
        fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?
    } else {
        String::new()
    };
    let mut handled = vec![false; updates.len()];
    let mut lines = Vec::new();
    for line in existing.lines() {
        let mut replaced = false;
        for (index, (key, value)) in updates.iter().enumerate() {
            if line.starts_with(&format!("{key}=")) {
                lines.push(format!("{key}={value}"));
                handled[index] = true;
                replaced = true;
                break;
            }
        }
        if !replaced {
            lines.push(line.to_string());
        }
    }
    for (handled, (key, value)) in handled.into_iter().zip(updates) {
        if !handled {
            lines.push(format!("{key}={value}"));
        }
    }
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

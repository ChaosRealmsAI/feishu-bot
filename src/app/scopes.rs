use super::*;

mod groups;

use groups::all_scope_groups;
pub(super) use groups::{ScopeGroups, OFFICE_SCOPE_GROUPS};

pub(super) fn print_scope_groups(group: &str, token_type: ApiAuthArg) -> Result<()> {
    let values = load_env_values().unwrap_or_default();
    let app_id =
        get_any(&values, &["FEISHU_APP_ID", "LARK_APP_ID"]).unwrap_or_else(|| "<app_id>".into());
    let groups = scope_groups(group)?;
    let token_type = scope_token_type(token_type);
    for (name, scopes) in groups {
        println!("[{name}]");
        for scope in &scopes {
            println!("- {scope}");
        }
        println!(
            "grant_url=https://open.feishu.cn/app/{}/auth?q={}&op_from=feishu-bot&token_type={}",
            app_id,
            scopes.join(","),
            token_type
        );
        println!();
    }
    Ok(())
}

pub(super) fn scope_token_type(token_type: ApiAuthArg) -> &'static str {
    match token_type {
        ApiAuthArg::Tenant => "tenant",
        ApiAuthArg::User => "user",
    }
}

pub(super) fn scope_groups(group: &str) -> Result<ScopeGroups> {
    let group = group.trim().to_ascii_lowercase();
    let all = all_scope_groups();
    if group == "all" {
        return Ok(all);
    }
    if group == "office" {
        let selected = OFFICE_SCOPE_GROUPS
            .iter()
            .filter_map(|wanted| {
                all.iter()
                    .find(|(name, _)| name == wanted)
                    .map(|(name, scopes)| (*name, scopes.clone()))
            })
            .collect();
        return Ok(selected);
    }
    let selected = all
        .into_iter()
        .filter(|(name, _)| *name == group.as_str())
        .collect::<Vec<_>>();
    if selected.is_empty() {
        bail!("unknown scope group: {group}");
    }
    Ok(selected)
}

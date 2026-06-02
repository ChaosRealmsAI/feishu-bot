use super::*;

pub(in crate::app::office) fn run_office_list(args: OfficeListArgs) -> Result<Value> {
    let registry = read_office_registry()?;
    let state_path = office_registry_path()?;
    let mut projects: Vec<_> = registry.projects.values().cloned().collect();
    projects.sort_by(|left, right| left.project.cmp(&right.project));
    let items: Vec<Value> = projects
        .into_iter()
        .map(|project| {
            if args.details {
                serde_json::to_value(project).unwrap_or_else(|_| json!({}))
            } else {
                json!({
                    "project": project.project,
                    "name": project.name,
                    "has_chat": project.chat_id.is_some(),
                    "has_wiki": project.wiki_index_node_token.is_some(),
                    "has_base": project.base_app_token.is_some() && project.base_table_id.is_some(),
                    "has_pinned_summary": project.pinned_summary_message_id.is_some(),
                    "updated_at": project.updated_at,
                })
            }
        })
        .collect();
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "state_file": state_path,
            "count": items.len(),
            "projects": items,
        }
    }))
}

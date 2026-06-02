use super::*;

pub(in crate::app::office) async fn ensure_office_wiki_index(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    let Some(space_id) = args
        .space_id
        .clone()
        .or_else(|| project.wiki_space_id.clone())
        .or_else(|| api.config.default_wiki_space_id.clone())
        .filter(|value| !value.trim().is_empty())
    else {
        next_actions.push(
            "Wiki index skipped because no --space-id or FEISHU_WIKI_SPACE_ID is configured."
                .to_string(),
        );
        return Ok(json!({ "status": "skipped_missing_space" }));
    };
    project.wiki_space_id = Some(space_id.clone());
    project.wiki_parent_node_token = args
        .parent_node_token
        .clone()
        .or_else(|| project.wiki_parent_node_token.clone())
        .or_else(|| api.config.default_wiki_parent_node_token.clone());

    if project.wiki_index_obj_token.is_some() && project.wiki_index_node_token.is_some() {
        return Ok(json!({
            "status": "reused",
            "space_id": space_id,
            "node_token": project.wiki_index_node_token,
            "document_id": project.wiki_index_obj_token,
            "url": project.wiki_index_node_token.as_deref().map(|token| wiki_url(api, token)),
        }));
    }

    let created = match create_wiki_doc(
        api,
        &space_id,
        project.wiki_parent_node_token.as_deref(),
        &format!("{} 项目主页", project.name),
        ContentTypeArg::Markdown,
        &office_index_markdown(project),
        args.auth,
    )
    .await
    {
        Ok(created) => created,
        Err(error) => {
            next_actions.push(format!(
                "Wiki index creation failed but the project chat can still be used. Grant the app/bot edit permission in the Wiki space, or rerun bootstrap with --auth user/--skip-wiki: {error:#}"
            ));
            return Ok(json!({
                "status": "error",
                "space_id": space_id,
                "error": format!("{error:#}"),
            }));
        }
    };
    project.wiki_index_node_token = created.node_token.clone();
    project.wiki_index_obj_token = Some(created.document_id.clone());
    Ok(json!({
        "status": "created",
        "space_id": space_id,
        "node_token": created.node_token,
        "document_id": created.document_id,
        "url": created.url,
        "create_response": created.create_response,
        "append_response": created.append_response,
    }))
}

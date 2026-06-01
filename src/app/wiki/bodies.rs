use super::*;

pub(in crate::app) fn build_doc_create_wiki_move_body(
    document_id: &str,
    parent_wiki_token: Option<String>,
    apply: bool,
) -> Value {
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String("docx".to_string()));
    body.insert(
        "obj_token".to_string(),
        Value::String(document_id.to_string()),
    );
    insert_opt_string(&mut body, "parent_wiki_token", parent_wiki_token);
    if apply {
        body.insert("apply".to_string(), Value::Bool(true));
    }
    Value::Object(body)
}

pub(super) fn build_wiki_create_space_body(args: WikiCreateSpaceArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki create-space body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "name", args.name);
    insert_opt_string(&mut body, "description", args.description);
    insert_opt_string(&mut body, "open_sharing", args.open_sharing);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_wiki_create_node_body(args: WikiCreateNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki create-node body",
        );
    }
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String(args.obj_type));
    body.insert("node_type".to_string(), Value::String(args.node_type));
    insert_opt_string(&mut body, "parent_node_token", args.parent_node_token);
    insert_opt_string(&mut body, "origin_node_token", args.origin_node_token);
    insert_opt_string(&mut body, "title", args.title);
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_move_node_body(args: WikiMoveNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki move-node body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "target_parent_token", args.target_parent_token);
    insert_opt_string(&mut body, "target_space_id", args.target_space_id);
    if body.is_empty() {
        bail!("wiki move-node requires --target-parent-token, --target-space-id, or raw JSON body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_copy_node_body(args: WikiCopyNodeArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki copy-node body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "target_parent_token", args.target_parent_token);
    insert_opt_string(&mut body, "target_space_id", args.target_space_id);
    insert_opt_string(&mut body, "title", args.title);
    if !body.contains_key("target_parent_token") && !body.contains_key("target_space_id") {
        bail!("wiki copy-node requires --target-parent-token, --target-space-id, or raw JSON body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_wiki_update_title_body(args: WikiUpdateTitleArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki update-title body",
        );
    }
    let title = args
        .title
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki update-title requires --title unless --body-json/--file/--stdin is used")
        })?;
    Ok(json!({ "title": title }))
}

pub(in crate::app) fn build_wiki_move_docs_to_wiki_body(
    args: WikiMoveDocsToWikiArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki move-docs-to-wiki body",
        );
    }
    let obj_type = args
        .obj_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki move-docs-to-wiki requires --obj-type unless raw JSON body is used")
        })?;
    let obj_token = args
        .obj_token
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki move-docs-to-wiki requires --obj-token unless raw JSON body is used")
        })?;
    let mut body = Map::new();
    body.insert("obj_type".to_string(), Value::String(obj_type));
    body.insert("obj_token".to_string(), Value::String(obj_token));
    insert_opt_string(&mut body, "parent_wiki_token", args.parent_wiki_token);
    if args.apply {
        body.insert("apply".to_string(), Value::Bool(true));
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_wiki_member_add_body(args: WikiMemberAddArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki member add body",
        );
    }
    let member_type = args
        .member_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member add requires --member-type unless raw JSON body is used")
        })?;
    let member_id = args
        .member_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member add requires --member-id unless raw JSON body is used")
        })?;
    Ok(json!({
        "member_type": member_type,
        "member_id": member_id,
        "member_role": args.member_role
    }))
}

pub(super) fn build_wiki_member_delete_body(args: WikiMemberDeleteArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki member delete body",
        );
    }
    let member_type = args
        .member_type
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki member delete requires --member-type unless raw JSON body is used")
        })?;
    Ok(json!({
        "member_type": member_type,
        "member_role": args.member_role
    }))
}

pub(super) fn build_wiki_setting_update_body(args: WikiSettingUpdateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki setting update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "create_setting", args.create_setting);
    insert_opt_string(&mut body, "security_setting", args.security_setting);
    insert_opt_string(&mut body, "comment_setting", args.comment_setting);
    if body.is_empty() {
        bail!("wiki setting update requires at least one setting flag or raw JSON body");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_wiki_search_body(args: WikiSearchArgs) -> Result<Value> {
    if args.page_size == 0 || args.page_size > 50 {
        bail!("wiki search page_size must be between 1 and 50");
    }
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "wiki search body",
        );
    }
    let query = args
        .query
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            anyhow!("wiki search requires --query unless --body-json/--file/--stdin is used")
        })?;
    if args.node_id.is_some() && args.space_id.is_none() {
        bail!("wiki search --node-id requires --space-id");
    }
    let mut body = Map::new();
    body.insert("query".to_string(), Value::String(query));
    insert_opt_string(&mut body, "space_id", args.space_id);
    insert_opt_string(&mut body, "node_id", args.node_id);
    Ok(Value::Object(body))
}

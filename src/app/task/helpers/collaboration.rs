use super::*;

pub(in crate::app) fn build_tasklist_create_body(args: TasklistCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist create body",
        );
    }
    let name = args
        .name
        .ok_or_else(|| anyhow!("tasklist create needs --name or raw body"))?;
    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(name));
    if !args.members.is_empty() {
        body.insert(
            "members".to_string(),
            Value::Array(task_members(args.members, &args.member_role)),
        );
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_tasklist_update_body(args: TasklistUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist update body",
        );
    }
    let mut tasklist = Map::new();
    let mut update_fields = Vec::new();
    if let Some(name) = args.name {
        tasklist.insert("name".to_string(), Value::String(name));
        update_fields.push(Value::String("name".to_string()));
    }
    if let Some(owner) = args.owner_json {
        tasklist.insert("owner".to_string(), parse_json_value(&owner, "owner-json")?);
        update_fields.push(Value::String("owner".to_string()));
    }
    if update_fields.is_empty() {
        bail!("tasklist update needs --name, --owner-json, or raw body");
    }
    Ok(json!({
        "tasklist": Value::Object(tasklist),
        "update_fields": Value::Array(update_fields),
        "origin_owner_to_role": args.origin_owner_to_role,
    }))
}

pub(in crate::app) fn build_tasklist_member_body(args: TasklistMemberWriteArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "tasklist member body",
        );
    }
    if let Some(members_json) = args.members_json {
        let value = parse_json_value(&members_json, "members-json")?;
        if value.get("members").is_some() {
            return ensure_json_object(value, "tasklist member body");
        }
        return Ok(json!({ "members": ensure_json_array(value, "members")? }));
    }
    let member_type = args.member_type.as_deref().unwrap_or("user");
    let mut members = Vec::new();
    members.extend(task_members_typed(args.editors, "editor", member_type));
    members.extend(task_members_typed(args.viewers, "viewer", member_type));
    if members.is_empty() {
        bail!("tasklist member command needs --editor/--viewer, --members-json, or raw body");
    }
    Ok(json!({ "members": members }))
}

pub(in crate::app) fn build_task_comment_create_body(args: TaskCommentCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "comment create body",
        );
    }
    let content = args
        .content
        .ok_or_else(|| anyhow!("comment create needs --content or raw body"))?;
    let mut body = Map::new();
    body.insert("content".to_string(), Value::String(content));
    body.insert(
        "resource_type".to_string(),
        Value::String("task".to_string()),
    );
    body.insert("resource_id".to_string(), Value::String(args.task_guid));
    if let Some(reply_to_comment_id) = args.reply_to_comment_id {
        body.insert(
            "reply_to_comment_id".to_string(),
            Value::String(reply_to_comment_id),
        );
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_task_comment_update_body(args: TaskCommentUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "comment update body",
        );
    }
    let content = args
        .content
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("comment update needs --content or raw body"))?;
    Ok(json!({
        "comment": {
            "content": content
        },
        "update_fields": ["content"]
    }))
}

pub(in crate::app) fn build_task_member_body(
    args: TaskMemberWriteArgs,
    include_client_token: bool,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        let mut body = ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task member body",
        )?;
        if include_client_token && body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }
    if let Some(members_json) = args.members_json {
        let value = parse_json_value(&members_json, "members-json")?;
        let mut body = if value.get("members").is_some() {
            ensure_json_object(value, "task member body")?
        } else {
            json!({ "members": ensure_json_array(value, "members")? })
        };
        if include_client_token && body.get("client_token").is_none() {
            body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
        }
        return Ok(body);
    }
    let member_type = args.member_type.as_deref().unwrap_or("user");
    let mut members = Vec::new();
    members.extend(task_members_typed(args.assignees, "assignee", member_type));
    members.extend(task_members_typed(args.followers, "follower", member_type));
    if members.is_empty() {
        bail!("task member command needs --assignee/--follower, --members-json, or raw body");
    }
    let mut body = json!({ "members": members });
    if include_client_token {
        body["client_token"] = Value::String(args.client_token.unwrap_or_else(random_uuid));
    }
    Ok(body)
}

pub(in crate::app) fn build_task_tasklist_body(args: TaskTasklistWriteArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "task tasklist relation body",
        );
    }
    let tasklist_guid = args
        .tasklist_guid
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("task tasklist relation needs --tasklist-guid or raw body"))?;
    let mut body = Map::new();
    body.insert("tasklist_guid".to_string(), Value::String(tasklist_guid));
    insert_opt_string(&mut body, "section_guid", args.section_guid);
    Ok(Value::Object(body))
}

use super::*;

pub(in crate::app) fn drive_permission_member_query(
    file_type: &str,
    need_notification: bool,
    member_type: Option<&str>,
) -> Vec<(String, String)> {
    let mut query = vec![("type".to_string(), file_type.to_string())];
    if need_notification {
        query.push(("need_notification".to_string(), "true".to_string()));
    }
    if let Some(member_type) = member_type.filter(|value| !value.trim().is_empty()) {
        query.push(("member_type".to_string(), member_type.to_string()));
    }
    query
}

pub(in crate::app) fn drive_permission_member_list_query(
    args: &DrivePermissionMemberListArgs,
) -> Result<Vec<(String, String)>> {
    if args.page_size == 0 || args.page_size > 200 {
        bail!("drive permission member-list page_size must be between 1 and 200");
    }
    let mut query = vec![
        ("type".to_string(), args.file_type.clone()),
        ("page_size".to_string(), args.page_size.to_string()),
    ];
    push_query_opt(&mut query, "page_token", args.page_token.clone());
    push_query_opt(&mut query, "member_type", args.member_type.clone());
    Ok(query)
}

pub(in crate::app) fn build_drive_public_update_body(
    args: DrivePermissionPublicUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "public permission body",
        );
    }
    let mut body = Map::new();
    if let Some(value) = args.external_access {
        body.insert("external_access".to_string(), Value::Bool(value));
    }
    if let Some(value) = args.invite_external {
        body.insert("invite_external".to_string(), Value::Bool(value));
    }
    insert_opt_string(&mut body, "security_entity", args.security_entity);
    insert_opt_string(&mut body, "comment_entity", args.comment_entity);
    insert_opt_string(&mut body, "share_entity", args.share_entity);
    insert_opt_string(&mut body, "link_share_entity", args.link_share_entity);
    if body.is_empty() {
        bail!("provide public permission fields or raw JSON via --body-json/--file/--stdin");
    }
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_drive_member_add_body(
    args: DrivePermissionMemberAddArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member add body",
        );
    }
    Ok(json!({
        "member_type": args.member_type,
        "member_id": args.member_id,
        "perm": args.perm,
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

pub(in crate::app) fn build_drive_member_update_body(
    args: DrivePermissionMemberUpdateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member update body",
        );
    }
    Ok(json!({
        "member_type": args.member_type,
        "perm": args.perm,
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

pub(in crate::app) fn build_drive_member_delete_body(
    args: DrivePermissionMemberDeleteArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "member delete body",
        );
    }
    Ok(json!({
        "perm_type": args.perm_type,
        "type": args.collaborator_type,
    }))
}

use super::*;

pub(in crate::app) fn build_approval_search_body(
    args: ApprovalSearchArgs,
    label: &str,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            label,
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "user_id", args.user_id);
    insert_opt_string(&mut body, "approval_code", args.approval_code);
    insert_opt_string(&mut body, "instance_code", args.instance_code);
    insert_opt_string(&mut body, "instance_external_id", args.instance_external_id);
    insert_opt_string(&mut body, "group_external_id", args.group_external_id);
    insert_opt_string(&mut body, "instance_title", args.instance_title);
    insert_opt_string(&mut body, "instance_status", args.instance_status);
    insert_opt_string(
        &mut body,
        "instance_start_time_from",
        args.instance_start_time_from,
    );
    insert_opt_string(
        &mut body,
        "instance_start_time_to",
        args.instance_start_time_to,
    );
    insert_opt_string(&mut body, "task_title", args.task_title);
    insert_opt_string(&mut body, "task_status", args.task_status);
    insert_string_array(&mut body, "task_status_list", args.task_status_list);
    insert_opt_string(&mut body, "task_start_time_from", args.task_start_time_from);
    insert_opt_string(&mut body, "task_start_time_to", args.task_start_time_to);
    insert_opt_string(&mut body, "locale", args.locale);
    insert_opt_i64(&mut body, "order", args.order);
    if body.is_empty() {
        bail!("{label} needs at least one filter or raw JSON body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_approval_instance_cancel_body(
    args: ApprovalInstanceCancelArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval instance cancel body",
        );
    }
    Ok(json!({
        "approval_code": args.approval_code,
        "instance_code": args.instance_code,
        "user_id": args.user_id,
    }))
}

pub(in crate::app) fn build_approval_task_action_body(
    args: ApprovalTaskActionArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval task action body",
        );
    }
    let mut body = Map::new();
    body.insert(
        "approval_code".to_string(),
        Value::String(args.approval_code),
    );
    body.insert(
        "instance_code".to_string(),
        Value::String(args.instance_code),
    );
    body.insert("user_id".to_string(), Value::String(args.user_id));
    body.insert("task_id".to_string(), Value::String(args.task_id));
    insert_opt_string(&mut body, "comment", args.comment);
    insert_opt_serialized_json(&mut body, "form", args.form_json, "approval form JSON")?;
    Ok(Value::Object(body))
}

pub(super) fn build_approval_task_transfer_body(args: ApprovalTaskTransferArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval task transfer body",
        );
    }
    let mut body = Map::new();
    body.insert(
        "approval_code".to_string(),
        Value::String(args.approval_code),
    );
    body.insert(
        "instance_code".to_string(),
        Value::String(args.instance_code),
    );
    body.insert("user_id".to_string(), Value::String(args.user_id));
    body.insert("task_id".to_string(), Value::String(args.task_id));
    body.insert(
        "transfer_user_id".to_string(),
        Value::String(args.transfer_user_id),
    );
    insert_opt_string(&mut body, "comment", args.comment);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_approval_task_add_sign_body(
    args: ApprovalTaskAddSignArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval task add-sign body",
        );
    }
    if args.add_sign_user_ids.is_empty() {
        bail!("approval task add-sign needs at least one --add-user-id or raw JSON body");
    }
    let add_sign_type = args
        .add_sign_type
        .ok_or_else(|| anyhow!("approval task add-sign needs --add-sign-type or raw JSON body"))?;
    let mut body = Map::new();
    body.insert(
        "approval_code".to_string(),
        Value::String(args.approval_code),
    );
    body.insert(
        "instance_code".to_string(),
        Value::String(args.instance_code),
    );
    body.insert("user_id".to_string(), Value::String(args.user_id));
    body.insert("task_id".to_string(), Value::String(args.task_id));
    insert_string_array(&mut body, "add_sign_user_ids", args.add_sign_user_ids);
    insert_opt_i64(&mut body, "add_sign_type", Some(add_sign_type));
    insert_opt_i64(&mut body, "approval_method", args.approval_method);
    insert_opt_string(&mut body, "comment", args.comment);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_approval_task_rollback_body(
    args: ApprovalTaskRollbackArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval task rollback body",
        );
    }
    if args.task_def_key_list.is_empty() {
        bail!("approval task rollback needs at least one --task-def-key or raw JSON body");
    }
    let mut body = Map::new();
    body.insert("user_id".to_string(), Value::String(args.user_id));
    body.insert("task_id".to_string(), Value::String(args.task_id));
    insert_string_array(&mut body, "task_def_key_list", args.task_def_key_list);
    insert_opt_string(&mut body, "reason", args.reason);
    insert_opt_string(&mut body, "extra", args.extra);
    Ok(Value::Object(body))
}

pub(super) fn build_approval_external_task_list_body(
    args: ApprovalExternalTaskListArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "approval external task list body",
        );
    }
    let mut body = Map::new();
    insert_string_array(&mut body, "approval_codes", args.approval_codes);
    insert_string_array(&mut body, "instance_ids", args.instance_ids);
    insert_string_array(&mut body, "user_ids", args.user_ids);
    insert_opt_string(&mut body, "status", args.status);
    if body.is_empty() {
        bail!("approval external task-list needs a filter or raw JSON body");
    }
    Ok(Value::Object(body))
}

fn insert_opt_serialized_json(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<String>,
    label: &str,
) -> Result<()> {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        let parsed = parse_json_value(&value, label)?;
        object.insert(
            key.to_string(),
            Value::String(serde_json::to_string(&parsed)?),
        );
    }
    Ok(())
}

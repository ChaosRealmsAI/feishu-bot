use super::*;

pub(in crate::app) fn drive_version_query(
    obj_type: &str,
    user_id_type: UserIdTypeArg,
    include_obj_type: bool,
) -> Result<Vec<(String, String)>> {
    if !matches!(obj_type, "docx" | "sheet") {
        bail!("drive version obj-type must be docx or sheet");
    }
    let mut query = vec![(
        "user_id_type".to_string(),
        user_id_type.resolve(None).to_string(),
    )];
    if include_obj_type {
        query.push(("obj_type".to_string(), obj_type.to_string()));
    }
    Ok(query)
}

pub(in crate::app) fn build_drive_version_create_body(
    args: DriveVersionCreateArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "drive version create body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("drive version create needs --name or raw JSON body"))?;
    if !matches!(args.obj_type.as_str(), "docx" | "sheet") {
        bail!("drive version obj-type must be docx or sheet");
    }
    Ok(json!({
        "name": name,
        "obj_type": args.obj_type,
    }))
}

pub(in crate::app) fn build_drive_subscription_create_body(
    args: DriveSubscriptionCreateArgs,
) -> Value {
    let mut body = Map::new();
    body.insert("file_type".to_string(), Value::String(args.file_type));
    body.insert(
        "subscription_type".to_string(),
        Value::String(args.subscription_type),
    );
    insert_opt_string(&mut body, "subscription_id", args.subscription_id);
    if let Some(value) = args.is_subscribe {
        body.insert("is_subcribe".to_string(), Value::Bool(value));
    }
    Value::Object(body)
}

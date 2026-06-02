use super::*;

pub(in crate::app) fn build_hire_job_open_body(args: HireJobOpenArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire job open body",
        );
    }
    let is_never_expired = args
        .is_never_expired
        .ok_or_else(|| anyhow!("hire job open needs --is-never-expired unless raw JSON is used"))?;
    if !is_never_expired && args.expiry_time.is_none() {
        bail!("hire job open needs --expiry-time when --is-never-expired false");
    }
    let mut body = Map::new();
    body.insert(
        "is_never_expired".to_string(),
        Value::Bool(is_never_expired),
    );
    insert_opt_i64(&mut body, "expiry_time", args.expiry_time);
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_hire_talent_create_body(args: HireTalentCreateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire talent combined_create body",
        );
    }
    let name = args
        .name
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("hire talent create needs --name unless raw JSON is used"))?;
    let mut basic_info = Map::new();
    basic_info.insert("name".to_string(), Value::String(name));
    insert_opt_string(&mut basic_info, "email", args.email);
    insert_opt_string(&mut basic_info, "mobile", args.mobile);
    insert_opt_string(
        &mut basic_info,
        "mobile_country_code",
        args.mobile_country_code,
    );
    insert_opt_string(&mut basic_info, "current_city_code", args.current_city_code);

    let mut body = Map::new();
    insert_opt_string(&mut body, "resume_source_id", args.resume_source_id);
    let folder_ids = clean_string_values(args.folder_ids);
    if !folder_ids.is_empty() {
        body.insert("folder_id_list".to_string(), json!(folder_ids));
    }
    insert_opt_string(&mut body, "creator_id", args.creator_id);
    insert_opt_u8(&mut body, "creator_account_type", args.creator_account_type);
    insert_opt_string(&mut body, "resume_attachment_id", args.resume_attachment_id);
    body.insert("basic_info".to_string(), Value::Object(basic_info));
    Ok(Value::Object(body))
}

pub(in crate::app) fn build_hire_location_query_body(args: HireLocationQueryArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "hire location query body",
        );
    }
    let location_type = args.location_type.ok_or_else(|| {
        anyhow!("hire location query needs --location-type unless raw JSON is used")
    })?;
    let mut body = Map::new();
    insert_opt_u8(&mut body, "location_type", Some(location_type));
    let code_list = clean_string_values(args.code_list);
    if !code_list.is_empty() {
        body.insert("code_list".to_string(), json!(code_list));
    }
    Ok(Value::Object(body))
}

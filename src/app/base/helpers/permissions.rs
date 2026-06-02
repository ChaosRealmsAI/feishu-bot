use super::super::*;

pub(in crate::app) fn base_role_path(
    api_version: BaseRoleApiVersionArg,
    app_token: &str,
    role_id: Option<&str>,
) -> String {
    let base = match api_version {
        BaseRoleApiVersionArg::V1 => format!("/bitable/v1/apps/{app_token}/roles"),
        BaseRoleApiVersionArg::V2 => format!("/base/v2/apps/{app_token}/roles"),
    };
    if let Some(role_id) = role_id {
        format!("{base}/{role_id}")
    } else {
        base
    }
}

pub(in crate::app) fn build_base_role_write_body(
    name: Option<String>,
    table_roles_json: Option<String>,
    block_roles_json: Option<String>,
    base_rule_json: Option<String>,
    allow_base_complex_edit: Option<bool>,
    allow_copy: Option<bool>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(read_json_value(body_json, file, stdin)?, "base role body");
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "role_name", name);
    if let Some(table_roles) = table_roles_json {
        body.insert(
            "table_roles".to_string(),
            ensure_json_array(
                parse_json_value(&table_roles, "table-roles-json")?,
                "table_roles",
            )?,
        );
    }
    if let Some(block_roles) = block_roles_json {
        body.insert(
            "block_roles".to_string(),
            ensure_json_array(
                parse_json_value(&block_roles, "block-roles-json")?,
                "block_roles",
            )?,
        );
    }
    let base_rule = build_base_rule_body(base_rule_json, allow_base_complex_edit, allow_copy)?;
    if let Some(base_rule) = base_rule {
        body.insert("base_rule".to_string(), base_rule);
    }
    if body.is_empty() {
        bail!("base role write needs --name, --table-roles-json, --block-roles-json, --base-rule-json, --allow-base-complex-edit, --allow-copy, or raw body");
    }
    Ok(Value::Object(body))
}

fn build_base_rule_body(
    base_rule_json: Option<String>,
    allow_base_complex_edit: Option<bool>,
    allow_copy: Option<bool>,
) -> Result<Option<Value>> {
    if base_rule_json.is_none() && allow_base_complex_edit.is_none() && allow_copy.is_none() {
        return Ok(None);
    }
    let mut rule = match base_rule_json {
        Some(raw) => {
            match ensure_json_object(parse_json_value(&raw, "base-rule-json")?, "base_rule")? {
                Value::Object(map) => map,
                _ => unreachable!("ensure_json_object returned a non-object"),
            }
        }
        None => Map::new(),
    };
    if let Some(allow) = allow_base_complex_edit {
        rule.insert(
            "base_complex_edit".to_string(),
            json!(if allow { 1 } else { 0 }),
        );
    }
    if let Some(allow) = allow_copy {
        rule.insert("copy".to_string(), json!(if allow { 1 } else { 0 }));
    }
    Ok(Some(Value::Object(rule)))
}

pub(in crate::app) fn build_base_member_add_body(
    member_id: Option<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(read_json_value(body_json, file, stdin)?, "base member body");
    }
    let member_id = member_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("base member add needs --member-id or raw body"))?;
    Ok(json!({ "member_id": member_id }))
}

pub(in crate::app) fn build_base_member_batch_body(
    mut members: Vec<String>,
    member_list_json: Option<String>,
    body_json: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if body_json.is_some() || file.is_some() || stdin {
        return ensure_json_object(
            read_json_value(body_json, file, stdin)?,
            "base member batch body",
        );
    }
    if let Some(member_list_json) = member_list_json {
        let value = parse_json_value(&member_list_json, "member-list-json")?;
        if value.get("member_list").is_some() {
            return ensure_json_object(value, "base member batch body");
        }
        return Ok(json!({ "member_list": ensure_json_array(value, "member_list")? }));
    }
    members.retain(|member| !member.trim().is_empty());
    if members.is_empty() {
        bail!("base member batch needs --member type:id, --member-list-json, or raw body");
    }
    let member_list = members
        .into_iter()
        .map(|member| {
            let (member_type, member_id) = member
                .split_once(':')
                .ok_or_else(|| anyhow!("--member must use type:id, for example open_id:ou_xxx"))?;
            let member_type = member_type.trim();
            let member_id = member_id.trim();
            if member_type.is_empty() || member_id.is_empty() {
                bail!("--member must use non-empty type:id");
            }
            Ok(json!({ "type": member_type, "id": member_id }))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({ "member_list": member_list }))
}

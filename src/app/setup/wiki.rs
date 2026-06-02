use super::*;

pub(super) async fn run_setup_wiki_bot(
    api: &mut FeishuClient,
    space_id: Option<String>,
    member_role: String,
    need_notification: Option<bool>,
    auth: ApiAuthArg,
) -> Result<Value> {
    let space_id = space_id
        .or_else(|| api.config.default_wiki_space_id.clone())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("setup wiki-bot needs --space-id or FEISHU_WIKI_SPACE_ID"))?;
    let bot = normalize_bot_info_response(api.get_json("/bot/v3/info", &[]).await?);
    let open_id = get_string(&bot, &["data", "open_id"])
        .ok_or_else(|| anyhow!("bot info response missing open_id: {bot}"))?;
    let path = format!("/wiki/v2/spaces/{}/members", encode_path_segment(&space_id));
    let initial_member_list = probe_value(
        wiki_request_json(
            api,
            Method::GET,
            &path,
            &[("page_size".to_string(), "50".to_string())],
            None,
            auth,
        )
        .await,
    );
    if wiki_member_has_role(&initial_member_list, &open_id, &member_role) {
        return Ok(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "status": "reused",
                "space_id": space_id,
                "member_type": "openid",
                "member_id": open_id,
                "member_role": member_role,
                "auth": format!("{auth:?}").to_lowercase(),
                "bot": bot,
                "member_add": { "status": "skipped_existing_member" },
                "member_list": initial_member_list,
                "next_actions": [
                    "Rerun `feishu-bot office bootstrap --project <project> --send-summary` without --skip-wiki.",
                    "Rerun `feishu-bot wiki route-check --write-probe --strict` to prove Wiki write readiness."
                ],
            }
        }));
    }
    let mut query = Vec::new();
    if let Some(value) = need_notification {
        query.push(("need_notification".to_string(), value.to_string()));
    }
    let body = build_wiki_member_add_body(WikiMemberAddArgs {
        space_id: space_id.clone(),
        member_type: Some("openid".to_string()),
        member_id: Some(open_id.clone()),
        member_role: member_role.clone(),
        need_notification,
        auth,
        body_json: None,
        file: None,
        stdin: false,
    })?;
    let member_add = wiki_request_json(api, Method::POST, &path, &query, Some(body), auth).await?;
    let member_list = probe_value(
        wiki_request_json(
            api,
            Method::GET,
            &path,
            &[("page_size".to_string(), "50".to_string())],
            None,
            auth,
        )
        .await,
    );
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "space_id": space_id,
            "member_type": "openid",
            "member_id": open_id,
            "member_role": member_role,
            "auth": format!("{auth:?}").to_lowercase(),
            "bot": bot,
            "member_add": member_add,
            "member_list": member_list,
            "next_actions": [
                "Rerun `feishu-bot office bootstrap --project <project> --send-summary` without --skip-wiki.",
                "Rerun `feishu-bot wiki route-check --write-probe --strict` to prove Wiki write readiness."
            ],
        }
    }))
}

fn wiki_member_has_role(member_list: &Value, open_id: &str, role: &str) -> bool {
    member_list
        .pointer("/response/data/members")
        .and_then(Value::as_array)
        .is_some_and(|members| {
            members.iter().any(|member| {
                member.get("member_id").and_then(Value::as_str) == Some(open_id)
                    && member.get("member_type").and_then(Value::as_str) == Some("openid")
                    && (member.get("member_role").and_then(Value::as_str) == Some(role)
                        || member.get("member_perm").and_then(Value::as_str) == Some(role))
            })
        })
}

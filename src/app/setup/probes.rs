use super::wiki::run_setup_wiki_bot;
use super::*;

pub(super) async fn setup_wiki_bot_probe(
    config: &Result<Config>,
    should_skip: bool,
    space_id: Option<String>,
) -> Value {
    if should_skip {
        return json!({ "status": "skipped" });
    }
    match config {
        Ok(config) => {
            let mut api = FeishuClient::new(config.clone());
            probe_value(
                run_setup_wiki_bot(
                    &mut api,
                    space_id,
                    "admin".to_string(),
                    Some(false),
                    ApiAuthArg::User,
                )
                .await,
            )
        }
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    }
}

pub(super) async fn setup_doctor_probe(config: &Config) -> Value {
    let mut api = FeishuClient::new(config.clone());
    match api.tenant_token().await {
        Ok(_) => json!({
            "ok": true,
            "tenant": {
                "tenant_access_token_configured": true,
                "base_url": config.base_url.clone(),
                "app_id": mask_app_id(&config.app_id),
                "default_user_id_configured": config.default_user_id.is_some(),
                "user_access_token_configured": config.user_access_token.is_some(),
                "wiki_space_id_configured": config.default_wiki_space_id.is_some(),
            }
        }),
        Err(error) => json!({
            "ok": false,
            "tenant": {
                "tenant_access_token_configured": false,
                "base_url": config.base_url.clone(),
                "app_id": mask_app_id(&config.app_id),
                "default_user_id_configured": config.default_user_id.is_some(),
                "user_access_token_configured": config.user_access_token.is_some(),
                "wiki_space_id_configured": config.default_wiki_space_id.is_some(),
                "error": format!("{error:#}"),
            }
        }),
    }
}

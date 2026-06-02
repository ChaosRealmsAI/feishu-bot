use super::*;

pub(super) async fn run_doctor_command(config: &Config, raw_json: bool) -> Result<()> {
    let mut api = FeishuClient::new(config.clone());
    let token = api.tenant_token().await?;
    let default_user_id = config
        .default_user_id
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let user_access_token = config
        .user_access_token
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let helpdesk_id = config
        .helpdesk_id
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let helpdesk_token = config
        .helpdesk_token
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let token_mask = mask_secret(&token);

    if raw_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "base_url": config.base_url,
                "app_id": mask_app_id(&config.app_id),
                "app_secret": mask_secret(&config.app_secret),
                "default_user_id": default_user_id,
                "doc_base_url": config.doc_base_url,
                "user_access_token": user_access_token,
                "helpdesk_id": helpdesk_id,
                "helpdesk_token": helpdesk_token,
                "tenant_access_token": token_mask,
                "ok": true,
            }))?
        );
    } else {
        println!("base_url={}", config.base_url);
        println!("app_id={}", mask_app_id(&config.app_id));
        println!("app_secret={}", mask_secret(&config.app_secret));
        println!("default_user_id={default_user_id}");
        println!("doc_base_url={}", config.doc_base_url);
        println!("user_access_token={user_access_token}");
        println!("helpdesk_id={helpdesk_id}");
        println!("helpdesk_token={helpdesk_token}");
        println!("tenant_access_token={token_mask} ");
    }
    Ok(())
}

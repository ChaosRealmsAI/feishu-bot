use super::output::finalize_oauth_token_response;
use super::request::{read_oauth_json, request_oauth_token};
use super::url::resolve_oauth_redirect_uri;
use super::*;

pub(super) async fn exchange_oauth_code(config: &Config, args: OauthTokenArgs) -> Result<Value> {
    let redirect_uri = resolve_oauth_redirect_uri(args.redirect_uri)?;
    let mut body = json!({
        "grant_type": "authorization_code",
        "client_id": config.app_id,
        "client_secret": config.app_secret,
        "code": args.code,
        "redirect_uri": redirect_uri,
    });
    if let Some(verifier) = args.code_verifier.filter(|value| !value.trim().is_empty()) {
        body["code_verifier"] = Value::String(verifier);
    }
    let response = request_oauth_token(config, body).await?;
    finalize_oauth_token_response(
        response,
        args.raw,
        args.print_env,
        args.save_env,
        args.env_file,
    )
}

pub(in crate::app) async fn refresh_oauth_token(
    config: &Config,
    args: OauthRefreshArgs,
) -> Result<Value> {
    let refresh_token = args
        .refresh_token
        .or_else(|| {
            load_env_values().ok().and_then(|values| {
                get_any(&values, &["FEISHU_REFRESH_TOKEN", "LARK_REFRESH_TOKEN"])
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "oauth refresh needs --refresh-token or FEISHU_REFRESH_TOKEN/LARK_REFRESH_TOKEN"
            )
        })?;
    let response = request_oauth_token(
        config,
        json!({
            "grant_type": "refresh_token",
            "client_id": config.app_id,
            "client_secret": config.app_secret,
            "refresh_token": refresh_token,
        }),
    )
    .await?;
    finalize_oauth_token_response(
        response,
        args.raw,
        args.print_env,
        args.save_env,
        args.env_file,
    )
}

pub(super) async fn get_oauth_user_info(config: &Config, args: OauthUserInfoArgs) -> Result<Value> {
    let token = args
        .access_token
        .or_else(|| config.user_access_token.clone())
        .ok_or_else(|| {
            anyhow!("oauth user-info needs --access-token or FEISHU_USER_ACCESS_TOKEN")
        })?;
    let url = format!("{}/authen/v1/user_info", config.base_url);
    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .context("GET /authen/v1/user_info")?;
    read_oauth_json(response).await
}

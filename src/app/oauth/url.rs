use super::*;
use base64::Engine;

const DEFAULT_OAUTH_REDIRECT_URI: &str = "http://localhost:8080/callback";
const DEFAULT_OAUTH_SCOPES: &[&str] = &["offline_access", "auth:user.id:read"];

pub(super) fn build_oauth_url_response(config: &Config, args: OauthUrlArgs) -> Result<Value> {
    let redirect_uri = resolve_oauth_redirect_uri(args.redirect_uri)?;
    let scopes = resolve_oauth_scopes(args.scope);
    let state = args
        .state
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(random_uuid);
    let code_verifier = if args.no_pkce {
        args.code_verifier
    } else {
        Some(
            args.code_verifier
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(generate_code_verifier),
        )
    };

    let mut url = reqwest::Url::parse(&oauth_authorize_url(config))?;
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("client_id", &config.app_id);
        query.append_pair("response_type", "code");
        query.append_pair("redirect_uri", &redirect_uri);
        query.append_pair("scope", &scopes.join(" "));
        query.append_pair("state", &state);
        if let Some(verifier) = &code_verifier {
            query.append_pair("code_challenge", &code_challenge_s256(verifier));
            query.append_pair("code_challenge_method", "S256");
        }
    }

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "authorization_url": url.as_str(),
            "redirect_uri": redirect_uri,
            "scope": scopes,
            "state": state,
            "code_verifier": code_verifier,
            "code_challenge_method": if args.no_pkce { Value::Null } else { Value::String("S256".to_string()) },
            "next_steps": [
                "Open authorization_url in a signed-in browser.",
                "After Feishu redirects to redirect_uri, copy the code query parameter.",
                "Run feishu-bot oauth token --code <code> --code-verifier <code_verifier> --save-env.",
                "Run feishu-bot --json dogfood verify --module task --include-response."
            ],
            "browser_command": format!("feishu-bot browser open --url \"{}\"", url.as_str()),
        }
    }))
}

pub(super) fn resolve_oauth_redirect_uri(explicit: Option<String>) -> Result<String> {
    if let Some(uri) = explicit.filter(|value| !value.trim().is_empty()) {
        return Ok(uri);
    }
    let values = load_env_values().unwrap_or_default();
    Ok(get_any(
        &values,
        &["FEISHU_OAUTH_REDIRECT_URI", "LARK_OAUTH_REDIRECT_URI"],
    )
    .unwrap_or_else(|| DEFAULT_OAUTH_REDIRECT_URI.to_string()))
}

pub(in crate::app) fn resolve_oauth_scopes(values: Vec<String>) -> Vec<String> {
    let scopes = values
        .into_iter()
        .flat_map(|value| {
            value
                .split_whitespace()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|scope| !scope.trim().is_empty())
        .collect::<Vec<_>>();
    if scopes.is_empty() {
        DEFAULT_OAUTH_SCOPES
            .iter()
            .map(ToString::to_string)
            .collect()
    } else {
        scopes
    }
}

pub(in crate::app) fn oauth_authorize_url(config: &Config) -> String {
    if config.base_url.contains("larksuite.com") {
        "https://accounts.larksuite.com/open-apis/authen/v1/authorize".to_string()
    } else {
        "https://accounts.feishu.cn/open-apis/authen/v1/authorize".to_string()
    }
}

fn generate_code_verifier() -> String {
    format!(
        "{}{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    )
}

pub(in crate::app) fn code_challenge_s256(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

use super::*;
use base64::Engine;

const DEFAULT_OAUTH_REDIRECT_URI: &str = "http://localhost:8080/callback";
const DEFAULT_OAUTH_SCOPES: &[&str] = &["offline_access", "auth:user.id:read"];

pub(super) async fn run_oauth_command(
    config: &Config,
    command: OauthCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OauthCommand::Url(args) => build_oauth_url_response(config, args)?,
        OauthCommand::Token(args) => exchange_oauth_code(config, args).await?,
        OauthCommand::Refresh(args) => refresh_oauth_token(config, args).await?,
        OauthCommand::UserInfo(args) => get_oauth_user_info(config, args).await?,
    };
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&data)?);
    } else {
        print_oauth_response(&data)?;
    }
    Ok(())
}

fn build_oauth_url_response(config: &Config, args: OauthUrlArgs) -> Result<Value> {
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

async fn exchange_oauth_code(config: &Config, args: OauthTokenArgs) -> Result<Value> {
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

async fn refresh_oauth_token(config: &Config, args: OauthRefreshArgs) -> Result<Value> {
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

async fn get_oauth_user_info(config: &Config, args: OauthUserInfoArgs) -> Result<Value> {
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

async fn request_oauth_token(config: &Config, body: Value) -> Result<Value> {
    let url = format!("{}/authen/v2/oauth/token", config.base_url);
    let response = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await
        .context("POST /authen/v2/oauth/token")?;
    read_oauth_json(response).await
}

async fn read_oauth_json(response: reqwest::Response) -> Result<Value> {
    let status = response.status();
    let text = response.text().await.context("read OAuth response")?;
    let json: Value =
        serde_json::from_str(&text).with_context(|| format!("parse OAuth response: {text}"))?;
    if !status.is_success() {
        bail!(
            "Feishu OAuth HTTP {status}: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    if json
        .get("error")
        .and_then(Value::as_str)
        .is_some_and(|value| !value.is_empty())
    {
        bail!(
            "Feishu OAuth failed: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    Ok(json)
}

fn finalize_oauth_token_response(
    response: Value,
    raw: bool,
    print_env: bool,
    save_env: bool,
    env_file: Option<PathBuf>,
) -> Result<Value> {
    let mut output = if raw {
        response.clone()
    } else {
        mask_oauth_token_response(&response)
    };
    let access_token = get_string(&response, &["access_token"]);
    let refresh_token = get_string(&response, &["refresh_token"]);
    if print_env {
        output["env"] = oauth_env_lines(access_token.as_deref(), refresh_token.as_deref());
    }
    if save_env {
        let path = env_file.unwrap_or_else(|| PathBuf::from(".env"));
        save_oauth_tokens_to_env(&path, access_token.as_deref(), refresh_token.as_deref())?;
        output["saved_env_file"] = Value::String(path.display().to_string());
    }
    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": output,
    }))
}

pub(super) fn mask_oauth_token_response(response: &Value) -> Value {
    let mut masked = response.clone();
    for key in ["access_token", "refresh_token", "id_token"] {
        if let Some(value) = masked.get_mut(key) {
            if let Some(token) = value.as_str() {
                *value = Value::String(mask_secret(token));
            }
        }
    }
    masked
}

fn oauth_env_lines(access_token: Option<&str>, refresh_token: Option<&str>) -> Value {
    let mut lines = Vec::new();
    if let Some(token) = access_token {
        lines.push(format!("export FEISHU_USER_ACCESS_TOKEN={token}"));
    }
    if let Some(token) = refresh_token {
        lines.push(format!("export FEISHU_REFRESH_TOKEN={token}"));
    }
    Value::Array(lines.into_iter().map(Value::String).collect())
}

fn save_oauth_tokens_to_env(
    path: &Path,
    access_token: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<()> {
    let mut updates = Vec::new();
    if let Some(token) = access_token {
        updates.push(("FEISHU_USER_ACCESS_TOKEN", token.to_string()));
    }
    if let Some(token) = refresh_token {
        updates.push(("FEISHU_REFRESH_TOKEN", token.to_string()));
    }
    if updates.is_empty() {
        bail!("OAuth response did not include access_token or refresh_token");
    }
    let existing = if path.exists() {
        fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?
    } else {
        String::new()
    };
    let mut handled = vec![false; updates.len()];
    let mut lines = Vec::new();
    for line in existing.lines() {
        let mut replaced = false;
        for (index, (key, value)) in updates.iter().enumerate() {
            if line.starts_with(&format!("{key}=")) {
                lines.push(format!("{key}={value}"));
                handled[index] = true;
                replaced = true;
                break;
            }
        }
        if !replaced {
            lines.push(line.to_string());
        }
    }
    for (handled, (key, value)) in handled.into_iter().zip(updates) {
        if !handled {
            lines.push(format!("{key}={value}"));
        }
    }
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
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

pub(super) fn resolve_oauth_scopes(values: Vec<String>) -> Vec<String> {
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

pub(super) fn oauth_authorize_url(config: &Config) -> String {
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

pub(super) fn code_challenge_s256(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

fn print_oauth_response(value: &Value) -> Result<()> {
    let data = value.get("data").unwrap_or(value);
    for (label, path) in [
        ("authorization_url", &["authorization_url"][..]),
        ("browser_command", &["browser_command"][..]),
        ("redirect_uri", &["redirect_uri"][..]),
        ("state", &["state"][..]),
        ("code_verifier", &["code_verifier"][..]),
        ("access_token", &["access_token"][..]),
        ("refresh_token", &["refresh_token"][..]),
        ("scope", &["scope"][..]),
        ("saved_env_file", &["saved_env_file"][..]),
    ] {
        if let Some(output) = get_string(data, path) {
            println!("{label}={output}");
        }
    }
    if let Some(scope) = data.get("scope").and_then(Value::as_array) {
        let items = scope.iter().filter_map(Value::as_str).collect::<Vec<_>>();
        if !items.is_empty() {
            println!("scope={}", items.join(" "));
        }
    }
    if let Some(lines) = data.get("env").and_then(Value::as_array) {
        for line in lines.iter().filter_map(Value::as_str) {
            println!("{line}");
        }
    }
    Ok(())
}

use super::*;
use base64::Engine;

#[derive(Debug, Deserialize)]
struct TenantTokenResponse {
    code: i64,
    msg: Option<String>,
    tenant_access_token: Option<String>,
    expire: Option<i64>,
}

impl FeishuClient {
    pub(in crate::app) async fn tenant_token(&mut self) -> Result<String> {
        if let Some(token) = &self.tenant_token {
            return Ok(token.clone());
        }

        let url = format!(
            "{}/auth/v3/tenant_access_token/internal",
            self.config.base_url
        );
        let res = self
            .http
            .post(url)
            .json(&json!({
                "app_id": self.config.app_id,
                "app_secret": self.config.app_secret,
            }))
            .send()
            .await
            .context("request tenant_access_token")?;
        let status = res.status();
        let text = res.text().await.context("read tenant token response")?;
        if !status.is_success() {
            bail!("tenant token HTTP {status}: {text}");
        }
        let parsed: TenantTokenResponse =
            serde_json::from_str(&text).context("parse tenant token response")?;
        if parsed.code != 0 {
            bail!(
                "tenant token failed: code={} msg={}",
                parsed.code,
                parsed.msg.unwrap_or_default()
            );
        }
        let _expires_in = parsed.expire;
        let token = parsed
            .tenant_access_token
            .ok_or_else(|| anyhow!("tenant token response missing tenant_access_token"))?;
        self.tenant_token = Some(token.clone());
        Ok(token)
    }

    pub(in crate::app) async fn token_for_api_auth(&mut self, auth: ApiAuthArg) -> Result<String> {
        match auth {
            ApiAuthArg::Tenant => self.tenant_token().await,
            ApiAuthArg::User => self.user_access_token(),
        }
    }

    pub(in crate::app) fn user_access_token(&self) -> Result<String> {
        self.config.user_access_token.clone().ok_or_else(|| {
            anyhow!(
                "this Feishu API requires user_access_token; set FEISHU_USER_ACCESS_TOKEN or LARK_USER_ACCESS_TOKEN"
            )
        })
    }

    pub(in crate::app) fn helpdesk_auth_header(&self) -> Result<String> {
        let helpdesk_id = self.config.helpdesk_id.as_deref().ok_or_else(|| {
            anyhow!("helpdesk APIs require FEISHU_HELPDESK_ID or LARK_HELPDESK_ID")
        })?;
        let helpdesk_token = self.config.helpdesk_token.as_deref().ok_or_else(|| {
            anyhow!("helpdesk APIs require FEISHU_HELPDESK_TOKEN or LARK_HELPDESK_TOKEN")
        })?;
        let auth_info = format!("{helpdesk_id}:{helpdesk_token}");
        Ok(base64::engine::general_purpose::STANDARD.encode(auth_info))
    }
}

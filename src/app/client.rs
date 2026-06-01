#![allow(clippy::too_many_arguments)]

use super::*;
use base64::Engine;

mod documents;

pub(super) struct FeishuClient {
    http: reqwest::Client,
    pub(super) config: Config,
    tenant_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TenantTokenResponse {
    code: i64,
    msg: Option<String>,
    tenant_access_token: Option<String>,
    expire: Option<i64>,
}

impl FeishuClient {
    pub(super) fn new(config: Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
            tenant_token: None,
        }
    }

    pub(super) async fn tenant_token(&mut self) -> Result<String> {
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

    pub(super) async fn post_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::POST, path, query, Some(body))
            .await
    }

    pub(super) async fn post_json_user(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        let token = self.user_access_token()?;
        self.request_json_with_token(Method::POST, path, query, Some(body), token)
            .await
    }

    pub(super) async fn post_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::POST, path, query, Some(body), auth, &[])
            .await
    }

    pub(super) async fn get_json_user(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        let token = self.user_access_token()?;
        self.request_json_with_token(Method::GET, path, query, None, token)
            .await
    }

    pub(super) async fn get_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::GET, path, query, None, auth, &[])
            .await
    }

    pub(super) async fn put_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::PUT, path, query, Some(body))
            .await
    }

    pub(super) async fn patch_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::PATCH, path, query, Some(body))
            .await
    }

    pub(super) async fn patch_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::PATCH, path, query, Some(body), auth, &[])
            .await
    }

    pub(super) async fn delete_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        self.request_json(Method::DELETE, path, query, body).await
    }

    pub(super) async fn get_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        self.request_json(Method::GET, path, query, None).await
    }

    pub(super) async fn get_helpdesk_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        self.request_helpdesk_json(Method::GET, path, query, None)
            .await
    }

    pub(super) async fn post_helpdesk_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_helpdesk_json(Method::POST, path, query, Some(body))
            .await
    }

    pub(super) async fn request_json(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        let token = self.tenant_token().await?;
        self.request_json_with_token(method, path, query, body, token)
            .await
    }

    pub(super) async fn token_for_api_auth(&mut self, auth: ApiAuthArg) -> Result<String> {
        match auth {
            ApiAuthArg::Tenant => self.tenant_token().await,
            ApiAuthArg::User => self.user_access_token(),
        }
    }

    pub(super) async fn request_helpdesk_json(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        if !path.starts_with('/') {
            bail!("OpenAPI path must start with /: {path}");
        }
        let token = self.tenant_token().await?;
        let helpdesk_auth = self.helpdesk_auth_header()?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self
            .http
            .request(method.clone(), url)
            .bearer_auth(token)
            .header("X-Lark-Helpdesk-Authorization", helpdesk_auth)
            .query(query);
        if let Some(body) = body {
            request = request.json(&body);
        }
        let method_label = method.as_str().to_string();
        let res = request
            .send()
            .await
            .with_context(|| format!("{method_label} {path}"))?;
        read_feishu_json(res).await
    }

    pub(super) async fn request_json_with_token(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        token: String,
    ) -> Result<Value> {
        self.request_json_with_token_and_headers(method, path, query, body, token, &[])
            .await
    }

    pub(super) async fn request_json_with_token_and_headers(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        token: String,
        headers: &[(String, String)],
    ) -> Result<Value> {
        if !path.starts_with('/') {
            bail!("OpenAPI path must start with /: {path}");
        }
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self
            .http
            .request(method.clone(), url)
            .bearer_auth(token)
            .query(query);
        for (key, value) in headers {
            request = request.header(key.as_str(), value.as_str());
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let method_label = method.as_str().to_string();
        let res = request
            .send()
            .await
            .with_context(|| format!("{method_label} {path}"))?;
        read_feishu_json(res).await
    }

    pub(super) async fn request_json_with_auth(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        auth: ApiAuthArg,
        headers: &[(String, String)],
    ) -> Result<Value> {
        let token = self.token_for_api_auth(auth).await?;
        self.request_json_with_token_and_headers(method, path, query, body, token, headers)
            .await
    }

    pub(super) async fn request_binary_with_auth(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        auth: ApiAuthArg,
        headers: &[(String, String)],
        range: Option<&str>,
    ) -> Result<Vec<u8>> {
        if !path.starts_with('/') {
            bail!("OpenAPI path must start with /: {path}");
        }
        let token = self.token_for_api_auth(auth).await?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut request = self
            .http
            .request(method.clone(), url)
            .bearer_auth(token)
            .query(query);
        for (key, value) in headers {
            request = request.header(key.as_str(), value.as_str());
        }
        if let Some(range) = range.filter(|value| !value.trim().is_empty()) {
            request = request.header(reqwest::header::RANGE, range);
        }
        let method_label = method.as_str().to_string();
        let res = request
            .send()
            .await
            .with_context(|| format!("{method_label} {path}"))?;
        read_binary_response(res).await
    }

    pub(super) async fn request_multipart_with_auth(
        &mut self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        fields: Vec<(String, String)>,
        files: Vec<(String, PathBuf)>,
        auth: ApiAuthArg,
        headers: &[(String, String)],
    ) -> Result<Value> {
        if !path.starts_with('/') {
            bail!("OpenAPI path must start with /: {path}");
        }
        if fields.is_empty() && files.is_empty() {
            bail!("multipart request needs at least one --field or --file");
        }
        let token = self.token_for_api_auth(auth).await?;
        let url = format!("{}{}", self.config.base_url, path);
        let mut form = reqwest::multipart::Form::new();
        for (key, value) in fields {
            form = form.text(key, value);
        }
        for (part_name, path) in files {
            let file_name = drive_upload_file_name(&path, None)?;
            let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
            form = form.part(
                part_name,
                reqwest::multipart::Part::bytes(bytes).file_name(file_name),
            );
        }
        let mut request = self
            .http
            .request(method.clone(), url)
            .bearer_auth(token)
            .query(query)
            .multipart(form);
        for (key, value) in headers {
            request = request.header(key.as_str(), value.as_str());
        }
        let method_label = method.as_str().to_string();
        let res = request
            .send()
            .await
            .with_context(|| format!("{method_label} {path} multipart"))?;
        read_feishu_json(res).await
    }

    pub(super) fn user_access_token(&self) -> Result<String> {
        self.config.user_access_token.clone().ok_or_else(|| {
            anyhow!(
                "this Feishu API requires user_access_token; set FEISHU_USER_ACCESS_TOKEN or LARK_USER_ACCESS_TOKEN"
            )
        })
    }

    pub(super) fn helpdesk_auth_header(&self) -> Result<String> {
        let helpdesk_id = self.config.helpdesk_id.as_deref().ok_or_else(|| {
            anyhow!("helpdesk APIs require FEISHU_HELPDESK_ID or LARK_HELPDESK_ID")
        })?;
        let helpdesk_token = self.config.helpdesk_token.as_deref().ok_or_else(|| {
            anyhow!("helpdesk APIs require FEISHU_HELPDESK_TOKEN or LARK_HELPDESK_TOKEN")
        })?;
        let auth_info = format!("{helpdesk_id}:{helpdesk_token}");
        Ok(base64::engine::general_purpose::STANDARD.encode(auth_info))
    }

    pub(super) async fn upload_drive_file(
        &mut self,
        file_path: &Path,
        file_name: String,
        parent_type: String,
        parent_node: String,
        checksum: Option<String>,
    ) -> Result<Value> {
        let metadata =
            fs::metadata(file_path).with_context(|| format!("stat {}", file_path.display()))?;
        let size = metadata.len();
        validate_drive_upload_size(size)?;
        let bytes = fs::read(file_path).with_context(|| format!("read {}", file_path.display()))?;
        let token = self.tenant_token().await?;
        let url = format!("{}/drive/v1/files/upload_all", self.config.base_url);
        let mut form = reqwest::multipart::Form::new()
            .text("file_name", file_name.clone())
            .text("parent_type", parent_type)
            .text("parent_node", parent_node)
            .text("size", size.to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name),
            );
        if let Some(checksum) = checksum.filter(|value| !value.trim().is_empty()) {
            form = form.text("checksum", checksum);
        }
        let res = self
            .http
            .post(url)
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .context("POST /drive/v1/files/upload_all")?;
        read_feishu_json(res).await
    }

    pub(super) async fn upload_drive_file_part(
        &mut self,
        upload_id: &str,
        seq: i64,
        bytes: &[u8],
        file_name: &str,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let token = self.token_for_api_auth(auth).await?;
        let url = format!("{}/drive/v1/files/upload_part", self.config.base_url);
        let form = reqwest::multipart::Form::new()
            .text("upload_id", upload_id.to_string())
            .text("seq", seq.to_string())
            .text("size", bytes.len().to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes.to_vec()).file_name(file_name.to_string()),
            );
        let res = self
            .http
            .post(url)
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .with_context(|| format!("POST /drive/v1/files/upload_part seq={seq}"))?;
        read_feishu_json(res).await
    }

    pub(super) async fn upload_drive_media(
        &mut self,
        file_path: &Path,
        file_name: String,
        parent_type: String,
        parent_node: String,
        checksum: Option<String>,
        extra: Option<String>,
    ) -> Result<Value> {
        let metadata =
            fs::metadata(file_path).with_context(|| format!("stat {}", file_path.display()))?;
        let size = metadata.len();
        validate_upload_size(size, 20 * 1024 * 1024, "drive media upload")?;
        let bytes = fs::read(file_path).with_context(|| format!("read {}", file_path.display()))?;
        let token = self.tenant_token().await?;
        let url = format!("{}/drive/v1/medias/upload_all", self.config.base_url);
        let mut form = reqwest::multipart::Form::new()
            .text("file_name", file_name.clone())
            .text("parent_type", parent_type)
            .text("parent_node", parent_node)
            .text("size", size.to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name),
            );
        if let Some(checksum) = checksum.filter(|value| !value.trim().is_empty()) {
            form = form.text("checksum", checksum);
        }
        if let Some(extra) = extra.filter(|value| !value.trim().is_empty()) {
            form = form.text("extra", extra);
        }
        let res = self
            .http
            .post(url)
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .context("POST /drive/v1/medias/upload_all")?;
        read_feishu_json(res).await
    }

    pub(super) async fn download_drive_file(
        &mut self,
        file_token: &str,
        range: Option<&str>,
    ) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!(
            "{}/drive/v1/files/{}/download",
            self.config.base_url, file_token
        );
        let mut request = self.http.get(url).bearer_auth(token);
        if let Some(range) = range.filter(|value| !value.trim().is_empty()) {
            request = request.header(reqwest::header::RANGE, range);
        }
        let res = request
            .send()
            .await
            .with_context(|| format!("GET /drive/v1/files/{file_token}/download"))?;
        read_binary_response(res).await
    }

    pub(super) async fn download_drive_media(
        &mut self,
        file_token: &str,
        range: Option<&str>,
        extra: Option<&str>,
    ) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!(
            "{}/drive/v1/medias/{}/download",
            self.config.base_url, file_token
        );
        let mut query = Vec::new();
        if let Some(extra) = extra.filter(|value| !value.trim().is_empty()) {
            query.push(("extra".to_string(), extra.to_string()));
        }
        let mut request = self.http.get(url).bearer_auth(token).query(&query);
        if let Some(range) = range.filter(|value| !value.trim().is_empty()) {
            request = request.header(reqwest::header::RANGE, range);
        }
        let res = request
            .send()
            .await
            .with_context(|| format!("GET /drive/v1/medias/{file_token}/download"))?;
        read_binary_response(res).await
    }

    pub(super) async fn download_message_resource(
        &mut self,
        message_id: &str,
        file_key: &str,
        resource_type: &str,
    ) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!(
            "{}/im/v1/messages/{}/resources/{}",
            self.config.base_url, message_id, file_key
        );
        let res = self
            .http
            .get(url)
            .bearer_auth(token)
            .query(&[("type", resource_type)])
            .send()
            .await
            .with_context(|| format!("GET /im/v1/messages/{message_id}/resources/{file_key}"))?;
        read_binary_response(res).await
    }

    pub(super) async fn upload_im_image(
        &mut self,
        file_path: &Path,
        image_type: &str,
    ) -> Result<Value> {
        let metadata =
            fs::metadata(file_path).with_context(|| format!("stat {}", file_path.display()))?;
        validate_upload_size(metadata.len(), 10 * 1024 * 1024, "message image upload")?;
        let file_name = drive_upload_file_name(file_path, None)?;
        let bytes = fs::read(file_path).with_context(|| format!("read {}", file_path.display()))?;
        let token = self.tenant_token().await?;
        let url = format!("{}/im/v1/images", self.config.base_url);
        let form = reqwest::multipart::Form::new()
            .text("image_type", image_type.to_string())
            .part(
                "image",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name),
            );
        let res = self
            .http
            .post(url)
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .context("POST /im/v1/images")?;
        read_feishu_json(res).await
    }

    pub(super) async fn upload_im_file(
        &mut self,
        file_path: &Path,
        file_name: String,
        file_type: &str,
        duration: Option<u64>,
    ) -> Result<Value> {
        let metadata =
            fs::metadata(file_path).with_context(|| format!("stat {}", file_path.display()))?;
        validate_upload_size(metadata.len(), 30 * 1024 * 1024, "message file upload")?;
        let bytes = fs::read(file_path).with_context(|| format!("read {}", file_path.display()))?;
        let token = self.tenant_token().await?;
        let url = format!("{}/im/v1/files", self.config.base_url);
        let mut form = reqwest::multipart::Form::new()
            .text("file_type", file_type.to_string())
            .text("file_name", file_name.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(file_name),
            );
        if let Some(duration) = duration {
            form = form.text("duration", duration.to_string());
        }
        let res = self
            .http
            .post(url)
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await
            .context("POST /im/v1/files")?;
        read_feishu_json(res).await
    }

    pub(super) async fn download_im_image(&mut self, image_key: &str) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!("{}/im/v1/images/{}", self.config.base_url, image_key);
        let res = self
            .http
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .with_context(|| format!("GET /im/v1/images/{image_key}"))?;
        read_binary_response(res).await
    }

    pub(super) async fn download_im_file(&mut self, file_key: &str) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!("{}/im/v1/files/{}", self.config.base_url, file_key);
        let res = self
            .http
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .with_context(|| format!("GET /im/v1/files/{file_key}"))?;
        read_binary_response(res).await
    }

    pub(super) async fn download_minutes_transcript(
        &mut self,
        minute_token: &str,
        query: &[(String, String)],
    ) -> Result<Vec<u8>> {
        let token = self.tenant_token().await?;
        let url = format!(
            "{}/minutes/v1/minutes/{}/transcript",
            self.config.base_url, minute_token
        );
        let res = self
            .http
            .get(url)
            .bearer_auth(token)
            .query(query)
            .send()
            .await
            .with_context(|| format!("GET /minutes/v1/minutes/{minute_token}/transcript"))?;
        read_binary_response(res).await
    }

    pub(super) async fn send_text(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        text: &str,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": "text",
            "content": json!({ "text": text }).to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(super) async fn send_interactive(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        card: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": "interactive",
            "content": card.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(super) async fn send_message_json(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        msg_type: &str,
        content: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": msg_type,
            "content": content.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(super) async fn reply_message_json(
        &mut self,
        message_id: &str,
        msg_type: &str,
        content: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}/reply");
        let mut body = json!({
            "msg_type": msg_type,
            "content": content.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(&path, &[], body).await
    }

    pub(super) async fn edit_message_json(
        &mut self,
        message_id: &str,
        msg_type: &str,
        content: Value,
    ) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}");
        self.put_json(
            &path,
            &[],
            json!({
                "msg_type": msg_type,
                "content": content.to_string(),
            }),
        )
        .await
    }

    pub(super) async fn delete_message(&mut self, message_id: &str) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}");
        self.delete_json(&path, &[], None).await
    }

    pub(super) async fn create_chat(
        &mut self,
        name: &str,
        description: Option<&str>,
        users: &[String],
        user_id_type: &str,
    ) -> Result<Value> {
        let mut body = json!({
            "name": name,
            "chat_mode": "group",
            "chat_type": "private",
            "group_message_type": "chat",
        });
        if let Some(description) = description {
            body["description"] = Value::String(description.to_string());
        }
        if !users.is_empty() {
            body["user_id_list"] =
                Value::Array(users.iter().map(|u| Value::String(u.clone())).collect());
        }
        self.post_json(
            "/im/v1/chats",
            &[("user_id_type".to_string(), user_id_type.to_string())],
            body,
        )
        .await
    }

    pub(super) async fn import_board_syntax(
        &mut self,
        whiteboard_id: &str,
        syntax: BoardSyntaxArg,
        code: &str,
        style_type: u8,
        diagram_type: u8,
        client_token: Option<String>,
    ) -> Result<Value> {
        let path = format!("/board/v1/whiteboards/{whiteboard_id}/nodes/plantuml");
        let mut query = Vec::new();
        push_query_opt(&mut query, "client_token", client_token);
        self.post_json(
            &path,
            &query,
            json!({
                "plant_uml_code": code,
                "style_type": style_type,
                "syntax_type": syntax.as_api_value(),
                "diagram_type": diagram_type,
            }),
        )
        .await
    }

    pub(super) async fn create_board_nodes(
        &mut self,
        whiteboard_id: &str,
        body: Value,
        user_id_type: UserIdTypeArg,
        client_token: Option<String>,
    ) -> Result<Value> {
        let path = format!("/board/v1/whiteboards/{whiteboard_id}/nodes");
        let mut query = vec![(
            "user_id_type".to_string(),
            user_id_type.resolve(None).to_string(),
        )];
        push_query_opt(&mut query, "client_token", client_token);
        self.post_json(&path, &query, body).await
    }
}

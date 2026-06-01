use super::*;

impl FeishuClient {
    pub(in crate::app) async fn upload_drive_file(
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

    pub(in crate::app) async fn upload_drive_file_part(
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

    pub(in crate::app) async fn upload_drive_media(
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

    pub(in crate::app) async fn download_drive_file(
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

    pub(in crate::app) async fn download_drive_media(
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

    pub(in crate::app) async fn download_message_resource(
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

    pub(in crate::app) async fn upload_im_image(
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

    pub(in crate::app) async fn upload_im_file(
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

    pub(in crate::app) async fn download_im_image(&mut self, image_key: &str) -> Result<Vec<u8>> {
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

    pub(in crate::app) async fn download_im_file(&mut self, file_key: &str) -> Result<Vec<u8>> {
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

    pub(in crate::app) async fn download_minutes_transcript(
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
}

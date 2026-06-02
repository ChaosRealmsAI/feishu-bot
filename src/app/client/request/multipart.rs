use super::*;

impl FeishuClient {
    pub(in crate::app) async fn request_multipart_with_auth(
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
}

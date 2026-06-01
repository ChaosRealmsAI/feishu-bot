use super::*;

impl FeishuClient {
    pub(in crate::app) async fn create_document(
        &mut self,
        title: &str,
        folder_token: Option<&str>,
    ) -> Result<Value> {
        self.create_document_with_auth(title, folder_token, ApiAuthArg::Tenant)
            .await
    }

    pub(in crate::app) async fn create_document_with_auth(
        &mut self,
        title: &str,
        folder_token: Option<&str>,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let mut body = json!({ "title": title });
        if let Some(folder_token) = folder_token {
            if !folder_token.trim().is_empty() {
                body["folder_token"] = Value::String(folder_token.to_string());
            }
        }
        self.post_json_auth("/docx/v1/documents", &[], body, auth)
            .await
    }

    pub(in crate::app) async fn append_document(
        &mut self,
        document_id: &str,
        block_id: &str,
        content: &str,
    ) -> Result<Value> {
        self.append_document_with_auth(document_id, block_id, content, ApiAuthArg::Tenant)
            .await
    }

    pub(in crate::app) async fn append_document_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        content: &str,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let blocks = markdown_to_blocks(content);
        if blocks.is_empty() {
            return Ok(json!({ "code": 0, "msg": "empty content, nothing appended" }));
        }

        let mut last = Value::Null;
        for chunk in blocks.chunks(50) {
            let path = format!(
                "/docx/v1/documents/{}/blocks/{}/children",
                document_id, block_id
            );
            last = self
                .post_json_auth(
                    &path,
                    &[("document_revision_id".to_string(), "-1".to_string())],
                    json!({
                        "index": -1,
                        "children": chunk,
                    }),
                    auth,
                )
                .await?;
        }
        Ok(last)
    }

    pub(in crate::app) async fn convert_content(
        &mut self,
        content_type: ContentTypeArg,
        content: &str,
    ) -> Result<Value> {
        self.post_json(
            "/docx/v1/documents/blocks/convert",
            &[],
            json!({
                "content_type": content_type.as_api_value(),
                "content": content,
            }),
        )
        .await
    }

    pub(in crate::app) async fn append_converted_content(
        &mut self,
        document_id: &str,
        block_id: &str,
        content_type: ContentTypeArg,
        content: &str,
    ) -> Result<Value> {
        self.append_converted_content_with_auth(
            document_id,
            block_id,
            content_type,
            content,
            ApiAuthArg::Tenant,
        )
        .await
    }

    pub(in crate::app) async fn append_converted_content_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        content_type: ContentTypeArg,
        content: &str,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let converted = self.convert_content(content_type, content).await?;
        let body = converted_to_descendant_body(converted)?;
        self.append_descendant_body_with_auth(document_id, block_id, body, auth)
            .await
    }

    pub(in crate::app) async fn append_raw_children_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        children: Vec<Value>,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        if children.is_empty() {
            return Ok(json!({ "code": 0, "msg": "empty children, nothing appended" }));
        }
        let path = format!(
            "/docx/v1/documents/{}/blocks/{}/children",
            document_id, block_id
        );
        self.post_json_auth(
            &path,
            &[("document_revision_id".to_string(), "-1".to_string())],
            json!({
                "index": -1,
                "children": children,
            }),
            auth,
        )
        .await
    }

    pub(in crate::app) async fn append_raw_children_at(
        &mut self,
        document_id: &str,
        block_id: &str,
        index: i64,
        children: Vec<Value>,
    ) -> Result<Value> {
        self.append_raw_children_at_with_auth(
            document_id,
            block_id,
            index,
            children,
            ApiAuthArg::Tenant,
        )
        .await
    }

    pub(in crate::app) async fn append_raw_children_at_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        index: i64,
        children: Vec<Value>,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        if children.is_empty() {
            return Ok(json!({ "code": 0, "msg": "empty children, nothing appended" }));
        }
        let path = format!(
            "/docx/v1/documents/{}/blocks/{}/children",
            document_id, block_id
        );
        self.post_json_auth(
            &path,
            &[("document_revision_id".to_string(), "-1".to_string())],
            json!({
                "index": index,
                "children": children,
            }),
            auth,
        )
        .await
    }

    pub(in crate::app) async fn patch_document_block(
        &mut self,
        document_id: &str,
        block_id: &str,
        body: Value,
    ) -> Result<Value> {
        self.patch_document_block_with_auth(document_id, block_id, body, ApiAuthArg::Tenant)
            .await
    }

    pub(in crate::app) async fn patch_document_block_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let path = format!("/docx/v1/documents/{document_id}/blocks/{block_id}");
        self.patch_json_auth(
            &path,
            &[("document_revision_id".to_string(), "-1".to_string())],
            body,
            auth,
        )
        .await
    }

    pub(in crate::app) async fn append_descendant_body_with_auth(
        &mut self,
        document_id: &str,
        block_id: &str,
        mut body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        ensure_descendant_defaults(&mut body)?;
        let path = format!(
            "/docx/v1/documents/{}/blocks/{}/descendant",
            document_id, block_id
        );
        self.post_json_auth(
            &path,
            &[("document_revision_id".to_string(), "-1".to_string())],
            body,
            auth,
        )
        .await
    }

    pub(in crate::app) async fn get_document_with_auth(
        &mut self,
        document_id: &str,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let path = format!("/docx/v1/documents/{document_id}");
        self.get_json_auth(&path, &[], auth).await
    }

    pub(in crate::app) async fn get_document_blocks(
        &mut self,
        document_id: &str,
        page_size: u16,
    ) -> Result<Value> {
        self.get_document_blocks_with_auth(document_id, page_size, ApiAuthArg::Tenant)
            .await
    }

    pub(in crate::app) async fn get_document_blocks_with_auth(
        &mut self,
        document_id: &str,
        page_size: u16,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let path = format!("/docx/v1/documents/{document_id}/blocks");
        self.get_json_auth(
            &path,
            &[("page_size".to_string(), page_size.to_string())],
            auth,
        )
        .await
    }

    pub(in crate::app) async fn raw_document(&mut self, document_id: &str) -> Result<Value> {
        self.raw_document_with_auth(document_id, ApiAuthArg::Tenant)
            .await
    }

    pub(in crate::app) async fn raw_document_with_auth(
        &mut self,
        document_id: &str,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        let path = format!("/docx/v1/documents/{document_id}/raw_content");
        self.get_json_auth(&path, &[], auth).await
    }

    pub(in crate::app) fn document_url(&self, document_id: &str) -> String {
        if self.config.doc_base_url.contains("{document_id}") {
            self.config
                .doc_base_url
                .replace("{document_id}", document_id)
        } else {
            format!(
                "{}/{}",
                self.config.doc_base_url.trim_end_matches('/'),
                document_id
            )
        }
    }
}

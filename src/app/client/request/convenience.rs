use super::*;

impl FeishuClient {
    pub(in crate::app) async fn post_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::POST, path, query, Some(body))
            .await
    }

    pub(in crate::app) async fn post_json_user(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        let token = self.user_access_token()?;
        self.request_json_with_token(Method::POST, path, query, Some(body), token)
            .await
    }

    pub(in crate::app) async fn post_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::POST, path, query, Some(body), auth, &[])
            .await
    }

    pub(in crate::app) async fn get_json_user(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        let token = self.user_access_token()?;
        self.request_json_with_token(Method::GET, path, query, None, token)
            .await
    }

    pub(in crate::app) async fn get_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::GET, path, query, None, auth, &[])
            .await
    }

    pub(in crate::app) async fn put_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::PUT, path, query, Some(body))
            .await
    }

    pub(in crate::app) async fn patch_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_json(Method::PATCH, path, query, Some(body))
            .await
    }

    pub(in crate::app) async fn patch_json_auth(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
        auth: ApiAuthArg,
    ) -> Result<Value> {
        self.request_json_with_auth(Method::PATCH, path, query, Some(body), auth, &[])
            .await
    }

    pub(in crate::app) async fn delete_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        self.request_json(Method::DELETE, path, query, body).await
    }

    pub(in crate::app) async fn get_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        self.request_json(Method::GET, path, query, None).await
    }

    pub(in crate::app) async fn get_helpdesk_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<Value> {
        self.request_helpdesk_json(Method::GET, path, query, None)
            .await
    }

    pub(in crate::app) async fn post_helpdesk_json(
        &mut self,
        path: &str,
        query: &[(String, String)],
        body: Value,
    ) -> Result<Value> {
        self.request_helpdesk_json(Method::POST, path, query, Some(body))
            .await
    }
}

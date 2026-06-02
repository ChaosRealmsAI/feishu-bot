use super::super::*;

fn test_config() -> Config {
    Config {
        app_id: "cli_test".to_string(),
        app_secret: "secret".to_string(),
        base_url: FEISHU_BASE_URL.to_string(),
        default_user_id: None,
        user_access_token: None,
        helpdesk_id: None,
        helpdesk_token: None,
        default_wiki_space_id: None,
        default_wiki_parent_node_token: None,
        default_doc_create_wiki: false,
        doc_base_url: "https://my.feishu.cn/docx".to_string(),
    }
}

#[tokio::test]
async fn rejects_invalid_openapi_path_before_json_send() {
    let mut api = FeishuClient::new(test_config());
    let error = api
        .request_json_with_token_and_headers(
            Method::GET,
            "missing-leading-slash",
            &[],
            None,
            "token".to_string(),
            &[],
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("OpenAPI path must start"));
}

#[tokio::test]
async fn rejects_invalid_openapi_path_before_auth_lookup() {
    let mut api = FeishuClient::new(test_config());

    let binary_error = api
        .request_binary_with_auth(
            Method::GET,
            "missing-leading-slash",
            &[],
            ApiAuthArg::User,
            &[],
            None,
        )
        .await
        .unwrap_err();
    assert!(binary_error.to_string().contains("OpenAPI path must start"));

    let multipart_error = api
        .request_multipart_with_auth(
            Method::POST,
            "missing-leading-slash",
            &[],
            Vec::new(),
            Vec::new(),
            ApiAuthArg::User,
            &[],
        )
        .await
        .unwrap_err();
    assert!(multipart_error
        .to_string()
        .contains("OpenAPI path must start"));

    let helpdesk_error = api
        .request_helpdesk_json(Method::GET, "missing-leading-slash", &[], None)
        .await
        .unwrap_err();
    assert!(helpdesk_error
        .to_string()
        .contains("OpenAPI path must start"));
}

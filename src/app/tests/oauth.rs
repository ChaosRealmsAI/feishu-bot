use super::super::*;

#[test]
fn builds_oauth_helpers_for_user_token_flow() {
    assert_eq!(
        code_challenge_s256("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );
    let scopes = resolve_oauth_scopes(vec![
        "offline_access auth:user.id:read".to_string(),
        "task:task:read".to_string(),
    ]);
    assert_eq!(
        scopes,
        vec!["offline_access", "auth:user.id:read", "task:task:read"]
    );
    let default_scopes = resolve_oauth_scopes(Vec::new());
    assert!(default_scopes.contains(&"offline_access".to_string()));

    let config = Config {
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
    };
    assert_eq!(
        oauth_authorize_url(&config),
        "https://accounts.feishu.cn/open-apis/authen/v1/authorize"
    );

    let masked = mask_oauth_token_response(&json!({
        "access_token": "u-1234567890",
        "refresh_token": "r-1234567890",
        "scope": "task:task:read",
    }));
    assert_ne!(masked["access_token"], "u-1234567890");
    assert_ne!(masked["refresh_token"], "r-1234567890");
}

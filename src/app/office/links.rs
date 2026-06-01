use super::*;

pub(super) fn web_root(api: &FeishuClient) -> String {
    let doc_base = api.config.doc_base_url.trim_end_matches('/');
    if let Some(root) = doc_base.strip_suffix("/docx") {
        return root.to_string();
    }
    if let Some((root, _)) = doc_base.split_once("/docx/") {
        return root.to_string();
    }
    doc_base.to_string()
}

pub(super) fn wiki_url(api: &FeishuClient, node_token: &str) -> String {
    format!("{}/wiki/{}", web_root(api), node_token)
}

pub(super) fn base_url(api: &FeishuClient, app_token: &str) -> String {
    format!("{}/base/{}", web_root(api), app_token)
}

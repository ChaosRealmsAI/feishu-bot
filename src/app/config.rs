use super::*;

pub(super) const FEISHU_BASE_URL: &str = "https://open.feishu.cn/open-apis";
pub(super) const LARK_BASE_URL: &str = "https://open.larksuite.com/open-apis";

#[derive(Clone)]
pub(super) struct Config {
    pub(super) app_id: String,
    pub(super) app_secret: String,
    pub(super) base_url: String,
    pub(super) default_user_id: Option<String>,
    pub(super) user_access_token: Option<String>,
    pub(super) helpdesk_id: Option<String>,
    pub(super) helpdesk_token: Option<String>,
    pub(super) default_wiki_space_id: Option<String>,
    pub(super) default_wiki_parent_node_token: Option<String>,
    pub(super) default_doc_create_wiki: bool,
    pub(super) doc_base_url: String,
}

impl Config {
    pub(super) fn load(use_lark: bool, base_url_override: Option<String>) -> Result<Self> {
        let values = load_env_values()?;
        let app_id = get_any(&values, &["FEISHU_APP_ID", "LARK_APP_ID"])
            .ok_or_else(|| anyhow!("missing FEISHU_APP_ID or LARK_APP_ID"))?;
        let app_secret = get_any(&values, &["FEISHU_APP_SECRET", "LARK_APP_SECRET"])
            .ok_or_else(|| anyhow!("missing FEISHU_APP_SECRET or LARK_APP_SECRET"))?;

        let base_url = base_url_override
            .or_else(|| get_any(&values, &["FEISHU_BASE_URL", "LARK_BASE_URL"]))
            .unwrap_or_else(|| {
                if use_lark {
                    LARK_BASE_URL.to_string()
                } else {
                    FEISHU_BASE_URL.to_string()
                }
            });

        let doc_base_url = get_any(
            &values,
            &[
                "FEISHU_DOC_BASE_URL",
                "LARK_DOC_BASE_URL",
                "FEISHU_DOC_HOST",
            ],
        )
        .map(|s| {
            if s.contains("{document_id}") {
                s
            } else {
                s.trim_end_matches('/').to_string()
            }
        })
        .unwrap_or_else(|| "https://my.feishu.cn/docx".to_string());

        Ok(Self {
            app_id,
            app_secret,
            base_url: base_url.trim_end_matches('/').to_string(),
            default_user_id: get_any(&values, &["FEISHU_USER_ID", "LARK_USER_ID"]),
            user_access_token: get_any(
                &values,
                &["FEISHU_USER_ACCESS_TOKEN", "LARK_USER_ACCESS_TOKEN"],
            ),
            helpdesk_id: get_any(&values, &["FEISHU_HELPDESK_ID", "LARK_HELPDESK_ID"]),
            helpdesk_token: get_any(&values, &["FEISHU_HELPDESK_TOKEN", "LARK_HELPDESK_TOKEN"]),
            default_wiki_space_id: get_any(
                &values,
                &["FEISHU_WIKI_SPACE_ID", "LARK_WIKI_SPACE_ID"],
            ),
            default_wiki_parent_node_token: get_any(
                &values,
                &[
                    "FEISHU_WIKI_PARENT_NODE_TOKEN",
                    "LARK_WIKI_PARENT_NODE_TOKEN",
                ],
            ),
            default_doc_create_wiki: get_bool_any(
                &values,
                &[
                    "FEISHU_DOC_CREATE_WIKI_DEFAULT",
                    "LARK_DOC_CREATE_WIKI_DEFAULT",
                    "FEISHU_WIKI_DEFAULT",
                    "LARK_WIKI_DEFAULT",
                ],
            )
            .unwrap_or(false),
            doc_base_url,
        })
    }
}

pub(super) fn load_env_values() -> Result<HashMap<String, String>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cwd = std::env::current_dir().context("current dir")?;
    let mut paths = vec![manifest_dir.join(".env")];
    let cwd_env = cwd.join(".env");
    if cwd_env != paths[0] {
        paths.push(cwd_env);
    }
    if let Ok(path) = std::env::var("FEISHU_ENV_FILE").or_else(|_| std::env::var("LARK_ENV_FILE")) {
        paths.push(PathBuf::from(path));
    }
    load_env_values_from_sources(paths, std::env::vars())
}

pub(super) fn load_env_values_from_sources(
    paths: impl IntoIterator<Item = PathBuf>,
    env_vars: impl IntoIterator<Item = (String, String)>,
) -> Result<HashMap<String, String>> {
    let mut values = HashMap::new();
    for path in paths {
        if path.exists() {
            load_dotenv_file(&path, &mut values)?;
        }
    }
    for (key, value) in env_vars {
        insert_env_value(&mut values, key, value);
    }
    Ok(values)
}

fn load_dotenv_file(path: &Path, values: &mut HashMap<String, String>) -> Result<()> {
    for item in dotenvy::from_path_iter(path).with_context(|| format!("read {}", path.display()))? {
        let (key, value) = item.with_context(|| format!("parse {}", path.display()))?;
        insert_env_value(values, key, value);
    }
    Ok(())
}

fn insert_env_value(values: &mut HashMap<String, String>, key: String, value: String) {
    if value.is_empty() {
        return;
    }
    values.insert(key, value);
}

pub(super) fn get_any(values: &HashMap<String, String>, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| values.get(*key).filter(|value| !value.is_empty()).cloned())
}

pub(super) fn get_bool_any(values: &HashMap<String, String>, keys: &[&str]) -> Option<bool> {
    get_any(values, keys).and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "y" | "on" => Some(true),
        "0" | "false" | "no" | "n" | "off" => Some(false),
        _ => None,
    })
}

pub(super) fn mask_secret(value: &str) -> String {
    if value.len() <= 8 {
        return "***".to_string();
    }
    format!("{}...{}", &value[..4], &value[value.len() - 4..])
}

pub(super) fn mask_app_id(value: &str) -> String {
    if value.len() <= 10 {
        return mask_secret(value);
    }
    format!("{}...{}", &value[..8], &value[value.len() - 4..])
}

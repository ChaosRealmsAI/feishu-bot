use super::*;

const OFFICE_STATE_ENV: &str = "FEISHU_OFFICE_STATE_FILE";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(in crate::app) struct OfficeProjectRegistry {
    #[serde(default)]
    pub(in crate::app) projects: HashMap<String, OfficeProject>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(in crate::app) struct OfficeProject {
    pub(in crate::app) project: String,
    pub(in crate::app) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) wiki_space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) wiki_parent_node_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) wiki_index_node_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) wiki_index_obj_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) base_node_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) base_app_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) base_table_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) pinned_summary_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::app) updated_at: Option<String>,
}

pub(in crate::app) fn office_project_key(project: &str) -> Result<String> {
    let value = project.trim();
    if value.is_empty() {
        bail!("--project cannot be empty");
    }
    Ok(value.to_string())
}

pub(super) fn get_office_project(
    registry: &OfficeProjectRegistry,
    project_key: &str,
) -> Result<OfficeProject> {
    registry.projects.get(project_key).cloned().ok_or_else(|| {
        anyhow!(
            "office project '{project_key}' is not bootstrapped; run `feishu-bot office bootstrap --project \"{project_key}\" --user \"$FEISHU_USER_ID\" --space-id \"$FEISHU_WIKI_SPACE_ID\" --send-summary` first"
        )
    })
}

pub(super) fn required_project_field<'a>(
    value: Option<&'a str>,
    project_key: &str,
    field: &str,
) -> Result<&'a str> {
    value
        .filter(|item| !item.trim().is_empty())
        .ok_or_else(|| {
            anyhow!(
                "office project '{project_key}' is missing {field}; rerun office bootstrap for this project"
            )
        })
}

pub(in crate::app) fn office_registry_path() -> Result<PathBuf> {
    if let Ok(path) = std::env::var(OFFICE_STATE_ENV) {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }
    let config_dir = dirs::config_dir()
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .ok_or_else(|| anyhow!("cannot find config directory"))?;
    Ok(config_dir.join("feishu").join("office-projects.json"))
}

pub(super) fn read_office_registry() -> Result<OfficeProjectRegistry> {
    let path = office_registry_path()?;
    if !path.exists() {
        return Ok(OfficeProjectRegistry::default());
    }
    let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

pub(super) fn write_office_registry(registry: &OfficeProjectRegistry) -> Result<()> {
    let path = office_registry_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&path, serde_json::to_vec_pretty(registry)?)
        .with_context(|| format!("write {}", path.display()))
}

pub(super) fn sync_legacy_project_chat(project: &OfficeProject) -> Result<()> {
    let Some(chat_id) = project.chat_id.clone() else {
        return Ok(());
    };
    let mut map = load_project_map()?;
    map.chats.insert(project.project.clone(), chat_id);
    save_project_map(&map)
}

pub(super) fn office_now() -> String {
    Local::now().to_rfc3339()
}

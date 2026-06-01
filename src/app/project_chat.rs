use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct ProjectMap {
    #[serde(flatten)]
    pub(super) chats: HashMap<String, String>,
}

pub(super) async fn get_or_create_project_chat(
    api: &mut FeishuClient,
    project: &str,
) -> Result<String> {
    let mut map = load_project_map()?;
    if let Some(chat_id) = map.chats.get(project) {
        return Ok(chat_id.clone());
    }

    let default_user = api
        .config
        .default_user_id
        .clone()
        .ok_or_else(|| anyhow!("missing FEISHU_USER_ID; pass --to or set FEISHU_USER_ID"))?;
    let user_type = infer_user_id_type(&default_user);
    let data = api
        .create_chat(
            project,
            Some(&format!("Feishu Bot project chat: {project}")),
            &[default_user],
            user_type,
        )
        .await?;
    let chat_id = get_string(&data, &["data", "chat_id"])
        .or_else(|| get_string(&data, &["data", "chat", "chat_id"]))
        .ok_or_else(|| anyhow!("create chat response missing chat_id: {data}"))?;
    map.chats.insert(project.to_string(), chat_id.clone());
    save_project_map(&map)?;
    Ok(chat_id)
}

fn project_map_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("cannot find config directory"))?;
    Ok(config_dir.join("feishu").join("projects.json"))
}

pub(super) fn load_project_map() -> Result<ProjectMap> {
    let mut chats = HashMap::new();
    let path = project_map_path()?;
    if path.exists() {
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let parsed: HashMap<String, String> =
            serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))?;
        chats.extend(parsed);
    }
    Ok(ProjectMap { chats })
}

pub(super) fn save_project_map(map: &ProjectMap) -> Result<()> {
    let path = project_map_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&path, serde_json::to_string_pretty(&map.chats)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

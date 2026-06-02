use super::*;

const PROJECT_LOG_TABLE_NAME: &str = "项目日志";

pub(in crate::app::office) async fn ensure_office_base(
    api: &mut FeishuClient,
    args: &OfficeBootstrapArgs,
    project: &mut OfficeProject,
    next_actions: &mut Vec<String>,
) -> Result<Value> {
    if project.base_app_token.is_some() && project.base_table_id.is_some() {
        return Ok(json!({
            "status": "reused",
            "app_token": project.base_app_token,
            "table_id": project.base_table_id,
        }));
    }

    let mut create_route = "base";
    let mut base_node_token = None;
    let app_token = if let Some(space_id) = project.wiki_space_id.as_deref() {
        let path = format!("/wiki/v2/spaces/{}/nodes", encode_path_segment(space_id));
        let body = json!({
            "obj_type": "bitable",
            "node_type": "origin",
            "title": format!("{} 项目多维表格", project.name),
        });
        match wiki_request_json(api, Method::POST, &path, &[], Some(body), args.auth).await {
            Ok(value) => {
                create_route = "wiki_bitable";
                base_node_token = get_string(&value, &["data", "node", "node_token"]);
                get_string(&value, &["data", "node", "obj_token"])
                    .or_else(|| get_string(&value, &["data", "obj_token"]))
                    .ok_or_else(|| anyhow!("wiki bitable response missing obj_token: {value}"))?
            }
            Err(error) => {
                next_actions.push(format!(
                    "Wiki bitable creation failed; fell back to plain Base create: {error:#}"
                ));
                create_plain_base(api, &project.name).await?
            }
        }
    } else {
        create_plain_base(api, &project.name).await?
    };

    let table = create_project_log_table(api, &app_token).await?;
    let table_id = extract_table_id(&table)
        .ok_or_else(|| anyhow!("project log table create response missing table_id: {table}"))?;
    project.base_node_token = base_node_token;
    project.base_app_token = Some(app_token.clone());
    project.base_table_id = Some(table_id.clone());
    Ok(json!({
        "status": "created",
        "route": create_route,
        "app_token": app_token,
        "node_token": project.base_node_token,
        "table_id": table_id,
        "table_response": table,
    }))
}

async fn create_plain_base(api: &mut FeishuClient, project_name: &str) -> Result<String> {
    let created = api
        .post_json(
            "/bitable/v1/apps",
            &[],
            json!({ "name": format!("{project_name} 项目多维表格") }),
        )
        .await?;
    get_string(&created, &["data", "app", "app_token"])
        .or_else(|| get_string(&created, &["data", "app_token"]))
        .ok_or_else(|| anyhow!("create Base response missing app_token: {created}"))
}

async fn create_project_log_table(api: &mut FeishuClient, app_token: &str) -> Result<Value> {
    let path = format!("/bitable/v1/apps/{app_token}/tables");
    let fields = ["类型", "标题", "状态", "链接", "摘要", "创建时间"]
        .into_iter()
        .map(|field_name| json!({ "field_name": field_name, "type": 1 }))
        .collect::<Vec<_>>();
    api.post_json(
        &path,
        &[],
        json!({
            "table": {
                "name": PROJECT_LOG_TABLE_NAME,
                "fields": fields,
            }
        }),
    )
    .await
}

pub(in crate::app::office) async fn append_project_base_record(
    api: &mut FeishuClient,
    project: &OfficeProject,
    kind: &str,
    title: &str,
    status: &str,
    url: &str,
    summary: &str,
) -> Value {
    let (Some(app_token), Some(table_id)) = (
        project.base_app_token.as_deref(),
        project.base_table_id.as_deref(),
    ) else {
        return json!({ "ok": false, "error": "project has no base_app_token/base_table_id" });
    };
    let path = format!("/bitable/v1/apps/{app_token}/tables/{table_id}/records");
    probe_value(
        api.post_json(
            &path,
            &[],
            json!({
                "fields": {
                    "类型": kind,
                    "标题": title,
                    "状态": status,
                    "链接": url,
                    "摘要": summary,
                    "创建时间": office_now(),
                }
            }),
        )
        .await,
    )
}

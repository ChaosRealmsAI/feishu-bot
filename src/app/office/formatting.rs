use super::*;

pub(super) fn office_index_markdown(project: &OfficeProject) -> String {
    format!(
        "# {}\n\n## 项目状态\n\n- 群聊：{}\n- 初始化时间：{}\n\n## 使用约定\n\n- 每一个新功能演示单独创建一篇报告文档。\n- 群聊只放短消息、链接、语音和待处理回复。\n- 项目日志写入多维表格，方便后续搜索和回顾。\n",
        project.name,
        project.chat_id.as_deref().unwrap_or("未创建"),
        project.created_at.as_deref().unwrap_or("未知"),
    )
}

pub(super) fn office_progress_message(
    title: &str,
    status: &str,
    summary: &str,
    report_url: Option<&str>,
) -> String {
    let mut lines = vec![
        format!("进度更新：{title}"),
        format!("状态：{status}"),
        format!("摘要：{summary}"),
    ];
    if let Some(url) = report_url.filter(|value| !value.trim().is_empty()) {
        lines.push(format!("详情：{url}"));
    }
    lines.join("\n")
}

pub(super) fn push_json_opt(body: &mut Map<String, Value>, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|item| !item.trim().is_empty()) {
        body.insert(key.to_string(), Value::String(value));
    }
}

pub(super) fn truncate_chars(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

use super::*;

pub(super) fn build_notification_card(args: &NotifyArgs, body: &str) -> Value {
    let (emoji, color, label) = match args.status {
        StatusArg::Done => ("OK", "green", "完成"),
        StatusArg::Error => ("ERR", "red", "失败"),
        StatusArg::Info => ("INFO", "blue", "进展"),
        StatusArg::Warning => ("WARN", "orange", "警告"),
    };

    let mut elements = Vec::new();
    let mut top = Vec::new();
    if let Some(goal) = &args.goal {
        top.push(format!("**目标**  {}", unescape_newlines(goal)));
    }
    if let Some(task) = &args.task {
        top.push(format!("**任务**  {}", unescape_newlines(task)));
    }
    if !top.is_empty() {
        elements.push(json!({
            "tag": "div",
            "text": { "tag": "lark_md", "content": top.join("\n") }
        }));
        elements.push(json!({ "tag": "hr" }));
    }
    if let Some(summary) = &args.summary {
        elements.push(json!({
            "tag": "div",
            "text": { "tag": "lark_md", "content": format!("**{}**", unescape_newlines(summary)) }
        }));
    }
    if let Some(details) = &args.details {
        let items = details
            .split(['|', '｜'])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| format!("- {s}"))
            .collect::<Vec<_>>();
        if !items.is_empty() {
            elements.push(json!({
                "tag": "div",
                "text": { "tag": "lark_md", "content": items.join("\n") }
            }));
        }
    }
    let trimmed = body.trim();
    if !trimmed.is_empty() {
        elements.push(json!({
            "tag": "div",
            "text": { "tag": "lark_md", "content": unescape_newlines(trimmed) }
        }));
    }
    if let Some(next) = &args.next {
        elements.push(json!({
            "tag": "div",
            "text": { "tag": "lark_md", "content": format!("> **下一步**  {}", unescape_newlines(next)) }
        }));
    }
    if let Some(link) = &args.link {
        elements.push(json!({
            "tag": "action",
            "actions": [{
                "tag": "button",
                "text": { "tag": "plain_text", "content": "查看详情" },
                "type": "primary",
                "url": link
            }]
        }));
    }
    elements.push(json!({ "tag": "hr" }));

    let session = args.session.clone().unwrap_or_else(random_uuid);
    let mut meta = vec![
        format!("项目 {}", args.project),
        format!("{}", Local::now().format("%H:%M")),
        format!("ID `{}`", session.chars().take(8).collect::<String>()),
    ];
    if let Some(progress) = &args.progress {
        meta.insert(1, format!("进度 {progress}"));
    }
    elements.push(json!({
        "tag": "note",
        "elements": [{ "tag": "lark_md", "content": meta.join(" | ") }]
    }));

    let title = args
        .task
        .clone()
        .unwrap_or_else(|| format!("{} - {}", args.project, label));

    json!({
        "config": { "wide_screen_mode": true },
        "header": {
            "title": { "tag": "plain_text", "content": format!("{emoji} {title}") },
            "template": color
        },
        "elements": elements
    })
}

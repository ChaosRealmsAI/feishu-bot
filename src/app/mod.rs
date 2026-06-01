use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use clap::{Args, Parser, Subcommand, ValueEnum};
use reqwest::Method;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

mod approval;
mod attendance;
mod base;
mod board;
mod bot;
mod calendar;
mod chat;
mod cli;
mod client;
mod config;
mod doc;
mod dogfood;
mod drive;
mod help;
mod helpdesk;
mod mail;
mod manifest;
mod message;
mod minutes;
mod oauth;
mod office;
mod okr;
mod output;
mod people;
mod raw_api;
mod search;
mod setup;
mod sheet;
mod task;
mod vc;
mod wiki;

use approval::*;
use attendance::*;
use base::*;
use board::*;
use bot::*;
use calendar::*;
use chat::*;
use cli::*;
use client::*;
use config::*;
use doc::*;
use dogfood::*;
use drive::*;
use help::*;
use helpdesk::*;
use mail::*;
use manifest::*;
use message::*;
use minutes::*;
use oauth::*;
use office::*;
use okr::*;
use output::*;
use people::*;
use raw_api::*;
use search::*;
use setup::*;
use sheet::*;
use task::*;
use vc::*;
use wiki::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectMap {
    #[serde(flatten)]
    chats: HashMap<String, String>,
}

pub async fn run() -> Result<()> {
    let Cli {
        lark,
        base_url,
        json: raw_json,
        command,
    } = Cli::parse();

    match command {
        Commands::Ai => {
            print!("{AI_USAGE}");
            Ok(())
        }
        Commands::Manifest(args) => print_manifest(&args),
        Commands::Scopes(args) => print_scope_groups(&args.group, args.token_type),
        Commands::Browser(command) => run_browser_command(command),
        Commands::Setup(command) => {
            run_setup_command(command, lark, base_url.clone(), raw_json).await
        }
        Commands::Office(command) if office_command_can_run_without_api(&command) => {
            run_office_local_command(command, raw_json)
        }
        Commands::Base(BaseCommand::ParseUrl(args)) => print_base_url_parse(args, raw_json),
        Commands::Doc(DocCommand::Capabilities) => {
            print!("{DOC_CAPABILITIES}");
            Ok(())
        }
        Commands::Doc(DocCommand::Template(args)) => print_doc_template(args.kind),
        Commands::Doc(DocCommand::Preview(args)) => preview_doc(args, raw_json),
        command => {
            let config = Config::load(lark, base_url.clone())?;
            run_api_command(command, config, raw_json).await
        }
    }
}

pub fn args_request_json() -> bool {
    std::env::args_os().any(|arg| arg.to_string_lossy() == "--json")
}

async fn run_api_command(command: Commands, config: Config, raw_json: bool) -> Result<()> {
    match command {
        Commands::Ai
        | Commands::Manifest(_)
        | Commands::Scopes(_)
        | Commands::Browser(_)
        | Commands::Setup(_)
        | Commands::Doc(DocCommand::Capabilities)
        | Commands::Doc(DocCommand::Template(_))
        | Commands::Doc(DocCommand::Preview(_)) => {
            unreachable!("non-API commands are handled before config loading")
        }
        Commands::Doctor => doctor(&config, raw_json).await,
        Commands::Token(args) => {
            let mut api = FeishuClient::new(config);
            let token = api.tenant_token().await?;
            if raw_json {
                let output = if args.raw {
                    json!({ "tenant_access_token": token })
                } else {
                    json!({ "tenant_access_token": mask_secret(&token) })
                };
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else if args.raw {
                println!("{token}");
            } else {
                println!("tenant_access_token={} ", mask_secret(&token));
            }
            Ok(())
        }
        Commands::Oauth(command) => run_oauth_command(&config, command, raw_json).await,
        Commands::Message(command) => {
            let mut api = FeishuClient::new(config);
            run_message_command(&mut api, command, raw_json).await
        }
        Commands::Bot(command) => {
            let mut api = FeishuClient::new(config);
            run_bot_command(&mut api, command, raw_json).await
        }
        Commands::Dogfood(command) => {
            let mut api = FeishuClient::new(config);
            run_dogfood_command(&mut api, command, raw_json).await
        }
        Commands::Office(command) => {
            let mut api = FeishuClient::new(config);
            run_office_command(&mut api, command, raw_json).await
        }
        Commands::Contact(command) => {
            let mut api = FeishuClient::new(config);
            run_contact_command(&mut api, command, raw_json).await
        }
        Commands::Directory(command) => {
            let mut api = FeishuClient::new(config);
            run_directory_command(&mut api, command, raw_json).await
        }
        Commands::Board(command) => {
            let mut api = FeishuClient::new(config);
            run_board_command(&mut api, command, raw_json).await
        }
        Commands::Notify(args) => {
            let mut api = FeishuClient::new(config);
            let body = read_content(args.text.clone(), args.file.clone(), args.stdin)?;
            let receive_id = if let Some(to) = args.to.clone() {
                to
            } else {
                get_or_create_project_chat(&mut api, &args.project).await?
            };
            let receive_id_type = if args.to.is_some() {
                args.to_type.resolve(&receive_id)
            } else {
                "chat_id"
            };
            let card = build_notification_card(&args, &body);
            let data = api
                .send_interactive(&receive_id, receive_id_type, card, None)
                .await?;
            print_response(raw_json, "notification sent", data)
        }
        Commands::Chat(command) => {
            let mut api = FeishuClient::new(config);
            run_chat_command(&mut api, command, raw_json).await
        }
        Commands::Base(command) => {
            let mut api = FeishuClient::new(config);
            run_base_command(&mut api, command, raw_json).await
        }
        Commands::Task(command) => {
            let mut api = FeishuClient::new(config);
            run_task_command(&mut api, command, raw_json).await
        }
        Commands::Drive(command) => {
            let mut api = FeishuClient::new(config);
            run_drive_command(&mut api, command, raw_json).await
        }
        Commands::Calendar(command) => {
            let mut api = FeishuClient::new(config);
            run_calendar_command(&mut api, command, raw_json).await
        }
        Commands::Vc(command) => {
            let mut api = FeishuClient::new(config);
            run_vc_command(&mut api, command, raw_json).await
        }
        Commands::Minutes(command) => {
            let mut api = FeishuClient::new(config);
            run_minutes_command(&mut api, command, raw_json).await
        }
        Commands::Search(command) => {
            let mut api = FeishuClient::new(config);
            run_search_command(&mut api, command, raw_json).await
        }
        Commands::Okr(command) => {
            let mut api = FeishuClient::new(config);
            run_okr_command(&mut api, command, raw_json).await
        }
        Commands::Attendance(command) => {
            let mut api = FeishuClient::new(config);
            run_attendance_command(&mut api, command, raw_json).await
        }
        Commands::Mail(command) => {
            let mut api = FeishuClient::new(config);
            run_mail_command(&mut api, command, raw_json).await
        }
        Commands::Corehr(command) => {
            let mut api = FeishuClient::new(config);
            run_corehr_command(&mut api, command, raw_json).await
        }
        Commands::Helpdesk(command) => {
            let mut api = FeishuClient::new(config);
            run_helpdesk_command(&mut api, command, raw_json).await
        }
        Commands::Hire(command) => {
            let mut api = FeishuClient::new(config);
            run_hire_command(&mut api, command, raw_json).await
        }
        Commands::Wiki(command) => {
            let mut api = FeishuClient::new(config);
            run_wiki_command(&mut api, command, raw_json).await
        }
        Commands::Sheet(command) => {
            let mut api = FeishuClient::new(config);
            run_sheet_command(&mut api, command, raw_json).await
        }
        Commands::Approval(command) => {
            let mut api = FeishuClient::new(config);
            run_approval_command(&mut api, command, raw_json).await
        }
        Commands::Api(command) => {
            let mut api = FeishuClient::new(config);
            run_raw_api_command(&mut api, command, raw_json).await
        }
        Commands::Doc(command) => {
            let mut api = FeishuClient::new(config);
            run_doc_command(&mut api, command, raw_json).await
        }
    }
}

async fn read_feishu_json(res: reqwest::Response) -> Result<Value> {
    let status = res.status();
    let text = res.text().await.context("read response")?;
    if status == StatusCode::NO_CONTENT {
        return Ok(Value::Null);
    }
    let json: Value = serde_json::from_str(&text)
        .with_context(|| format!("parse Feishu response JSON: {text}"))?;
    if !status.is_success() {
        bail!(
            "Feishu HTTP {status}: {}",
            serde_json::to_string_pretty(&json)?
        );
    }
    if let Some(code) = json.get("code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = json.get("msg").and_then(Value::as_str).unwrap_or("");
            bail!("Feishu API failed: code={code} msg={msg} response={json}");
        }
    }
    Ok(json)
}

async fn read_binary_response(res: reqwest::Response) -> Result<Vec<u8>> {
    let status = res.status();
    if !status.is_success() {
        let text = res.text().await.context("read error response")?;
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            bail!(
                "Feishu HTTP {status}: {}",
                serde_json::to_string_pretty(&json)?
            );
        }
        bail!("Feishu HTTP {status}: {text}");
    }
    Ok(res.bytes().await.context("read binary response")?.to_vec())
}

async fn doctor(config: &Config, raw_json: bool) -> Result<()> {
    let mut api = FeishuClient::new(config.clone());
    let token = api.tenant_token().await?;
    let default_user_id = config
        .default_user_id
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let user_access_token = config
        .user_access_token
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let helpdesk_id = config
        .helpdesk_id
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let helpdesk_token = config
        .helpdesk_token
        .as_deref()
        .map(mask_secret)
        .unwrap_or_else(|| "missing".to_string());
    let token_mask = mask_secret(&token);

    if raw_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "base_url": config.base_url,
                "app_id": mask_app_id(&config.app_id),
                "app_secret": mask_secret(&config.app_secret),
                "default_user_id": default_user_id,
                "doc_base_url": config.doc_base_url,
                "user_access_token": user_access_token,
                "helpdesk_id": helpdesk_id,
                "helpdesk_token": helpdesk_token,
                "tenant_access_token": token_mask,
                "ok": true,
            }))?
        );
    } else {
        println!("base_url={}", config.base_url);
        println!("app_id={}", mask_app_id(&config.app_id));
        println!("app_secret={}", mask_secret(&config.app_secret));
        println!("default_user_id={default_user_id}");
        println!("doc_base_url={}", config.doc_base_url);
        println!("user_access_token={user_access_token}");
        println!("helpdesk_id={helpdesk_id}");
        println!("helpdesk_token={helpdesk_token}");
        println!("tenant_access_token={token_mask} ");
    }
    Ok(())
}

fn build_notification_card(args: &NotifyArgs, body: &str) -> Value {
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

async fn get_or_create_project_chat(api: &mut FeishuClient, project: &str) -> Result<String> {
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

fn push_query_opt(query: &mut Vec<(String, String)>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        if !value.trim().is_empty() {
            query.push((key.to_string(), value));
        }
    }
}

fn push_query_opt_i64(query: &mut Vec<(String, String)>, key: &str, value: Option<i64>) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

fn push_query_opt_u8(query: &mut Vec<(String, String)>, key: &str, value: Option<u8>) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

fn push_query_repeated(query: &mut Vec<(String, String)>, key: &str, values: Vec<String>) {
    for value in values {
        if !value.trim().is_empty() {
            query.push((key.to_string(), value));
        }
    }
}

fn has_json_input(text: &Option<String>, file: &Option<PathBuf>, stdin: bool) -> bool {
    text.is_some() || file.is_some() || stdin
}

fn clean_string_values(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect()
}

fn validate_value_count(label: &str, count: usize, max: usize, required: bool) -> Result<()> {
    if required && count == 0 {
        bail!("at least one {label} is required");
    }
    if count > max {
        bail!("{label} cannot repeat more than {max} times");
    }
    Ok(())
}

fn encode_path_segment(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut output = String::new();
    for byte in value.as_bytes() {
        let keep = byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~');
        if keep {
            output.push(*byte as char);
        } else {
            output.push('%');
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    output
}

fn insert_opt_string(object: &mut Map<String, Value>, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        object.insert(key.to_string(), Value::String(value));
    }
}

fn insert_opt_i64(object: &mut Map<String, Value>, key: &str, value: Option<i64>) {
    if let Some(value) = value {
        object.insert(key.to_string(), Value::Number(value.into()));
    }
}

fn insert_opt_u8(object: &mut Map<String, Value>, key: &str, value: Option<u8>) {
    if let Some(value) = value {
        object.insert(key.to_string(), json!(value));
    }
}

fn insert_string_array(object: &mut Map<String, Value>, key: &str, values: Vec<String>) {
    let values = values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .map(Value::String)
        .collect::<Vec<_>>();
    if !values.is_empty() {
        object.insert(key.to_string(), Value::Array(values));
    }
}

fn parse_query_pairs(values: Vec<String>) -> Result<Vec<(String, String)>> {
    parse_key_value_pairs(values, "query")
}

fn parse_header_pairs(values: Vec<String>) -> Result<Vec<(String, String)>> {
    parse_key_value_pairs(values, "header")
}

fn parse_key_value_pairs(values: Vec<String>, label: &str) -> Result<Vec<(String, String)>> {
    values
        .into_iter()
        .map(|item| {
            let (key, value) = item
                .split_once('=')
                .ok_or_else(|| anyhow!("{label} must be key=value, got {item}"))?;
            if key.is_empty() {
                bail!("{label} key cannot be empty: {item}");
            }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

fn parse_file_part_pairs(values: Vec<String>) -> Result<Vec<(String, PathBuf)>> {
    values
        .into_iter()
        .map(|item| {
            let (key, value) = item
                .split_once('=')
                .ok_or_else(|| anyhow!("file part must be part_name=path, got {item}"))?;
            if key.is_empty() {
                bail!("file part name cannot be empty: {item}");
            }
            if value.trim().is_empty() {
                bail!("file part path cannot be empty: {item}");
            }
            Ok((key.to_string(), PathBuf::from(value)))
        })
        .collect()
}

fn read_json_value(text: Option<String>, file: Option<PathBuf>, stdin: bool) -> Result<Value> {
    let text = read_content(text, file, stdin)?;
    parse_json_value(&text, "JSON")
}

fn read_optional_json_value(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Option<Value>> {
    read_optional_content(text, file, stdin)?
        .map(|text| parse_json_value(&text, "JSON"))
        .transpose()
}

fn parse_json_value(text: &str, label: &str) -> Result<Value> {
    serde_json::from_str(text).with_context(|| format!("parse {label}"))
}

fn ensure_json_array(value: Value, label: &str) -> Result<Value> {
    if value.is_array() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON array")
    }
}

fn ensure_json_object(value: Value, label: &str) -> Result<Value> {
    if value.is_object() {
        Ok(value)
    } else {
        bail!("{label} must be a JSON object")
    }
}

fn read_record_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let record_ids = if let Some(record_ids) = value.get("record_ids") {
            record_ids.clone()
        } else if let Some(records) = value.get("records") {
            records.clone()
        } else {
            value
        };
        return ensure_json_array(record_ids, "record_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("provide --record-id at least once, or JSON via --record-ids-json/--records-json/--file/--stdin");
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

fn read_table_ids_json(
    mut ids: Vec<String>,
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Value> {
    if let Some(value) = read_optional_json_value(text, file, stdin)? {
        let table_ids = if let Some(table_ids) = value.get("table_ids") {
            table_ids.clone()
        } else if let Some(tables) = value.get("tables") {
            tables.clone()
        } else {
            value
        };
        return ensure_json_array(table_ids, "table_ids");
    }
    ids.retain(|id| !id.trim().is_empty());
    if ids.is_empty() {
        bail!("provide --table-id at least once, or JSON via --table-ids-json/--file/--stdin");
    }
    Ok(Value::Array(ids.into_iter().map(Value::String).collect()))
}

fn collect_json_string_array(
    mut values: Vec<String>,
    text: Option<String>,
    label: &str,
) -> Result<Option<Value>> {
    if let Some(text) = text {
        let value = parse_json_value(&text, label)?;
        let array = if let Some(nested) = value.get(label) {
            nested.clone()
        } else {
            value
        };
        return Ok(Some(ensure_json_array(array, label)?));
    }
    values.retain(|value| !value.trim().is_empty());
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(Value::Array(
        values.into_iter().map(Value::String).collect(),
    )))
}

impl ReceiveIdTypeArg {
    fn resolve(self, id: &str) -> &'static str {
        match self {
            ReceiveIdTypeArg::OpenId => "open_id",
            ReceiveIdTypeArg::UnionId => "union_id",
            ReceiveIdTypeArg::UserId => "user_id",
            ReceiveIdTypeArg::Email => "email",
            ReceiveIdTypeArg::ChatId => "chat_id",
            ReceiveIdTypeArg::Auto => infer_receive_id_type(id),
        }
    }
}

impl UserIdTypeArg {
    fn resolve(self, sample: Option<&str>) -> &'static str {
        match self {
            UserIdTypeArg::OpenId => "open_id",
            UserIdTypeArg::UnionId => "union_id",
            UserIdTypeArg::UserId => "user_id",
            UserIdTypeArg::Auto => sample.map(infer_user_id_type).unwrap_or("open_id"),
        }
    }
}

impl OkrUserIdTypeArg {
    fn as_api_value(self) -> &'static str {
        match self {
            OkrUserIdTypeArg::OpenId => "open_id",
            OkrUserIdTypeArg::UnionId => "union_id",
            OkrUserIdTypeArg::UserId => "user_id",
            OkrUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl AttendanceEmployeeTypeArg {
    fn as_api_value(self) -> &'static str {
        match self {
            AttendanceEmployeeTypeArg::EmployeeId => "employee_id",
            AttendanceEmployeeTypeArg::EmployeeNo => "employee_no",
        }
    }
}

impl DepartmentIdTypeArg {
    fn as_api_value(self) -> &'static str {
        match self {
            DepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            DepartmentIdTypeArg::DepartmentId => "department_id",
        }
    }
}

impl ContentTypeArg {
    fn as_api_value(self) -> &'static str {
        match self {
            ContentTypeArg::Markdown => "markdown",
            ContentTypeArg::Html => "html",
        }
    }
}

fn infer_receive_id_type(id: &str) -> &'static str {
    if id.starts_with("oc_") {
        "chat_id"
    } else if id.starts_with("ou_") {
        "open_id"
    } else if id.starts_with("on_") {
        "union_id"
    } else if id.contains('@') {
        "email"
    } else {
        "user_id"
    }
}

fn infer_user_id_type(id: &str) -> &'static str {
    match infer_receive_id_type(id) {
        "chat_id" | "email" => "open_id",
        other => other,
    }
}

fn read_content(text: Option<String>, file: Option<PathBuf>, stdin: bool) -> Result<String> {
    read_optional_content(text, file, stdin)?
        .ok_or_else(|| anyhow!("provide --text/--content, --file, or --stdin"))
}

fn read_optional_content(
    text: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
) -> Result<Option<String>> {
    if let Some(text) = text {
        return Ok(Some(text));
    }
    if let Some(path) = file {
        return Ok(Some(
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?,
        ));
    }
    if stdin {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).context("read stdin")?;
        return Ok(Some(buf));
    }
    Ok(None)
}

fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn unescape_newlines(value: &str) -> String {
    value.replace("\\n", "\n")
}

fn project_map_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("cannot find config directory"))?;
    Ok(config_dir.join("feishu").join("projects.json"))
}

fn load_project_map() -> Result<ProjectMap> {
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

fn save_project_map(map: &ProjectMap) -> Result<()> {
    let path = project_map_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&path, serde_json::to_string_pretty(&map.chats)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn run_browser_command(command: BrowserCommand) -> Result<()> {
    match command {
        BrowserCommand::Ensure => {
            let script = std::env::var("FEISHU_PLAYWRIGHT_ENSURE")
                .unwrap_or_else(|_| "ensure-extension-mcp.sh".to_string());
            run_status(ProcessCommand::new("bash").arg(script).arg("--background"))
        }
        BrowserCommand::Tabs => run_mcpc(&[
            "tools-call",
            "browser_tabs",
            "action:=list",
            "--timeout",
            "20",
        ]),
        BrowserCommand::Open(args) => {
            let url_arg = format!("url:={}", args.url);
            run_mcpc(&[
                "tools-call",
                "browser_navigate",
                &url_arg,
                "--timeout",
                "30",
            ])
        }
        BrowserCommand::Drive => run_mcpc(&[
            "tools-call",
            "browser_navigate",
            "url:=https://my.feishu.cn/drive/home/",
            "--timeout",
            "30",
        ]),
    }
}

fn run_mcpc(args: &[&str]) -> Result<()> {
    let mut command = ProcessCommand::new("npx");
    command.arg("--yes").arg("@apify/mcpc").arg("@browser");
    for arg in args {
        command.arg(arg);
    }
    run_status(&mut command)
}

fn run_status(command: &mut ProcessCommand) -> Result<()> {
    let status = command.status().context("run command")?;
    if !status.success() {
        bail!("command exited with status {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests;

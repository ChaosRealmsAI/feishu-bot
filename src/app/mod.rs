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
mod browser_control;
mod calendar;
mod chat;
mod cli;
mod client;
mod common;
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
mod notify_card;
mod oauth;
mod office;
mod okr;
mod output;
mod people;
mod project_chat;
mod raw_api;
mod scopes;
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
use browser_control::*;
use calendar::*;
use chat::*;
use cli::*;
use client::*;
pub(in crate::app) use common::*;
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
use notify_card::*;
use oauth::*;
use office::*;
use okr::*;
use output::*;
use people::*;
use project_chat::*;
use raw_api::*;
use scopes::*;
use search::*;
use setup::*;
use sheet::*;
use task::*;
use vc::*;
use wiki::*;

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

#[cfg(test)]
mod tests;

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
mod dispatch;
mod doc;
mod doctor;
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
mod response;
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
use dispatch::*;
use doc::*;
use doctor::*;
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
pub(in crate::app) use response::*;
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

#[cfg(test)]
mod tests;

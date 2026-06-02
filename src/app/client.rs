#![allow(clippy::too_many_arguments)]

use super::*;

mod board;
mod documents;
mod im;
mod media;
mod request;

pub(super) struct FeishuClient {
    http: reqwest::Client,
    pub(super) config: Config,
    tenant_token: Option<String>,
}

impl FeishuClient {
    pub(super) fn new(config: Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
            tenant_token: None,
        }
    }
}

use super::*;

mod env;
mod output;
mod request;
mod token;
mod url;

pub(in crate::app) use token::refresh_oauth_token;
use token::{exchange_oauth_code, get_oauth_user_info};
use url::build_oauth_url_response;

#[cfg(test)]
pub(super) use output::mask_oauth_token_response;
#[cfg(test)]
pub(super) use url::{code_challenge_s256, oauth_authorize_url, resolve_oauth_scopes};

pub(super) async fn run_oauth_command(
    config: &Config,
    command: OauthCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        OauthCommand::Url(args) => build_oauth_url_response(config, args)?,
        OauthCommand::Token(args) => exchange_oauth_code(config, args).await?,
        OauthCommand::Refresh(args) => refresh_oauth_token(config, args).await?,
        OauthCommand::UserInfo(args) => get_oauth_user_info(config, args).await?,
    };
    if raw_json {
        println!("{}", serde_json::to_string_pretty(&data)?);
    } else {
        output::print_oauth_response(&data)?;
    }
    Ok(())
}

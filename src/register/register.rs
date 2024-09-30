use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use super::Command;
use crate::DISCORD_API_VERSION;

/// Registers Discord commands with the specified application.
///
/// # Arguments
///
/// * `app_id` - The Discord application ID.
/// * `token` - The bot token for authentication.
/// * `cmds` - A vector of commands to be registered.
///
/// # Errors
///
/// Returns an error if the request to register commands fails.
pub async fn register_commands(
    app_id: &str,
    token: &str,
    cmds: Vec<Command>,
) -> Result<(), Box<dyn Error>> {
    let endpoint = format!(
        "https://discord.com/api/{}/applications/{}/commands",
        DISCORD_API_VERSION, app_id
    );

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.append(
        "Authorization",
        HeaderValue::from_str(&format!("Bot {}", token))?,
    );

    let response = client
        .put(&endpoint)
        .headers(headers)
        .json(&cmds)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Commands registered");
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(format!("Error registering commands: {}", error_message).into())
    }
}

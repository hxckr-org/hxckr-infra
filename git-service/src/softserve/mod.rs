pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod ssh;
pub mod tracing;

use crate::softserve::models::{CreateRepoResponse, CreateTokenResponse, CreateUserResponse};
use ssh::execute_command;

pub fn create_user(username: &str) -> Result<CreateUserResponse, Box<dyn std::error::Error>> {
    let command = format!("user create {}", username);
    Ok(execute_command(&command).map(|_| CreateUserResponse {
        username: username.to_string(),
        message: format!("User {} created successfully on Soft Serve", username),
    })?)
}

pub fn create_token(token_name: &str) -> Result<CreateTokenResponse, Box<dyn std::error::Error>> {
    let command = format!("token create {}", token_name);
    let output = execute_command(&command)?;
    let token = output.trim().to_string();
    Ok(CreateTokenResponse {
        token_name: token_name.to_string(),
        token,
    })
}

pub fn create_repo(
    repo_name: &str,
    repo_url: &str,
) -> Result<CreateRepoResponse, Box<dyn std::error::Error>> {
    let import_command = format!("repo import {} {}", repo_name, repo_url);
    execute_command(&import_command)?;

    let token_command = format!("token create {}_token", repo_name);
    let token_output = execute_command(&token_command)?;
    let token = token_output.trim().to_string();

    let server_url = self::config::get_server_url();

    Ok(CreateRepoResponse {
        repo_name: repo_name.to_string(),
        repo_url: format!("http://{}@{}/{}.git", token, server_url, repo_name),
    })
}

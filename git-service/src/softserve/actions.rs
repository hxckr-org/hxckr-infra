use crate::softserve::config;
use crate::softserve::models::{CreateRepoResponse, CreateTokenResponse, CreateUserResponse};
use crate::softserve::ssh::execute_command;

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
    let scheme = if self::config::get_is_prod() { "https" } else { "http" };

    Ok(CreateRepoResponse {
        repo_name: repo_name.to_string(),
        repo_url: format!("{}://{}@{}/{}.git", scheme, token, server_url, repo_name),
    })
}

pub fn setup_webhook(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = self::config::get_webhook_url();

    let webhook_command = format!(
        "repo webhook create {} {} -e push --content-type json",
        repo_name, webhook_url
    );

    execute_command(&webhook_command)?;
    Ok(())
}

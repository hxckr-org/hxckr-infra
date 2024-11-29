use crate::softserve::config;
use crate::softserve::models::{
    CreateRepoResponse, CreateTokenResponse, CreateUserResponse, DeleteRepoResponse,
    ListReposResponse,
};
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

    let server_url = config::get_server_url();
    let is_prod = config::get_is_prod();
    let scheme = if config::get_is_prod() {
        "https"
    } else {
        "http"
    };

    let formatted_url = if !is_prod {
        // For local environment, include the Soft Serve port
        let softserve_port = config::get_softserve_http_port();
        format!(
            "{}://{}@{}:{}/{}.git",
            scheme, token, server_url, softserve_port, repo_name
        )
    } else {
        // For production, use the server URL as is
        format!("{}://{}@{}/{}.git", scheme, token, server_url, repo_name)
    };

    Ok(CreateRepoResponse {
        repo_name: repo_name.to_string(),
        repo_url: formatted_url,
    })
}

pub fn setup_webhook(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = config::get_webhook_url();

    let webhook_command = format!(
        "repo webhook create {} {} -e push --content-type json",
        repo_name, webhook_url
    );

    execute_command(&webhook_command)?;
    Ok(())
}

pub fn list_repos() -> Result<ListReposResponse, Box<dyn std::error::Error>> {
    let command = "repo list";
    let output = execute_command(command)?;
    let repositories: Vec<String> = output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(ListReposResponse { repositories })
}

pub fn delete_repo(repo_name: &str) -> Result<DeleteRepoResponse, Box<dyn std::error::Error>> {
    let command = format!("repo delete {}", repo_name);
    execute_command(&command)?;

    Ok(DeleteRepoResponse {
        repo_name: repo_name.to_string(),
        message: format!("Repository {} deleted successfully", repo_name),
    })
}

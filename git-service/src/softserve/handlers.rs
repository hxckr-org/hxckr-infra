use super::errors::GitServiceError;
use super::models::{CreateRepoRequest, CreateTokenRequest, CreateUserRequest, DeleteRepoRequest};
use super::ssh::execute_command;
use super::{create_repo, create_token, create_user, delete_repo, list_repos, setup_webhook};
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use tracing::{error, info};

pub fn check_user_exists(username: &str) -> Result<bool, GitServiceError> {
    let command = "user list";
    execute_command(command)
        .map(|output| {
            let users: Vec<&str> = output.lines().collect();
            users
                .iter()
                .any(|&user_name| user_name.to_lowercase().trim() == username.trim().to_lowercase())
        })
        .map_err(|e| {
            error!("Failed to check user existence: {}", e);
            GitServiceError::FailedToCheckUserExistence(e.to_string())
        })
}

pub async fn handle_create_user(
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, GitServiceError> {
    if check_user_exists(&user.username)? {
        error!("Attempt to create an existing user: {}", user.username);
        return Err(GitServiceError::UserAlreadyExists(user.username.clone()));
    }

    create_user(&user.username)
        .map(|response| {
            info!("User created: {}", user.username);
            Ok(HttpResponse::Ok().json(response))
        })
        .map_err(|e| {
            error!("Failed to create user {}: {}", user.username, e);
            GitServiceError::FailedToCreateUser(e.to_string())
        })?
}

pub async fn handle_create_token(
    token_req: web::Json<CreateTokenRequest>,
) -> Result<HttpResponse, GitServiceError> {
    create_token(&token_req.token_name)
        .map(|response| {
            info!("Token created: {}", token_req.token_name);
            Ok(HttpResponse::Ok().json(response))
        })
        .map_err(|e| {
            error!("Failed to create token {}: {}", token_req.token_name, e);
            GitServiceError::FailedToCreateToken(e.to_string())
        })?
}

pub fn check_repo_exists(repo_name: &str) -> Result<bool, GitServiceError> {
    let command = "repo list";
    execute_command(command)
        .map(|output| {
            let repos: Vec<&str> = output.lines().collect();
            repos
                .iter()
                .any(|&repo| repo.to_lowercase().trim() == repo_name.trim().to_lowercase())
        })
        .map_err(|e| {
            error!("Failed to check repo existence: {}", e);
            GitServiceError::FailedToCheckRepoExistence(e.to_string())
        })
}

pub async fn handle_create_repo(
    repo_req: web::Json<CreateRepoRequest>,
) -> Result<HttpResponse, GitServiceError> {
    if check_repo_exists(&repo_req.repo_name)? {
        error!("Attempt to create an existing repo: {}", repo_req.repo_name);
        return Err(GitServiceError::RepositoryAlreadyExists(
            repo_req.repo_name.clone(),
        ));
    }

    match create_repo(&repo_req.repo_name, &repo_req.repo_url) {
        Ok(response) => {
            info!("Repository created: {}", repo_req.repo_name);

            // Set up webhook after repo creation
            match setup_webhook(&repo_req.repo_name) {
                Ok(_) => {
                    info!("Webhook set up for repository: {}", repo_req.repo_name);
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    error!("Failed to set up webhook for {}: {}", repo_req.repo_name, e);
                    Err(GitServiceError::FailedToSetupWebhook(format!(
                        "Repository created successfully, but webhook setup failed: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            error!("Failed to create repo {}: {}", repo_req.repo_name, e);
            Err(GitServiceError::FailedToCreateRepository(e.to_string()))
        }
    }
}

pub async fn handle_list_repos() -> Result<HttpResponse, GitServiceError> {
    list_repos()
        .map(|response| {
            info!("Listed {} repositories", response.repositories.len());
            Ok(HttpResponse::Ok().json(response))
        })
        .map_err(|e| {
            error!("Failed to list repositories: {}", e);
            GitServiceError::FailedToListRepositories(e.to_string())
        })?
}

pub async fn handle_delete_repo(
    repo_req: web::Json<DeleteRepoRequest>,
) -> Result<HttpResponse, GitServiceError> {
    if !check_repo_exists(&repo_req.repo_name)? {
        error!(
            "Attempt to delete non-existent repo: {}",
            repo_req.repo_name
        );
        return Err(GitServiceError::RepositoryNotFound(
            repo_req.repo_name.clone(),
        ));
    }

    delete_repo(&repo_req.repo_name)
        .map(|response| {
            info!("Repository deleted: {}", repo_req.repo_name);
            Ok(HttpResponse::Ok().json(response))
        })
        .map_err(|e| {
            error!("Failed to delete repo {}: {}", repo_req.repo_name, e);
            GitServiceError::FailedToDeleteRepository(e.to_string())
        })?
}

pub async fn test_connection() -> Result<HttpResponse, GitServiceError> {
    execute_command("info")
        .map(|_| {
            info!("Connection to Soft Serve successful");
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Connection to Soft Serve successful."
            })))
        })
        .map_err(|e| {
            error!("Connection to Soft Serve failed: {}", e);
            GitServiceError::ConnectionFailed(e.to_string())
        })?
}

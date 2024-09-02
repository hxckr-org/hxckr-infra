use super::errors::GitServiceError;
use super::models::{CreateRepoRequest, CreateTokenRequest, CreateUserRequest};
use super::ssh::execute_command;
use super::{create_repo, create_token, create_user};
use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn handle_create_user(
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, GitServiceError> {
    if check_user_exists(&user.username)? {
        return Err(GitServiceError::UserAlreadyExists(user.username.clone()));
    }

    create_user(&user.username)
        .map(|response| Ok(HttpResponse::Ok().json(response)))
        .map_err(|e| GitServiceError::FailedToCreateUser(e.to_string()))?
}

pub async fn handle_create_token(
    token_req: web::Json<CreateTokenRequest>,
) -> Result<HttpResponse, GitServiceError> {
    create_token(&token_req.token_name)
        .map(|response| Ok(HttpResponse::Ok().json(response)))
        .map_err(|e| GitServiceError::FailedToCreateToken(e.to_string()))?
}

pub fn check_repo_exists(repo_name: &str) -> Result<bool, GitServiceError> {
    let command = "repo list";
    execute_command(command)
        .map(|output| {
            let repos: Vec<&str> = output.lines().collect();
            Ok(repos.iter().any(|&repo| repo.trim() == repo_name))
        })
        .map_err(|e| GitServiceError::FailedToCheckRepoExistence(e.to_string()))?
}

pub fn check_user_exists(username: &str) -> Result<bool, GitServiceError> {
    let command = "user list";
    execute_command(command)
        .map(|output| {
            let users: Vec<&str> = output.lines().collect();
            Ok(users.iter().any(|&user| user.trim() == username))
        })
        .map_err(|e| GitServiceError::FailedToCheckUserExistence(e.to_string()))?
}

pub async fn handle_create_repo(
    repo_req: web::Json<CreateRepoRequest>,
) -> Result<HttpResponse, GitServiceError> {
    if check_repo_exists(&repo_req.repo_name)? {
        return Err(GitServiceError::RepositoryAlreadyExists(
            repo_req.repo_name.clone(),
        ));
    }

    create_repo(&repo_req.repo_name, &repo_req.repo_url)
        .map(|response| Ok(HttpResponse::Ok().json(response)))
        .map_err(|e| GitServiceError::FailedToCreateRepository(e.to_string()))?
}

pub async fn test_connection() -> Result<HttpResponse, GitServiceError> {
    execute_command("info")
        .map(|_| {
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Connection to Soft Serve successful."
            })))
        })
        .map_err(|e| GitServiceError::ConnectionFailed(e.to_string()))?
}

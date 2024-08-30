use super::models::{CreateRepoRequest, CreateTokenRequest, CreateUserRequest};
use super::ssh::execute_command;
use super::{create_repo, create_token, create_user};
use actix_web::{web, HttpResponse, Responder};

pub async fn handle_create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    match create_user(&user.username) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn handle_create_token(token_req: web::Json<CreateTokenRequest>) -> impl Responder {
    match create_token(&token_req.token_name) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn handle_create_repo(repo_req: web::Json<CreateRepoRequest>) -> impl Responder {
    match create_repo(&repo_req.repo_name, &repo_req.repo_url) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn test_connection() -> impl Responder {
    match execute_command("info") {
        Ok(_) => HttpResponse::Ok().body(format!("Connection to Soft Serve successful.")),
        Err(e) => HttpResponse::InternalServerError().body(format!(
            "Connection failed, make sure you start soft serve: {}",
            e
        )),
    }
}

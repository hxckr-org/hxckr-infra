use super::models::{
    CreateTokenRequest, CreateTokenResponse, CreateUserRequest, CreateUserResponse,
};
use super::{create_token, create_user};
use actix_web::{web, HttpResponse, Responder};

pub async fn handle_create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    match create_user(&user.username) {
        Ok(message) => HttpResponse::Ok().json(CreateUserResponse {
            username: user.username.clone(),
            message,
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn handle_create_token(token_req: web::Json<CreateTokenRequest>) -> impl Responder {
    match create_token(&token_req.token_name) {
        Ok(token) => HttpResponse::Ok().json(CreateTokenResponse {
            token_name: token_req.token_name.clone(),
            token: token.trim().to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn test_connection() -> impl Responder {
    match create_user("test_user") {
        Ok(_) => HttpResponse::Ok().body("Connection to Soft Serve successful"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Connection failed: {}", e)),
    }
}

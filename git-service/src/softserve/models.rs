use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub token_name: String,
}

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    pub repo_name: String,
    pub repo_url: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub username: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub token_name: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct CreateRepoResponse {
    pub repo_name: String,
    pub repo_url: String,
}

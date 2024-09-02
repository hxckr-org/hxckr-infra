use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitServiceError {
    #[error("Failed to create user: {0}")]
    FailedToCreateUser(String),
    #[error("Failed to create token: {0}")]
    FailedToCreateToken(String),
    #[error("Failed to check repository existence: {0}")]
    FailedToCheckRepoExistence(String),
    #[error("Repository already exists: {0}")]
    RepositoryAlreadyExists(String),
    #[error("Failed to create repository: {0}")]
    FailedToCreateRepository(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Failed to check user existence: {0}")]
    FailedToCheckUserExistence(String),
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Session error: {0}")]
    SessionError(String),
    #[error("Handshake error: {0}")]
    HandshakeError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Channel error: {0}")]
    ChannelError(String),
    #[error("Command execution error: {0}")]
    CommandExecutionError(String),
    #[error("Output read error: {0}")]
    OutputReadError(String),
    #[error("Channel close error: {0}")]
    ChannelCloseError(String),
    #[error("Exit status error: {0}")]
    ExitStatusError(String),
    #[error("Command failed with status {0}: {1}")]
    CommandFailedError(i32, String),
}

impl ResponseError for GitServiceError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status).json(json!({
            "status": "error",
            "message": error_message
        }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            GitServiceError::FailedToCreateUser(_) => StatusCode::BAD_REQUEST,
            GitServiceError::FailedToCreateToken(_) => StatusCode::UNAUTHORIZED,
            GitServiceError::FailedToCheckRepoExistence(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::RepositoryAlreadyExists(_) => StatusCode::CONFLICT,
            GitServiceError::FailedToCreateRepository(_) => StatusCode::UNPROCESSABLE_ENTITY,
            GitServiceError::ConnectionFailed(_) => StatusCode::SERVICE_UNAVAILABLE,
            GitServiceError::FailedToCheckUserExistence(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            GitServiceError::ConnectionError(_) => StatusCode::SERVICE_UNAVAILABLE,
            GitServiceError::SessionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::HandshakeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            GitServiceError::ChannelError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::CommandExecutionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::OutputReadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::ChannelCloseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::ExitStatusError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitServiceError::CommandFailedError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

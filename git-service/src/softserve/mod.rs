pub mod config;
pub mod handlers;
pub mod models;
mod ssh;

use ssh::execute_command;

pub fn create_user(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = format!("user create {}", username);
    execute_command(&command)
        .map(|_| format!("User {} created successfully on Soft Serve", username))
}

pub fn create_token(token_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = format!("token create {}", token_name);
    execute_command(&command)
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    username: String,
    message: String,
}

const SOFTSERVE_HOST: &str = "localhost";
const SOFTSERVE_PORT: u16 = 23231;
const SOFTSERVE_USER: &str = "admin";
const SOFTSERVE_KEY_PATH: &str = "~/.ssh/id_ed25519";

fn create_user_on_softserve(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tcp = TcpStream::connect((SOFTSERVE_HOST, SOFTSERVE_PORT))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_pubkey_file(
        SOFTSERVE_USER,
        None,
        std::path::Path::new(SOFTSERVE_KEY_PATH),
        None,
    )?;

    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }

    let mut channel = sess.channel_session()?;

    // Soft Serve command to create a user
    let command = format!("soft-serve users add {}", username);
    channel.exec(&command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;

    channel.wait_close()?;
    let exit_status = channel.exit_status()?;

    if exit_status == 0 {
        Ok(format!(
            "User {} created successfully on Soft Serve",
            username
        ))
    } else {
        Err(format!(
            "Failed to create user. Exit status: {}. Output: {}",
            exit_status, output
        )
        .into())
    }
}

async fn create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    match create_user_on_softserve(&user.username) {
        Ok(message) => HttpResponse::Ok().json(CreateUserResponse {
            username: user.username.clone(),
            message,
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn test_connection() -> impl Responder {
    match TcpStream::connect((SOFTSERVE_HOST, SOFTSERVE_PORT)) {
        Ok(tcp_stream) => {
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp_stream);
            match sess.handshake() {
                Ok(_) => HttpResponse::Ok().body("SSH connection successful"),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("SSH handshake failed: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to connect: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(test_connection))
            .route("/create_user", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

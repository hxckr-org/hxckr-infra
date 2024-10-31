mod softserve;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use softserve::handlers;
use softserve::tracing::{init_tracing, tracing_logger};
use std::env;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    init_tracing(); // Initialize tracing

    // Get host and port from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    info!("Starting server on {}", bind_addr);
    HttpServer::new(|| {
        App::new()
            .wrap(tracing_logger())
            .route("/", web::get().to(handlers::test_connection))
            .route("/create_user", web::post().to(handlers::handle_create_user))
            .route(
                "/create_token",
                web::post().to(handlers::handle_create_token),
            )
            .route("/create_repo", web::post().to(handlers::handle_create_repo))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

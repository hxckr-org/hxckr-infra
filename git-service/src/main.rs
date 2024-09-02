mod softserve;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use softserve::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    env_logger::init(); // Initialize the logger

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(handlers::test_connection))
            .route("/create_user", web::post().to(handlers::handle_create_user))
            .route(
                "/create_token",
                web::post().to(handlers::handle_create_token),
            )
            .route("/create_repo", web::post().to(handlers::handle_create_repo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod softserve;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use softserve::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::test_connection))
            .route("/create_user", web::post().to(handlers::handle_create_user))
            .route(
                "/create_token",
                web::post().to(handlers::handle_create_token),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

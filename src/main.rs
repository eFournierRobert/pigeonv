use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

mod database;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let database = match database::Database::new().await {
        Ok(d) => d,
        Err(e) => panic!("error while initializing database connection: {}", e),
    };

    let app = Router::new()
        .route("/", get(handlers::get_index))
        .route("/submit", post(handlers::post_form))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;
use tracing::info;

use crate::database::Database;

mod database;
mod handlers;
mod services;
mod templates;

#[derive(Clone)]
struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database = match Database::new().await {
        Ok(d) => d,
        Err(e) => panic!("error while initializing database connection: {}", e),
    };
    let state = AppState { db: database };

    let app = Router::new()
        .route("/", get(handlers::get_index))
        .route("/submit", post(handlers::post_form))
        .route("/m/{uuid}", get(handlers::get_message))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    info!("Router started and listening on localhost:8080");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

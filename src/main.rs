use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::index::get_index))
        .route("/submit", post(handlers::index::post_form))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

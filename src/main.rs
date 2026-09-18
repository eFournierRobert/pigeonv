use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::services::ServeDir;

mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

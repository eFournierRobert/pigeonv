//! Pigeonv — a self-expiring paste service.
//!
//! Stack: Axum 0.8, askama templates, SQLite via sqlx (bundled, no system SQLite
//! needed). Single binary. The database (`db/pigeonv.db`) and the `/static`
//! mount are resolved relative to the process working directory, so run from
//! the repository root.
//!
//! Listens on 127.0.0.1:8080 (hardcoded, no `PORT` env var).
//!
//! Routes:
//! - `GET /` — paste creation form
//! - `POST /submit` — creates a paste, shows the share link page
//! - `GET /m/{uuid}` — shows a paste (404 if unknown or expired)
//! - `/static/...` — static assets

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;
use tracing::info;

use crate::database::Database;

/// SQLite access: pool and raw queries.
mod database;
/// Axum route handlers.
mod handlers;
/// Validation and business logic.
mod services;
/// Askama templates and error rendering.
mod templates;

/// Shared state passed to all handlers.
#[derive(Clone)]
struct AppState {
    db: Database,
}

/// Initializes the database (panics on failure), then serves on 127.0.0.1:8080.
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

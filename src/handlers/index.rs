use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::templates;

pub async fn handler() -> impl IntoResponse {
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

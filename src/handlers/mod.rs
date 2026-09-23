//! Axum route handlers.
//!
//! Deliberately thin: parse the request, delegate validation and business logic
//! to the `services` module, and render the result with askama templates.

use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::{AppState, services, templates};

/// Body of the `POST /submit` form.
///
/// Field names (`valeur`, `expiration`) must match the `name` attributes in
/// `templates/index.html`; renaming one side breaks deserialization.
#[derive(Deserialize)]
pub struct NewMessageForm {
    valeur: String,
    expiration: chrono::NaiveDate,
}

/// `GET /` — the paste creation form.
pub async fn get_index() -> Html<String> {
    tracing::info!("Received request: GET /");
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

/// `POST /submit` — stores a new paste and renders the share link page.
pub async fn post_form(
    State(state): State<AppState>,
    Form(form): Form<NewMessageForm>,
) -> impl IntoResponse {
    tracing::info!("Received request: POST /submit");

    match services::insert_message(&state.db, form.valeur, form.expiration).await {
        Ok(uuid) => {
            tracing::info!("Returning UUID {}", uuid);
            let t = templates::link::LinkTemplate {
                uuid: uuid.to_string(),
            };
            Html(t.render().unwrap()).into_response()
        }
        Err(err) => err.into_response(),
    }
}

/// `GET /m/{uuid}` — renders the paste, or the 404 error page if unknown or expired.
pub async fn get_message(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    tracing::info!("Received request: GET /m/{}", uuid);

    match services::get_message(&state.db, uuid).await {
        Ok(value) => {
            let t = templates::message::MessageTemplate { message: value };

            Html(t.render().unwrap()).into_response()
        }
        Err(err) => err.into_response(),
    }
}

use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::Html,
};
use serde::Deserialize;

use crate::{AppState, services, templates};

#[derive(Deserialize)]
pub struct NewMessageForm {
    valeur: String,
    expiration: chrono::NaiveDate,
}

pub async fn get_index() -> Html<String> {
    tracing::info!("Received request: GET /");
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

pub async fn post_form(
    State(state): State<AppState>,
    Form(form): Form<NewMessageForm>,
) -> Html<String> {
    tracing::info!("Received request: POST /submit");

    match services::insert_message(&state.db, form.valeur, form.expiration).await {
        Ok(uuid) => {
            tracing::info!("Returning UUID {}", uuid);
            let t = templates::link::LinkTemplate {
                uuid: uuid.to_string(),
            };
            Html(t.render().unwrap())
        }
        Err(err) => templates::error::load_error_template(err),
    }
}

pub async fn get_message(State(state): State<AppState>, Path(uuid): Path<String>) -> Html<String> {
    tracing::info!("Received request: GET /m/{}", uuid);

    match services::get_message(&state.db, uuid).await {
        Ok(value) => {
            let t = templates::message::MessageTemplate { message: value };

            Html(t.render().unwrap())
        }
        Err(err) => templates::error::load_error_template(err),
    }
}

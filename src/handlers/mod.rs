use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::{AppState, templates};

#[derive(Deserialize)]
pub struct NewMessageForm {
    valeur: String,
    expiration: chrono::NaiveDate,
}

pub async fn get_index() -> impl IntoResponse {
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

pub async fn post_form(
    State(state): State<AppState>,
    Form(form): Form<NewMessageForm>,
) -> impl IntoResponse {
    let t = templates::link::LinkTemplate {
        link: form.valeur.clone(),
    };

    Html(t.render().unwrap())
}

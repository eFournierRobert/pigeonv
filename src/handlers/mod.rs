use askama::Template;
use axum::{
    Form,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::templates;

#[derive(Deserialize)]
pub struct NewMessageForm {
    valeur: String,
    expiration: chrono::NaiveDate,
}

pub async fn get_index() -> impl IntoResponse {
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

pub async fn post_form(Form(form): Form<NewMessageForm>) -> impl IntoResponse {
    let t = templates::link::LinkTemplate {
        link: form.valeur.clone(),
    };

    Html(t.render().unwrap())
}

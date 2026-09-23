use askama::Template;
use axum::{Form, extract::State, response::Html};
use serde::Deserialize;

use crate::{AppState, services, templates};

#[derive(Deserialize)]
pub struct NewMessageForm {
    valeur: String,
    expiration: chrono::NaiveDate,
}

pub async fn get_index() -> Html<String> {
    let t = templates::index::IndexTemplate {};

    Html(t.render().unwrap())
}

pub async fn post_form(
    State(state): State<AppState>,
    Form(form): Form<NewMessageForm>,
) -> Html<String> {
    match services::insert_message(&state.db, form.valeur, form.expiration).await {
        Ok(uuid) => {
            let t = templates::link::LinkTemplate {
                link: format!("localhost/{}", uuid.to_string()),
            };
            Html(t.render().unwrap())
        }
        Err(err) => templates::error::load_error_template(err),
    }
}

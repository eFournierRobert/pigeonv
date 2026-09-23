use askama::Template;
use axum::response::Html;

use crate::services::ServiceErrors;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub error: String,
}

pub fn load_error_template(err: ServiceErrors) -> Html<String> {
    let err_message = match err {
        ServiceErrors::DatabaseErr => String::from("Erreur serveur"),
        ServiceErrors::ExpirationDateAfterCurrentDate => {
            String::from("Date d'expiration avant la date d'aujourd'hui")
        }
        ServiceErrors::WrongValues => String::from("Message ou date d'expiration invalide"),
        ServiceErrors::InvalidUuid => String::from("Lien invalide"),
    };

    let t = ErrorTemplate { error: err_message };
    Html(t.render().unwrap())
}

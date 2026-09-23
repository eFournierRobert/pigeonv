use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub error: String,
}

#[derive(Debug)]
pub enum ServiceErrors {
    ExpirationDateAfterCurrentDate,
    WrongValues,
    DatabaseErr,
    InvalidUuid,
}

impl IntoResponse for ServiceErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ServiceErrors::DatabaseErr => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Une erreur interne est survenue",
            ),
            ServiceErrors::ExpirationDateAfterCurrentDate => (
                StatusCode::BAD_REQUEST,
                "Date d'expiration avant la date d'aujourd'hui",
            ),
            ServiceErrors::InvalidUuid => (StatusCode::NOT_FOUND, "Lien invalide"),
            ServiceErrors::WrongValues => (
                StatusCode::BAD_REQUEST,
                "Message ou date d'expiration invalide",
            ),
        };

        let t = ErrorTemplate {
            error: String::from(message),
        };

        match t.render() {
            Ok(html) => (status, Html(html)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

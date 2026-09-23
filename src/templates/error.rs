//! Shared error handling: maps service failures to HTTP responses.

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

/// Error page rendered for every service failure.
#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub error: String,
}

/// All service-level failure modes.
///
/// Each variant maps (see the `IntoResponse` impl) to an HTTP status and a
/// French user-facing message rendered by `error.html`.
#[derive(Debug)]
pub enum ServiceErrors {
    ExpirationDateAfterCurrentDate,
    WrongValues,
    DatabaseErr,
    InvalidUuid,
}

impl IntoResponse for ServiceErrors {
    /// Renders this error as `error.html` with the matching status code.
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

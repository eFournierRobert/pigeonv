//! Business logic and validation.
//!
//! Handlers delegate here. Failures are mapped to `ServiceErrors`
//! (defined in the `templates` module).

use std::str::FromStr;

use chrono::Local;
use uuid::Uuid;

use crate::{
    database::{Database, message},
    templates::error::ServiceErrors,
};

/// Validates and stores a new paste, returning its uuid.
///
/// The expiration date must be in the future (strictly after today, UTC) and
/// the value must be non-empty.
pub async fn insert_message(
    state: &Database,
    value: String,
    expiration: chrono::NaiveDate,
) -> Result<Uuid, ServiceErrors> {
    let current_date = Local::now().naive_utc().date();
    if current_date >= expiration {
        return Err(ServiceErrors::ExpirationDateAfterCurrentDate);
    }

    if value.is_empty() {
        return Err(ServiceErrors::WrongValues);
    }

    let uuid = Uuid::new_v4();
    message::insert_message(&state.pool, value, expiration, uuid)
        .await
        .map_err(|e| {
            tracing::error!("Database insert failed: {e}");
            ServiceErrors::DatabaseErr
        })
}

/// Looks up a paste by uuid. Unknown or expired pastes yield a 404.
pub async fn get_message(state: &Database, uuid_string: String) -> Result<String, ServiceErrors> {
    let uuid = match uuid::Uuid::from_str(&uuid_string) {
        Ok(u) => u,
        Err(_) => return Err(ServiceErrors::InvalidUuid),
    };

    let message = match message::get_message(&state.pool, uuid).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Database {uuid} select failed: {e}");
            return Err(ServiceErrors::InvalidUuid);
        }
    };

    let current_date = Local::now().naive_utc().date();
    if current_date >= message.expiration {
        return Err(ServiceErrors::InvalidUuid);
    }

    Ok(message.value)
}

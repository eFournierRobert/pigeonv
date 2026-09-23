use std::str::FromStr;

use chrono::Local;
use uuid::Uuid;

use crate::{
    database::{Database, message},
    templates::error::ServiceErrors,
};

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
        .map_err(|_| ServiceErrors::DatabaseErr)
}

pub async fn get_message(state: &Database, uuid_string: String) -> Result<String, ServiceErrors> {
    let uuid = match uuid::Uuid::from_str(&uuid_string) {
        Ok(u) => u,
        Err(_) => return Err(ServiceErrors::InvalidUuid),
    };

    let message = match message::get_message(&state.pool, uuid).await {
        Ok(m) => m,
        Err(_) => return Err(ServiceErrors::InvalidUuid),
    };

    let current_date = Local::now().naive_utc().date();
    if current_date >= message.expiration {
        return Err(ServiceErrors::InvalidUuid);
    }

    Ok(message.value)
}

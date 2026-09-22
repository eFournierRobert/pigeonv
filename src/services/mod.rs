use chrono::Local;
use uuid::Uuid;

use crate::database::{Database, message};

pub enum ServiceErrors {
    ExpirationDateAfterCurrentDate,
    DatabaseErr,
}

pub async fn insert_message(
    state: &Database,
    value: String,
    expiration: chrono::NaiveDate,
) -> Result<Uuid, ServiceErrors> {
    let current_date = Local::now().naive_utc().date();
    if current_date >= expiration {
        return Err(ServiceErrors::ExpirationDateAfterCurrentDate);
    }

    let uuid = Uuid::new_v4();
    message::insert_message(&state.pool, value, expiration, uuid)
        .await
        .map_err(|_| ServiceErrors::DatabaseErr)
}

use crate::database::Database;

pub fn insert_message(
    state: &Database,
    value: String,
    expiration: chrono::NaiveDate,
) -> anyhow::Result<String> {
    Ok(String::new())
}

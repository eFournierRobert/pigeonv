use sqlx::{Pool, Row, Sqlite};
use uuid::Uuid;

pub struct Message {
    pub id: i32,
    pub uuid: Uuid,
    pub value: String,
    pub expiration: chrono::NaiveDate,
}

pub async fn insert_message(
    pool: &Pool<Sqlite>,
    value: String,
    expiration: chrono::NaiveDate,
    uuid: Uuid,
) -> anyhow::Result<Uuid> {
    let row = sqlx::query(
        r#"
        INSERT INTO messages (value, expiration, uuid)
        VALUES (?1, ?2, ?3)
        RETURNING uuid
        "#,
    )
    .bind(value)
    .bind(expiration.to_string())
    .bind(uuid.to_string())
    .fetch_one(pool)
    .await?;

    let row_uuid: String = row.try_get("uuid")?;
    let inserted_uuid = Uuid::parse_str(&row_uuid)?;

    Ok(inserted_uuid)
}

pub async fn get_message(pool: &Pool<Sqlite>, uuid: Uuid) -> anyhow::Result<Message> {
    let m: (String, String) = sqlx::query_as(
        r#"
        SELECT value, expiration FROM messages WHERE uuid = ?1"#,
    )
    .bind(uuid.to_string())
    .fetch_one(pool)
    .await?;

    Ok(Message {
        id: -1,
        uuid,
        value: m.0,
        expiration: chrono::NaiveDate::parse_from_str(&m.1, "%Y-%m-%d")?,
    })
}

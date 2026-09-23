use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};
use tracing::info;

pub mod message;

const DB_URL: &str = "sqlite://db/pigeonv.db";

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        info!("Starting database");

        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            Sqlite::create_database(DB_URL).await?;
            info!("Database {} created", DB_URL);
        }

        let pool = SqlitePool::connect(DB_URL).await?;
        info!("Database {} connected", DB_URL);

        sqlx::migrate!().run(&pool).await?;
        info!("Database migrations completed");

        Ok(Database { pool })
    }
}

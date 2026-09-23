//! SQLite access layer.
//!
//! Owns the connection pool. `DB_URL` (`sqlite://db/pigeonv.db`) is relative to
//! the process working directory. At startup, the database file is created if
//! missing and pending migrations from `migrations/` are applied.

use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};
use tracing::info;

pub mod message;

const DB_URL: &str = "sqlite://db/pigeonv.db";

/// Shared SQLite connection pool.
#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    /// Connects to the database, creating it if missing, and applies pending
    /// migrations from `migrations/`.
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

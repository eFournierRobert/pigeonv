use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite://pigeonv.db";

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Created database!"),
                Err(err) => panic!("error: {}", err),
            }
        }

        let pool = SqlitePool::connect(DB_URL).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Database { pool })
    }
}

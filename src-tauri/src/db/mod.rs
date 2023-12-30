use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn init() -> Result<SqlitePool, sqlx::Error> {
    const DB_URL: &str = "sqlite:sqlite.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await?;
        println!("Successfully created database");
    }

    let pool = SqlitePool::connect(DB_URL).await?;
    Ok(pool)
}

use log::LevelFilter;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, sqlite::SqliteConnectOptions, ConnectOptions};
use url::Url;


pub async fn init() -> Result<SqlitePool, sqlx::Error> {
    const DB_URL: &str = "sqlite:sqlite.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await?;
        println!("Successfully created database");
    }

    let db_url = Url::parse(DB_URL).unwrap();
    let pool = SqlitePool::connect_with(SqliteConnectOptions::from_url(&db_url)?.log_statements(LevelFilter::Trace)).await?;
    Ok(pool)
}

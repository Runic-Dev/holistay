use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const CREATE_ROOM_GROUP_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS room_groups (
        id TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        image_url TEXT NOT NULL
    )
";

pub async fn init() -> Result<SqlitePool, sqlx::Error> {
    const DB_URL: &str = "sqlite:sqlite.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await?;
        println!("Successfully created database");
    }

    let pool = SqlitePool::connect(DB_URL).await?;
    sqlx::query(CREATE_ROOM_GROUP_TABLE).execute(&pool).await?;
    Ok(pool)
}


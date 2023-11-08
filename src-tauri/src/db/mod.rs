use sqlx::{Sqlite, migrate::MigrateDatabase};

pub async fn init() {
    const DB_URL: &str = "sqlite://sqlite.db";
    if !Sqlite::database_exists(DB_URL).await.unwrap() {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Successfully created database"),
            Err(err) => panic!("Error creating database: {:?}", err),
        }
    }
}

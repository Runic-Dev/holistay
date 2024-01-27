use std::fs;

use base64::{engine::general_purpose, Engine};
use tokio::sync::MutexGuard;
use sqlx::{Sqlite, Pool};
use uuid::Uuid;

use crate::models::PropertyPartial;

use super::NewPropertyRequest;

pub async fn get_property_partials(pool_lock: MutexGuard<'_, Pool<Sqlite>>) -> Result<Vec<PropertyPartial>, sqlx::Error> {
    sqlx::query_as::<Sqlite, PropertyPartial>("SELECT id, name, image FROM property")
        .fetch_all(&*pool_lock).await
}

pub async fn add_new_property(pool_lock: MutexGuard<'_, Pool<Sqlite>>, new_property_request: NewPropertyRequest) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let id = Uuid::new_v4();
    let image_file = fs::read(new_property_request.image).expect("Problem reading image file");
    let encoded = general_purpose::STANDARD.encode(image_file);
    sqlx::query("INSERT INTO property (id, name, image) VALUES (?, ?, ?)")
        .bind(id.to_string())
        .bind(new_property_request.name)
        .bind(encoded)
        .execute(&*pool_lock).await 
}

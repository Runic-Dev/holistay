use std::fs;

use base64::{engine::general_purpose, Engine};
use sqlx::{Sqlite, Pool};
use tokio::sync::MutexGuard;
use uuid::Uuid;

use crate::models::RoomPartial;

use super::requests::{NewRoomRequest, GetRoomsRequest};

pub async fn add_new_room(pool_lock: MutexGuard<'_, Pool<Sqlite>>, new_room_request: NewRoomRequest) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let id = Uuid::new_v4();
    let encoded = if let Some(image_path) = new_room_request.image {
        let image_file = fs::read(image_path).expect("Problem reading image file");
        Some(general_purpose::STANDARD.encode(image_file))
    } else {
        None
    };

    sqlx::query("INSERT INTO room (id, room_group_id, name, image) VALUES (?, ?, ?, ?)")
        .bind(id.to_string())
        .bind(new_room_request.room_group_id)
        .bind(new_room_request.name)
        .bind(encoded)
        .execute(&*pool_lock).await 
}

pub async fn get_room_partials(pool_lock: MutexGuard<'_, Pool<Sqlite>>, get_rooms_request: GetRoomsRequest) -> Result<Vec<RoomPartial>, sqlx::Error> {
    sqlx::query_as::<Sqlite, RoomPartial>("SELECT id, name, image FROM room WHERE room_group_id = ?")
        .bind(get_rooms_request.room_group_id)
        .fetch_all(&*pool_lock).await
}



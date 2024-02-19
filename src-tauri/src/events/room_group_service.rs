use std::fs;

use base64::{engine::general_purpose, Engine};
use sqlx::{Sqlite, Pool};
use tokio::sync::MutexGuard;
use uuid::Uuid;

use crate::models::RoomGroup;

use super::requests::{NewRoomGroupRequest, GetRoomGroupsRequest};

pub async fn add_new_room_group(pool_lock: MutexGuard<'_, Pool<Sqlite>>, new_room_group_request: NewRoomGroupRequest) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let id = Uuid::new_v4();
    let encoded_image = if let Some(image_location) = new_room_group_request.image {
        let image_file = fs::read(image_location).expect("Problem reading image file");
        Some(general_purpose::STANDARD.encode(image_file))
    } else {
        None
    };
    sqlx::query("INSERT INTO room_group (id, property_id, name, image) VALUES (?, ?, ?, ?)")
        .bind(id.to_string())
        .bind(new_room_group_request.property_id)
        .bind(new_room_group_request.name)
        .bind(encoded_image)
        .execute(&*pool_lock).await 
}

pub async fn get_room_groups(pool_lock: MutexGuard<'_, Pool<Sqlite>>, get_room_groups_request: GetRoomGroupsRequest) -> Result<Vec<RoomGroup>, sqlx::Error> {
    sqlx::query_as!(
        RoomGroup,
        "SELECT id, name, image FROM room_group WHERE property_id = ?",
        get_room_groups_request.property_id
    ).fetch_all(&*pool_lock).await
}

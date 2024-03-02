use std::fs;

use base64::{engine::general_purpose, Engine};
use tokio::sync::MutexGuard;
use sqlx::{Sqlite, Pool, prelude::FromRow};
use uuid::Uuid;

use crate::models::{PropertyPartial, Property, RoomGroup};

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

pub async fn get_property(pool_lock: MutexGuard<'_, Pool<Sqlite>>, property_id: String) -> Result<Property, sqlx::Error> {

    match sqlx::query_as::<Sqlite, PropertyRoomGroupRow>(
        "SELECT p.id as property_id, p.name as property_name, p.image as property_image, rg.id as room_group_id, rg.name as room_group_name, rg.image as room_group_image FROM property p LEFT OUTER JOIN room_group rg ON p.id = rg.property_id WHERE p.id = ?",
    )
    .bind(property_id)
    .fetch_all(&*pool_lock)
    .await {
            Ok(property_room_group_rows) => {
                let property_data = match property_room_group_rows.first() {
                    Some(first_entry) => first_entry.clone(),
                    None => panic!("Dang"),
                };
                let room_groups = property_room_group_rows.into_iter().filter_map(|x| Some(RoomGroup {
                    id : x.room_group_id?,
                    name: x.room_group_name?,
                    image: x.room_group_image,
                })).collect::<Vec<RoomGroup>>();            
                Ok(Property {
                    id: property_data.property_id,
                    name: property_data.property_name,
                    image: property_data.property_image,
                    room_groups
                })
            },
            Err(err) => {
                println!("Error getting property from database: {err:?}");
                Err(err)
            },
        }
}

#[derive(Clone, FromRow)]
struct PropertyRoomGroupRow {
    pub property_id: String,
    pub property_name: String,
    pub property_image: Option<String>,
    pub room_group_id: Option<String>,
    pub room_group_name: Option<String>,
    pub room_group_image: Option<String>
}

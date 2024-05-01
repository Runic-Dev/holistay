use std::{fs, collections::HashSet};

use tokio::sync::MutexGuard;
use base64::{engine::general_purpose, Engine};
use sqlx::{Sqlite, Pool, prelude::FromRow};
use uuid::Uuid;
use crate::models::domain::property::{Property, PropertyPartial};
use crate::models::domain::room::Room;
use crate::models::domain::room_group::RoomGroup;
use crate::models::requests::{NewDescriptionRequest, NewPropertyRequest};


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

pub async fn get_property(pool_lock: MutexGuard<'_, Pool<Sqlite>>, property_id: String) -> Result<Option<Property>, sqlx::Error> {
    match sqlx::query_as::<Sqlite, PropertyRoomGroupRoomRow>(
"
SELECT p.id as property_id, p.name as property_name, p.image as property_image, 
rg.id as room_group_id, rg.name as room_group_name, rg.image as room_group_image, 
r.id as room_id, r.name as room_name, r.image as room_image FROM property p LEFT OUTER JOIN room_group rg 
ON p.id = rg.property_id LEFT OUTER JOIN room r ON rg.id = r.room_group_id WHERE p.id = ?",
    )
    .bind(property_id.clone())
    .fetch_all(&*pool_lock)
    .await {
            Ok(property_room_group_rows) => {
                if let Some(property_data) = property_room_group_rows.first() {
                    let property = Property {
                        id: property_data.property_id.to_string(),
                        name: property_data.property_name.to_string(),
                        image: property_data.property_image.clone(),
                        room_groups: room_groups_from_rows(&property_room_group_rows)
                    };
                    Ok(Some(property))
                } else {
                    Ok(None)
                }
            },
            Err(err) => {
                println!("Error getting property from database: {err:?}");
                Err(err)
            },
        }
}

pub async fn update_description(pool_lock: MutexGuard<'_, Pool<Sqlite>>, new_property_desc_request: NewDescriptionRequest) -> Result<(String, String), sqlx::Error> {
    sqlx::query("UPDATE property SET description = ? WHERE id = ?")
        .bind(&new_property_desc_request.description)
        .bind(&new_property_desc_request.id)
        .execute(&*pool_lock).await.map_or_else(|err| Err(err), |_| Ok((new_property_desc_request.id, new_property_desc_request.description)))
}

#[derive(Clone, FromRow)]
struct PropertyRoomGroupRoomRow {
    pub property_id: String,
    pub property_name: String,
    pub property_image: Option<String>,
    pub room_group_id: Option<String>,
    pub room_group_name: Option<String>,
    pub room_group_image: Option<String>,
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub room_image: Option<String>
}

impl PartialEq for RoomGroup {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RoomGroup {}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Room {}

fn room_groups_from_rows(rows: &Vec<PropertyRoomGroupRoomRow>) -> Vec<RoomGroup> {
    rows.into_iter().fold(HashSet::<RoomGroup>::new(), |mut rg_set, row| {
        if let (Some(room_group_id), Some(room_group_name)) = (row.room_group_id.as_ref(), row.room_group_name.as_ref()) {
            let room_group = RoomGroup {
                id: room_group_id.to_string(),
                name: room_group_name.to_string(),
                image: row.room_group_image.clone(),
                description: String::new(),
                rooms: rooms_from_rows(rows.clone(), room_group_id)
            };
            rg_set.insert(room_group);
        }
        rg_set
    }).into_iter().collect()
}

fn rooms_from_rows(rows: Vec<PropertyRoomGroupRoomRow>, room_group_id: &str) -> Vec<Room> {
    rows.into_iter().filter(|f| f.room_group_id.clone().is_some_and(|x| x == room_group_id)).fold(HashSet::<Room>::new(), |mut room_set, row| {
        if let (Some(room_id), Some(room_name)) = (row.room_id, row.room_name) {
            let room = Room { id: room_id, name: room_name, image: row.room_image.clone() };
            room_set.insert(room);
        }
        room_set
    }).into_iter().collect()
}


use std::{fs, collections::HashMap};

use base64::{engine::general_purpose, Engine};
use tokio::sync::MutexGuard;
use sqlx::{Sqlite, Pool, prelude::FromRow};
use uuid::Uuid;

use crate::models::{PropertyPartial, Property, RoomGroup, Room};

use super::{NewPropertyRequest, requests::NewDescriptionRequest};

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
    match sqlx::query_as::<Sqlite, PropertyRoomGroupRoomRow>(
"
SELECT p.id as property_id, p.name as property_name, p.image as property_image, 
rg.id as room_group_id, rg.name as room_group_name, rg.image as room_group_image, 
r.id as room_id, r.room_group_id as room_room_group_id, r.name as room_name, r.image as room_image FROM property p LEFT OUTER JOIN room_group rg 
ON p.id = rg.property_id LEFT OUTER JOIN room r ON rg.id = r.room_group_id WHERE p.id = ?",
    )
    .bind(property_id)
    .fetch_all(&*pool_lock)
    .await {
            Ok(property_room_group_rows) => {
                let property_data = match property_room_group_rows.first() {
                    Some(first_entry) => first_entry.clone(),
                    None => panic!("Dang"),
                };

                let mut roomsMap = HashMap::<String, Vec<Room>>::new();

                property_room_group_rows.iter().filter(|x| {
                    x.room_id.is_some() && x.room_name.is_some() && x.room_room_group_id.is_some()
                }).for_each(|row| {
                    if let (Some(room_id), Some(room_room_group_id), Some(room_name)) = (row.room_id.as_ref(), row.room_room_group_id.as_ref(), row.room_name.as_ref()) {
                        let room = Room { id: room_id.to_string(), name: room_name.to_string(), image: row.room_image.clone() };
                        if let Some(room_vec) = roomsMap.get_mut(room_room_group_id) {
                            room_vec.push(room);
                        } else {
                            roomsMap.insert(room_room_group_id.to_string(), vec![room]);
                        }
                    }
                });


                let room_groups = property_room_group_rows.into_iter().filter_map(|x| Some(RoomGroup {
                    id : x.room_group_id.clone()?,
                    name: x.room_group_name?,
                    image: x.room_group_image,
                    description: String::new(),
                    rooms: roomsMap.get(&x.room_group_id?).map_or_else(|| Vec::<Room>::new(), |room_vec| room_vec.clone())
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

pub async fn update_description(pool_lock: MutexGuard<'_, Pool<Sqlite>>, new_property_desc_request: NewDescriptionRequest) -> Result<(String, String), sqlx::Error> {
    sqlx::query("UPDATE property SET description = ? WHERE id = ?")
        .bind(&new_property_desc_request.description)
        .bind(&new_property_desc_request.id)
        .execute(&*pool_lock).await.map_or_else(|err| Err(err), |_| Ok((new_property_desc_request.id, new_property_desc_request.description)))
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

#[derive(Clone, FromRow)]
struct PropertyRoomGroupRoomRow {
    pub property_id: String,
    pub property_name: String,
    pub property_image: Option<String>,
    pub room_group_id: Option<String>,
    pub room_group_name: Option<String>,
    pub room_group_image: Option<String>,
    pub room_id: Option<String>,
    pub room_room_group_id: Option<String>,
    pub room_name: Option<String>,
    pub room_image: Option<String>
}


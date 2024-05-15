use crate::models::domain::property::{Property, PropertyPartial};
use crate::models::requests::{AddNewPropertyRequest, NewDescriptionRequest};
use crate::repositories::db_intermediary::property_room_group_room_row::{
    CanBeConvertedToRoomGroups, PropertyRoomGroupRoomRow,
};
use base64::engine::general_purpose;
use base64::Engine;
use sqlx::{Sqlite, SqlitePool};
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

pub struct PropertyRepository {
    conn_pool: Arc<Mutex<SqlitePool>>,
}

impl PropertyRepository {
    pub fn new(conn_pool: Arc<Mutex<SqlitePool>>) -> Self {
        Self { conn_pool }
    }
}

#[cfg_attr(test, automock)]
pub trait IsPropertyRepository {
    fn get_property_partials(&self) -> impl std::future::Future<Output = Result<Vec<PropertyPartial>, sqlx::Error>> + Send;
    fn add_new_property(
        &self,
        new_property_request: AddNewPropertyRequest,
    ) -> impl std::future::Future<Output = Result<(String, Option<String>), sqlx::Error>> + Send;
    fn get_property(&self, property_id: String) -> impl std::future::Future<Output = Result<Option<Property>, sqlx::Error>> + Send;
    fn update_description(
        &self,
        new_property_desc_request: NewDescriptionRequest,
    ) -> impl std::future::Future<Output = Result<(String, String), sqlx::Error>> + Send;
}

impl IsPropertyRepository for PropertyRepository {
    async fn get_property_partials(&self) -> Result<Vec<PropertyPartial>, sqlx::Error> {
        let pool_lock = self.conn_pool.lock().await;
        sqlx::query_as::<Sqlite, PropertyPartial>("SELECT id, name, image FROM property")
            .fetch_all(&*pool_lock)
            .await
    }

    async fn add_new_property(
        &self,
        new_property_request: AddNewPropertyRequest,
    ) -> Result<(String, Option<String>), sqlx::Error> {
        let pool_lock = self.conn_pool.lock().await;
        let id = Uuid::new_v4();
        let image = match fs::read(new_property_request.image)
            .map(|image_file| general_purpose::STANDARD.encode(image_file))
        {
            Ok(encoded) => Some(encoded),
            Err(_) => None,
        };
        sqlx::query("INSERT INTO property (id, name, image) VALUES (?, ?, ?)")
            .bind(id.to_string())
            .bind(new_property_request.name)
            .bind(&image)
            .execute(&*pool_lock)
            .await
            .map(|_| (id.to_string(), image))
    }

    async fn get_property(&self, property_id: String) -> Result<Option<Property>, sqlx::Error> {
        let pool_lock = self.conn_pool.lock().await;
        match sqlx::query_as::<Sqlite, PropertyRoomGroupRoomRow>(
                "
SELECT p.id as property_id, p.name as property_name, p.image as property_image, p.description as property_description,
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
                            description: property_data.property_description.clone(),
                            image: property_data.property_image.clone(),
                            room_groups: property_room_group_rows.room_groups_from_rows()
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

    async fn update_description(
        &self,
        new_property_desc_request: NewDescriptionRequest,
    ) -> Result<(String, String), sqlx::Error> {
        let pool_lock = self.conn_pool.lock().await;
        sqlx::query("UPDATE property SET description = ? WHERE id = ?")
            .bind(&new_property_desc_request.description)
            .bind(&new_property_desc_request.id)
            .execute(&*pool_lock)
            .await
            .map_or_else(
                |err| Err(err),
                |_| {
                    Ok((
                        new_property_desc_request.id,
                        new_property_desc_request.description,
                    ))
                },
            )
    }
}

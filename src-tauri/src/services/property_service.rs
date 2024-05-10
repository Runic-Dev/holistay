use std::fs;
use std::future::Future;
use std::sync::Arc;

use crate::models::requests::{AddNewPropertyRequest, NewDescriptionRequest};
use crate::repositories::property_repository::{IsPropertyRepository, PropertyRepository};
use crate::services::responses::property_service_responses::{
    AddNewPropertyResponse, GetPropertiesResponse, GetPropertyPartialsResponse,
};
use base64::{engine::general_purpose, Engine};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

pub trait IsPropertyService {
    fn get_property(
        &self,
        property_id: String,
    ) -> impl Future<Output = GetPropertiesResponse> + Send;
    fn get_property_partials(&self) -> impl Future<Output = GetPropertyPartialsResponse> + Send;
    fn add_new_property(
        &self,
        new_property_request: AddNewPropertyRequest,
    ) -> impl Future<Output = AddNewPropertyResponse> + Send;
}

pub struct PropertyService {
    property_repository: Arc<Mutex<PropertyRepository>>,
}

impl PropertyService {
    pub fn new(property_repository: Arc<Mutex<PropertyRepository>>) -> Mutex<Self> {
        Mutex::from(PropertyService {
            property_repository,
        })
    }
}

impl IsPropertyService for PropertyService {
    async fn get_property(&self, property_id: String) -> GetPropertiesResponse {
        let repo_lock = self.property_repository.lock().await;
        match repo_lock.get_property(property_id).await {
            Ok(property_option) => match property_option {
                Some(property) => GetPropertiesResponse::Successful { property },
                None => GetPropertiesResponse::Unsuccessful {
                    error_message: String::from("Property not found"),
                },
            },
            Err(_) => {
                // TODO: Observability to be considered around here
                GetPropertiesResponse::Unsuccessful {
                    error_message: String::from("Error communicating with database"),
                }
            }
        }
    }

    async fn get_property_partials(&self) -> GetPropertyPartialsResponse {
        let repo_lock = self.property_repository.lock().await;
        match repo_lock.get_property_partials().await {
            Ok(property_partials) => GetPropertyPartialsResponse::Successful { property_partials },
            Err(_) => GetPropertyPartialsResponse::Unsuccessful {
                error_message: String::from("Error communicating with database"),
            },
        }
    }

    async fn add_new_property(
        &self,
        new_property_request: AddNewPropertyRequest,
    ) -> AddNewPropertyResponse {
        let repo_lock = self.property_repository.lock().await;
        match repo_lock.add_new_property(new_property_request).await {
            Ok(property_id) => AddNewPropertyResponse::Successful { property_id },
            Err(err) => {
                println!("Unsuccessful!");
                AddNewPropertyResponse::Unsuccessful {
                    error_message: err.to_string(),
                }
            }
        }
    }
}

pub async fn add_new_property(
    pool_lock: MutexGuard<'_, Pool<Sqlite>>,
    new_property_request: AddNewPropertyRequest,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let id = Uuid::new_v4();
    let image_file = fs::read(new_property_request.image).expect("Problem reading image file");
    let encoded = general_purpose::STANDARD.encode(image_file);
    sqlx::query("INSERT INTO property (id, name, image) VALUES (?, ?, ?)")
        .bind(id.to_string())
        .bind(new_property_request.name)
        .bind(encoded)
        .execute(&*pool_lock)
        .await
}

pub async fn update_description(
    pool_lock: MutexGuard<'_, Pool<Sqlite>>,
    new_property_desc_request: NewDescriptionRequest,
) -> Result<(String, String), sqlx::Error> {
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

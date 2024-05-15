use std::fs;

use crate::models::requests::{AddNewPropertyRequest, NewDescriptionRequest};
use crate::repositories::property_repository::IsPropertyRepository;
use crate::services::responses::property_service_responses::{
    AddNewPropertyResponse, GetPropertiesResponse, GetPropertyPartialsResponse,
};
use base64::{engine::general_purpose, Engine};
use sqlx::{Pool, Sqlite};
use tokio::sync::MutexGuard;
use uuid::Uuid;

pub trait IsPropertyService {
    fn get_property(&self, property_id: String) -> impl std::future::Future<Output = GetPropertiesResponse> + Send;
    fn get_property_partials(&self) -> impl std::future::Future<Output = GetPropertyPartialsResponse> + Send;
    fn add_new_property(
        &self,
        new_property_request: AddNewPropertyRequest,
    ) -> impl std::future::Future<Output = AddNewPropertyResponse> + Send;
}

pub struct PropertyService<T: IsPropertyRepository> {
    property_repository: T,
}

impl<T: IsPropertyRepository> PropertyService<T> {
    pub fn new(property_repository: T) -> PropertyService<T> {
        Self {
            property_repository,
        }
    }
}

impl<T: IsPropertyRepository + std::marker::Sync> IsPropertyService for PropertyService<T> {
    async fn get_property(&self, property_id: String) -> GetPropertiesResponse {
        match self.property_repository.get_property(property_id).await {
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
        match self.property_repository.get_property_partials().await {
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
        match self
            .property_repository
            .add_new_property(new_property_request)
            .await
        {
            Ok((property_id, image_option)) => AddNewPropertyResponse::Successful {
                property_id,
                image_option,
            },
            Err(err) => AddNewPropertyResponse::Unsuccessful {
                error_message: err.to_string(),
            },
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

#[cfg(test)]
pub mod property_service_should {
    use crate::{
        models::domain::property::PropertyPartial,
        repositories::property_repository::MockIsPropertyRepository,
    };

    use super::{PropertyService, IsPropertyService};

    #[tokio::test]
    pub async fn call_property_repository() {
        let mut mock_repo = MockIsPropertyRepository::new();
        mock_repo
            .expect_get_property_partials()
            .once()
            .returning(|| {
                Box::pin(async move {
                    Ok(vec![PropertyPartial {
                        id: "test_id".to_string(),
                        name: "test_name".to_string(),
                        image: "test_image".to_string(),
                    }])
                })
            });
        let property_service = PropertyService::new(mock_repo);
        let _ = property_service.get_property_partials().await;
    }
}

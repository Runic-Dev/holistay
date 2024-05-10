use crate::models::domain::property::PropertyPartial;
use crate::models::domain::HasFrontEndModel;
use crate::models::requests::{AddNewPropertyRequest, GetPropertyRequest};
use crate::services::property_service::IsPropertyService;
use crate::services::responses::property_service_responses::{
    AddNewPropertyResponse, GetPropertiesResponse, GetPropertyPartialsResponse,
};
use crate::AppState;
use serde_json::{json, Value};
use tauri::{command, generate_handler, Builder, Runtime, State};

#[command]
pub async fn get_property_partials(
    state: State<'_, AppState>,
) -> Result<Vec<PropertyPartial>, String> {
    let property_service_lock = state.property_service.lock().await;
    match property_service_lock.get_property_partials().await {
        GetPropertyPartialsResponse::Successful { property_partials } => Ok(property_partials),
        GetPropertyPartialsResponse::Unsuccessful { error_message } => Err(error_message),
    }
}
#[command]
pub async fn get_property(
    state: State<'_, AppState>,
    request: GetPropertyRequest,
) -> Result<Value, String> {
    let property_service_lock = state.property_service.lock().await;
    match property_service_lock
        .get_property(request.property_id)
        .await
    {
        GetPropertiesResponse::Successful { property } => Ok(property.to_json_value()),
        GetPropertiesResponse::Unsuccessful { error_message } => Err(error_message),
    }
}

#[command]
pub async fn add_new_property(
    state: State<'_, AppState>,
    request: AddNewPropertyRequest,
) -> Result<String, String> {
    let property_service_lock = state.property_service.lock().await;
    match property_service_lock.add_new_property(request).await {
        AddNewPropertyResponse::Successful { property_id} => Ok(property_id),
        AddNewPropertyResponse::Unsuccessful { error_message } => Err(error_message),
    }
}

pub trait CommandInitializer {
    fn configure_commands(self) -> Self;
}

impl<R: Runtime> CommandInitializer for Builder<R> {
    fn configure_commands(self) -> Self {
        self.invoke_handler(generate_handler![
            get_property_partials,
            get_property,
            add_new_property
        ])
    }
}

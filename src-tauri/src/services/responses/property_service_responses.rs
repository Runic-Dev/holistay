use crate::models::domain::property::{Property, PropertyPartial};

pub enum GetPropertiesResponse {
    Successful { property: Property },
    Unsuccessful { error_message: String }
}

pub enum GetPropertyPartialsResponse {
    Successful { property_partials: Vec<PropertyPartial> },
    Unsuccessful { error_message: String }
}

pub enum AddNewPropertyResponse {
    Successful { property_id: String },
    Unsuccessful { error_message: String }
}
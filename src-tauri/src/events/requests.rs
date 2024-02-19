use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewPropertyRequest {
    pub name: String,
    pub image: String
}

#[derive(Serialize, Deserialize)]
pub struct NewRoomGroupRequest {
    pub name: String,
    // TODO: This should really be Uuid. Make it Uuid and deal with Deserialization
    pub property_id: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRoomGroupsRequest {
    pub property_id: String,
}



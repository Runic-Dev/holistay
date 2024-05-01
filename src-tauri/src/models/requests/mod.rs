use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewPropertyRequest {
    pub name: String,
    pub image: String
}

#[derive(Serialize, Deserialize)]
pub struct NewRoomGroupRequest {
    pub name: String,
    pub property_id: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRoomGroupsRequest {
    pub property_id: String,
}

#[derive(Deserialize, Clone)]
pub struct LoginRegisterRequest {
    pub username: String,
    pub password: String,
    pub stay_logged_in: bool,
}

#[derive(Deserialize, Clone)]
pub struct NewDescriptionRequest {
    pub id: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct NewRoomRequest {
    pub name: String,
    pub room_group_id: String,
    pub image: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct GetRoomsRequest {
    pub room_group_id: String,
}

#[derive(Deserialize, Clone)]
pub struct GetPropertyRequest {
    pub property_id: String,
}

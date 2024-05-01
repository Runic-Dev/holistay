use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::domain::room_group::RoomGroup;

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub room_groups: Vec<RoomGroup>
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PropertyPartial {
    pub id: String,
    pub name: String,
    pub image: String,
}


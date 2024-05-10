use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use crate::models::domain::HasFrontEndModel;
use crate::models::domain::room_group::RoomGroup;

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub room_groups: Vec<RoomGroup>
}

impl HasFrontEndModel for Property {
    fn to_json_value(&self) -> Value {
        json!({
            "id": self.id,
            "name": self.name,
            "image": self.image,
            "description": self.description,
            "roomGroups": self.room_groups
        })
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PropertyPartial {
    pub id: String,
    pub name: String,
    pub image: String,
}


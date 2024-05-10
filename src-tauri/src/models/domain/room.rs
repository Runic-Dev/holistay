use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct RoomPartial {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Room {}

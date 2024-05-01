use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::sqlite::SqliteRow;
use crate::models::domain::room::Room;

#[derive(Serialize, Deserialize, Hash)]
pub struct RoomGroup {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub description: String,
    pub rooms: Vec<Room>
}

impl FromRow<'_, SqliteRow> for RoomGroup {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            image: row.try_get("image")?,
            description: row.try_get("description")?,
            rooms: vec![]
        })
    }
}


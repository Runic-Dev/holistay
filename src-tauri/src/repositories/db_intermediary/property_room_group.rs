use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::sqlite::SqliteRow;

#[derive(Serialize, Deserialize)]
pub struct PropertyRoomGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub roomgroup_id: String,
    pub roomgroup_name: String,
    pub roomgroup_image: String
}

impl FromRow<'_, SqliteRow> for PropertyRoomGroup {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("property_id")?,
            name: row.try_get("property_name")?,
            description: row.try_get("description")?,
            image: row.try_get("property_image")?,
            roomgroup_id: row.try_get("roomgroup_id")?,
            roomgroup_name: row.try_get("roomgroup_name")?,
            roomgroup_image: row.try_get("roomgroup_image")?
        })
    }
}


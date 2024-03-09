use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

pub mod user;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Contact {
    name: String,
    phone: String,
    email: String,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct RoomGroup {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub description: String,
    pub rooms: Vec<Room>
}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct RegisteredUser {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Clone)]
pub struct LoggedInUser {
    pub username: String,
}

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

#[derive(Serialize, Deserialize, FromRow)]
pub struct RoomPartial {
    pub id: String,
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct PropertyRoomGroup {
    pub id: String,
    pub name: String,
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
            image: row.try_get("property_image")?,
            roomgroup_id: row.try_get("roomgroup_id")?,
            roomgroup_name: row.try_get("roomgroup_name")?,
            roomgroup_image: row.try_get("roomgroup_image")?
        })
    }
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


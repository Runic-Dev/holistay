use serde::{
    de::{self, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use serde_json::Value;
use sqlx::{prelude::FromRow, sqlite::SqliteRow};
use sqlx::{Error, Row};
use uuid::Uuid;
use crate::models::responses::{HasHolistayResponse, HolistayResponse};

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|_| Error::RowNotFound)?;

        Ok(Self {
            id,
            name: row.try_get("username")?,
        })
    }
}

impl User {
    pub const fn new(id: Uuid, username: String) -> Self {
        Self {
            id, name: username
        }
    }
    pub fn from_user_row(user_row: UserRow) -> Result<User, uuid::Error> {
        Ok(Self {
            id: Uuid::parse_str(&user_row.id)?,
            name: user_row.name
        })
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 2)?;

        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;

        state.end()
    }
}

struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct User")
    }

    fn visit_map<A>(self, mut map: A) -> Result<User, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut id = None;
        let mut username = None;

        while let Some(key) = map.next_key()? {
            match key {
                "id" => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    let id_string: &str = map.next_value()?;
                    match Uuid::parse_str(id_string) {
                        Ok(parsed_id) => id = Some(parsed_id),
                        Err(_) => {
                            return Err(de::Error::custom("Unable to parse Uuid from given string"))
                        }
                    }
                }
                "username" => {
                    if username.is_some() {
                        return Err(de::Error::duplicate_field("username"));
                    }
                    username = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, &["id", "username"])),
            }
        }
        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let username = username.ok_or_else(|| de::Error::missing_field("username"))?;

        Ok(User { id, name: username })
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(UserVisitor)
    }
}

impl HasHolistayResponse<User> for Option<User> {
    fn to_response(self, value: Value) -> HolistayResponse<User> {
        HolistayResponse::new(value, self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: String,
    pub name: String
}


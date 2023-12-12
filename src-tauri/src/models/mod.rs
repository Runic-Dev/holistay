pub mod user;

use serde::{Serialize, ser::SerializeStruct, Deserialize};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("User", 2)?;

        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("username", &self.username)?;

        state.end()
    }
}

impl Deserialize for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}

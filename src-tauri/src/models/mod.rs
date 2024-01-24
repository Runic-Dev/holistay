use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Serialize, Deserialize)]
pub struct RoomGroup {}

#[derive(Deserialize, Clone)]
pub struct LoginRegisterAttempt {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct RegisteredUser {
    pub username: String,
}

#[derive(Serialize, Clone)]
pub struct LoggedInUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Property {
    pub id: String,
    pub name: String,
}

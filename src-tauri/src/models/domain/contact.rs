use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Contact {
    name: String,
    phone: String,
    email: String,
}


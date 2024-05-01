use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RegisteredUser {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Clone)]
pub struct LoggedInUser {
    pub username: String,
}


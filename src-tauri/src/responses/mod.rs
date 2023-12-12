use serde::{Deserialize, Serialize};

use crate::models::User;

pub mod errors;

#[derive(Serialize, Deserialize)]
pub struct CurrentUserResponse {
    id: String,
    username: String,
}

impl CurrentUserResponse {
    pub fn from(current_user: User) {

    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoCurrentUser {
    message: String
}

impl Default for NoCurrentUser {
    fn default() -> Self {
        Self { message: "No user logged in".to_string() }
    }
}

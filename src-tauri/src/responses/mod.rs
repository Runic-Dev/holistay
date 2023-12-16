use serde::Serialize;
use serde_json::Value;

pub mod errors;

#[derive(Serialize)]
pub struct HolistayResponse<T: Serialize> {
    message: Value, 
    body: Option<T>
}

impl<T: Serialize> HolistayResponse<T> {
    pub fn new(message: Value, body: Option<T>) -> Self {
        Self {
            message,
            body
        }
    }
}

pub trait HasHolistayResponse<T: Serialize> {
    fn to_response(self, value: Value) -> HolistayResponse<T>;
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String
}


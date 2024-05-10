use serde_json::Value;

pub mod property;
pub mod address;
pub mod contact;
pub mod room_group;
pub mod room;

pub trait HasFrontEndModel {
    fn to_json_value(&self) -> Value;
}
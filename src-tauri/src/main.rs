// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_properties(holistay_state: tauri::State<HolistayState>) -> serde_json::Value {
    json!(holistay_state.properties)
}

struct HolistayState {
    properties: Vec<Property>
}

impl HolistayState {
    pub fn new() -> Self {
        Self {
            properties: vec![]
        }
    }
}

#[derive(Serialize)]
struct Property {
    id: String,
    name: String,
    image_url: String,
    address: Address,
    contact: Contact,
    room_groups: Vec<RoomGroup>
}

impl Property {
    pub fn new(name: &str, image_url: &str, address: Address, contact: Contact, room_groups: Vec<RoomGroup>) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid.to_string(),
            name: name.to_string(),
            image_url: image_url.to_string(),
            address,
            contact,
            room_groups
        }
    }
}

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String
}

#[derive(Serialize)]
struct Contact {
    name: String,
    phone: String,
    email: String
}

#[derive(Serialize)]
struct RoomGroup {}

fn main() {
    tauri::Builder::default()
        .manage(HolistayState::new())
        .setup(|_app| {

            db::init();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

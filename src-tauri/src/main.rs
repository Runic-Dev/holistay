// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod models;
pub mod responses;

use models::User;
use responses::{CurrentUserResponse, errors::NoCurrentUser};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use sqlx::{FromRow, Pool, Sqlite};
use uuid::Uuid;

mod db;

// #[tauri::command]
// async fn get_properties(holistay_state: tauri::State<HolistayState>) -> serde_json::Value {
//     let response = match holistay_state.conn_pool.lock() {
//         Ok(conn_lock) => {
//             let query = sqlx::query_as::<_, Property>("SELECT * FROM property")
//                 .fetch(&conn_lock)
//                 .await;
//             todo!();
//         }
//         Err(_) => todo!(),
//     };
// }

#[tauri::command]
async fn get_current_user(holistay_state: tauri::State<'_, HolistayState>) -> Result<CurrentUserResponse, NoCurrentUser> {
    match holistay_state.current_user.lock() {
        Ok(user_option) => {
            Ok(user_option.as_ref().map_or_else(|| NoCurrentUser, |current_user| Ok(CurrentUserResponse)))
        },
        Err(err) => Ok(json!(format!("Failed to lock current user: {:?}", err))),
    }
}

struct HolistayState {
    conn_pool: Mutex<Pool<Sqlite>>,
    current_user: Mutex<Option<User>>,
}

impl HolistayState {
    pub fn new(conn_pool: Pool<Sqlite>) -> Self {
        let mutex_pool = Mutex::from(conn_pool);
        Self {
            conn_pool: mutex_pool,
            current_user: Mutex::from(None),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SuccessfulResponse {
    payload: Property,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Property {
    id: String,
    name: String,
    image_url: String,
    address: Address,
    contact: Contact,
    room_groups: Vec<RoomGroup>,
}

impl Property {
    pub fn new(
        name: &str,
        image_url: &str,
        address: Address,
        contact: Contact,
        room_groups: Vec<RoomGroup>,
    ) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            id: uuid.to_string(),
            name: name.to_string(),
            image_url: image_url.to_string(),
            address,
            contact,
            room_groups,
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
struct Address {
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct RoomGroup {}

#[tokio::main]
async fn main() {
    match db::init().await {
        Ok(pool) => {
            tauri::Builder::default()
                .setup(|_app| Ok(()))
                .manage(HolistayState::new(pool))
                .invoke_handler(tauri::generate_handler![get_current_user])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {}", err),
    }
}

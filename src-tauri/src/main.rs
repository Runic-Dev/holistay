// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use events::CanConfigureEvents;
use tauri::{EventLoopMessage, Event};
use tokio::sync::mpsc::Sender;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::Mutex;

pub mod events;
mod models;
pub mod responses;

use models::user::User;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

mod db;

#[tauri::command]
fn get_current_user(holistay_state: tauri::State<'_, HolistayState>) {
    tauri::async_runtime::spawn(async {
        let current_user_lock = holistay_state.current_user.lock().await;
        if let Some(current_user) = *current_user_lock {
            holistay_state
                .event_channel
                .send(HolistayEvent::UpdateLoggedInUser(current_user));
        } else {
            holistay_state
                .event_channel
                .send(HolistayEvent::NoLoggedInUser);
        };
    });
}

struct RegisterUserData {
    username: String,
    password: String,
}

enum HolistayEvent {
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(Event),
}

struct HolistayState {
    conn_pool: Mutex<Pool<Sqlite>>,
    current_user: Mutex<Option<User>>,
    event_channel: Sender<HolistayEvent>,
}

impl HolistayState {
    pub fn new(conn_pool: Pool<Sqlite>, event_channel: Sender<HolistayEvent>) -> Self {
        let mutex_pool = Mutex::from(conn_pool);
        Self {
            conn_pool: mutex_pool,
            current_user: Mutex::from(None),
            event_channel,
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

#[derive(Deserialize)]
struct RegisterAttempt {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    match db::init().await {
        Ok(pool) => {
            let (tx, rx) = tokio::sync::mpsc::channel(32);
            tauri::Builder::default()
                .manage(HolistayState::new(pool, tx))
                .setup(move |app| {
                    app.configure_event_listening();
                    app.configure_event_sending(rx);
                    Ok(())
                })
                .invoke_handler(tauri::generate_handler![get_current_user])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {}", err),
    }
}

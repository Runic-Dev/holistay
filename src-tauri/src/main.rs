// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use events::login_user;
use startup::listen_to_frontend;
use tauri::Manager;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::Mutex;

pub mod events;
mod models;
pub mod responses;
pub mod startup;

use models::user::User;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::events::register_user;

mod db;

pub enum HolistayEvent {
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterAttempt),
    LoginAttempt(LoginRegisterAttempt),
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

#[derive(Deserialize, Clone)]
pub struct LoginRegisterAttempt {
    username: String,
    password: String,
}

#[derive(Serialize, Clone)]
struct LoggedInUser {
    username: String,
}

#[tokio::main]
async fn main() {
    match db::init().await {
        Ok(pool) => {
            if let Err(err) = sqlx::migrate!("./src/db/migrations/").run(&pool).await {
                panic!("Failed to run migrations: {:?}", err);
            }
            let (tx, mut rx) = tokio::sync::mpsc::channel::<HolistayEvent>(32);
            tauri::Builder::default()
                .manage(HolistayState::new(pool.clone()))
                .setup(move |app| {
                    let app_handle = app.app_handle();
                    listen_to_frontend(app, tx);
                    let mutex_pool = Mutex::new(pool);
                    tauri::async_runtime::spawn(async move {
                        while let Some(holistay_event) = rx.recv().await {
                            match holistay_event {
                                HolistayEvent::UpdateLoggedInUser(_) => todo!(),
                                HolistayEvent::Error(_) => todo!(),
                                HolistayEvent::NoLoggedInUser => todo!(),
                                HolistayEvent::RegisterAttempt(register_attempt) => {
                                    let pool_lock = mutex_pool.lock().await;
                                    let _ = match register_user(
                                        pool_lock.clone(),
                                        register_attempt.clone(),
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            println!("Sending out event!");
                                            Ok(app_handle.emit_all(
                                                "user_logged_in",
                                                LoggedInUser {
                                                    username: register_attempt.username,
                                                },
                                            ))
                                        }
                                        Err(err) => Err(println!("{:?}", err)),
                                    };
                                }
                                HolistayEvent::LoginAttempt(login_attempt) => {
                                    let pool_lock = mutex_pool.lock().await;
                                    let _ =
                                        match login_user(pool_lock.clone(), login_attempt.clone())
                                            .await
                                        {
                                            Ok(user) => Ok(app_handle.emit_all(
                                                "user_logged_in",
                                                LoggedInUser {
                                                    username: user.username,
                                                },
                                            )),
                                            Err(err) => Err(println!("{:?}", err)),
                                        };
                                }
                            }
                        }
                    });
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {}", err),
    }
}

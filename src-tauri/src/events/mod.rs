mod property_service;
mod auth_service;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle, Manager};

use crate::{
    models::LoginRegisterAttempt,
    models::{user::User, LoggedInUser, RegisteredUser},
};

use self::{auth_service::{register_user, login_user}, property_service::{get_property_partials, add_new_property}};

pub enum HolistayEvent {
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterAttempt),
    LoginAttempt(LoginRegisterAttempt),
    NewProperty(NewPropertyRequest),
    GetProperites,
    PropertyDataRequested(String)
}

#[derive(Serialize, Deserialize)]
pub struct NewPropertyRequest {
    pub name: String,
    pub image: String
}

/// # Panics
pub fn listen_to_frontend(app: &App, tx: Sender<HolistayEvent>) {
    let tx_clone = tx.clone();
    app.listen_global("register_attempt", move |event| {
        let payload = event.payload().expect("Payload expected");
        let register_attempt: LoginRegisterAttempt =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let register_event = HolistayEvent::RegisterAttempt(register_attempt);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
    let tx_clone = tx.clone();
    app.listen_global("login_attempt", move |event| {
        let payload = event.payload().expect("Argh there's no bladdy payload");
        let login_attempt: LoginRegisterAttempt =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let login_event = HolistayEvent::LoginAttempt(login_attempt);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(login_event).await;
        });
    });
    let tx_clone = tx.clone();
    app.listen_global("add_new_property", move |event| {
        let payload = event.payload().expect("Argh there's no bladdy payload");
        let new_property_request: NewPropertyRequest = serde_json::from_str(payload).expect("Couldn't parse NewPropertyRequest from payload");
        let new_property_event = HolistayEvent::NewProperty(new_property_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_property_event).await;
        });
    });
    let tx_clone = tx.clone();
    app.listen_global("get_properties", move |_event| {
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(HolistayEvent::GetProperites).await;
        });
    });
    app.listen_global("property_data", move |event| {
        let tx_clone = tx.clone();
        let property_id = event.payload().expect("No payload found with property_data event");
        let property_data_req = HolistayEvent::PropertyDataRequested(property_id.to_string());
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(property_data_req).await;
        });
    });
}

#[allow(clippy::significant_drop_tightening)]
pub fn holistay_event_handler(
    app_handle: AppHandle,
    pool: Pool<Sqlite>,
    mut rx: Receiver<HolistayEvent>,
) {
    let mutex_pool = Mutex::from(pool);
    tauri::async_runtime::spawn(async move {
        while let Some(holistay_event) = rx.recv().await {
            match holistay_event {
                HolistayEvent::UpdateLoggedInUser(_) => todo!(),
                HolistayEvent::Error(_) => todo!(),
                HolistayEvent::NoLoggedInUser => todo!(),
                HolistayEvent::RegisterAttempt(register_attempt) => {
                    let pool_lock = mutex_pool.lock().await;
                    register_user(pool_lock, register_attempt.clone())
                        .await
                        .map_or_else(
                            |err| { let _ = app_handle.emit_all( "failed_user_registration",
                                    json!({
                                        "error_message": err.to_string()
                                    }),
                                );
                            },
                            |()| {
                                let _ = app_handle.emit_all(
                                    "user_registered",
                                    RegisteredUser {
                                        username: register_attempt.username,
                                    },
                                );
                            },
                        );
                }
                HolistayEvent::LoginAttempt(login_attempt) => {
                    let pool_lock = mutex_pool.lock().await;
                    login_user(pool_lock, login_attempt).await
                        .map_or_else( 
                            |err| { println!("{err:?}") }, 
                            |user| { let _ = 
                                app_handle.emit_all("user_logged_in", LoggedInUser { username: user.username }); }) 
                }
                HolistayEvent::NewProperty(new_property_request) => {
                    let pool_lock = mutex_pool.lock().await;
                    add_new_property(pool_lock, new_property_request)
                        .await
                        .map_or_else(
                            |err| println!("{}", err.to_string()), 
                            |result| println!("Successfully entered rows to database: {}", result.rows_affected()))
                },
                HolistayEvent::GetProperites => {
                    let pool_lock = mutex_pool.lock().await;
                    get_property_partials(pool_lock).await
                        .map_or_else(
                            |err| println!("Error loading properties: {:?}", err), 
                            |property_partials| { let _ = app_handle
                                .emit_all("properties_loaded", &property_partials); });
                },
                HolistayEvent::PropertyDataRequested(_property_id) => {
                    // TODO: Get full property details
                    // let pool_lock = mutex_pool.lock().await;
                    // match sqlx::query_as::<Sqlite, Property>("SELECT p.*, rg.* FROM property p JOIN roomgroup rg ON p.id = rg.property_id WHERE p.id = ?")
                    //     .bind(property_id)
                    //     .fetch_one(&*pool_lock).await {
                    //         Ok(_) => todo!(),
                    //         Err(_) => todo!(),
                    //     }
                },
            }
        }
    });
}


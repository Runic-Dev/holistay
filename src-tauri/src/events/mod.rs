mod auth;

use base64::{encode, Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use std::fs;

use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle, Manager};
use uuid::Uuid;

use crate::{
    models::LoginRegisterAttempt,
    models::{user::User, LoggedInUser, RegisteredUser, Property},
};

use self::auth::register_user;

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
                    register_user(pool_lock.clone(), register_attempt.clone())
                        .await
                        .map_or_else(
                            |err| {
                                let _ = app_handle.emit_all(
                                    "failed_user_registration",
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
                    let _ = match {
                        let conn_pool = pool_lock.clone();
                        let login_attempt = login_attempt.clone();
                        async move {
                            sqlx::query_as::<Sqlite, User>("SELECT * FROM user INNER JOIN auth ON user.username = auth.username WHERE user.username = ? AND auth.password = ?")
                                .bind(login_attempt.username)
                                .bind(login_attempt.password)
                                .fetch_one(&conn_pool)
                                .await
                        }
                    }.await {
                        Ok(user) => Ok(app_handle.emit_all(
                            "user_logged_in",
                            LoggedInUser {
                                username: user.username,
                            },
                        )),
                        Err(err) => Err(println!("{err:?}")),
                    };
                }
                HolistayEvent::NewProperty(new_property_request) => {
                    let id = Uuid::new_v4();
                    let pool_lock = mutex_pool.lock().await;
                    let image_file = fs::read(new_property_request.image).expect("Problem reading image file");
                    let encoded = general_purpose::STANDARD.encode(image_file);
                    match sqlx::query("INSERT INTO property (id, name, image) VALUES (?, ?, ?)")
                        .bind(id.to_string())
                        .bind(new_property_request.name)
                        .bind(encoded)
                        .execute(&*pool_lock).await {
                            Ok(result) => {
                                println!("Successfully entered rows to database: {}", result.rows_affected());
                            },
                            Err(err) => {
                                println!("{}", err.to_string())
                            },
                        }
                },
                HolistayEvent::GetProperites => {
                    let pool_lock = mutex_pool.lock().await;
                    match sqlx::query_as::<Sqlite, Property>("SELECT id, name, image FROM property")
                        .fetch_all(&*pool_lock).await {
                            Ok(results) => {
                                let _ = app_handle.emit_all("properties_loaded", &results);
                            },
                            Err(err) => {
                                println!("Problem loading properties: {}", err)
                            },
                        }
                },
                HolistayEvent::PropertyDataRequested(_) => {

                },
            }
        }
    });
}


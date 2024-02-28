mod property_service;
mod auth_service;
mod requests;
mod room_group_service;

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

use self::{auth_service::{register_user, login_user}, property_service::{get_property_partials, add_new_property, get_property}, requests::{NewPropertyRequest, NewRoomGroupRequest, GetRoomGroupsRequest}, room_group_service::{add_new_room_group, get_room_groups}};

pub enum HolistayEvent {
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterAttempt),
    LoginAttempt(LoginRegisterAttempt),
    NewProperty(NewPropertyRequest),
    NewRoomGroup(NewRoomGroupRequest),
    GetProperties,
    PropertyDataRequested(String),
    GetRoomGroups(GetRoomGroupsRequest),
}


/// # Panics
pub fn listen_to_frontend(app: &App, tx: Sender<HolistayEvent>) {
    handle_register_attempt(app, tx.clone());
    handle_login_attempt(app, tx.clone());
    handle_add_new_property(app, tx.clone());
    handle_add_new_room_group(app, tx.clone());
    handle_add_property(app, tx.clone());
    handle_get_room_groups(app, tx.clone());
    handle_get_property_data(app, tx);
}

fn handle_get_property_data(app: &App, tx: Sender<HolistayEvent>) {
    app.listen_global("get_property_data", move |event| {
        dbg!("event passed with property id: {}", &event);
        let tx_clone = tx.clone();
        //todo: payloads from svelte should have their own objects. no primitive obsessions here!
        let property_id: String = serde_json::from_str(event.payload().expect("no payload found with property_data event")).expect("couldn't parse property id");
        let property_data_req = HolistayEvent::PropertyDataRequested(property_id.to_string());
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(property_data_req).await;
        });
    });
}

fn handle_get_room_groups(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("get_room_groups", move |event| {
        let payload = event.payload().expect("No payload found for get room groups request");
        let tx_clone = tx_clone.clone();
        let get_room_groups_request: GetRoomGroupsRequest = serde_json::from_str(payload).expect("Couldn't parse NewPropertyRequest from payload");
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(HolistayEvent::GetRoomGroups(get_room_groups_request)).await;
        });
    });
}

fn handle_add_property(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("get_properties", move |_event| {
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(HolistayEvent::GetProperties).await;
        });
    });
}

fn handle_add_new_room_group(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("add_new_room_group", move |event| {
        let payload = event.payload().expect("No payload found for new room group request");
        let new_room_group_request: NewRoomGroupRequest = serde_json::from_str(payload).expect("Couldn't parse NewRoomGroupRequest from payload");
        let new_room_group_event = HolistayEvent::NewRoomGroup(new_room_group_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_room_group_event).await;
        });
    });
}

fn handle_add_new_property(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("add_new_property", move |event| {
        let payload = event.payload().expect("No payload found for new property request");
        let new_property_request: NewPropertyRequest = serde_json::from_str(payload).expect("Couldn't parse NewPropertyRequest from payload");
        let new_property_event = HolistayEvent::NewProperty(new_property_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_property_event).await;
        });
    });
}

fn handle_login_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
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
}

fn handle_register_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
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
                HolistayEvent::GetProperties => {
                    let pool_lock = mutex_pool.lock().await;
                    get_property_partials(pool_lock).await
                        .map_or_else(
                            |err| println!("Error loading properties: {:?}", err), 
                            |property_partials| { let _ = app_handle
                                .emit_all("properties_loaded", &property_partials); });
                },
                HolistayEvent::PropertyDataRequested(property_id) => {
                    let pool_lock = mutex_pool.lock().await;
                    get_property(pool_lock, property_id).await
                        .map_or_else(
                            |err| println!("Error loading property: {err:?}"), 
                            |property| {
                                let _ = app_handle.emit_all("property_data", json!({
                                    "success": true,
                                    "property": &property
                                }));
                        });
                },
                HolistayEvent::NewRoomGroup(new_room_group_request) => {
                    let pool_lock = mutex_pool.lock().await;
                    add_new_room_group(pool_lock, new_room_group_request)
                        .await
                        .map_or_else(
                            |err| println!("{}", err.to_string()), 
                            |result| println!("Successfully entered rows to database: {}", result.rows_affected()))
                },
                HolistayEvent::GetRoomGroups(get_room_groups_request) => {
                    let pool_lock = mutex_pool.lock().await;
                    get_room_groups(pool_lock, get_room_groups_request)
                        .await
                        .map_or_else(|err| println!("Error loading room groups: {err:?}"), |room_groups| {
                            let _ = app_handle.emit_all("room_groups_data", &room_groups);
                        });
                },
            }
        }
    });
}


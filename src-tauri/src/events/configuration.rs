use tokio::sync::mpsc::Receiver;

use serde_json::json;
use sqlx::{Sqlite, Pool};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use crate::models::{RegisteredUser, LoggedInUser};

use super::{HolistayEvent, auth_service::{register_user, login_user}, property_service::{add_new_property, get_property_partials, get_property}, room_group_service::{add_new_room_group, get_room_groups}};

pub async fn configure_event_handler(mut rx: Receiver<HolistayEvent>, mutex_pool: Mutex<Pool<Sqlite>>, app_handle: AppHandle) {
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
}


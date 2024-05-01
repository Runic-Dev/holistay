use tokio::sync::mpsc::Receiver;

use serde_json::json;
use sqlx::{Sqlite, Pool};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use crate::event_system::events::HolistayEvent;
use crate::models::auth::{LoggedInUser, RegisteredUser};
use crate::models::domain::property::Property;

use crate::services::{auth_service, property_service, room_group_service, room_service};

pub async fn configure_event_handler(mut rx: Receiver<HolistayEvent>, mutex_pool: Mutex<Pool<Sqlite>>, app_handle: AppHandle) {
    while let Some(holistay_event) = rx.recv().await {
        match holistay_event {
            HolistayEvent::UpdateLoggedInUser(_) => todo!(),
            HolistayEvent::Error(_) => todo!(),
            HolistayEvent::NoLoggedInUser => todo!(),
            HolistayEvent::RegisterAttempt(register_attempt) => {
                let pool_lock = mutex_pool.lock().await;
                auth_service::register_user(pool_lock, register_attempt.clone())
                    .await
                    .map_or_else(
                        |err| { let _ = app_handle.emit_all( "failed_user_registration",
                                json!({
                                    "error_message": err.to_string()
                                }),
                            );
                        },
                        |id| {
                            let _ = app_handle.emit_all(
                                "user_registered",
                                RegisteredUser {
                                    id: id.to_string(),
                                    username: register_attempt.username,
                                },
                            );
                        },
                    );
            }
            HolistayEvent::LoginAttempt(login_attempt) => {
                let pool_lock = mutex_pool.lock().await;
                auth_service::login_user(pool_lock, login_attempt).await
                    .map_or_else( 
                        |err| { println!("{err:?}") }, 
                        |user| { let _ = 
                            app_handle.emit_all("user_logged_in", LoggedInUser { username: user.username }); }) 
            }
            HolistayEvent::NewProperty(new_property_request) => {
                let pool_lock = mutex_pool.lock().await;
                property_service::add_new_property(pool_lock, new_property_request)
                    .await
                    .map_or_else(
                        |err| println!("{}", err.to_string()), 
                        |result| println!("Successfully entered rows to database: {}", result.rows_affected()))
            },
            HolistayEvent::GetProperties => {
                let pool_lock = mutex_pool.lock().await;
                property_service::get_property_partials(pool_lock).await
                    .map_or_else(
                        |err| println!("Error loading properties: {:?}", err), 
                        |property_partials| { let _ = app_handle
                            .emit_all("properties_loaded", &property_partials); });
            },
            HolistayEvent::PropertyDataRequested(get_property_request) => {
                let pool_lock = mutex_pool.lock().await;
                property_service::get_property(pool_lock, get_property_request.property_id).await
                    .map_or_else(
                        |err| println!("Error loading property: {err:?}"), 
                        |property| {
                            let payload = match property {
                                Some(ref checked_property) => {
                                    json!({
                                        "success": true,
                                        "data": checked_property
                                    })
                                }
                                None => json!({ "success": false })
                            };
                            let _ = app_handle.emit_all("property_data", payload);
                    });
            }
            HolistayEvent::NewRoomGroup(new_room_group_request) => {
                let pool_lock = mutex_pool.lock().await;
                room_group_service::add_new_room_group(pool_lock, new_room_group_request)
                    .await
                    .map_or_else(
                        |err| println!("{}", err.to_string()), 
                        |result| println!("Successfully entered rows to database: {}", result.rows_affected()))
            },
            HolistayEvent::GetRoomGroups(get_room_groups_request) => {
                let pool_lock = mutex_pool.lock().await;
                room_group_service::get_room_groups(pool_lock, get_room_groups_request)
                    .await
                    .map_or_else(|err| println!("Error loading room groups: {err:?}"), |room_groups| {
                        let _ = app_handle.emit_all("room_groups_data", &room_groups);
                    });
            },
            HolistayEvent::Init => {
                let pool_lock = mutex_pool.lock().await;
                auth_service::get_default_user(pool_lock)
                    .await
                    .map_or_else(|err| println!("Error checking for default user: {err:?}"), |user_row_option| {
                        let _ = app_handle.emit_all("init_response", json!({
                            "user": &user_row_option
                        }));
                    });
            },
            HolistayEvent::NewRoomGroupDescription(new_room_group_desc_request) => {
                let pool_lock = mutex_pool.lock().await;
                room_group_service::update_description(pool_lock, new_room_group_desc_request)
                    .await
                    .map_or_else(|err| println!("Error updating room group description: {err:?}"), |(id, desc)| {
                        let event_name = format!("room_group_desc_updated_for_{}", id);
                        let _ = app_handle.emit_all(&event_name, json!({
                            "description": desc
                        }));
                    });
            },
            HolistayEvent::NewPropertyDescription(new_property_desc_request) => {
                let pool_lock = mutex_pool.lock().await;
                property_service::update_description(pool_lock, new_property_desc_request)
                    .await
                    .map_or_else(|err| println!("Error updating property description: {err:?}"), |(id, desc)| {
                        let event_name = format!("property_desc_updated_for_{}", id);
                        let _ = app_handle.emit_all(&event_name, json!({
                            "description": desc
                        }));
                    });
            },
            HolistayEvent::NewRoom(new_room_request) => {
                let pool_lock = mutex_pool.lock().await;
                room_service::add_new_room(pool_lock, new_room_request)
                    .await
                    .map_or_else(
                        |err| println!("{}", err.to_string()), 
                        |result| println!("Successfully entered rows to database: {}", result.rows_affected()))

            },
            HolistayEvent::GetRooms(get_rooms_request) => {
                let pool_lock = mutex_pool.lock().await;
                room_service::get_room_partials(pool_lock, get_rooms_request).await
                    .map_or_else(
                        |err| println!("Error loading rooms: {:?}", err), 
                        |room_partials| { let _ = app_handle
                            .emit_all("rooms_loaded", &room_partials); });
            },
        }
    }
}

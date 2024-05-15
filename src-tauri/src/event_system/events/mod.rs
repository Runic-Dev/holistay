use std::sync::Arc;
use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::event_system::config::config::configure_event_handler;
use crate::event_system::handlers;
use crate::models::requests::{GetPropertyRequest, LoginRegisterRequest, NewDescriptionRequest, AddNewPropertyRequest, NewRoomGroupRequest, NewRoomRequest};

use crate::models::user::User;

pub enum HolistayEvent {
    Init,
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterRequest),
    LoginAttempt(LoginRegisterRequest),
    NewProperty(AddNewPropertyRequest),
    NewRoomGroup(NewRoomGroupRequest),
    // GetProperties,
    PropertyDataRequested(GetPropertyRequest),
    // GetRoomGroups(GetRoomGroupsRequest),
    NewRoomGroupDescription(NewDescriptionRequest),
    NewPropertyDescription(NewDescriptionRequest),
    NewRoom(NewRoomRequest),
    // GetRooms(GetRoomsRequest),
}


/// # Panics
pub fn listen_to_frontend(app: &App, tx: Sender<HolistayEvent>) {
    handlers::handle_init(app, tx.clone());
    handlers::auth::register_attempt(app, tx.clone());
    handlers::auth::login_attempt(app, tx.clone());
    handlers::property::add_new_property(app, tx.clone());
    handlers::room_group::add_new_room_group(app, tx.clone());
    // handlers::property::get_properties(app, tx.clone());
    handlers::property::get_property_data(app, tx.clone());
    handlers::room_group::new_room_group_description(app, tx.clone());
    handlers::property::new_property_description(app, tx.clone());
    handlers::room::add_new_room(app, tx.clone());
}

#[allow(clippy::significant_drop_tightening)]
pub fn init_event_handler(
    app_handle: AppHandle,
    pool: Arc<Mutex<Pool<Sqlite>>>,
    rx: Receiver<HolistayEvent>,
) {
    tauri::async_runtime::spawn(async move {
        configure_event_handler(rx, pool, app_handle).await;
    });
}


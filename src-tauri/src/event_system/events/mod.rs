use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::event_system::config::config::configure_event_handler;
use crate::event_system::handlers;
use crate::models::requests::{GetRoomGroupsRequest, GetRoomsRequest, LoginRegisterRequest, NewDescriptionRequest, NewPropertyRequest, NewRoomGroupRequest, NewRoomRequest};

use crate::models::user::User;

pub enum HolistayEvent {
    Init,
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterRequest),
    LoginAttempt(LoginRegisterRequest),
    NewProperty(NewPropertyRequest),
    NewRoomGroup(NewRoomGroupRequest),
    GetProperties,
    PropertyDataRequested(String),
    GetRoomGroups(GetRoomGroupsRequest),
    NewRoomGroupDescription(NewDescriptionRequest),
    NewPropertyDescription(NewDescriptionRequest),
    NewRoom(NewRoomRequest),
    GetRooms(GetRoomsRequest),
}


/// # Panics
pub fn listen_to_frontend(app: &App, tx: Sender<HolistayEvent>) {
    handlers::handle_init(app, tx.clone());
    handlers::auth::handle_register_attempt(app, tx.clone());
    handlers::auth::handle_login_attempt(app, tx.clone());
    handlers::property::handle_add_new_property(app, tx.clone());
    handlers::room_group::handle_add_new_room_group(app, tx.clone());
    handlers::property::handle_add_property(app, tx.clone());
    handlers::room_group::handle_get_room_groups(app, tx.clone());
    handlers::property::handle_get_property_data(app, tx.clone());
    handlers::room_group::handle_new_room_group_desc(app, tx.clone());
    handlers::property::handle_new_property_desc(app, tx.clone());
    handlers::room::handle_add_new_room(app, tx.clone());
    handlers::room::handle_get_rooms(app, tx.clone());
}

#[allow(clippy::significant_drop_tightening)]
pub fn init_event_handler(
    app_handle: AppHandle,
    pool: Pool<Sqlite>,
    rx: Receiver<HolistayEvent>,
) {
    let mutex_pool = Mutex::from(pool);
    tauri::async_runtime::spawn(async move {
        configure_event_handler(rx, mutex_pool, app_handle).await;
    });
}


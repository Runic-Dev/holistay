mod property_service;
mod auth_service;
mod requests;
mod room_group_service;
mod event_handlers;
mod configuration;

use tokio::sync::{ mpsc::{Receiver, Sender}, Mutex };

use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle};

use crate::models::user::User;

use self::{
    requests::{
        LoginRegisterRequest,
        NewPropertyRequest, 
        NewRoomGroupRequest, 
        GetRoomGroupsRequest, NewDescriptionRequest
    }, 
    configuration::configure_event_handler
};

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
    NewPropertyDescription(NewDescriptionRequest)
}


/// # Panics
pub fn listen_to_frontend(app: &App, tx: Sender<HolistayEvent>) {
    event_handlers::handle_init(app, tx.clone());
    event_handlers::handle_register_attempt(app, tx.clone());
    event_handlers::handle_login_attempt(app, tx.clone());
    event_handlers::handle_add_new_property(app, tx.clone());
    event_handlers::handle_add_new_room_group(app, tx.clone());
    event_handlers::handle_add_property(app, tx.clone());
    event_handlers::handle_get_room_groups(app, tx.clone());
    event_handlers::handle_get_property_data(app, tx.clone());
    event_handlers::handle_new_room_group_desc(app, tx.clone());
    event_handlers::handle_new_property_desc(app, tx.clone());
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


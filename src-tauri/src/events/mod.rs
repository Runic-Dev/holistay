mod property_service;
mod auth_service;
mod requests;
mod room_group_service;
mod event_handlers;
mod configuration;

use crate::events::event_handlers::handle_register_attempt;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle};

use crate::{
    models::LoginRegisterAttempt,
    models::user::User,
};

use self::{
    requests::{
        NewPropertyRequest, 
        NewRoomGroupRequest, 
        GetRoomGroupsRequest
    }, 
    event_handlers::{
        handle_login_attempt, 
        handle_add_new_property, 
        handle_add_new_room_group, 
        handle_add_property, 
        handle_get_room_groups, 
        handle_get_property_data
    }, 
    configuration::configure_event_handler
};

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


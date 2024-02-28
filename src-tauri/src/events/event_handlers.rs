use tauri::{App, Manager};
use tokio::sync::mpsc::Sender;

use crate::models::LoginRegisterAttempt;

use super::{HolistayEvent, requests::{GetRoomGroupsRequest, NewRoomGroupRequest, NewPropertyRequest}};

pub fn handle_get_property_data(app: &App, tx: Sender<HolistayEvent>) {
    app.listen_global("get_property_data", move |event| {
        let tx_clone = tx.clone();
        let property_id: String = serde_json::from_str(event.payload().expect("no payload found with property_data event")).expect("couldn't parse property id");
        let property_data_req = HolistayEvent::PropertyDataRequested(property_id.to_string());
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(property_data_req).await;
        });
    });
}

pub fn handle_get_room_groups(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("get_room_groups", move |event| {
        let payload = event.payload().expect("No payload found for get room groups request");
        let tx_clone = tx_clone.clone();
        let get_room_groups_request: GetRoomGroupsRequest = serde_json::from_str(payload).expect("Couldn't parse NewPropertyRequest from payload");
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(HolistayEvent::GetRoomGroups(get_room_groups_request)).await;
        });
    });
}

pub fn handle_add_property(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("get_properties", move |_event| {
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(HolistayEvent::GetProperties).await;
        });
    });
}

pub fn handle_add_new_room_group(app: &App, tx_clone: Sender<HolistayEvent>) {
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

pub fn handle_add_new_property(app: &App, tx_clone: Sender<HolistayEvent>) {
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

pub fn handle_login_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
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

pub fn handle_register_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
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


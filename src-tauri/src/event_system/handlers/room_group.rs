use tauri::{App, Manager};
use tokio::sync::mpsc::Sender;
use crate::event_system::events::HolistayEvent;
use crate::models::requests::{GetRoomGroupsRequest, NewDescriptionRequest, NewRoomGroupRequest};

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

pub fn handle_new_room_group_desc(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("new_room_group_description", move |event| {
        let payload = event.payload().expect("Payload expected");
        let new_room_group_description_request: NewDescriptionRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let register_event = HolistayEvent::NewRoomGroupDescription(new_room_group_description_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
}



use tauri::{App, Manager};
use tokio::sync::mpsc::Sender;
use crate::event_system::events::HolistayEvent;
use crate::models::requests::{GetRoomsRequest, NewRoomRequest};

pub fn add_new_room(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("add_new_room", move |event| {
        let payload = event.payload().expect("Payload expected");
        let new_room_request: NewRoomRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let new_room_event = HolistayEvent::NewRoom(new_room_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_room_event).await;
        });
    });
}

/*pub fn get_rooms(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("get_rooms", move |event| {
        let payload = event.payload().expect("Payload expected");
        let get_rooms_request: GetRoomsRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let new_room_event = HolistayEvent::GetRooms(get_rooms_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_room_event).await;
        });
    });
}
*/
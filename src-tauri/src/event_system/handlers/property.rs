use tauri::{App, command, Manager};
use tokio::sync::mpsc::Sender;
use crate::event_system::events::HolistayEvent;
use crate::models::requests::{GetPropertyRequest, NewDescriptionRequest, AddNewPropertyRequest};

pub fn get_property_data(app: &App, tx: Sender<HolistayEvent>) {
    app.listen_global("get_property_data", move |event| {
        let tx_clone = tx.clone();
        match event.payload() {
            Some(property_id) => {
                let property_id_request: GetPropertyRequest = serde_json::from_str(property_id).expect("Invalid request type");
                let property_data_req = HolistayEvent::PropertyDataRequested(property_id_request);
                tauri::async_runtime::spawn(async move {
                    let _ = tx_clone.send(property_data_req).await;
                });
            }
            None => {
                todo!("I need to handle this case")
            }
        }
    });
}

pub fn add_new_property(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("add_new_property", move |event| {
        let payload = event.payload().expect("No payload found for new property request");
        let new_property_request: AddNewPropertyRequest = serde_json::from_str(payload).expect("Couldn't parse NewPropertyRequest from payload");
        let new_property_event = HolistayEvent::NewProperty(new_property_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(new_property_event).await;
        });
    });
}
pub fn new_property_description(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("new_property_description", move |event| {
        let payload = event.payload().expect("Payload expected");
        let new_property_description_request: NewDescriptionRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let register_event = HolistayEvent::NewPropertyDescription(new_property_description_request);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
}


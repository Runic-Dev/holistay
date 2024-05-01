use tauri::{App, Manager};
use tokio::sync::mpsc::Sender;
use crate::event_system::events::HolistayEvent;
use crate::models::requests::LoginRegisterRequest;

pub fn login_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("login_attempt", move |event| {
        let payload = event.payload().expect("Argh there's no bladdy payload");
        let login_attempt: LoginRegisterRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let login_event = HolistayEvent::LoginAttempt(login_attempt);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(login_event).await;
        });
    });
}

pub fn register_attempt(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("register_attempt", move |event| {
        let payload = event.payload().expect("Payload expected");
        let register_attempt: LoginRegisterRequest =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let register_event = HolistayEvent::RegisterAttempt(register_attempt);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
}

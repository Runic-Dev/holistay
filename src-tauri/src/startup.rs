use tokio::sync::mpsc::Sender;

use tauri::{App, Manager};

use crate::{LoginRegisterAttempt, HolistayEvent};

pub fn listen_to_frontend(app: &mut App, tx: Sender<HolistayEvent>) {
    let tx_clone = tx.clone();
    app.listen_global("register_attempt", move |event| {
        let payload = event.payload().expect("Argh there's no bladdy payload");
        let register_attempt: LoginRegisterAttempt =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let register_event = HolistayEvent::RegisterAttempt(register_attempt);
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
    app.listen_global("login_attempt", move |event| {
        let payload = event.payload().expect("Argh there's no bladdy payload");
        let login_attempt: LoginRegisterAttempt =
            serde_json::from_str(payload).expect("Couldn't parse struct from payload");
        let login_event = HolistayEvent::LoginAttempt(login_attempt);
        let tx_clone = tx.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(login_event).await;
        });
    });
}

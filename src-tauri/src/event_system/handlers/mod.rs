use tauri::{App, Manager};
use tokio::sync::mpsc::Sender;
use crate::event_system::events::HolistayEvent;

pub(crate) mod property;
pub(crate) mod room_group;
pub(crate) mod room;
pub(crate) mod auth;

pub fn handle_init(app: &App, tx_clone: Sender<HolistayEvent>) {
    app.listen_global("init", move |_| {
        let register_event = HolistayEvent::Init;
        let tx_clone = tx_clone.clone();
        tauri::async_runtime::spawn(async move {
            let _ = tx_clone.send(register_event).await;
        });
    });
}

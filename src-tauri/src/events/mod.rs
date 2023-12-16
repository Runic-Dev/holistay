use tokio::sync::mpsc::Receiver;

use tauri::{App, Event, Manager};
use uuid::Uuid;

use crate::{models::user::User, HolistayEvent, HolistayState, RegisterAttempt};

pub trait CanConfigureEvents {
    fn configure_event_listening(self);
    fn configure_event_sending(self, rx: Receiver<HolistayEvent>);
}

impl CanConfigureEvents for &'static mut App {
    fn configure_event_listening(self) {
        register_attempt(self);
    }

    fn configure_event_sending(self, mut rx: Receiver<HolistayEvent>) {
        if let Some(main_window) = self.get_window("main") {
            tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        HolistayEvent::UpdateLoggedInUser(user) => {
                            main_window.emit("update_logged_in_user", user);
                        }
                        HolistayEvent::Error(_) => todo!(),
                        HolistayEvent::NoLoggedInUser => todo!(),
                        HolistayEvent::RegisterAttempt(_) => todo!(),
                    }
                }
            });
        }
    }
}

async fn register_attempt(app: &'static mut App) {
    let state = app.state::<HolistayState>();
    if let Some(main_window) = app.get_window("main") {
        main_window.listen("register_attempt", move |event| {
            state
                .event_channel
                .send(HolistayEvent::RegisterAttempt(event));
        });
    };
}

async fn handle_register_attempt(event: Event, state: tauri::State<'_, HolistayState>) {
    match event
        .payload()
        .map_or(Err("No payload found"), |payload_str| {
            serde_json::from_str(payload_str).map_or(
                Err("Could not parse register attempt from string"),
                |register_attempt: RegisterAttempt| Ok(register_attempt),
            )
        }) {
        Ok(register_attempt) => register_user(state, register_attempt).await,
        Err(err_message) => {
            state
                .event_channel
                .send(HolistayEvent::Error(err_message.to_string()));
        }
    }
}

async fn register_user(
    holistay_state: tauri::State<'_, HolistayState>,
    register_attempt: RegisterAttempt,
) {
    let conn_lock = holistay_state.conn_pool.lock().await;
    let event_channel = holistay_state.event_channel.clone();
    let conn_clone = conn_lock.clone();
    tauri::async_runtime::spawn(async move {
        if let Ok(tran) = conn_clone.begin().await {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO user (?,?)")
                .bind(id.clone().to_string())
                .bind(register_attempt.username.clone())
                .execute(&conn_clone)
                .await;

            sqlx::query("INSERT INTO auth (?,?,?)")
                .bind(Uuid::new_v4().to_string())
                .bind(register_attempt.username.clone())
                .bind(register_attempt.password)
                .execute(&conn_clone)
                .await;

            match tran.commit().await {
                Ok(_) => event_channel.send(HolistayEvent::UpdateLoggedInUser(User::new(
                    id,
                    register_attempt.username,
                ))),
                Err(_) => todo!(),
            };
        } else {
            event_channel.send(HolistayEvent::Error(
                "Failed to create transaction for registering user".to_string(),
            ));
        }
    });
}

mod auth;

use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

use sqlx::{Pool, Sqlite};

use tauri::{App, AppHandle, Manager};

use crate::{
    models::LoginRegisterAttempt,
    models::{user::User, LoggedInUser},
};

use self::auth::{login_user, register_user};

pub enum HolistayEvent {
    UpdateLoggedInUser(User),
    Error(String),
    NoLoggedInUser,
    RegisterAttempt(LoginRegisterAttempt),
    LoginAttempt(LoginRegisterAttempt),
}

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

pub fn holistay_event_handler(
    app_handle: AppHandle,
    pool: Pool<Sqlite>,
    mut rx: Receiver<HolistayEvent>,
) {
    let mutex_pool = Mutex::from(pool);
    tauri::async_runtime::spawn(async move {
        while let Some(holistay_event) = rx.recv().await {
            match holistay_event {
                HolistayEvent::UpdateLoggedInUser(_) => todo!(),
                HolistayEvent::Error(_) => todo!(),
                HolistayEvent::NoLoggedInUser => todo!(),
                HolistayEvent::RegisterAttempt(register_attempt) => {
                    let pool_lock = mutex_pool.lock().await;
                    let _ = match register_user(pool_lock.clone(), register_attempt.clone()).await {
                        Ok(_) => {
                            println!("Sending out event!");
                            Ok(app_handle.emit_all(
                                "user_logged_in",
                                LoggedInUser {
                                    username: register_attempt.username,
                                },
                            ))
                        }
                        Err(err) => Err(println!("{:?}", err)),
                    };
                }
                HolistayEvent::LoginAttempt(login_attempt) => {
                    let pool_lock = mutex_pool.lock().await;
                    let _ = match login_user(pool_lock.clone(), login_attempt.clone()).await {
                        Ok(user) => Ok(app_handle.emit_all(
                            "user_logged_in",
                            LoggedInUser {
                                username: user.username,
                            },
                        )),
                        Err(err) => Err(println!("{:?}", err)),
                    };
                }
            }
        }
    });
}

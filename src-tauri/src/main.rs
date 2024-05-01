// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use tauri::Manager;
use crate::event_system::events::{HolistayEvent, init_event_handler, listen_to_frontend};

pub mod event_system;
mod models;

mod db;
pub mod utils;
pub mod errors;
pub mod services;

#[tokio::main]
async fn main() {
    env_logger::init();
    match db::init().await {
        Ok(pool) => {
            if let Err(err) = sqlx::migrate!("./src/db/migrations/").run(&pool).await {
                panic!("Failed to run migrations: {err:?}");
            }
            let (tx, rx) = tokio::sync::mpsc::channel::<HolistayEvent>(32);
            tauri::Builder::default()
                .setup(move |app| {
                    listen_to_frontend(app, tx);
                    let app_handle = app.app_handle();
                    init_event_handler(app_handle, pool, rx);
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {err}"),
    }
}

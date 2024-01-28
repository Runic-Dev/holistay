// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use events::{HolistayEvent, listen_to_frontend, holistay_event_handler};
use tauri::Manager;

pub mod events;
mod models;
pub mod responses;

mod db;
pub mod utils;

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
                    holistay_event_handler(app_handle, pool, rx);
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {err}"),
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::command_handler::CommandInitializer;
use crate::event_system::events::{HolistayEvent, init_event_handler, listen_to_frontend};
use crate::repositories::property_repository::PropertyRepository;
use crate::services::property_service::PropertyService;

pub mod event_system;
mod models;

mod db;
pub mod utils;
pub mod errors;
pub mod services;
mod command_handler;
mod repositories;

pub struct AppState {
    property_service: PropertyService<PropertyRepository>
}
#[tokio::main]
async fn main() {
    env_logger::init();
    match db::init().await {
        Ok(pool) => {
            if let Err(err) = sqlx::migrate!("./src/db/migrations/").run(&pool).await {
                panic!("Failed to run migrations: {err:?}");
            }
            let (tx, rx) = tokio::sync::mpsc::channel::<HolistayEvent>(32);
            let wrapped_pool = Arc::new(Mutex::from(pool));
            let property_repository = PropertyRepository::new(wrapped_pool.clone());
            let property_service = PropertyService::new(property_repository);

            tauri::Builder::default()
                .manage(AppState { property_service })
                .configure_commands()
                .setup(move |app| {
                    listen_to_frontend(app, tx);
                    let app_handle = app.app_handle();
                    init_event_handler(app_handle, wrapped_pool, rx);
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => panic!("Unable to initialize application: {err}"),
    }
}

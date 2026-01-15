use std::sync::{Arc, Mutex};

use domain::combat_context::CombatContext;
use domain::gamedb::GameDB;
use domain::project_state::ProjectState;
use domain::setting::AppSetting;
use log::LevelFilter;
use services::app_service::AppService;
use state::AppState;
use tauri_plugin_log::{Target, TargetKind};

pub mod domain;
pub mod infra;
pub mod interface;
pub mod services;
pub mod state;
pub mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppService::new())
        .manage(AppState {
            setting: Mutex::new(AppSetting::new()),
            preview_result: Mutex::new(None),
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level(if cfg!(debug_assertions) {
                    LevelFilter::Debug
                } else {
                    LevelFilter::Info
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

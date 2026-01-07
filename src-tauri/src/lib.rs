use std::sync::Mutex;

use domain::combat_context::CombatContext;
use domain::gamedb::GameDB;
use domain::project_state::ProjectState;
use domain::setting::AppSetting;
use state::AppState;

pub mod services;
pub mod command;
pub mod domain;
pub mod state;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState{
            project: Mutex::new(ProjectState::new()),
            runtime: Mutex::new(CombatContext::new()),
            game_db: GameDB::link(),
            setting: AppSetting::new(),
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

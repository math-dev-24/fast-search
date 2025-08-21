mod application;
mod commands;
mod domain;
mod infrastructure;
mod shared;

use commands::*;
use crate::application::factories::service_factory::get_service_repository;
use crate::infrastructure::watcher::init_watcher::start_file_watcher_on_startup;
use crate::shared::config::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let service_repository = get_service_repository().expect("Failed to initialize service repository");

    service_repository.init().expect("Failed to initialize database");

    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    
    builder
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
        // System
        system_commands::get_stat,
        system_commands::diagnose_scan_issues,

        // Indexing
        indexing_commands::sync_files_and_folders,
        indexing_commands::start_content_indexing,

        // Watcher
        watch_commands::start_file_watcher,
        watch_commands::stop_file_watcher,
        watch_commands::restart_file_watcher,
        watch_commands::get_file_watcher_status,

        // File
        file_commands::save_paths,
        file_commands::get_all_types,
        file_commands::search_files,
        file_commands::reset_data,
        file_commands::open_file,
        file_commands::get_all_folders,
        file_commands::get_all_paths,

        //AI
        ai_commands::ai_search,
        ai_commands::ai_health_check,
        ai_commands::ai_list_models
    ])
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
        start_file_watcher_on_startup(app, window);
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

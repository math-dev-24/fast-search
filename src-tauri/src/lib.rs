mod application;
mod commands;
mod domain;
mod infrastructure;
mod shared;

use commands::*;
use crate::application::factories::service_factory::get_service_repository;
use crate::infrastructure::filesystem::global_watcher_manager::initialize_global_watcher_from_db;
use tauri::Manager;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let service_repository = get_service_repository().expect("Failed to initialize service repository");
    service_repository.init().expect("Failed to initialize database");

    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    
    builder.invoke_handler(tauri::generate_handler![
        // System
        system_commands::get_stat,
        system_commands::diagnose_scan_issues,

        // Indexing
        indexing_commands::sync_files_and_folders,
        indexing_commands::start_content_indexing,

        // File
        file_commands::save_paths,
        file_commands::get_all_types,
        file_commands::search_files,
        file_commands::reset_data,
        file_commands::open_file,
        file_commands::get_all_folders,
        file_commands::get_all_paths,

        // File Watcher
        watcher_commands::start_file_watcher,
        watcher_commands::stop_file_watcher,
        watcher_commands::is_file_watcher_active,
        watcher_commands::get_watched_paths,
        watcher_commands::get_active_watchers,
        watcher_commands::stop_all_watchers,
        
        // Global File Watcher
        watcher_commands::get_default_watcher_status,
        watcher_commands::restart_default_watcher,
        watcher_commands::stop_default_watcher,

        //AI
        ai_commands::ai_search,
        ai_commands::ai_health_check,
        ai_commands::ai_list_models
    ])
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
        // Async initialization - non-blocking
        initialize_global_watcher_from_db(window);
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

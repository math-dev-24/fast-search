mod application;
mod commands;
mod domain;
mod infrastructure;
mod shared;

use commands::*;
use crate::infrastructure::watcher::init_watcher::start_file_watcher_on_startup;
use crate::shared::config::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialiser le logging structuré
    let env_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "fast_search=info,tauri=warn".to_string());
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(true)
        .init();

    tracing::info!("🚀 Starting Fast Search application");

    let app_state = match AppState::new() {
        Ok(state) => {
            tracing::info!("✅ Application state initialized successfully");
            state
        },
        Err(e) => {
            tracing::error!("❌ Failed to initialize application state: {}", e);
            std::process::exit(1);
        }
    };

    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    
    builder
        .manage(app_state)
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
        ai_commands::ai_list_models,
        ai_commands::ai_save_api_key,
        ai_commands::ai_has_api_key,
        ai_commands::ai_delete_api_key
    ])
    .setup(|app| {
        if let Some(window) = app.get_webview_window("main") {
            tracing::info!("📁 Starting file watcher on startup");
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                start_file_watcher_on_startup(&app_handle, window);
            });
        } else {
            tracing::warn!("⚠️ Main window not found during setup");
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
        tracing::error!("❌ Error while running tauri application: {}", e);
        std::process::exit(1);
    });
}

use crate::application::factories::service_factory::get_service_repository;
use crate::shared::config::AppState;
use tauri::Manager;

pub fn start_file_watcher_on_startup(app: &tauri::App, window: tauri::WebviewWindow) {
    match get_service_repository() {
        Ok(service_repo) => {
            match service_repo.get_all_paths() {
                Ok(paths) => {
                    if !paths.is_empty() {
                        println!("Starting file watcher on startup with {} paths", paths.len());

                        let app_state = app.state::<AppState>();
                        let watcher_manager = app_state.file_watcher_manager.clone();

                        tauri::async_runtime::spawn(async move {
                            match watcher_manager.start_watching(window.clone(), paths.clone()) {
                                Ok(()) => {
                                    println!("✅ File watcher started successfully on startup for {} paths", paths.len());
                                },
                                Err(e) => {
                                    eprintln!("❌ Failed to start file watcher on startup: {}", e);
                                    use crate::application::events::emitters::{emit_error_event, EVENT_WATCHER_ERROR};
                                    emit_error_event(&window, EVENT_WATCHER_ERROR, format!("Auto-start failed: {}", e));
                                }
                            }
                        });
                    } else {
                        println!("ℹ️ No paths configured, file watcher not started");
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to get paths for file watcher: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to get service repository for file watcher: {}", e);
        }
    }
}
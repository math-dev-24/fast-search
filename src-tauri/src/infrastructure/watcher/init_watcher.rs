use crate::shared::config::AppState;
use tauri::Manager;

pub fn start_file_watcher_on_startup(app: &tauri::AppHandle, window: tauri::WebviewWindow) {
    let app_state = app.state::<AppState>();
    let paths = {
        match app_state.service_repository.lock() {
            Ok(repo) => match repo.get_all_paths() {
                Ok(paths) => paths,
                Err(e) => {
                    tracing::error!("Failed to get paths for file watcher: {}", e);
                    return;
                }
            },
            Err(e) => {
                tracing::error!("Failed to lock repository for watcher startup: {}", e);
                return;
            }
        }
    };

    if !paths.is_empty() {
        tracing::info!("Starting file watcher on startup with {} paths", paths.len());
        let watcher_manager = app_state.file_watcher_manager.clone();
        tauri::async_runtime::spawn(async move {
            match watcher_manager.start_watching(window.clone(), paths.clone()) {
                Ok(()) => {
                    tracing::info!("File watcher started successfully on startup for {} paths", paths.len());
                },
                Err(e) => {
                    tracing::error!("Failed to start file watcher on startup: {}", e);
                    use crate::application::events::emitters::{emit_error_event, EVENT_WATCHER_ERROR};
                    emit_error_event(&window, EVENT_WATCHER_ERROR, format!("Auto-start failed: {}", e));
                }
            }
        });
    } else {
        tracing::info!("No paths configured, file watcher not started");
    }
}
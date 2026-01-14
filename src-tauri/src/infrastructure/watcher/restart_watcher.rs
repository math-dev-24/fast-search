use crate::shared::config::AppState;

pub fn restart_file_watcher_with_new_paths_only(
    window: tauri::WebviewWindow,
    new_paths: Vec<String>,
    state: &tauri::State<'_, AppState>
) {
    if !new_paths.is_empty() {
        tracing::info!("Restarting file watcher with {} paths", new_paths.len());

        let watcher_manager = state.file_watcher_manager.clone();
        let window_clone = window.clone();

        tauri::async_runtime::spawn(async move {
            match watcher_manager.restart_watching(window_clone.clone(), new_paths.clone()) {
                Ok(()) => {
                    tracing::info!("File watcher restarted with {} paths", new_paths.len());
                    use crate::application::events::emitters::{emit_event, EVENT_WATCHER_STARTED};
                    emit_event(&window_clone, EVENT_WATCHER_STARTED, serde_json::json!({
                        "message": format!("Watcher restarted with {} paths", new_paths.len()),
                        "path_count": new_paths.len(),
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }));
                },
                Err(e) => {
                    tracing::error!("Failed to restart file watcher with paths: {}", e);
                    use crate::application::events::emitters::{emit_error_event, EVENT_WATCHER_ERROR};
                    emit_error_event(&window_clone, EVENT_WATCHER_ERROR,
                                     format!("Failed to restart watcher: {}", e));
                }
            }
        });
    } else {
        tracing::warn!("No paths provided to restart file watcher");
    }
}
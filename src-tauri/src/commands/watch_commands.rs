use tauri::{command, WebviewWindow};
use crate::AppState;

#[command]
pub async fn start_file_watcher(
    window: WebviewWindow,
    paths: Vec<String>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    println!("Starting file watcher for paths: {:?}", paths);

    if paths.is_empty() {
        return Err("Aucun chemin spécifié".to_string());
    }

    match state.file_watcher_manager.start_watching(window, paths.clone()) {
        Ok(()) => {
            Ok(format!("File watcher started for {} paths", paths.len()))
        },
        Err(e) => {
            Err(format!("Failed to start file watcher: {}", e))
        }
    }
}

#[command]
pub async fn stop_file_watcher(
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    println!("Stopping file watcher");

    match state.file_watcher_manager.stop_watching() {
        Ok(()) => {
            Ok("File watcher stopped".to_string())
        },
        Err(e) => {
            Err(format!("Failed to stop file watcher: {}", e))
        }
    }
}

#[command]
pub async fn restart_file_watcher(
    window: WebviewWindow,
    new_paths: Vec<String>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    println!("Restarting file watcher with new paths: {:?}", new_paths);

    if new_paths.is_empty() {
        return Err("Aucun chemin spécifié".to_string());
    }

    match state.file_watcher_manager.restart_watching(window, new_paths.clone()) {
        Ok(()) => {
            Ok(format!("File watcher restarted for {} paths", new_paths.len()))
        },
        Err(e) => {
            Err(format!("Failed to restart file watcher: {}", e))
        }
    }
}

#[command]
pub async fn get_file_watcher_status(
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    Ok(state.file_watcher_manager.get_status())
}

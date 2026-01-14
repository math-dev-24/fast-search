use crate::domain::entities::file::File;
use crate::domain::entities::search::SearchQuery;
use crate::infrastructure::filesystem::open_file::open_file_in_explorer;
use crate::infrastructure::filesystem::scanner::scan_files_async;
use crate::infrastructure::watcher::restart_watcher::restart_file_watcher_with_new_paths_only;
use crate::shared::config::AppState;
use crate::shared::helpers::{with_service_repository, with_service_repository_readonly};

#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    open_file_in_explorer(path)?;
    Ok(())
}

#[tauri::command]
pub fn search_files(
    query: SearchQuery,
    state: tauri::State<'_, AppState>
) -> Result<Vec<File>, String> {
    with_service_repository_readonly(&state, |repo| repo.search(&query))
}

#[tauri::command]
pub fn get_all_types(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    with_service_repository_readonly(&state, |repo| repo.get_all_types())
}

#[tauri::command]
pub fn get_all_folders(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    with_service_repository_readonly(&state, |repo| repo.get_all_folders())
}

#[tauri::command]
pub fn get_all_paths(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    with_service_repository_readonly(&state, |repo| repo.get_all_paths())
}

#[tauri::command]
pub fn reset_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_service_repository_readonly(&state, |repo| repo.reset_data())
}

#[tauri::command]
pub fn save_paths(
    paths: Vec<String>,
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let new_paths = with_service_repository(&state, |repo| {
        repo.insert_paths(paths.clone())
    })?;

    let mut results = Vec::new();

    if !new_paths.is_empty() {
        let service_repository = state.service_repository.clone();
        scan_files_async(window.clone(), new_paths.clone(), service_repository);
        tracing::info!("Started scan for {} new paths", new_paths.len());
        results.push(format!("Started scan for {} new paths", new_paths.len()));
    }

    let all_paths = with_service_repository_readonly(&state, |repo| {
        repo.get_all_paths()
    })?;

    let number_of_paths = all_paths.len();

    if !all_paths.is_empty() {
        restart_file_watcher_with_new_paths_only(window, all_paths, &state);
        tracing::info!("File watcher restarted with {} total paths", number_of_paths);
        results.push(format!("File watcher restarted with {} total paths", number_of_paths));
    } else {
        let watcher_manager = state.file_watcher_manager.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = watcher_manager.stop_watching() {
                tracing::error!("Failed to stop file watcher: {}", e);
            }
        });

        results.push("No paths configured, file watcher stopped".to_string());
    }
    Ok(results.join("; "))
}
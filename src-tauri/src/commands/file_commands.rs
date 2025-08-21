use crate::domain::entities::file::File;
use crate::domain::entities::search::SearchQuery;
use crate::infrastructure::filesystem::open_file::open_file_in_explorer;
use crate::application::factories::service_factory::get_service_repository;
use crate::infrastructure::filesystem::scanner::scan_files_async;
use crate::infrastructure::watcher::restart_watcher::restart_file_watcher_with_new_paths_only;
use crate::shared::config::AppState;

#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    open_file_in_explorer(path)?;
    Ok(())
}

#[tauri::command]
pub fn search_files( query: SearchQuery) -> Result<Vec<File>, String> {
    let service_repository = get_service_repository()?;
    let files = service_repository.search(&query)?;
    Ok(files)
}


#[tauri::command]
pub fn get_all_types() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let type_files = service_repository.get_all_types()?;
    Ok(type_files)
}

#[tauri::command]
pub fn get_all_folders() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let folders = service_repository.get_all_folders()?;
    Ok(folders)
}

#[tauri::command]
pub fn get_all_paths() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let paths = service_repository.get_all_paths()?;
    Ok(paths)
}

#[tauri::command]
pub fn reset_data() -> Result<(), String> {
    let service_repository = get_service_repository()?;
    service_repository.reset_data()?;
    Ok(())
}

#[tauri::command]
pub fn save_paths(
    paths: Vec<String>,
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let mut service_repository = get_service_repository()?;

    let new_paths = service_repository
        .insert_paths(paths.clone())
        .map_err(|e| format!("Failed to insert paths: {}", e))?;

    let mut results = Vec::new();

    if !new_paths.is_empty() {
        scan_files_async(window.clone(), new_paths.clone());
        println!("Started scan for {} new paths", new_paths.len());
        results.push(format!("Started scan for {} new paths", new_paths.len()));
    }

    let all_paths = service_repository.get_all_paths()
        .map_err(|e| format!("Failed to get all paths: {}", e))?;

    let number_of_paths = all_paths.len();

    if !all_paths.is_empty() {
        restart_file_watcher_with_new_paths_only(window, all_paths, &state);
        println!("File watcher restarted with {} total paths", number_of_paths);
        results.push(format!("File watcher restarted with {} total paths", number_of_paths));
    } else {
        let watcher_manager = state.file_watcher_manager.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = watcher_manager.stop_watching() {
                eprintln!("❌ Failed to stop file watcher: {}", e);
            }
        });

        results.push("No paths configured, file watcher stopped".to_string());
    }
    Ok(results.join("; "))
}
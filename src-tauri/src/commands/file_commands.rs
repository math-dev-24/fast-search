use crate::domain::entities::file::File;
use crate::domain::entities::search::SearchQuery;
use crate::infrastructure::filesystem::open_file::open_file_in_explorer;
use crate::application::factories::service_factory::get_service_repository;
use crate::infrastructure::filesystem::scanner::scan_files_async;

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
pub fn save_paths(paths: Vec<String>, window: tauri::WebviewWindow) -> Result<(), String> {
    let mut service_repository = get_service_repository()?;
    let new_paths = service_repository.insert_paths(paths).expect("Failed to insert paths");
    if !new_paths.is_empty() {
        scan_files_async(window, new_paths);
    }
    Ok(())
}
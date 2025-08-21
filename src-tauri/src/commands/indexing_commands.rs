use crate::application::factories::service_factory::get_service_repository;
use crate::application::use_cases::index_content::index_content_async;
use crate::infrastructure::filesystem::scanner::scan_files_async;

#[tauri::command]
pub fn sync_files_and_folders(window: tauri::WebviewWindow) -> Result<(), String> {
    let service_repository = get_service_repository()?;
    let paths = service_repository.get_all_paths()?;
    scan_files_async(window, paths);
    Ok(())
}

#[tauri::command]
pub fn start_content_indexing(window: tauri::WebviewWindow) -> Result<(), String> {
    index_content_async(window);
    Ok(())
}
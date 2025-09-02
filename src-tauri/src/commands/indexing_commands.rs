use crate::application::factories::service_factory::get_service_repository;
use crate::application::use_cases::index_content::index_content_async;
use crate::infrastructure::filesystem::scanner::scan_files_async;

#[tauri::command]
pub fn sync_files_and_folders(window: tauri::WebviewWindow) -> Result<(), String> {
    let paths = get_service_repository()?.get_all_paths()?;
    
    if paths.is_empty() {
        return Err("Aucun chemin configuré pour l'indexation".to_string());
    }

    let valid_paths: Vec<String> = paths.into_iter()
        .filter(|path| std::path::Path::new(path).exists())
        .collect();
        
    if valid_paths.is_empty() {
        return Err("Aucun chemin valide trouvé pour l'indexation".to_string());
    }
    
    println!("[INFO] Démarrage de la synchronisation pour {} chemins", valid_paths.len());

    scan_files_async(window, valid_paths);

    Ok(())
}

#[tauri::command]
pub fn start_content_indexing(window: tauri::WebviewWindow) -> Result<(), String> {
    index_content_async(window);
    Ok(())
}
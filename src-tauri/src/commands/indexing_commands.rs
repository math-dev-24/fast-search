use crate::application::use_cases::index_content::index_content_async;
use crate::infrastructure::filesystem::scanner::scan_files_async;
use crate::shared::config::AppState;
use crate::shared::helpers::with_service_repository_readonly;

#[tauri::command]
pub fn sync_files_and_folders(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let paths = with_service_repository_readonly(&state, |repo| repo.get_all_paths())?;
    
    if paths.is_empty() {
        return Err("Aucun chemin configuré pour l'indexation".to_string());
    }

    let valid_paths: Vec<String> = paths.into_iter()
        .filter(|path| std::path::Path::new(path).exists())
        .collect();
        
    if valid_paths.is_empty() {
        return Err("Aucun chemin valide trouvé pour l'indexation".to_string());
    }
    
    tracing::info!("Démarrage de la synchronisation pour {} chemins", valid_paths.len());

    let service_repository = state.service_repository.clone();
    scan_files_async(window, valid_paths, service_repository);

    Ok(())
}

#[tauri::command]
pub fn start_content_indexing(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    // Utiliser le repository singleton depuis AppState
    let service_repository = state.service_repository.clone();
    index_content_async(window, service_repository);
    Ok(())
}
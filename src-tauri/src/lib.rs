mod adapters;
mod services;
mod entities;
mod utils;
mod ports;

use std::sync::Arc;

use adapters::ai::lm_studio::LmStudio;
use services::ai_service::AiService;
use utils::{file::open_file_in_explorer, generator::get_service_repository, scan::scan_files_async, indexer::index_content_async, ai::{get_available_models, check_model_available}};
use entities::{file::File, stat::Stat, search::SearchQuery};

const LOCAL_URL_AI: &str = "http://192.168.108.172:1234";

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    open_file_in_explorer(path)?;
    Ok(())
}

#[tauri::command]
fn get_stat() -> Result<Stat, String> {
    let service_repository = get_service_repository()?;
    let stat = service_repository.get_stat()?;
    Ok(stat)
}

#[tauri::command]
fn get_all_types() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let type_files = service_repository.get_all_types()?;
    Ok(type_files)
}

#[tauri::command]
fn get_all_folders() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let folders = service_repository.get_all_folders()?;
    Ok(folders)
}

#[tauri::command]   
fn get_all_paths() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let paths = service_repository.get_all_paths()?;
    Ok(paths)
}

#[tauri::command]
fn sync_files_and_folders(window: tauri::WebviewWindow) -> Result<(), String> {
    let service_repository = get_service_repository()?;
    let paths = service_repository.get_all_paths()?;
    scan_files_async(window, paths);
    Ok(())
}

#[tauri::command]
fn reset_data() -> Result<(), String> {
    let service_repository = get_service_repository()?;
    service_repository.reset_data()?;
    Ok(())
}

#[tauri::command]
fn save_paths(paths: Vec<String>, window: tauri::WebviewWindow) -> Result<(), String> {
    let mut service_repository = get_service_repository()?;
    let new_paths = service_repository.insert_paths(paths).expect("Failed to insert paths");
    if !new_paths.is_empty() {
        scan_files_async(window, new_paths);
    }
    Ok(())
}

#[tauri::command]
async fn ai_search(natural_query: String, model: String) -> Result<SearchQuery, String> {
    let ai_adapter = LmStudio::new(Some(LOCAL_URL_AI.to_string()), Some(model.clone()));
    let ai_service = AiService::new(Arc::new(ai_adapter));

    let available_models = get_available_models(&ai_service).await?;

    if !check_model_available(&model, &available_models) {
        return Err(format!("Model {} not available", model));
    }

    let search_query = ai_service.generate(&natural_query).await
        .map_err(|e| format!("AI generation failed: {}", e))?;
    Ok(search_query)
}

#[tauri::command]
async fn ai_health_check(model: String) -> Result<bool, String> {
    let ai_adapter = LmStudio::new(Some(LOCAL_URL_AI.to_string()), Some(model));
    let ai_service = AiService::new(Arc::new(ai_adapter));
    let health_check = ai_service.health_check().await
        .map_err(|e| format!("Health check failed: {}", e))?;
    Ok(health_check)
}

#[tauri::command]
async fn ai_list_models() -> Result<Vec<String>, String> {
    let ai_adapter = LmStudio::new(Some(LOCAL_URL_AI.to_string()), None);
    let ai_service = AiService::new(Arc::new(ai_adapter));
    let models = ai_service.list_models().await
        .map_err(|e| format!("Failed to list models: {}", e))?;
    Ok(models)
}

#[tauri::command]
fn search_files( query: SearchQuery) -> Result<Vec<File>, String> {
    let service_repository = get_service_repository()?;
    let files = service_repository.search(&query)?;
    Ok(files)
}

#[tauri::command]
fn start_content_indexing(window: tauri::WebviewWindow) -> Result<(), String> {
    index_content_async(window);
    Ok(())
}

#[tauri::command]
fn diagnose_scan_issues(paths: Vec<String>) -> Result<Vec<String>, String> {
    let mut issues = Vec::new();
    
    for path in paths {
        let path_obj = std::path::Path::new(&path);
        
        if !path_obj.exists() {
            issues.push(format!("Chemin inexistant: {}", path));
            continue;
        }
        
        if !path_obj.is_dir() {
            issues.push(format!("Le chemin n'est pas un dossier: {}", path));
            continue;
        }
        
        // Vérifier les permissions
        if let Err(e) = std::fs::metadata(path_obj) {
            issues.push(format!("Erreur d'accès au dossier {}: {}", path, e));
            continue;
        }
        
        // Vérifier la taille du dossier (approximative)
        let mut total_size = 0u64;
        let mut file_count = 0u64;
        
        if let Ok(entries) = std::fs::read_dir(path_obj) {
            for entry in entries.take(1000) {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            total_size += metadata.len();
                            file_count += 1;
                        }
                    }
                }
            }
            
            if total_size > 1024 * 1024 * 1024 { // 1GB
                issues.push(format!("Dossier très volumineux détecté: {} ({} fichiers, {} MB)", 
                    path, file_count, total_size / (1024 * 1024)));
            }
        }
    }
    
    if issues.is_empty() {
        issues.push("Aucun problème détecté avec les chemins fournis".to_string());
    }
    
    Ok(issues)
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting Tauri application");
    
    let service_repository = get_service_repository().expect("Failed to initialize service repository");
    service_repository.init().expect("Failed to initialize database");

    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    
    builder.invoke_handler(tauri::generate_handler![
        get_stat, 
        sync_files_and_folders,
        get_all_types, search_files, 
        start_content_indexing,
        open_file, 
        reset_data,
        get_all_folders,
        save_paths,
        get_all_paths,
        diagnose_scan_issues,
        ai_search,
        ai_health_check,
        ai_list_models
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

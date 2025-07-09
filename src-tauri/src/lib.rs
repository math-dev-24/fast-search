mod adapters;
mod services;
mod entities;
mod utils;
mod ports;

use entities::{file::File, stat::Stat};
use utils::{collect::collect_files_and_folders, generator::get_service_repository};
use std::path::Path;
use std::env;
use std::process::Command;


#[tauri::command]
fn get_stat() -> Result<Stat, String> {
    let service_repository = get_service_repository()?;
    let stat = service_repository.get_stat()?;
    Ok(stat)
}

#[tauri::command]
fn get_current_dir() -> Result<String, String> {
    env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn sync_files_and_folders(path: String) -> Result<(), String> {
    println!("Début de la synchronisation pour le chemin: {}", path);
    
    let mut service_repository = get_service_repository()?;

    let files = collect_files_and_folders(&Path::new(&path));
    
    println!("Fichiers trouvés: {}", files.len());
    
    service_repository.insert(&files)?;
    
    println!("Synchronisation terminée avec succès");
    Ok(())
}

#[tauri::command]
fn get_type_files() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let type_files = service_repository.get_type_files()?;
    Ok(type_files)
}

#[tauri::command]
fn search_files(search: String, types: Option<Vec<String>>, is_dir: bool, folders: Option<Vec<String>>) -> Result<Vec<File>, String> {
    let types_vec = types.unwrap_or_default();
    let folders_vec = folders.unwrap_or_default();
    
    let service_repository = get_service_repository()?;

    let files = service_repository.search(&search, &types_vec, is_dir, &folders_vec)?;
    Ok(files)
}

#[tauri::command]
fn open_file_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Convertir le chemin en PathBuf et le normaliser
        let path_buf = PathBuf::from(&path);
        let canonical_path = path_buf.canonicalize()
            .map_err(|e| format!("Impossible de résoudre le chemin: {}", e))?;
        
        // Convertir en string avec les séparateurs Windows
        let path_str = canonical_path.to_string_lossy();
        
        println!("Ouverture du fichier: {}", path_str);
        
        // Sur Windows, utiliser explorer /select, pour sélectionner le fichier
        // et ouvrir le dossier parent
        let mut command = Command::new("explorer");
        command.arg("/select,");
        command.arg(path_str.as_ref());
        
        let output = command.output()
            .map_err(|e| format!("Erreur lors de l'exécution d'explorer: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Explorer a retourné une erreur: {}", stderr));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .args([&path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}   

#[tauri::command]
fn reset_data() -> Result<(), String> {
    let service_repository = get_service_repository()?;
    service_repository.reset_data()?;
    Ok(())
}   

#[tauri::command]
fn get_all_folders() -> Result<Vec<String>, String> {
    let service_repository = get_service_repository()?;
    let folders = service_repository.get_all_folders()?;
    Ok(folders)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("=== Tauri backend starting ===");
    println!("Initializing database...");

    let service_repository = get_service_repository().unwrap();
    service_repository.init().unwrap();

    println!("Initializing builder...");
    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    let builder = builder.invoke_handler(tauri::generate_handler![get_stat, sync_files_and_folders, get_current_dir, get_type_files, search_files, open_file_in_explorer, reset_data, get_all_folders]);
    println!("Builder initialized");

    println!("Running tauri application...");

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

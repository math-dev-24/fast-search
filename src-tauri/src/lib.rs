mod adapters;
mod services;
mod entities;
mod utils;
mod ports;

use entities::{file::File, stat::Stat};
use tauri::{async_runtime, Manager};
use utils::{generator::get_service_repository, scan::scan_files_async};
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
    let service_repository = get_service_repository().unwrap();
    let paths = service_repository.get_all_paths().unwrap();
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
    let current_paths = service_repository.get_all_paths()?;

    service_repository.insert_paths(paths.clone())?;


    let mut new_paths: Vec<String> = Vec::new();

    for path in paths {
        if !current_paths.contains(&path) {
            new_paths.push(path);
        }
    }

    if !new_paths.is_empty() {
        scan_files_async(window, new_paths);
    }

    Ok(())
}

#[tauri::command]
fn search_files(
    search: String, 
    types: Option<Vec<String>>, 
    is_dir: bool, 
    folders: Option<Vec<String>>, 
    size_limit: Vec<usize>, 
    date_range: Vec<usize>,
    date_mode: String
) -> Result<Vec<File>, String> {
        
    let types_vec = types.unwrap_or_default();
    let folders_vec = folders.unwrap_or_default();
    
    let service_repository = get_service_repository()?;

    let files = service_repository.search(&search, &types_vec, is_dir, &folders_vec, &size_limit, &date_range, &date_mode)?;
    Ok(files)
}

#[tauri::command]
fn open_file_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let path_buf = PathBuf::from(&path);
        let canonical_path = path_buf.canonicalize()
            .map_err(|e| format!("Impossible de résoudre le chemin: {}", e))?;
        
        let path_str = canonical_path.to_string_lossy();
        
        println!("Ouverture du fichier: {}", path_str);
        
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


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let service_repository = get_service_repository().unwrap();
    service_repository.init().unwrap();

    let builder = tauri::Builder::default();
    let builder = builder.plugin(tauri_plugin_opener::init());
    let builder = builder.plugin(tauri_plugin_dialog::init());
    builder.invoke_handler(tauri::generate_handler![
        get_stat, 
        sync_files_and_folders, 
        get_current_dir, 
        get_all_types, search_files, 
        open_file_in_explorer, 
        reset_data, 
        get_all_folders,
        save_paths,
        get_all_paths
    ])
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
    
        async_runtime::spawn(async move {
            let service_repository = get_service_repository().unwrap();
            let paths = service_repository.get_all_paths().unwrap();
            scan_files_async(window, paths);
        });
        
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

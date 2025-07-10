use crate::utils::collect::collect_files_and_folders;
use crate::utils::generator::get_service_repository;
use std::path::Path;
use tauri::{Emitter, WebviewWindow};
use crate::entities::file::File;
use crate::entities::scan::{ScanProgress, ScanCollected, InsertProgress, ScanFinished};


pub fn scan_files_async(window: WebviewWindow, paths: Vec<String>) {
    tauri::async_runtime::spawn(async move {
        let mut service_repository = match get_service_repository() {
            Ok(repo) => repo,
            Err(e) => {
                let _ = window.emit("scan_files_error", format!("Erreur initialisation: {}", e));
                return;
            }
        };

        let _ = window.emit("scan_files_started", {});

        let mut all_files: Vec<File> = Vec::new();
        let total_paths = paths.len();

        // Scan de chaque chemin avec progress global
        for (path_index, path) in paths.iter().enumerate() {
            let path_obj = Path::new(path);
            
            // Vérification préalable du chemin
            if !path_obj.exists() {
                let _ = window.emit("scan_files_error", format!("Chemin inexistant: {}", path));
                continue;
            }

            let window_clone = window.clone();
            let files_for_path = collect_files_and_folders(path_obj, move |current, message| {
                let overall_progress = (path_index as f64 + (current as f64 / 1000.0)) / total_paths as f64;
                let _ = window_clone.emit("scan_files_progress", ScanProgress {
                    progress: overall_progress,
                    message: message.to_string(),
                    current_path: path.clone(),
                });
            });
            
            all_files.extend(files_for_path);
        }

        let total_files = all_files.len();
        let _ = window.emit("scan_files_collected", ScanCollected {
            total: total_files,
            message: format!("Collecte terminée: {} fichiers trouvés", total_files),
        });

        // Insertion par chunks avec progress
        let chunk_size = 1000; // Augmenté pour de meilleures performances
        let total_chunks = (total_files + chunk_size - 1) / chunk_size;
        
        for (chunk_index, file_chunk) in all_files.chunks(chunk_size).enumerate() {
            if let Err(e) = service_repository.insert(file_chunk.to_vec()) {
                let _ = window.emit("scan_files_error", format!("Erreur insertion: {}", e));
                return;
            }

            let progress = (chunk_index + 1) as f64 / total_chunks as f64;
            let _ = window.emit("scan_files_insert_progress", InsertProgress {
                progress,
                processed: (chunk_index + 1) * chunk_size.min(total_files - chunk_index * chunk_size),
                total: total_files,
            });
        }

        let _ = window.emit("scan_files_finished", ScanFinished {
            total: total_files,
            message: "Synchronisation terminée avec succès".to_string(),
        });
    });
}
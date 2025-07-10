use crate::utils::collect::collect_files_and_folders;
use crate::utils::generator::get_service_repository;
use std::path::Path;
use tauri::{Emitter, WebviewWindow};
use crate::entities::file::File;


pub fn scan_files_async(window: WebviewWindow, paths: Vec<String>) {
    tauri::async_runtime::spawn(async move {
        let mut service_repository = get_service_repository().unwrap();

        let _ = window.emit("scan_files_started", {});

        let mut files: Vec<File> = Vec::new();

        for path in paths {
            let files_path = collect_files_and_folders(&Path::new(&path));
            files.extend(files_path);
        }

        let total = files.len();

        let chunk_size = 100;
        let total_chunks = (total + chunk_size - 1) / chunk_size;
        let _ = window.emit("scan_files_total", total_chunks);

        for (i, file_chunk) in files.chunks(chunk_size).enumerate() {
            if let Err(e) = service_repository.insert(file_chunk.to_vec()) {
                let _ = window.emit("scan_files_error", e.to_string());
                return;
            }

            let progress = (i + 1) as f64 / total_chunks as f64;
            let _ = window.emit("scan_files_progress", progress);
        }

        let _ = window.emit("scan_files_finished", "Synchronisation terminée avec succès");
    });
}
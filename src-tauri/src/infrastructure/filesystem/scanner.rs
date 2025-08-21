use crate::application::events::emitters::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                                           EVENT_SCAN_STARTED, EVENT_SCAN_PROGRESS, EVENT_SCAN_COLLECTED,
                                           EVENT_SCAN_INSERT_PROGRESS, EVENT_SCAN_FINISHED, EVENT_SCAN_ERROR,
                                           EVENT_STAT_UPDATED};
use std::path::Path;
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};
use crate::application::factories::service_factory::get_service_repository;
use crate::domain::entities::file::File;
use crate::domain::entities::scan::{ScanProgress, ScanCollected, InsertProgress, ScanFinished};
use crate::domain::entities::progress::ScanProgressTracker;
use crate::infrastructure::filesystem::collect::collect_files_and_folders;

const CHUNK_SIZE: usize = 500;

pub fn scan_files_async(window: WebviewWindow, paths: Vec<String>) {
    tauri::async_runtime::spawn(async move {

        let service_repository = match get_service_repository() {
            Ok(repo) => Arc::new(Mutex::new(repo)),
            Err(e) => {
                emit_error_event(&window, EVENT_SCAN_ERROR, format!("Erreur initialisation: {}", e));
                return;
            }
        };

        emit_started_event(&window, EVENT_SCAN_STARTED);

        let total_paths = paths.len();

        if total_paths == 0 {
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun chemin à scanner".to_string(),
            });
            return;
        }

        let progress_tracker = Arc::new(Mutex::new(ScanProgressTracker::new(total_paths)));
        let mut all_files: Vec<File> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        // Phase 1: Collecte des fichiers avec gestion d'erreur améliorée
        for (path_index, path) in paths.iter().enumerate() {
            let path_obj = Path::new(path);

            if !path_obj.exists() {
                let error_msg = format!("Chemin inexistant: {}", path);
                errors.push(error_msg.clone());
                emit_error_event(&window, EVENT_SCAN_ERROR, error_msg);
                continue;
            }

            let window_clone = window.clone();
            let progress_tracker_clone = progress_tracker.clone();

            {
                let tracker = progress_tracker_clone.lock().unwrap();
                if tracker.is_timeout() {
                    let error_msg = format!("Timeout lors du traitement du chemin: {}", path);
                    errors.push(error_msg.clone());
                    emit_error_event(&window, EVENT_SCAN_ERROR, error_msg);
                    continue;
                }
            }

            let files_for_path = collect_files_and_folders(path_obj, move |current, message| {
                let mut tracker = progress_tracker_clone.lock().unwrap();
                tracker.current_path_index = path_index;

                if tracker.is_timeout() {
                    return;
                }

                let path_progress = current as f64 / 1000.0;
                let overall_progress = tracker.update_path_progress(path_progress);

                if tracker.should_update_progress() {
                    emit_event(&window_clone, EVENT_SCAN_PROGRESS, ScanProgress {
                        progress: overall_progress,
                        message: message.to_string(),
                        current_path: path.clone(),
                    });
                    tracker.update_progress_time();
                }
            });

            all_files.extend(files_for_path);

            {
                let mut tracker = progress_tracker.lock().unwrap();
                tracker.next_path();
                tracker.set_total_files(all_files.len());
            }
        }

        let total_files = all_files.len();

        // Émission de l'événement de collecte terminée
        emit_event(&window, EVENT_SCAN_COLLECTED, ScanCollected {
            total: total_files,
            message: format!("Collecte terminée: {} fichiers trouvés", total_files),
        });

        if total_files == 0 {
            let message = if errors.is_empty() {
                "Aucun fichier trouvé".to_string()
            } else {
                format!("Aucun fichier trouvé. Erreurs: {}", errors.join(", "))
            };

            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message,
            });
            return;
        }

        let total_chunks = (total_files + CHUNK_SIZE - 1) / CHUNK_SIZE;

        let mut insert_errors = 0;

        for (chunk_index, file_chunk) in all_files.chunks(CHUNK_SIZE).enumerate() {
            let insert_result = {
                let mut repo = service_repository.lock().unwrap();
                repo.insert(file_chunk.to_vec())
            };

            if let Err(e) = insert_result {
                insert_errors += 1;

                println!("[WARNING] Chunk insertion error {}: {}", chunk_index, e);

                if insert_errors > 5 {
                    emit_error_event(&window, EVENT_SCAN_ERROR, "Too many insertion errors, scan stopped");
                    println!("[ERROR] Too many insertion errors, scan stopped");
                    return;
                }
            }

            let progress = (chunk_index + 1) as f64 / total_chunks as f64;
            let processed = (chunk_index + 1) * CHUNK_SIZE.min(total_files - chunk_index * CHUNK_SIZE);

            emit_event(&window, EVENT_SCAN_INSERT_PROGRESS, InsertProgress {
                progress,
                processed,
                total: total_files,
            });

            {
                let repo = service_repository.lock().unwrap();
                if let Ok(stat) = repo.get_stat() {
                    emit_event(&window, EVENT_STAT_UPDATED, stat);
                }
            }
        }

        let success_count = total_files - (insert_errors * CHUNK_SIZE);

        let message = if errors.is_empty() && insert_errors == 0 {
            format!("Synchronisation terminée avec succès: {} fichiers traités", success_count)
        } else {
            format!("Synchronisation terminée: {} fichiers traités, {} erreurs", success_count, errors.len() + insert_errors)
        };

        emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
            total: success_count,
            message,
        });

        {
            let repo = service_repository.lock().unwrap();
            if let Ok(stat) = repo.get_stat() {
                emit_event(&window, EVENT_STAT_UPDATED, stat);
            }
        }
    });
}
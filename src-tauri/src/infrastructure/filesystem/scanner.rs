use crate::application::events::emitters::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                                           EVENT_SCAN_STARTED, EVENT_SCAN_PROGRESS, EVENT_SCAN_COLLECTED,
                                           EVENT_SCAN_INSERT_PROGRESS, EVENT_SCAN_FINISHED, EVENT_SCAN_ERROR,
                                           EVENT_STAT_UPDATED};
use std::path::Path;
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};
use crate::domain::entities::file::File;
use crate::domain::entities::scan::{ScanProgress, ScanCollected, InsertProgress, ScanFinished};
use crate::domain::entities::progress::ScanProgressTracker;
use crate::infrastructure::filesystem::collect::collect_files_and_folders;
use crate::application::use_cases::index_content::index_content_async;
use crate::domain::services::file_service::FileService;
use crate::infrastructure::repository::sqlite::Db;

const CHUNK_SIZE: usize = 500;

struct ScanContext {
    service_repository: Arc<Mutex<crate::domain::services::file_service::FileService<crate::infrastructure::repository::sqlite::Db>>>,
    window: WebviewWindow,
    progress_tracker: Arc<Mutex<ScanProgressTracker>>,
}

impl ScanContext {
    fn emit_stat_update(&self) {
        if let Ok(repo) = self.service_repository.lock() {
            if let Ok(stat) = repo.get_stat() {
                emit_event(&self.window, EVENT_STAT_UPDATED, stat);
            }
        }
    }

    fn emit_scan_error(&self, message: String) {
        emit_error_event(&self.window, EVENT_SCAN_ERROR, message);
    }
}

fn collect_all_files(paths: &[String], context: &ScanContext) -> Result<Vec<File>, Vec<String>> {
    let mut all_files = Vec::new();
    let mut errors = Vec::new();

    for (path_index, path) in paths.iter().enumerate() {
        let path_obj = Path::new(path);

        if !path_obj.exists() {
            let error_msg = format!("Chemin inexistant: {}", path);
            errors.push(error_msg.clone());
            context.emit_scan_error(error_msg);
            continue;
        }

        let window_clone = context.window.clone();
        let progress_tracker_clone = context.progress_tracker.clone();

        let files_for_path = collect_files_and_folders(path_obj, move |current, message| {
            if let Ok(mut tracker) = progress_tracker_clone.lock() {
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
            }
        });

        all_files.extend(files_for_path);

        if let Ok(mut tracker) = context.progress_tracker.lock() {
            tracker.next_path();
            tracker.set_total_files(all_files.len());
        }
    }

    if errors.is_empty() {
        Ok(all_files)
    } else {
        Err(errors)
    }
}

fn insert_files_in_chunks(files: &[File], context: &ScanContext) -> Result<usize, String> {
    let total_files = files.len();
    let total_chunks = (total_files + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let mut insert_errors = 0;

    for (chunk_index, file_chunk) in files.chunks(CHUNK_SIZE).enumerate() {
        let insert_result = {
            match context.service_repository.lock() {
                Ok(mut repo) => repo.insert(file_chunk.to_vec()),
                Err(e) => {
                    let error_msg = format!("Failed to lock repository: {}", e);
                    context.emit_scan_error(error_msg.clone());
                    return Err(error_msg);
                }
            }
        };

        if let Err(e) = insert_result {
            insert_errors += 1;
            tracing::warn!("Chunk insertion error {}: {}", chunk_index, e);

            if insert_errors > 5 {
                let error_msg = "Too many insertion errors, scan stopped";
                context.emit_scan_error(error_msg.to_string());
                return Err(error_msg.to_string());
            }
        }

        let progress = (chunk_index + 1) as f64 / total_chunks as f64;
        let processed = (chunk_index + 1) * CHUNK_SIZE.min(total_files - chunk_index * CHUNK_SIZE);

        emit_event(&context.window, EVENT_SCAN_INSERT_PROGRESS, InsertProgress {
            progress,
            processed,
            total: total_files,
        });

        tracing::debug!("Émission des stats après insertion chunk {}", chunk_index + 1);
        context.emit_stat_update();
    }

    Ok(total_files - (insert_errors * CHUNK_SIZE))
}

pub fn scan_files_async(
    window: WebviewWindow,
    paths: Vec<String>,
    service_repository: Arc<Mutex<FileService<Db>>>
) {
    tauri::async_runtime::spawn(async move {

        if paths.is_empty() {
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun chemin à scanner".to_string(),
            });
            return;
        }

        let progress_tracker = Arc::new(Mutex::new(ScanProgressTracker::new(paths.len())));
        let context = ScanContext {
            service_repository,
            window: window.clone(),
            progress_tracker,
        };

        emit_started_event(&window, EVENT_SCAN_STARTED);

        // Phase 1: Collecte des fichiers
        let all_files = match collect_all_files(&paths, &context) {
            Ok(files) => files,
            Err(errors) => {
                let message = format!("Erreurs lors de la collecte: {}", errors.join(", "));
                emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished { total: 0, message });
                return;
            }
        };

        let total_files = all_files.len();

        // Émission de l'événement de collecte terminée
        emit_event(&window, EVENT_SCAN_COLLECTED, ScanCollected {
            total: total_files,
            message: format!("Collecte terminée: {} fichiers trouvés", total_files),
        });

        if total_files == 0 {
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun fichier trouvé".to_string(),
            });
            return;
        }

        // Phase 2: Insertion des fichiers en base
        let success_count = match insert_files_in_chunks(&all_files, &context) {
            Ok(count) => count,
            Err(e) => {
                emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                    total: 0,
                    message: format!("Erreur lors de l'insertion: {}", e),
                });
                return;
            }
        };

        // Finalisation
        emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
            total: success_count,
            message: format!("Synchronisation terminée avec succès: {} fichiers traités", success_count),
        });

        context.emit_stat_update();

        // Phase 3: Démarrer l'indexation du contenu automatiquement
        tracing::info!("Démarrage de l'indexation du contenu automatique");
        let service_repo = context.service_repository.clone();
        index_content_async(window, service_repo);
    });
}
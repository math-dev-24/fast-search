use crate::application::events::emitters::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                                           EVENT_SCAN_STARTED, EVENT_SCAN_PROGRESS, EVENT_SCAN_COLLECTED,
                                           EVENT_SCAN_INSERT_PROGRESS, EVENT_SCAN_FINISHED, EVENT_SCAN_ERROR,
                                           EVENT_STAT_UPDATED};
use std::path::Path;
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};
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
    fn lock_repository(&self) -> std::sync::MutexGuard<'_, FileService<Db>> {
        match self.service_repository.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::error!("Service repository lock was poisoned (scanner), recovering to avoid permanent failure");
                poisoned.into_inner()
            }
        }
    }

    fn emit_stat_update(&self) {
        let repo = self.lock_repository();
        if let Ok(stat) = repo.get_stat() {
            emit_event(&self.window, EVENT_STAT_UPDATED, stat);
        }
    }

    fn emit_scan_error(&self, message: String) {
        emit_error_event(&self.window, EVENT_SCAN_ERROR, message);
    }
}

fn collect_and_insert_files(paths: &[String], context: &ScanContext) -> Result<usize, Vec<String>> {
    let mut total_inserted = 0usize;
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

        let inserted_for_path = match insert_files_in_chunks(&files_for_path, context) {
            Ok(count) => count,
            Err(e) => {
                errors.push(e);
                0
            }
        };
        total_inserted += inserted_for_path;

        if let Ok(mut tracker) = context.progress_tracker.lock() {
            tracker.next_path();
            tracker.set_total_files(total_inserted);
        }
    }

    if errors.is_empty() {
        Ok(total_inserted)
    } else {
        Err(errors)
    }
}

fn insert_files_in_chunks(files: &[crate::domain::entities::file::File], context: &ScanContext) -> Result<usize, String> {
    let total_files = files.len();
    let total_chunks = (total_files + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let mut insert_errors = 0;

    for (chunk_index, file_chunk) in files.chunks(CHUNK_SIZE).enumerate() {
        let insert_result = {
            let mut repo = context.lock_repository();
            repo.insert(file_chunk.to_vec())
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

        // Phase 1 + 2: collecte et insertion par chemin pour limiter la mémoire
        let success_count = match collect_and_insert_files(&paths, &context) {
            Ok(count) => count,
            Err(errors) => {
                let message = format!("Erreurs lors de la collecte: {}", errors.join(", "));
                emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished { total: 0, message });
                return;
            }
        };

        // Émission de l'événement de collecte terminée
        emit_event(&window, EVENT_SCAN_COLLECTED, ScanCollected {
            total: success_count,
            message: format!("Collecte/insertion terminée: {} fichiers traités", success_count),
        });

        if success_count == 0 {
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun fichier trouvé".to_string(),
            });
            return;
        }

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
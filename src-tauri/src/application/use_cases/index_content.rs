use crate::application::events::emitters::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                           EVENT_INDEX_STARTED, EVENT_INDEX_PROGRESS, EVENT_INDEX_FINISHED,
                           EVENT_INDEX_ERROR, EVENT_STAT_UPDATED};
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};
use std::panic;
use crate::domain::entities::scan::{IndexProgress, IndexFinished};
use crate::domain::entities::file::File;
use crate::domain::services::content_indexer_service::ContentIndexerService;
use crate::domain::services::file_service::FileService;
use crate::infrastructure::repository::sqlite::Db;


#[derive(Debug, Clone)]
struct ProgressTracker {
    total_files: usize,
    processed_files: usize,
    successful_files: usize,
    failed_files: usize,
    last_progress_update: std::time::Instant,
}

impl ProgressTracker {
    fn new(total_files: usize) -> Self {
        Self {
            total_files,
            processed_files: 0,
            successful_files: 0,
            failed_files: 0,
            last_progress_update: std::time::Instant::now(),
        }
    }

    fn increment_processed(&mut self, success: bool) {
        self.processed_files += 1;
        if success {
            self.successful_files += 1;
        } else {
            self.failed_files += 1;
        }
    }

    fn update_progress_time(&mut self) {
        self.last_progress_update = std::time::Instant::now();
    }

    fn get_progress(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            self.processed_files as f64 / self.total_files as f64
        }
    }
}



fn can_index_file(file: &File) -> bool {
    if !file.path.exists() || !file.path.is_file() {
        return false;
    }

    let content_indexer = ContentIndexerService::new();
    content_indexer.can_index_file(file)
}

fn lock_repository(service_repository: &Arc<Mutex<FileService<Db>>>) -> std::sync::MutexGuard<'_, FileService<Db>> {
    match service_repository.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("Service repository lock was poisoned (index_content), recovering to avoid permanent failure");
            poisoned.into_inner()
        }
    }
}

async fn process_single_file(
    file: File,
    service_repository: Arc<Mutex<FileService<Db>>>,
) -> Result<(), String> {
    let file_path = file.path.display().to_string();

    if !can_index_file(&file) {
        let mut repo = lock_repository(&service_repository);
        repo.update_file_index_status(&file, String::new(), false)
            .map_err(|e| format!("Erreur mise à jour fichier non indexable {}: {}", file_path, e))?;
        return Ok(());
    }

    let file_clone = file.clone();
    let read_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut content_indexer = ContentIndexerService::new();
        content_indexer.index_file_content(&file_clone)
    }));

    let text_content = match read_result {
        Ok(Ok(content)) => {
            tracing::debug!("Indexation réussie: {} ({} chars)", file_path, content.len());
            content
        },
        Ok(Err(e)) => {
            tracing::warn!("Échec indexation: {} - {}", file_path, e);
            let mut repo = lock_repository(&service_repository);
            repo.update_file_index_status(&file, String::new(), false)
                .map_err(|update_err| format!("Erreur mise à jour après échec pour {}: {}", file_path, update_err))?;
            return Ok(());
        },
        Err(panic_info) => {
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic".to_string()
            };
            tracing::error!("Panic lors de l'indexation de {}: {}", file_path, panic_msg);
            let mut repo = lock_repository(&service_repository);
            repo.update_file_index_status(&file, String::new(), false)
                .map_err(|update_err| format!("Erreur mise à jour après panic pour {}: {}", file_path, update_err))?;
            return Ok(());
        }
    };

    let mut repo = lock_repository(&service_repository);
    repo.update_file_index_status(&file, text_content, true)
        .map_err(|e| format!("Erreur mise à jour succès pour {}: {}", file_path, e))?;

    Ok(())
}


pub fn index_content_async(
    window: WebviewWindow,
    service_repository: Arc<Mutex<FileService<Db>>>
) {
    tauri::async_runtime::spawn(async move {

        emit_started_event(&window, EVENT_INDEX_STARTED);

        let uncontent_indexed_files = {
            let repo = lock_repository(&service_repository);
            match repo.get_uncontent_indexed_files() {
                Ok(files) => files,
                Err(e) => {
                    emit_error_event(&window, EVENT_INDEX_ERROR, format!("Erreur récupération fichiers: {}", e));
                    return;
                }
            }
        };

        let total_files = uncontent_indexed_files.len();

        if total_files == 0 {
            emit_finished_event(&window, EVENT_INDEX_FINISHED, IndexFinished {
                total: 0,
                message: "Aucun fichier nécessite une indexation du contenu".to_string(),
            });
            return;
        }

        // Initialisation du tracker de progrès
        let progress_tracker = Arc::new(Mutex::new(ProgressTracker::new(total_files)));

        tracing::info!("Démarrage de l'indexation de contenu pour {} fichiers", total_files);
        
        // Traitement séquentiel par chunks plus petit pour un meilleur feedback
        const CHUNK_SIZE_INDEX: usize = 10;
        let chunks: Vec<Vec<File>> = uncontent_indexed_files
            .chunks(CHUNK_SIZE_INDEX)
            .map(|chunk| chunk.to_vec())
            .collect();

        let total_chunks = chunks.len();
        tracing::info!("Traitement par {} chunks de {} fichiers", total_chunks, CHUNK_SIZE_INDEX);

        for (chunk_index, file_chunk) in chunks.into_iter().enumerate() {
            tracing::info!("Traitement du chunk {} / {}", chunk_index + 1, total_chunks);
            
            // Traitement parallèle du chunk avec futures
            let mut handles = Vec::new();
            
            for file in file_chunk {
                let service_repo_clone = service_repository.clone();
                let handle = tokio::spawn(async move {
                    process_single_file(file, service_repo_clone).await
                });
                handles.push(handle);
            }

            // Attendre tous les fichiers du chunk
            for handle in handles {
                match handle.await {
                    Ok(result) => {
                        if let Ok(mut tracker) = progress_tracker.lock() {
                            tracker.increment_processed(result.is_ok());
                        }

                        if let Err(e) = result {
                            tracing::warn!("Erreur indexation fichier: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Erreur task indexation: {}", e);
                        if let Ok(mut tracker) = progress_tracker.lock() {
                            tracker.increment_processed(false);
                        }
                    }
                }
            }

            // Mise à jour du progrès après chaque chunk
            {
                if let Ok(mut tracker) = progress_tracker.lock() {
                    let progress = tracker.get_progress();
                    let message = format!(
                        "Indexation du contenu: {} fichiers traités sur {} ({}%)",
                        tracker.processed_files,
                        total_files,
                        (progress * 100.0) as usize
                    );

                    emit_event(&window, EVENT_INDEX_PROGRESS, IndexProgress {
                        progress,
                        message,
                        processed: tracker.processed_files,
                        total: total_files,
                    });

                    tracker.update_progress_time();
                }
            }

            {
                let repo = lock_repository(&service_repository);
                if let Ok(stat) = repo.get_stat() {
                    tracing::debug!("Émission des stats après chunk {}: {} fichiers indexés", chunk_index + 1, stat.content_indexed_files);
                    emit_event(&window, EVENT_STAT_UPDATED, stat);
                }
            }
        }


        let final_stat = {
            let repo = lock_repository(&service_repository);
            repo.get_stat().ok()
        };
        
        if let Some(stat) = final_stat {
            emit_event(&window, EVENT_STAT_UPDATED, stat);
        }

        
        let message = if let Ok(final_tracker) = progress_tracker.lock() {
            format!(
                "Indexation terminée: {} fichiers traités ({} indexés avec succès, {} échecs)",
                final_tracker.processed_files,
                final_tracker.successful_files,
                final_tracker.failed_files
            )
        } else {
            "Indexation terminée".to_string()
        };

        let total_processed = if let Ok(tracker) = progress_tracker.lock() {
            tracker.processed_files
        } else {
            0
        };

        emit_finished_event(&window, EVENT_INDEX_FINISHED, IndexFinished {
            total: total_processed,
            message,
        });
    });
}
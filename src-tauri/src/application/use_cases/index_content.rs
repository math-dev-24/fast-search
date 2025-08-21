use crate::application::events::emitters::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                           EVENT_INDEX_STARTED, EVENT_INDEX_PROGRESS, EVENT_INDEX_FINISHED,
                           EVENT_INDEX_ERROR, EVENT_STAT_UPDATED};
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::domain::entities::scan::{IndexProgress, IndexFinished};
use crate::domain::entities::file::File;
use crate::domain::services::content_indexer_service::ContentIndexerService;
use crate::domain::services::file_service::FileService;
use crate::application::factories::service_factory::get_service_repository;
use crate::infrastructure::repository::sqlite::Db;

const CHUNK_SIZE: usize = 50;

const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(1000);

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

    fn should_update_progress(&self) -> bool {
        self.last_progress_update.elapsed() >= PROGRESS_UPDATE_INTERVAL
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
        println!("NON INDEXABLE (n'existe pas ou n'est pas un fichier): {}", file.path.display());
        return false;
    }

    let content_indexer = ContentIndexerService::new();
    let can_index = content_indexer.can_index_file(file);

    if !can_index {
        println!("NON INDEXABLE (extension non supportée): {}", file.path.display());
    }

    can_index
}

async fn process_single_file(
    file: File,
    service_repository: Arc<Mutex<FileService<Db>>>,
) -> Result<(), String> {
    let mut repo = service_repository.lock()
        .map_err(|e| format!("Erreur d'accès au repository: {}", e))?;

    if !can_index_file(&file) {
        repo.update_file_index_status(&file, String::new(), false)
            .map_err(|e| format!("Erreur mise à jour fichier non indexable: {}", e))?;
        return Ok(());
    }

    let mut content_indexer = ContentIndexerService::new();

    let text_content = match content_indexer.index_file_content(&file) {
        Ok(content) => content,
        Err(e) => {
            // En cas d'erreur de lecture, marquer le fichier comme non indexable
            repo.update_file_index_status(&file, String::new(), false)
                .map_err(|update_err| format!("Erreur mise à jour fichier après échec lecture: {}", update_err))?;
            return Err(format!("Erreur lecture fichier {}: {}", file.path.display(), e));
        }
    };

    // Marquer le fichier comme indexé avec succès
    repo.update_file_index_status(&file, text_content, true)
        .map_err(|e| format!("Erreur mise à jour fichier: {}", e))?;

    Ok(())
}

pub fn index_content_async(window: WebviewWindow) {
    tauri::async_runtime::spawn(async move {
        // Initialisation du service repository
        let service_repository = match get_service_repository() {
            Ok(repo) => Arc::new(Mutex::new(repo)),
            Err(e) => {
                emit_error_event(&window, EVENT_INDEX_ERROR, format!("Erreur initialisation: {}", e));
                return;
            }
        };

        // Émission de l'événement de démarrage
        emit_started_event(&window, EVENT_INDEX_STARTED);

        // Récupération des fichiers non indexés
        let uncontent_indexed_files = {
            let repo = service_repository.lock().unwrap();
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

        // Traitement séquentiel par chunks pour éviter les problèmes de durée de vie
        let chunks: Vec<Vec<File>> = uncontent_indexed_files
            .chunks(CHUNK_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();

        for file_chunk in chunks.into_iter() {
            // Traitement séquentiel des fichiers dans le chunk
            for file in file_chunk {
                let result = process_single_file(file, service_repository.clone()).await;

                // Mise à jour du tracker de progrès
                let mut tracker = progress_tracker.lock().unwrap();
                tracker.increment_processed(result.is_ok());

                if let Err(e) = result {
                    eprintln!("Erreur lors du traitement du fichier: {}", e);
                }
            }

            // Mise à jour du progrès après chaque chunk
            {
                let mut tracker = progress_tracker.lock().unwrap();
                if tracker.should_update_progress() {
                    let progress = tracker.get_progress();
                    let message = format!(
                        "Indexation du contenu: {} fichiers traités sur {} ({}%)",
                        tracker.processed_files,
                        total_files,
                        (tracker.processed_files as f64 / total_files as f64 * 100.0) as usize
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

            // Mise à jour des statistiques après chaque chunk
            if let Ok(repo) = service_repository.lock() {
                if let Ok(stat) = repo.get_stat() {
                    emit_event(&window, EVENT_STAT_UPDATED, stat);
                }
            }
        }

        // Émission de l'événement de fin
        let final_tracker = progress_tracker.lock().unwrap();
        let message = format!(
            "Indexation terminée: {} fichiers traités ({} indexés avec succès, {} échecs)",
            final_tracker.processed_files,
            final_tracker.successful_files,
            final_tracker.failed_files
        );

        emit_finished_event(&window, EVENT_INDEX_FINISHED, IndexFinished {
            total: final_tracker.processed_files,
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
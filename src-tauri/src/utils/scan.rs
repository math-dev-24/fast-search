use crate::utils::collect::collect_files_and_folders;
use crate::utils::generator::get_service_repository;
use crate::utils::events::{emit_event, emit_error_event, emit_started_event, emit_finished_event,
                          EVENT_SCAN_STARTED, EVENT_SCAN_PROGRESS, EVENT_SCAN_COLLECTED,
                          EVENT_SCAN_INSERT_PROGRESS, EVENT_SCAN_FINISHED, EVENT_SCAN_ERROR,
                          EVENT_STAT_UPDATED};
use std::path::Path;
use tauri::WebviewWindow;
use crate::entities::file::File;
use crate::entities::scan::{ScanProgress, ScanCollected, InsertProgress, ScanFinished};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Configuration
const CHUNK_SIZE: usize = 1000;
const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(500);

// Structure pour le suivi du progrès de scan
#[derive(Debug, Clone)]
struct ScanProgressTracker {
    total_paths: usize,
    current_path_index: usize,
    total_files: usize,
    last_progress_update: std::time::Instant,
}

impl ScanProgressTracker {
    fn new(total_paths: usize) -> Self {
        Self {
            total_paths,
            current_path_index: 0,
            total_files: 0,
            last_progress_update: std::time::Instant::now(),
        }
    }

    fn update_path_progress(&mut self, path_progress: f64) -> f64 {
        let path_weight = 1.0 / self.total_paths as f64;
        let current_path_contribution = (self.current_path_index as f64 + path_progress) * path_weight;
        current_path_contribution
    }

    fn next_path(&mut self) {
        self.current_path_index += 1;
    }

    fn set_total_files(&mut self, total: usize) {
        self.total_files = total;
    }



    fn should_update_progress(&self) -> bool {
        self.last_progress_update.elapsed() >= PROGRESS_UPDATE_INTERVAL
    }

    fn update_progress_time(&mut self) {
        self.last_progress_update = std::time::Instant::now();
    }
}

pub fn scan_files_async(window: WebviewWindow, paths: Vec<String>) {
    tauri::async_runtime::spawn(async move {
        // Initialisation du service repository
        let service_repository = match get_service_repository() {
            Ok(repo) => Arc::new(Mutex::new(repo)),
            Err(e) => {
                emit_error_event(&window, EVENT_SCAN_ERROR, format!("Erreur initialisation: {}", e));
                return;
            }
        };

        // Émission de l'événement de démarrage
        emit_started_event(&window, EVENT_SCAN_STARTED);

        let total_paths = paths.len();
        if total_paths == 0 {
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun chemin à scanner".to_string(),
            });
            return;
        }

        // Initialisation du tracker de progrès
        let progress_tracker = Arc::new(Mutex::new(ScanProgressTracker::new(total_paths)));
        let mut all_files: Vec<File> = Vec::new();

        // Phase 1: Collecte des fichiers
        for (path_index, path) in paths.iter().enumerate() {
            let path_obj = Path::new(path);
            
            if !path_obj.exists() {
                emit_error_event(&window, EVENT_SCAN_ERROR, format!("Chemin inexistant: {}", path));
                continue;
            }

            let window_clone = window.clone();
            let progress_tracker_clone = progress_tracker.clone();
            
            let files_for_path = collect_files_and_folders(path_obj, move |current, message| {
                let mut tracker = progress_tracker_clone.lock().unwrap();
                tracker.current_path_index = path_index;
                
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
            
            // Mise à jour du tracker
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
            emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
                total: 0,
                message: "Aucun fichier trouvé".to_string(),
            });
            return;
        }

        // Phase 2: Insertion en base de données
        let total_chunks = (total_files + CHUNK_SIZE - 1) / CHUNK_SIZE;
        
        for (chunk_index, file_chunk) in all_files.chunks(CHUNK_SIZE).enumerate() {
            // Insertion du chunk
            {
                let mut repo = service_repository.lock().unwrap();
                if let Err(e) = repo.insert(file_chunk.to_vec()) {
                    emit_error_event(&window, EVENT_SCAN_ERROR, format!("Erreur insertion: {}", e));
                    return;
                }
            }

            // Mise à jour du progrès
            let progress = (chunk_index + 1) as f64 / total_chunks as f64;
            let processed = (chunk_index + 1) * CHUNK_SIZE.min(total_files - chunk_index * CHUNK_SIZE);
            
            emit_event(&window, EVENT_SCAN_INSERT_PROGRESS, InsertProgress {
                progress,
                processed,
                total: total_files,
            });

            // Mise à jour des statistiques
            {
                let repo = service_repository.lock().unwrap();
                if let Ok(stat) = repo.get_stat() {
                    emit_event(&window, EVENT_STAT_UPDATED, stat);
                }
            }
        }

        // Émission de l'événement de fin
        emit_finished_event(&window, EVENT_SCAN_FINISHED, ScanFinished {
            total: total_files,
            message: format!("Synchronisation terminée avec succès: {} fichiers traités", total_files),
        });

        // Dernière mise à jour des statistiques
        {
            let repo = service_repository.lock().unwrap();
            if let Ok(stat) = repo.get_stat() {
                emit_event(&window, EVENT_STAT_UPDATED, stat);
            }
        }
    });
}
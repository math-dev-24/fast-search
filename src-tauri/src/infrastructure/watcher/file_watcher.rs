use crate::application::events::emitters::{
    emit_event, emit_error_event, emit_started_event, emit_finished_event,
    EVENT_WATCHER_STARTED, EVENT_WATCHER_STOPPED, EVENT_WATCHER_ERROR,
    EVENT_FILE_CREATED, EVENT_FILE_MODIFIED, EVENT_FILE_DELETED
};
use crate::shared::errors::{AppError, AppResult};
use notify::{Watcher, RecursiveMode, Config, PollWatcher, Event, EventKind};
use std::time::Duration;
use std::path::Path;
use tokio::task::JoinHandle;
use tauri::WebviewWindow;
use std::sync::{Arc, Mutex};

pub struct AsyncFileWatcher {
    watcher: Option<PollWatcher>,
    is_watching: bool,
    task_handle: Option<JoinHandle<()>>,
    window: WebviewWindow,
    watched_paths: Vec<String>,
}

impl AsyncFileWatcher {
    pub fn new(window: WebviewWindow) -> Self {
        Self {
            watcher: None,
            is_watching: false,
            task_handle: None,
            window,
            watched_paths: Vec::new(),
        }
    }

    pub fn start_watching(&mut self, paths: Vec<String>) -> AppResult<()> {
        if self.is_watching {
            self.stop_watching()?;
        }

        if paths.is_empty() {
            let error_msg = "Aucun chemin à surveiller".to_string();
            emit_error_event(&self.window, EVENT_WATCHER_ERROR, &error_msg);
            return Err(AppError::Validation(error_msg));
        }

        for path_str in &paths {
            let path = Path::new(path_str);
            if !path.exists() {
                let error_msg = format!("Chemin inexistant: {}", path_str);
                emit_error_event(&self.window, EVENT_WATCHER_ERROR, &error_msg);
                return Err(AppError::Validation(error_msg));
            }
        }

        let (notify_tx, notify_rx) = std::sync::mpsc::channel();

        let config = Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(false);

        let mut watcher = PollWatcher::new(notify_tx, config)
            .map_err(|e| {
                let error_msg = format!("Failed to create watcher: {}", e);
                emit_error_event(&self.window, EVENT_WATCHER_ERROR, &error_msg);
                AppError::Internal(error_msg)
            })?;

        // Ajouter les chemins à surveiller
        for path_str in &paths {
            let path = Path::new(path_str);
            watcher.watch(path, RecursiveMode::Recursive)
                .map_err(|e| {
                    let error_msg = format!("Failed to watch path {}: {}", path_str, e);
                    emit_error_event(&self.window, EVENT_WATCHER_ERROR, &error_msg);
                    AppError::Internal(error_msg)
                })?;
        }

        let window_clone = self.window.clone();
        let window_error_clone = self.window.clone();

        // Tâche pour écouter les événements de fichiers
        let task_handle = tokio::task::spawn(async move {
            tokio::task::spawn_blocking(move || {
                loop {
                    match notify_rx.recv() {
                        Ok(Ok(event)) => {
                            if let Some(file_event) = convert_notify_event(event) {
                                emit_file_event(&window_clone, &file_event);
                            }
                        },
                        Ok(Err(e)) => {
                            let error_msg = format!("File watcher error: {:?}", e);
                            emit_error_event(&window_clone, EVENT_WATCHER_ERROR, error_msg);
                        },
                        Err(_) => {
                            // Canal fermé, on sort de la boucle
                            break;
                        }
                    }
                }
            }).await.unwrap_or_else(|e| {
                let error_msg = format!("File watcher task error: {:?}", e);
                emit_error_event(&window_error_clone, EVENT_WATCHER_ERROR, error_msg);
            });
        });

        self.watcher = Some(watcher);
        self.is_watching = true;
        self.task_handle = Some(task_handle);
        self.watched_paths = paths.clone();

        // Émettre l'événement de démarrage
        emit_started_event(&self.window, EVENT_WATCHER_STARTED);

        tracing::info!("File watcher started for {} paths: {:?}", paths.len(), paths);
        Ok(())
    }

    pub fn stop_watching(&mut self) -> AppResult<()> {
        if !self.is_watching {
            return Ok(());
        }

        // Arrêter le watcher
        self.watcher = None;

        // Arrêter la tâche
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }

        self.is_watching = false;
        let watched_count = self.watched_paths.len();
        self.watched_paths.clear();

        // Émettre l'événement d'arrêt
        emit_finished_event(&self.window, EVENT_WATCHER_STOPPED, serde_json::json!({
            "message": format!("File watching stopped ({} paths)", watched_count),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

        tracing::info!("File watcher stopped");
        Ok(())
    }

    pub fn restart_watching(&mut self, new_paths: Vec<String>) -> AppResult<()> {
        self.stop_watching()?;
        self.start_watching(new_paths)
    }

    pub fn is_watching(&self) -> bool {
        self.is_watching
    }

    pub fn get_watched_paths(&self) -> &[String] {
        &self.watched_paths
    }
}

fn convert_notify_event(event: Event) -> Option<(String, String)> {
    if let Some(path) = event.paths.first() {
        let path_str = path.to_string_lossy().to_string();

        match event.kind {
            EventKind::Create(_) => Some((EVENT_FILE_CREATED.to_string(), path_str)),
            EventKind::Modify(_) => Some((EVENT_FILE_MODIFIED.to_string(), path_str)),
            EventKind::Remove(_) => Some((EVENT_FILE_DELETED.to_string(), path_str)),
            _ => None,
        }
    } else {
        None
    }
}

fn emit_file_event(window: &WebviewWindow, event: &(String, String)) {
    let (event_type, path) = event;
    let payload = serde_json::json!({
        "path": path,
        "event_type": match event_type.as_str() {
            EVENT_FILE_CREATED => "created",
            EVENT_FILE_MODIFIED => "modified",
            EVENT_FILE_DELETED => "deleted",
            _ => "unknown"
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    emit_event(window, event_type, payload);
}

impl Drop for AsyncFileWatcher {
    fn drop(&mut self) {
        let _ = self.stop_watching();
    }
}

// Gestionnaire global pour un seul watcher
pub struct FileWatcherManager {
    watcher: Arc<Mutex<Option<AsyncFileWatcher>>>,
}

impl FileWatcherManager {
    pub fn new() -> Self {
        Self {
            watcher: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_watching(&self, window: WebviewWindow, paths: Vec<String>) -> AppResult<()> {
        let mut watcher_guard = self.watcher.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock watcher: {}", e)))?;

        match watcher_guard.as_mut() {
            Some(existing_watcher) => {
                // Redémarrer avec les nouveaux chemins
                existing_watcher.restart_watching(paths)
            },
            None => {
                // Créer un nouveau watcher
                let mut new_watcher = AsyncFileWatcher::new(window);
                new_watcher.start_watching(paths)?;
                *watcher_guard = Some(new_watcher);
                Ok(())
            }
        }
    }

    pub fn stop_watching(&self) -> AppResult<()> {
        let mut watcher_guard = self.watcher.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock watcher: {}", e)))?;

        if let Some(watcher) = watcher_guard.as_mut() {
            watcher.stop_watching()?;
        }

        *watcher_guard = None;
        Ok(())
    }

    pub fn restart_watching(&self, window: WebviewWindow, new_paths: Vec<String>) -> AppResult<()> {
        self.start_watching(window, new_paths)
    }

    pub fn get_status(&self) -> serde_json::Value {
        match self.watcher.lock() {
            Ok(watcher_guard) => {
                match watcher_guard.as_ref() {
                    Some(watcher) => {
                        serde_json::json!({
                            "is_watching": watcher.is_watching(),
                            "watched_paths": watcher.get_watched_paths(),
                            "path_count": watcher.get_watched_paths().len()
                        })
                    },
                    None => {
                        serde_json::json!({
                            "is_watching": false,
                            "watched_paths": [],
                            "path_count": 0
                        })
                    }
                }
            },
            Err(_) => {
                serde_json::json!({
                    "is_watching": false,
                    "watched_paths": [],
                    "path_count": 0,
                    "error": "Failed to get watcher status"
                })
            }
        }
    }
}
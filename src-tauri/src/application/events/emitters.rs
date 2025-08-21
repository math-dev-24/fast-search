use tauri::{WebviewWindow, Emitter};

// Constantes pour les événements de scan
pub const EVENT_SCAN_STARTED: &str = "scan_files_started";
pub const EVENT_SCAN_PROGRESS: &str = "scan_files_progress";
pub const EVENT_SCAN_COLLECTED: &str = "scan_files_collected";
pub const EVENT_SCAN_INSERT_PROGRESS: &str = "scan_files_insert_progress";
pub const EVENT_SCAN_FINISHED: &str = "scan_files_finished";
pub const EVENT_SCAN_ERROR: &str = "scan_files_error";

// Constantes pour les événements d'indexation
pub const EVENT_INDEX_STARTED: &str = "index_content_started";
pub const EVENT_INDEX_PROGRESS: &str = "index_content_progress";
pub const EVENT_INDEX_FINISHED: &str = "index_content_finished";
pub const EVENT_INDEX_ERROR: &str = "index_content_error";

// Event watchers
pub const EVENT_WATCHER_STARTED: &str = "watcher_started";
pub const EVENT_WATCHER_STOPPED: &str = "watcher_stopped";
pub const EVENT_WATCHER_ERROR: &str = "watcher_error";
pub const EVENT_FILE_CREATED: &str = "file_created";
pub const EVENT_FILE_MODIFIED: &str = "file_modified";
pub const EVENT_FILE_DELETED: &str = "file_deleted";

// Constantes pour les événements généraux
pub const EVENT_STAT_UPDATED: &str = "stat_updated";

/// Fonction pour émettre des événements avec gestion d'erreur
pub fn emit_event<T: serde::Serialize + Clone>(window: &WebviewWindow, event: &str, payload: T) {
    if let Err(e) = window.emit(event, payload) {
        eprintln!("Erreur lors de l'émission de l'événement {}: {}", event, e);
    }
}


/// Fonction pour émettre des événements d'erreur avec gestion d'erreur
pub fn emit_error_event<T: serde::Serialize + Clone>(window: &WebviewWindow, event: &str, error_message: T) {
    emit_event(window, event, error_message);
}

/// Fonction pour émettre des événements de début avec gestion d'erreur
pub fn emit_started_event(window: &WebviewWindow, event: &str) {
    emit_event(window, event, serde_json::json!({}));
}

/// Fonction pour émettre des événements de fin avec gestion d'erreur
pub fn emit_finished_event<T: serde::Serialize + Clone>(window: &WebviewWindow, event: &str, payload: T) {
    emit_event(window, event, payload);
} 
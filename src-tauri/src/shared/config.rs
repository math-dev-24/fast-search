use std::sync::Arc;
use crate::infrastructure::watcher::file_watcher::FileWatcherManager;

pub struct AppState {
    pub file_watcher_manager: Arc<FileWatcherManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            file_watcher_manager: Arc::new(FileWatcherManager::new()),
        }
    }
}
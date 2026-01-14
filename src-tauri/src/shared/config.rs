use std::sync::Arc;
use std::sync::Mutex;
use crate::infrastructure::watcher::file_watcher::FileWatcherManager;
use crate::domain::services::file_service::FileService;
use crate::infrastructure::repository::sqlite::Db;

pub struct AppState {
    pub file_watcher_manager: Arc<FileWatcherManager>,
    pub service_repository: Arc<Mutex<FileService<Db>>>,
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        let service_repository = crate::application::factories::service_factory::get_service_repository()?;
        
        // Initialiser la base de données
        service_repository.init()
            .map_err(|e| format!("Failed to initialize database: {}", e))?;
        
        Ok(Self {
            file_watcher_manager: Arc::new(FileWatcherManager::new()),
            service_repository: Arc::new(Mutex::new(service_repository)),
        })
    }
}
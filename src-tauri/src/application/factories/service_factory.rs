use crate::infrastructure::repository::sqlite::Db;
use crate::domain::ports::repository::FileRepository;
use crate::domain::services::file_service::FileService;
use std::fs;

fn get_db_path() -> Result<String, String> {
    // Utiliser dirs pour la compatibilité cross-platform
    let data_dir = dirs::data_dir()
        .map(|dir| dir.join("fast-search"))
        .or_else(|| {
            // Fallback vers le répertoire courant si dirs::data_dir() échoue
            std::env::current_dir()
                .ok()
                .map(|dir| dir.join("data"))
        })
        .ok_or_else(|| "Failed to determine data directory".to_string())?;

    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;
    
    let db_path = data_dir.join("fast-search-lite-db.db");
    Ok(db_path.to_string_lossy().to_string())
}


pub fn get_service_repository() -> Result<FileService<Db>, String> {
    let db_path = get_db_path()?;
    let db_adapter = Db::new(&db_path)
        .map_err(|e| format!("Failed to create database: {}", e))?;
    let service_repository = FileService::new(db_adapter);
    Ok(service_repository)
}
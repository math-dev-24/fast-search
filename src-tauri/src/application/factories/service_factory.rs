use crate::infrastructure::repository::sqlite::Db;
use crate::domain::ports::repository::FileRepository;
use crate::domain::services::file_service::FileService;
use std::env;
use std::fs;

fn get_db_path() -> Result<String, String> {
    let data_dir = env::var("APPDATA")
        .map(|appdata| format!("{}\\fast-search", appdata))
        .or_else(|_| env::current_dir()
            .map(|dir| dir.join("data").to_string_lossy().to_string())
            .map_err(|e| e.to_string()))?;

    fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    Ok(format!("{}\\fast-search-lite-db.db", data_dir))
}


pub fn get_service_repository() -> Result<FileService<Db>, String> {
    let db_path = get_db_path()?;
    let db_adapter = Db::new(&db_path)
        .map_err(|e| format!("Failed to create database: {}", e))?;
    let service_repository = FileService::new(db_adapter);
    Ok(service_repository)
}
use crate::domain::entities::file::File;
use crate::domain::services::reader_service::ReaderService;
use crate::shared::errors::{AppError, AppResult};

pub struct ContentIndexerService {
    reader_service: ReaderService,
}



impl ContentIndexerService {
    pub fn new() -> Self {
        Self {
            reader_service: ReaderService::new(),
        }
    }

    pub fn index_file_content(&mut self, file: &File) -> AppResult<String> {
        if !ReaderService::can_read_file(&file) {
            return Err(AppError::NotFound("Impossible de lire le fichier".to_string()));
        }

        self.reader_service.read(file)
    }

    pub fn can_index_file(&self, file: &File) -> bool {
        ReaderService::can_read_file(file)
    }
}

impl Default for ContentIndexerService {
    fn default() -> Self {
        Self::new()
    }
} 
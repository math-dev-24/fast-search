use crate::domain::ports::reader::Reader;
use crate::domain::entities::file::File;
use crate::shared::errors::{AppError, AppResult};
use std::fs;
use std::io::Read;
use std::path::Path;

pub struct TextReader;

impl TextReader {
    pub fn new() -> Self {
        Self
    }
}

impl Reader for TextReader {
    fn read(&self, file: &File) -> AppResult<String> {
        let file_path = Path::new(&file.path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(AppError::NotFound(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", file)));
        }

        let metadata = fs::metadata(file_path)
            .map_err(|e| AppError::FileSystem(e))?;
        
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(AppError::Validation(format!("Fichier trop volumineux: {} bytes", metadata.len())));
        }

        let mut file = fs::File::open(file_path)
            .map_err(|e| AppError::FileSystem(e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::FileSystem(e))?;

        Ok(content)
    }
}

impl Default for TextReader {
    fn default() -> Self {
        Self::new()
    }
} 
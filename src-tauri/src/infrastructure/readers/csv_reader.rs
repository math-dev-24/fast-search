use crate::domain::ports::reader::Reader;
use std::fs;
use std::io::Read;
use std::path::Path;
use crate::domain::entities::file::File;
use crate::shared::errors::{AppError, AppResult};

pub struct CsvReader;

impl CsvReader {
    pub fn new() -> Self {
        Self
    }

    fn parse_csv_content(&self, content: &str) -> String {
        content
            .lines()
            .take(1000) // Limiter à 1000 lignes pour éviter les fichiers trop longs
            .map(|line| {
                // Diviser par virgules et nettoyer chaque champ
                line.split(',')
                    .map(|field| field.trim().replace('"', ""))
                    .filter(|field| !field.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Reader for CsvReader {
    fn read(&self, file: &File) -> AppResult<String> {
        let file_path = Path::new(&file.path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(AppError::NotFound(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", file)));
        }

        // Vérifier la taille du fichier (limite à 10MB pour les fichiers CSV)
        let metadata = fs::metadata(file_path)
            .map_err(|e| AppError::FileSystem(e))?;
        
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(AppError::Validation(format!("Le fichier est trop volumineux: {} bytes", metadata.len())));
        }

        let mut file = fs::File::open(file_path)
            .map_err(|e| AppError::FileSystem(e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::FileSystem(e))?;

        // Parser le contenu CSV
        let parsed_content = self.parse_csv_content(&content);
        
        Ok(parsed_content)
    }
}

impl Default for CsvReader {
    fn default() -> Self {
        Self::new()
    }
} 
use crate::ports::reader::Reader;
use std::fs;
use std::path::Path;

pub struct WordReader;

impl WordReader {
    pub fn new() -> Self {
        Self
    }

    fn extract_text_from_docx(&self, _path: &str) -> Result<String, String> {
        Err("Lecture des fichiers Word non encore implémentée".to_string())
    }
}

impl Reader for WordReader {
    fn read(&self, path: &str) -> Result<String, String> {
        let file_path = Path::new(path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", path));
        }

        // Vérifier la taille du fichier (limite à 20MB pour les fichiers Word)
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Erreur lors de la lecture des métadonnées: {}", e))?;
        
        if metadata.len() > 20 * 1024 * 1024 {
            return Err(format!("Fichier Word trop volumineux: {} bytes", metadata.len()));
        }

        // Extraire le texte du document Word
        self.extract_text_from_docx(path)
    }
}

impl Default for WordReader {
    fn default() -> Self {
        Self::new()
    }
} 
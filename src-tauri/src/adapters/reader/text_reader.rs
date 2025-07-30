use crate::ports::reader::Reader;
use std::fs;
use std::io::Read;
use std::path::Path;
use crate::entities::file::File;

pub struct TextReader;

impl TextReader {
    pub fn new() -> Self {
        Self
    }
}

impl Reader for TextReader {
    fn read(&self, file: &File) -> Result<String, String> {
        let file_path = Path::new(&file.path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", file));
        }

        // Vérifier la taille du fichier (limite à 10MB pour les fichiers texte)
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Erreur lors de la lecture des métadonnées: {}", e))?;
        
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(format!("Fichier trop volumineux: {} bytes", metadata.len()));
        }

        let mut file = fs::File::open(file_path)
            .map_err(|e| format!("Erreur lors de l'ouverture du fichier: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Erreur lors de la lecture du fichier: {}", e))?;

        Ok(content)
    }
}

impl Default for TextReader {
    fn default() -> Self {
        Self::new()
    }
} 
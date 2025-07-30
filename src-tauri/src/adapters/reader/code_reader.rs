use crate::ports::reader::Reader;
use std::fs;
use std::io::Read;
use std::path::Path;
use crate::entities::file::File;

pub struct CodeReader;

impl CodeReader {
    pub fn new() -> Self {
        Self
    }

    fn clean_code_content(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| {
                !line.is_empty() && 
                !line.starts_with("//") && 
                !line.starts_with("#") && 
                !line.starts_with("/*") &&
                !line.starts_with("*") &&
                !line.starts_with("*/") &&
                !line.starts_with("<!--") &&
                !line.starts_with("-->")
            })
            .take(2000) // Limiter à 2000 lignes pour éviter les fichiers trop longs
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Reader for CodeReader {
    fn read(&self, file: &File) -> Result<String, String> {
        let file_path = Path::new(&file.path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", file));
        }

        // Vérifier la taille du fichier (limite à 5MB pour les fichiers de code)
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Erreur lors de la lecture des métadonnées: {}", e))?;
        
        if metadata.len() > 5 * 1024 * 1024 {
            return Err(format!("Fichier trop volumineux: {} bytes", metadata.len()));
        }

        let mut file = fs::File::open(file_path)
            .map_err(|e| format!("Erreur lors de l'ouverture du fichier: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Erreur lors de la lecture du fichier: {}", e))?;

        // Nettoyer le contenu en supprimant les commentaires et lignes vides
        let cleaned_content = self.clean_code_content(&content);
        
        Ok(cleaned_content)
    }
}

impl Default for CodeReader {
    fn default() -> Self {
        Self::new()
    }
} 
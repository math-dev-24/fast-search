use crate::ports::reader::Reader;
use std::fs;
use std::path::Path;
use lopdf::Document;

pub struct PdfReader;

impl PdfReader {
    pub fn new() -> Self {
        Self
    }

    fn extract_text_from_pdf(&self, path: &str) -> Result<String, String> {
        let doc = Document::load(path)
            .map_err(|e| format!("Erreur lors du chargement du document PDF: {}", e))?;
        
        let mut text_content = String::new();
        
        // Extraire le texte de chaque page
        for page_id in doc.get_pages().keys() {
            if let Ok(text) = doc.extract_text(&[*page_id]) {
                text_content.push_str(&text);
                text_content.push(' ');
            }
        }
        
        // Nettoyer et limiter le contenu
        let cleaned_content = text_content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .take(5000) // Limiter à 5000 lignes
            .collect::<Vec<_>>()
            .join(" ");
        
        // Limiter la taille totale
        if cleaned_content.len() > 50000 {
            Ok(cleaned_content.chars().take(50000).collect())
        } else {
            Ok(cleaned_content)
        }
    }
}

impl Reader for PdfReader {
    fn read(&self, path: &str) -> Result<String, String> {
        let file_path = Path::new(path);
        
        if !file_path.exists() || !file_path.is_file() {
            return Err(format!("Le fichier n'existe pas ou n'est pas un fichier: {}", path));
        }

        // Vérifier la taille du fichier (limite à 50MB pour les fichiers PDF)
        let metadata = fs::metadata(file_path)
            .map_err(|e| format!("Erreur lors de la lecture des métadonnées: {}", e))?;
        
        if metadata.len() > 50 * 1024 * 1024 {
            return Err(format!("Fichier PDF trop volumineux: {} bytes", metadata.len()));
        }

        // Extraire le texte du PDF
        self.extract_text_from_pdf(path)
    }
}

impl Default for PdfReader {
    fn default() -> Self {
        Self::new()
    }
} 
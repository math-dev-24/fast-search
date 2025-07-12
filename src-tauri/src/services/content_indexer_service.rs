use crate::entities::file::File;
use crate::services::reader_service::ReaderService;

pub struct ContentIndexerService {
    reader_service: ReaderService,
}

impl ContentIndexerService {
    pub fn new() -> Self {
        Self {
            reader_service: ReaderService::new(),
        }
    }

    pub fn index_file_content(&mut self, file: &File) -> Result<String, String> {
        let file_path_str = file.path.to_string_lossy();
        
        // Vérifier si le fichier peut être lu
        if !ReaderService::can_read_file(&file_path_str) {
            return Err(format!("Type de fichier non supporté: {}", file.path.display()));
        }

        // Lire le contenu du fichier en utilisant le reader service
        self.reader_service.read(file)
    }

    pub fn can_index_file(&self, file_path: &str) -> bool {
        ReaderService::can_read_file(file_path)
    }

    pub fn get_supported_extensions(&self) -> Vec<String> {
        vec![
            // Fichiers de code
            "js".to_string(), "ts".to_string(), "jsx".to_string(), "tsx".to_string(),
            "py".to_string(), "java".to_string(), "cpp".to_string(), "c".to_string(),
            "h".to_string(), "hpp".to_string(), "rs".to_string(), "go".to_string(),
            "php".to_string(), "rb".to_string(), "pl".to_string(), "sh".to_string(),
            "sql".to_string(), "html".to_string(), "htm".to_string(), "css".to_string(),
            "xml".to_string(), "yaml".to_string(), "yml".to_string(), "toml".to_string(),
            "ini".to_string(), "cfg".to_string(), "conf".to_string(),
            
            // Fichiers CSV
            "csv".to_string(), "tsv".to_string(),
            
            // Fichiers PDF
            "pdf".to_string(),
            
            // Fichiers Word
            "docx".to_string(), "doc".to_string(),
            
            // Fichiers texte simples
            "txt".to_string(), "md".to_string(), "json".to_string(), "log".to_string(),
        ]
    }
}

impl Default for ContentIndexerService {
    fn default() -> Self {
        Self::new()
    }
} 
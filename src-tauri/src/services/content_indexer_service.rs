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
        
        if !ReaderService::can_read_file(&file_path_str) {
            return Err(format!("Type de fichier non supporté: {}", file.path.display()));
        }

        self.reader_service.read(file)
    }

    pub fn can_index_file(&self, file_path: &str) -> bool {
        ReaderService::can_read_file(file_path)
    }
}

impl Default for ContentIndexerService {
    fn default() -> Self {
        Self::new()
    }
} 
use crate::domain::ports::reader::Reader;
use crate::infrastructure::readers::{TextReader, CodeReader, CsvReader, PdfReader, WordReader};
use crate::domain::entities::file::File;
use std::path::Path;

pub struct ReaderService {
    reader: Box<dyn Reader>,
}

impl ReaderService {
    pub fn new() -> Self {
        Self {
            reader: Box::new(TextReader::new())
        }
    }

    pub fn read(&mut self, file: &File) -> Result<String, String> {
        // Sélectionner le reader approprié basé sur l'extension du fichier
        self.reader = Self::get_reader_for_file(&file);
        
        self.reader.read(&file)
    }

    fn get_reader_for_file(file: &File) -> Box<dyn Reader> {
        let path = Path::new(&file.path);
        
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            
            match ext_str.as_str() {
                // Fichiers de code
                "js" | "ts" | "jsx" | "tsx" | "py" | "java" | "cpp" | "c" | "h" | "hpp" | 
                "rs" | "go" | "php" | "rb" | "pl" | "sh" | "sql" | "html" | "htm" | "css" |
                "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" => {
                    Box::new(CodeReader::new())
                },
                
                // Fichiers CSV
                "csv" | "tsv" => {
                    Box::new(CsvReader::new())
                },
                
                // Fichiers PDF
                "pdf" => {
                    Box::new(PdfReader::new())
                },
                
                // Fichiers Word
                "docx" | "doc" => {
                    Box::new(WordReader::new())
                },
                
                // Fichiers texte simples
                "txt" | "md" | "json" | "log" => {
                    Box::new(TextReader::new())
                },
                _ => {
                    Box::new(TextReader::new())
                }
            }
        } else {
            Box::new(TextReader::new())
        }
    }

    pub fn can_read_file(file: &File) -> bool {
        let path = Path::new(&file.path);
        
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            
            let supported_extensions = [
                "js", "ts", "jsx", "tsx", "py", "java", "cpp", "c", "h", "hpp", 
                "rs", "go", "php", "rb", "pl", "sh", "sql", "html", "htm", "css",
                "xml", "yaml", "yml", "toml", "ini", "cfg", "conf",
                "csv", "tsv",
                "pdf",
                "docx", "doc",
                "txt", "md", "json", "log"
            ];
            
            supported_extensions.contains(&ext_str.as_str())
        } else {
            false
        }
    }
}

impl Default for ReaderService {
    fn default() -> Self {
        Self::new()
    }
}
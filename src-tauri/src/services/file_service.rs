use crate::ports::repository::FileRepository;
use crate::entities::file::File;
use crate::entities::stat::Stat;

pub struct FileService<T: FileRepository> {
    repository: T,
}


impl<T: FileRepository> FileService<T> {

    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn init(&self) -> Result<(), String> {
        self.repository.init().map_err(|e| e.to_string())
    }

    pub fn search(&self, query: &str, file_types: &[String], is_dir: bool, folders: &[String], size_limit: &[usize], date_range: &[usize], date_mode: &str) -> Result<Vec<File>, String> {
        self.repository.search(query, file_types, is_dir, folders, size_limit, date_range, date_mode).map_err(|e| e.to_string())
    }

    pub fn get_stat(&self) -> Result<Stat, String> {
        self.repository.get_stat().map_err(|e| e.to_string())
    }

    pub fn get_all_types(&self) -> Result<Vec<String>, String> {
        self.repository.get_all_types().map_err(|e| e.to_string())
    }

    pub fn get_all_paths(&self) -> Result<Vec<String>, String> {
        self.repository.get_all_paths().map_err(|e| e.to_string())
    }

    pub fn insert(&mut self, files: Vec<File>) -> Result<(), String> {
        self.repository.insert(files).map_err(|e| e.to_string())
    }

    pub fn reset_data(&self) -> Result<(), String> {
        self.repository.reset_data().map_err(|e| e.to_string())
    }

    pub fn get_all_folders(&self) -> Result<Vec<String>, String> {
        self.repository.get_all_folders().map_err(|e| e.to_string())
    }

    pub fn insert_paths(&mut self, paths: Vec<String>) -> Result<(), String> {
        self.repository.insert_paths(paths).map_err(|e| e.to_string())
    }

}
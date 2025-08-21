use crate::domain::ports::repository::FileRepository;
use crate::domain::entities::file::File;
use crate::domain::entities::stat::Stat;
use crate::domain::entities::search::SearchQuery;

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

    pub fn search(&self, query: &SearchQuery) -> Result<Vec<File>, String> {
        self.repository.search(query).map_err(|e| e.to_string())
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

    pub fn insert_paths(&mut self, paths: Vec<String>) -> Result<Vec<String>, String> {
        self.repository.insert_paths(paths).map_err(|e| e.to_string())
    }

    pub fn get_uncontent_indexed_files(&self) -> Result<Vec<File>, String> {
        self.repository.get_uncontent_indexed_files().map_err(|e| e.to_string())
    }

    pub fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> Result<(), String> {
        self.repository.update_file_index_status(file, content_hash, is_indexable).map_err(|e| e.to_string())
    }

}
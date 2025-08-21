use crate::domain::entities::file::File;
use crate::domain::entities::stat::Stat;
use crate::domain::entities::search::SearchQuery;
use crate::shared::errors::AppResult;

pub trait FileRepository {
    fn new(path: &str) -> AppResult<Self> where Self: Sized;
    fn init(&self) -> AppResult<()>;
    fn insert(&mut self, files: Vec<File>) -> AppResult<()>;
    fn insert_paths(&mut self, paths: Vec<String>) -> AppResult<Vec<String>>;
    fn get_stat(&self) -> AppResult<Stat>;
    fn get_all_types(&self) -> AppResult<Vec<String>>;
    fn get_all_paths(&self) -> AppResult<Vec<String>>;
    fn get_all_folders(&self) -> AppResult<Vec<String>>;
    fn search(&self, query: &SearchQuery) -> AppResult<Vec<File>>;
    fn reset_data(&self) -> AppResult<()>;
    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> AppResult<()>;
    fn get_uncontent_indexed_files(&self) -> AppResult<Vec<File>>;
}
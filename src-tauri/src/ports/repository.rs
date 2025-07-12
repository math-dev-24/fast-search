use crate::entities::file::File;
use crate::entities::stat::Stat;
use rusqlite::{Result as SqliteResult};

pub trait FileRepository {
    fn new(path: &str) -> SqliteResult<Self> where Self: Sized;
    fn init(&self) -> SqliteResult<()>;
    fn insert(&mut self, files: Vec<File>) -> SqliteResult<()>;
    fn insert_paths(&mut self, paths: Vec<String>) -> SqliteResult<()>;
    fn get_stat(&self) -> SqliteResult<Stat>;
    fn get_all_types(&self) -> SqliteResult<Vec<String>>;
    fn get_all_paths(&self) -> SqliteResult<Vec<String>>;
    fn get_all_folders(&self) -> SqliteResult<Vec<String>>;
    fn search(&self, query: &str, file_types: &[String], is_dir: bool, folders: &[String], size_limit: &[usize], date_range: &[usize], date_mode: &str, in_content: bool) -> SqliteResult<Vec<File>>;
    fn reset_data(&self) -> SqliteResult<()>;
    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> SqliteResult<()>;
    fn get_uncontent_indexed_files(&self) -> SqliteResult<Vec<File>>;
}
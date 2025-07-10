use crate::entities::file::File;
use crate::entities::stat::Stat;
use rusqlite::{Result as SqliteResult};

pub trait FileRepository {
    fn new(path: &str) -> SqliteResult<Self> where Self: Sized;
    fn init(&self) -> SqliteResult<()>;
    fn insert(&mut self, files: Vec<File>) -> SqliteResult<()>;
    fn get_stat(&self) -> SqliteResult<Stat>;
    fn get_type_files(&self) -> SqliteResult<Vec<String>>;
    fn get_all_folders(&self) -> SqliteResult<Vec<String>>;
    fn search(&self, query: &str, file_types: &[String], is_dir: bool, folders: &[String], size_limit: &[usize], date_range: &[usize], date_mode: &str) -> SqliteResult<Vec<File>>;
    fn reset_data(&self) -> SqliteResult<()>;
}
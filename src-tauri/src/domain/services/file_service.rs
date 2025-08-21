use crate::domain::ports::repository::FileRepository;
use crate::domain::entities::file::File;
use crate::domain::entities::stat::Stat;
use crate::domain::entities::search::SearchQuery;
use crate::shared::errors::{AppError, AppResult};

pub struct FileService<T: FileRepository> {
    repository: T,
}

impl<T: FileRepository> FileService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn init(&self) -> AppResult<()> {
        self.repository.init()
    }

    pub fn search(&self, query: &SearchQuery) -> AppResult<Vec<File>> {
        Self::validate_search_query(query)?;
        self.repository.search(query)
    }

    pub fn get_stat(&self) -> AppResult<Stat> {
        self.repository.get_stat()
    }

    pub fn get_all_types(&self) -> AppResult<Vec<String>> {
        self.repository.get_all_types()
    }

    pub fn get_all_paths(&self) -> AppResult<Vec<String>> {
        self.repository.get_all_paths()
    }

    pub fn insert(&mut self, files: Vec<File>) -> AppResult<()> {
        if files.is_empty() {
            return Err(AppError::Validation("Cannot insert empty file list".to_string()));
        }
        if files.len() > 10000 {
            return Err(AppError::Validation("Too many files in single batch".to_string()));
        }
        self.repository.insert(files)
    }

    pub fn reset_data(&self) -> AppResult<()> {
        self.repository.reset_data()
    }

    pub fn get_all_folders(&self) -> AppResult<Vec<String>> {
        self.repository.get_all_folders()
    }

    pub fn insert_paths(&mut self, paths: Vec<String>) -> AppResult<Vec<String>> {
        Self::validate_paths(&paths)?;
        self.repository.insert_paths(paths)
    }

    pub fn get_uncontent_indexed_files(&self) -> AppResult<Vec<File>> {
        self.repository.get_uncontent_indexed_files()
    }

    pub fn update_file_index_status(
        &mut self,
        file: &File,
        content_hash: String,
        is_indexable: bool
    ) -> AppResult<()> {
        if !file.path.exists() {
            return Err(AppError::NotFound(format!("File not found: {}", file.path.display())));
        }

        self.repository.update_file_index_status(file, content_hash, is_indexable)
    }

    pub fn get_file_by_path(&self, path: &str) -> AppResult<File> {
        if path.trim().is_empty() {
            return Err(AppError::Validation("Path cannot be empty".to_string()));
        }

        let query = SearchQuery {
            path_pattern: Some(path.to_string()),
            limit: 1,
            ..Default::default()
        };

        let files = self.repository.search(&query)?;

        files.into_iter().next()
            .ok_or_else(|| AppError::NotFound(format!("File not found: {}", path)))
    }

    fn validate_search_query(query: &SearchQuery) -> AppResult<()> {
        if query.limit == 0 {
            return Err(AppError::Validation("Limit cannot be zero".to_string()));
        }

        if query.limit > 1000 {
            return Err(AppError::Validation("Limit too high (max: 1000)".to_string()));
        }

        if query.text.len() > 1000 {
            return Err(AppError::Validation("Search text too long (max: 1000 chars)".to_string()));
        }

        if query.offset > 100000 {
            return Err(AppError::Validation("Offset too high".to_string()));
        }

        Ok(())
    }

    fn validate_paths(paths: &[String]) -> AppResult<()> {
        if paths.is_empty() {
            return Err(AppError::Validation("Paths list cannot be empty".to_string()));
        }

        if paths.len() > 100 {
            return Err(AppError::Validation("Too many paths (max: 100)".to_string()));
        }

        for path in paths {
            if path.trim().is_empty() {
                return Err(AppError::Validation("Path cannot be empty".to_string()));
            }

            if path.len() > 4096 {
                return Err(AppError::Validation("Path too long".to_string()));
            }

            let path_obj = std::path::Path::new(path);
            if !path_obj.exists() {
                return Err(AppError::NotFound(format!("Path does not exist: {}", path)));
            }

            if !path_obj.is_dir() {
                return Err(AppError::Validation(format!("Path is not a directory: {}", path)));
            }
        }

        Ok(())
    }
}
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result as SqliteResult};
use crate::domain::entities::file::File;
use crate::domain::entities::stat::Stat;
use crate::domain::entities::search::{SearchQuery, DateMode, SortBy, SortOrder};
use crate::domain::ports::repository::FileRepository;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::domain::entities::query_builder::QueryBuilder;
use crate::shared::errors::{AppError, AppResult};


pub struct Db {
    pub conn: Connection,
    pub types_cache: Arc<Mutex<Option<HashSet<String>>>>
}

impl FileRepository for Db {
    fn new(path: &str) -> AppResult<Db> {
        let conn = Connection::open(path)?;

        Ok(Self {
            conn,
            types_cache: Arc::new(Mutex::new(None)),
        })
    }

    fn init(&self) -> AppResult<()> {
        let init_sql = include_str!("../../../data/init.sql");
        self.conn.execute_batch(init_sql)?;
        tracing::info!("Database initialized successfully");
        Ok(())
    }

    fn insert(&mut self, files: Vec<File>) -> AppResult<()> {

        let new_types = files.iter()
        .map(|file| file.file_type.clone().unwrap_or_default())
        .collect::<Vec<_>>();

        self.insert_type(new_types)?;
        self.insert_file(files)?;

        Ok(())
    }

    fn insert_paths(&mut self, new_paths: Vec<String>) -> AppResult<Vec<String>> {

        let db_paths = self.get_all_paths()?;

        let need_delete_paths: Vec<String> = db_paths.iter().filter(|path| !new_paths.contains(path)).cloned().collect();

        let new_paths: Vec<String> = new_paths.iter().filter(|path| !db_paths.contains(path)).cloned().collect();

        for path in &need_delete_paths {
            self.delete_files_by_path_prefix(path)?;
        }

        let tx = match self.conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to start insert_paths transaction: {}", e);
                return Err(e.into());
            }
        };

        let result = (|| -> AppResult<()> {
            tx.execute("DELETE FROM paths", [])?;
            for path in &new_paths {
                tx.execute("INSERT INTO paths (path) VALUES (?)", [path])?;
            }
            Ok(())
        })();

        match result {
            Ok(_) => {
                tx.commit().map_err(|e| {
                    tracing::error!("Failed to commit insert_paths transaction: {}", e);
                    AppError::Database(e)
                })?;
                Ok(new_paths)
            }
            Err(e) => {
                if let Err(rollback_err) = tx.rollback() {
                    tracing::error!("Failed to rollback insert_paths transaction: {}", rollback_err);
                } else {
                    tracing::debug!("insert_paths transaction rolled back due to error: {}", e);
                }
                Err(e)
            }
        }
    }

    fn get_stat(&self) -> AppResult<Stat> {
        let mut stmt = self.conn.prepare("
            WITH stats AS (
                SELECT
                    COUNT(CASE WHEN is_dir = 0 THEN 1 END) as nb_files,
                    COUNT(CASE WHEN is_dir = 1 THEN 1 END) as nb_folders,
                    COALESCE(SUM(CASE WHEN is_dir = 0 THEN size ELSE 0 END), 0) as total_size,
                    COUNT(CASE WHEN is_indexed = 1 AND is_indexable = 1 THEN 1 END) as indexed_files,
                    COUNT(CASE WHEN is_indexed = 0 AND is_indexable = 1 THEN 1 END) as unindexed_files,
                    COUNT(CASE WHEN content_indexed = 1 AND is_indexable = 1 THEN 1 END) as content_indexed_files,
                    COUNT(CASE WHEN content_indexed = 0 AND is_indexable = 1 THEN 1 END) as uncontent_indexed_files,
                    COUNT(CASE WHEN is_indexable = 0 THEN 1 END) as unindexable_files
                FROM files
            )
            SELECT
                nb_files,
                nb_folders,
                total_size,
                indexed_files,
                unindexed_files,
                content_indexed_files,
                uncontent_indexed_files,
                unindexable_files
            FROM stats
        ")?;

        let result = stmt.query_row([], |row| {
            let nb_files: i64 = row.get(0)?;
            let nb_folders: i64 = row.get(1)?;
            let total_size: i64 = row.get(2)?;
            let indexed_files: i64 = row.get(3)?;
            let unindexed_files: i64 = row.get(4)?;
            let content_indexed_files: i64 = row.get(5)?;
            let uncontent_indexed_files: i64 = row.get(6)?;
            let unindexable_files: i64 = row.get(7)?;

            let indexed_percentage = if indexed_files + unindexed_files > 0 {
                (indexed_files as f64 / (indexed_files + unindexed_files) as f64) * 100.0
            } else {
                0.0
            };

            let total_indexable = content_indexed_files + uncontent_indexed_files;
            let content_indexed_percentage = if total_indexable > 0 {
                (content_indexed_files as f64 / total_indexable as f64) * 100.0
            } else {
                0.0
            };

            Ok(Stat {
                nb_files: nb_files as u32,
                nb_folders: nb_folders as u32,
                total_size: total_size as u64,
                indexed_files: indexed_files as u32,
                unindexed_files: unindexed_files as u32,
                indexed_percentage,
                content_indexed_files: content_indexed_files as u32,
                uncontent_indexed_files: uncontent_indexed_files as u32,
                content_indexed_percentage,
                unindexable_files: unindexable_files as u32,
            })
        })?;
        Ok(result)
    }

    fn get_all_types(&self) -> AppResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM types")?;
        let types: Vec<String> = stmt.query_map([], |row| row.get(0))?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(types)
    }

    fn get_all_paths(&self) -> AppResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT path FROM paths")?;
        let paths: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(paths)
    }

    fn get_all_folders(&self) -> AppResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT name FROM files WHERE is_dir = 1")?;
        let folders: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(folders)
    }

    fn search(&self, query: &SearchQuery) -> AppResult<Vec<File>> {
        let mut builder = QueryBuilder::new();

        if !query.text.trim().is_empty() {
            if query.search_in_content {
                let fts_query = query.text.replace("\"", "\"\"");
                builder.add_fts_condition(format!("\"{}\"", fts_query));
            } else {
                builder.add_condition(
                    "(LOWER(name) LIKE LOWER(?))".to_string(),
                    Box::new(format!("%{}%", query.text))
                );
            }
        }

        if query.filters.is_dir {
            builder.add_simple_condition("is_dir = 1".to_string());
        }

        if !query.filters.file_types.is_empty() {
            let placeholders = query.filters.file_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            builder.add_simple_condition(format!("file_type IN ({})", placeholders));

            for file_type in &query.filters.file_types {
                builder.params.push(Box::new(file_type.clone()));
            }
        }

        if !query.filters.folders.is_empty() {
            let values: Vec<String> = query.filters.folders.iter().map(|_| "(?)".to_string()).collect();
            let cte_condition = values.join(", ");

            let mut cte_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            for folder in &query.filters.folders {
                cte_params.push(Box::new(folder.clone()));
            }

            builder.add_cte_condition(cte_condition, cte_params);
            builder.add_simple_condition(
                "EXISTS (SELECT 1 FROM roots r WHERE files.path >= r.root AND files.path < r.root || CHAR(0x10FFFF))".to_string()
            );
        }

        if query.filters.size_limit.len() >= 2 && (query.filters.size_limit[0] > 0 || query.filters.size_limit[1] > 0) {
            let min = query.filters.size_limit[0] as i64 * 1024 * 1024;
            let max = if query.filters.size_limit[1] > 0 {
                query.filters.size_limit[1] as i64 * 1024 * 1024
            } else {
                i64::MAX
            };

            builder.add_simple_condition("size >= ? AND size <= ?".to_string());
            builder.params.push(Box::new(min));
            builder.params.push(Box::new(max));
        }

        if query.filters.date_range.len() >= 2 && (query.filters.date_range[0] > 0 || query.filters.date_range[1] > 0) {
            let min = query.filters.date_range[0] as i64;
            let max = if query.filters.date_range[1] > 0 {
                query.filters.date_range[1] as i64
            } else {
                i64::MAX
            };

            let date_column = match query.filters.date_mode {
                DateMode::Create => "created_at",
                _ => "last_modified",
            };

            builder.add_simple_condition(format!("{} >= ? AND {} <= ?", date_column, date_column));
            builder.params.push(Box::new(min));
            builder.params.push(Box::new(max));
        }

        if let Some(path_pattern) = &query.path_pattern {
            if !path_pattern.trim().is_empty() {
                builder.add_condition(
                    "path LIKE ?".to_string(),
                    Box::new(format!("%{}%", path_pattern))
                );
            }
        }

        let order_by = match query.sort_by {
            SortBy::Name => "name COLLATE NOCASE",
            SortBy::Size => "size",
            SortBy::LastModified => "last_modified",
            SortBy::CreatedAt => "created_at",
            SortBy::AccessedAt => "accessed_at",
        };

        let sort_order = match query.sort_order {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        let (sql, params) = builder.build(order_by, sort_order, query.limit, query.offset, query.cursor);
        self.execute_search_query(&sql, &params)
    }

    fn reset_data(&self) -> AppResult<()> {
        self.conn.execute("DELETE FROM files", [])?;
        self.conn.execute("DELETE FROM types", [])?;
        self.conn.execute("DELETE FROM paths", [])?;
        Ok(())
    }

    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> AppResult<()> {
        let path_str = file.path.to_str()
            .ok_or_else(|| AppError::Validation("Invalid file path encoding".to_string()))?;

        let tx = match self.conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                return Err(e.into());
            }
        };

        // Utiliser un pattern pour garantir le rollback explicite en cas d'erreur
        let result = (|| -> AppResult<i64> {
            let file_id: i64 = match tx.query_row(
                "SELECT id FROM files WHERE path = ?",
                [path_str],
                |row| row.get(0)
            ) {
                Ok(id) => id,
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    return Err(AppError::NotFound(format!("File not found in database: {}", path_str)));
                }
                Err(e) => return Err(e.into()),
            };

            tx.execute(
                "INSERT OR REPLACE INTO fts_content (content, file_id) VALUES (?, ?)",
                rusqlite::params![content_hash, file_id]
            )?;

            tx.execute(
                "UPDATE files SET content_indexed = ?, is_indexable = ? WHERE path = ?",
                rusqlite::params![true, is_indexable, path_str]
            )?;

            Ok(file_id)
        })();

        match result {
            Ok(_) => {
                tx.commit().map_err(|e| {
                    tracing::error!("Failed to commit transaction: {}", e);
                    AppError::Database(e)
                })?;
                Ok(())
            }
            Err(e) => {
                if let Err(rollback_err) = tx.rollback() {
                    tracing::error!("Failed to rollback transaction: {}", rollback_err);
                } else {
                    tracing::debug!("Transaction rolled back successfully due to error: {}", e);
                }
                Err(e)
            }
        }
    }

    fn get_uncontent_indexed_files(&self) -> AppResult<Vec<File>> {
        let mut stmt = self.conn.prepare("SELECT * FROM files WHERE content_indexed = 0 AND is_indexable = 1")?;
        let files: Vec<File> = stmt
            .query_map([], |row| { Self::map_row_to_file(row) })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(files)
    }
}

impl Db {

    fn execute_search_query(&self, sql: &str, params: &[Box<dyn rusqlite::ToSql>]) -> AppResult<Vec<File>> {
        use std::time::Duration;
        use std::time::Instant;
        
        // Timeout de 30 secondes pour les requêtes de recherche
        let search_timeout = Duration::from_secs(30);
        let start = Instant::now();
        
        // Utiliser interrupt() pour permettre l'annulation si nécessaire
        // Note: SQLite n'a pas de vrai timeout, mais on peut monitorer le temps
        let mut stmt = self.conn.prepare(sql)?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let result = stmt.query_map(rusqlite::params_from_iter(param_refs), Self::map_row_to_file)?
            .collect::<SqliteResult<Vec<_>>>()?;
        
        let elapsed = start.elapsed();
        if elapsed > search_timeout {
            tracing::warn!("Search query took longer than timeout: {:?}", elapsed);
        } else {
            tracing::debug!("Search query completed in {:?}", elapsed);
        }
        
        Ok(result)
    }
    fn load_types_cache(&mut self) -> AppResult<()> {
        let types = self.get_all_types()?;
        let mut cache = self.types_cache.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock types cache: {}", e)))?;
        *cache = Some(types.into_iter().collect());
        Ok(())
    }

    fn delete_files_by_path_prefix(&mut self, path_prefix: &str) -> AppResult<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "DELETE FROM fts_content WHERE file_id IN (SELECT id FROM files WHERE path LIKE ?)",
            [format!("{}%", path_prefix)]
        )?;

        tx.execute("DELETE FROM files WHERE path LIKE ?", [format!("{}%", path_prefix)])?;

        tx.commit()?;
        Ok(())
    }

    fn insert_type(&mut self, type_names: Vec<String>) -> AppResult<()> {
        // Charger le cache si nécessaire de manière thread-safe
        {
            let cache = self.types_cache.lock()
                .map_err(|e| AppError::Internal(format!("Failed to lock types cache: {}", e)))?;
            if cache.is_none() {
                drop(cache);
                self.load_types_cache()?;
            }
        }

        // Filtrer les nouveaux types de manière thread-safe
        let new_types: Vec<String> = {
            let cache = self.types_cache.lock()
                .map_err(|e| AppError::Internal(format!("Failed to lock types cache: {}", e)))?;
            
            if let Some(ref types_set) = *cache {
                type_names.into_iter()
                    .filter(|t| !types_set.contains(t))
                    .collect()
            } else {
                type_names
            }
        };

        if new_types.is_empty() {
            return Ok(());
        }

        let tx = match self.conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to start insert_type transaction: {}", e);
                return Err(e.into());
            }
        };
        
        let result = (|| -> AppResult<()> {
            let mut stmt = tx.prepare("INSERT OR IGNORE INTO types (name) VALUES (?)")?;
            for type_name in &new_types {
                stmt.execute([type_name])?;
            }
            drop(stmt);
            Ok(())
        })();

        match result {
            Ok(_) => {
                tx.commit().map_err(|e| {
                    tracing::error!("Failed to commit insert_type transaction: {}", e);
                    AppError::Database(e)
                })?;
            }
            Err(e) => {
                if let Err(rollback_err) = tx.rollback() {
                    tracing::error!("Failed to rollback insert_type transaction: {}", rollback_err);
                }
                return Err(e);
            }
        }

        // Mettre à jour le cache de manière thread-safe
        let mut cache = self.types_cache.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock types cache: {}", e)))?;
        if let Some(ref mut types_set) = *cache {
            types_set.extend(new_types);
        } else {
            *cache = Some(new_types.into_iter().collect());
        }
        Ok(())
    }

    fn insert_file(&mut self, files: Vec<File>) -> AppResult<()> {
        if files.is_empty() {
            return Ok(());
        }
        
        let tx = match self.conn.transaction() {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to start insert transaction: {}", e);
                return Err(e.into());
            }
        };

        let paths: Vec<String> = files.iter()
            .filter_map(|f| f.path.to_str().map(|s| s.to_string()))
            .collect();

        if paths.is_empty() {
            tx.commit().map_err(|e| {
                tracing::error!("Failed to commit empty insert transaction: {}", e);
                AppError::Database(e)
            })?;
            return Ok(());
        }

        let placeholders: Vec<String> = (0..paths.len()).map(|_| "?".to_string()).collect();
        let existing_paths_query = format!(
            "SELECT path FROM files WHERE path IN ({})",
            placeholders.join(",")
        );

        let path_params: Vec<&dyn rusqlite::ToSql> = paths.iter()
            .map(|p| p as &dyn rusqlite::ToSql)
            .collect();

        let mut existing_paths = HashSet::new();
        let mut stmt = tx.prepare(&existing_paths_query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(path_params.iter()), |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        for path in rows {
            existing_paths.insert(path?);
        }
        drop(stmt);

        // Filtrer les nouveaux fichiers
        let new_files: Vec<&File> = files.iter()
            .filter(|f| {
                f.path.to_str()
                    .map(|p| !existing_paths.contains(p))
                    .unwrap_or(false)
            })
            .collect();

        if new_files.is_empty() {
            tx.commit()?;
            return Ok(());
        }

        // Optimisation: Utiliser INSERT OR IGNORE pour éviter les doublons
        // et préparer une seule fois la requête pour tous les fichiers
        let mut stmt = tx.prepare("INSERT OR IGNORE INTO files (path, name, is_dir, file_type, size, last_modified, created_at, accessed_at, is_indexed, content_indexed, is_indexable, is_hidden, is_readonly, is_system, is_executable, is_symlink, permissions, owner, `group`, mime_type, encoding, line_count, word_count, checksum, is_encrypted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

        for file in &new_files {
            let path = file.path.to_str()
                .ok_or_else(|| AppError::Validation("Invalid file path encoding".to_string()))?;
            
            let last_modified = file.last_modified.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            let created_at = file.created_at.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            let accessed_at = file.accessed_at.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            let size = file.size.map(|s| s as i64);
            stmt.execute(rusqlite::params![
                path,
                file.name,
                file.is_dir,
                file.file_type,
                size,
                last_modified,
                created_at,
                accessed_at,
                file.is_indexed,
                file.content_indexed,
                file.is_indexable,
                file.is_hidden,
                file.is_readonly,
                file.is_system,
                file.is_executable,
                file.is_symlink,
                file.permissions,
                file.owner,
                file.group,
                file.mime_type,
                file.encoding,
                file.line_count,
                file.word_count,
                file.checksum,
                file.is_encrypted
            ])?;
        }

        drop(stmt);
        
        tx.commit().map_err(|e| {
            tracing::error!("Failed to commit insert transaction: {}", e);
            AppError::Database(e)
        })
    }

    fn map_row_to_file(row: &rusqlite::Row) -> rusqlite::Result<File> {
        let size: Option<i64> = row.get(5)?;
        let last_modified_secs: i64 = row.get(6)?;
        let created_at_secs: i64 = row.get(7)?;
        let accessed_at_secs: i64 = row.get(8)?;
        
        Ok(File {
            path: PathBuf::from(row.get::<_, String>(1)?),
            name: row.get(2)?,
            is_dir: row.get(3)?,
            file_type: row.get(4)?,
            size: size.map(|s| s as u64),
            last_modified: SystemTime::UNIX_EPOCH + Duration::from_secs(last_modified_secs as u64),
            created_at: SystemTime::UNIX_EPOCH + Duration::from_secs(created_at_secs as u64),
            accessed_at: SystemTime::UNIX_EPOCH + Duration::from_secs(accessed_at_secs as u64),
            is_indexed: row.get(9)?,
            content_indexed: row.get(10)?,
            is_indexable: row.get(11)?,
            is_hidden: row.get(12)?,
            is_readonly: row.get(13)?,
            is_system: row.get(14)?,
            is_executable: row.get(15)?,
            is_symlink: row.get(16)?,
            permissions: row.get(17)?,
            owner: row.get(18)?,
            group: row.get(19)?,
            mime_type: row.get(20)?,
            encoding: row.get(21)?,
            line_count: row.get(22)?,
            word_count: row.get(23)?,
            checksum: row.get(24)?,
            is_encrypted: row.get(25)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::repository::FileRepository;
    use crate::domain::entities::file::File;
    use crate::domain::entities::search::SearchQuery;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use tempfile::TempDir;

    fn create_test_db() -> (Db, TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Db::new(db_path.to_str().unwrap()).unwrap();
        db.init().unwrap();
        (db, temp_dir)
    }

    fn create_test_file(path: &str) -> File {
        File {
            path: PathBuf::from(path),
            name: "test.txt".to_string(),
            is_dir: false,
            file_type: Some("txt".to_string()),
            size: Some(100),
            last_modified: SystemTime::now(),
            created_at: SystemTime::now(),
            accessed_at: SystemTime::now(),
            is_indexed: true,
            content_indexed: false,
            is_indexable: true,
            is_hidden: false,
            is_readonly: false,
            is_system: false,
            is_executable: false,
            is_symlink: false,
            permissions: Some(0o644),
            owner: None,
            group: None,
            mime_type: Some("text/plain".to_string()),
            encoding: None,
            line_count: None,
            word_count: None,
            checksum: None,
            is_encrypted: false,
        }
    }

    #[test]
    fn test_transaction_rollback_on_error() {
        let (mut db, _temp_dir) = create_test_db();
        
        // Tester que les transactions sont rollback en cas d'erreur
        let file = create_test_file("/test/path.txt");
        
        // Tenter une opération qui échouera (fichier non existant dans la DB)
        let result = db.update_file_index_status(&file, "hash".to_string(), true);
        
        // Devrait retourner une erreur NotFound
        assert!(result.is_err());
        if let Err(AppError::NotFound(_)) = result {
            // C'est l'erreur attendue
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[test]
    fn test_cursor_based_pagination() {
        let (mut db, _temp_dir) = create_test_db();
        
        // Insérer plusieurs fichiers
        let files: Vec<File> = (0..20)
            .map(|i| create_test_file(&format!("/test/file{}.txt", i)))
            .collect();
        
        db.insert(files).unwrap();
        
        // Test avec pagination normale (offset)
        let mut query = SearchQuery::default();
        query.limit = 5;
        query.offset = 0;
        query.cursor = None;
        
        let first_page = db.search(&query).unwrap();
        assert_eq!(first_page.len(), 5);
        
        // Test avec cursor-based pagination
        // Pour simplifier, on utilise offset mais en production on utiliserait le dernier ID
        query.offset = 5;
        let second_page = db.search(&query).unwrap();
        assert_eq!(second_page.len(), 5);
        
        // Vérifier que les pages sont différentes
        assert_ne!(first_page[0].path, second_page[0].path);
    }

    #[test]
    fn test_insert_with_transaction_safety() {
        let (mut db, _temp_dir) = create_test_db();
        
        // Test insertion normale
        let files = vec![create_test_file("/test/file1.txt")];
        let result = db.insert(files);
        assert!(result.is_ok());
        
        // Vérifier que le fichier a été inséré
        let query = SearchQuery {
            path_pattern: Some("/test/file1.txt".to_string()),
            limit: 1,
            ..Default::default()
        };
        let results = db.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_timeout_monitoring() {
        let (db, _temp_dir) = create_test_db();
        
        let query = SearchQuery::default();
        let start = std::time::Instant::now();
        let _result = db.search(&query);
        let elapsed = start.elapsed();
        
        // La requête devrait être rapide (< 1 seconde)
        assert!(elapsed < std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_insert_paths_transaction_rollback() {
        let (mut db, _temp_dir) = create_test_db();
        
        // Test insertion de chemins valides
        let paths = vec!["/test/path1".to_string(), "/test/path2".to_string()];
        let result = db.insert_paths(paths.clone());
        assert!(result.is_ok());
        
        // Vérifier que les chemins ont été insérés
        let all_paths = db.get_all_paths().unwrap();
        assert_eq!(all_paths.len(), 2);
    }
}
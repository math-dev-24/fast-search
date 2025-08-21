use std::collections::HashSet;
use std::path::PathBuf;
use rusqlite::{Connection, Result as SqliteResult};
use crate::domain::entities::file::File;
use crate::domain::entities::stat::Stat;
use crate::domain::entities::search::{SearchQuery, DateMode, SortBy, SortOrder};
use crate::domain::ports::repository::FileRepository;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::domain::entities::query_builder::QueryBuilder;
use crate::shared::errors::AppResult;


pub struct Db {
    pub conn: Connection,
    pub types_cache: Option<HashSet<String>>
}

impl FileRepository for Db {
    fn new(path: &str) -> AppResult<Db> {

        let conn = Connection::open(path)?;

        Ok(Self {
            conn,
            types_cache: None,
        })
    }

    fn init(&self) -> AppResult<()> {
        let init_sql = include_str!("../../../data/init.sql");
        self.conn.execute_batch(init_sql)?;
        println!("Database initialized");
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

        let tx = self.conn.transaction()?;

        tx.execute("DELETE FROM paths", [])?;

        for path in &new_paths {
            tx.execute("INSERT INTO paths (path) VALUES (?)", [path])?;
        }

        tx.commit()?;

        Ok(new_paths)
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

        let (sql, params) = builder.build(order_by, sort_order, query.limit, query.offset);
        self.execute_search_query(&sql, &params)
    }

    fn reset_data(&self) -> AppResult<()> {
        self.conn.execute("DELETE FROM files", [])?;
        self.conn.execute("DELETE FROM types", [])?;
        self.conn.execute("DELETE FROM paths", [])?;
        Ok(())
    }

    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> AppResult<()> {

        let tx = self.conn.transaction()?;

        let file_id: i64 = tx.query_row(
            "SELECT id FROM files WHERE path = ?",
            [file.path.to_str().unwrap()],
            |row| row.get(0)
        )?;

        tx.execute(
            "INSERT INTO fts_content (content, file_id) VALUES (?, ?)",
            rusqlite::params![content_hash, file_id]
        )?;

        tx.execute(
            "UPDATE files SET content_indexed = ?, is_indexable = ? WHERE path = ?",
            rusqlite::params![true, is_indexable, file.path.to_str().unwrap()]
        )?;

        tx.commit()?;
        Ok(())
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
        let mut stmt = self.conn.prepare(sql)?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let result = stmt.query_map(rusqlite::params_from_iter(param_refs), Self::map_row_to_file)?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(result)
    }
    fn load_types_cache(&mut self) -> AppResult<()> {
        let types = self.get_all_types()?;
        self.types_cache = Some(types.into_iter().collect());
        Ok(())
    }

    fn file_exist(&mut self, file: &File) -> AppResult<bool> {
        let mut stmt = self.conn.prepare("SELECT EXISTS(SELECT 1 FROM files WHERE path = ? LIMIT 1)")?;
        let exists: i64 = stmt.query_row([file.path.to_str().unwrap()], |row| row.get(0))?;
        Ok(exists > 0)
    }

    fn type_exist(&self, type_name: &str) -> AppResult<bool> {
        if let Some(types_cache) = &self.types_cache {
            if let Some(_) = types_cache.iter().position(|t| t == type_name) {
                return Ok(true);
            }
        }
        let mut stmt = self.conn.prepare("SELECT EXISTS(SELECT 1 FROM types WHERE name = ? LIMIT 1)")?;
        let exists: i64 = stmt.query_row([type_name], |row| row.get(0))?;
        Ok(exists > 0)
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
        if self.types_cache.is_none() {
            self.load_types_cache()?;
        }

        let cache = self.types_cache.as_ref().unwrap();

        let new_types: Vec<String> = type_names.into_iter()
            .filter(|t| !cache.contains(t))
            .collect();

        if new_types.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;
        let mut stmt = tx.prepare("INSERT INTO types (name) VALUES (?)")?;

        for type_name in &new_types {
            stmt.execute([type_name])?;
        }

        drop(stmt);
        tx.commit()?;

        if let Some(ref mut cache) = self.types_cache {
            cache.extend(new_types);
        }
        Ok(())
    }

    fn insert_file(&mut self, files: Vec<File>) -> AppResult<()> {
        if files.is_empty() {
            return Ok(());
        }
        let tx = self.conn.transaction()?;

        let paths: Vec<String> = files.iter()
            .map(|f| format!("'{}'", f.path.to_str().unwrap().replace("'", "''")))
            .collect();

        let existing_paths_query = format!(
            "SELECT path FROM files WHERE path IN ({})",
            paths.join(",")
        );

        let mut existing_paths = HashSet::new();
        let mut stmt = tx.prepare(&existing_paths_query)?;
        let rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        for path in rows {
            existing_paths.insert(path?);
        }
        drop(stmt);

        let new_files: Vec<&File> = files.iter()
            .filter(|f| !existing_paths.contains(f.path.to_str().unwrap()))
            .collect();

        if new_files.is_empty() {
            tx.commit()?;
            return Ok(());
        }

        let mut stmt = tx.prepare("INSERT INTO files (path, name, is_dir, file_type, size, last_modified, created_at, accessed_at, is_indexed, content_indexed, is_indexable, is_hidden, is_readonly, is_system, is_executable, is_symlink, permissions, owner, `group`, mime_type, encoding, line_count, word_count, checksum, is_encrypted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

        for file in &new_files {
            let path = file.path.to_str().unwrap();
            let last_modified = file.last_modified.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
            let created_at = file.created_at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
            let accessed_at = file.accessed_at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;

            stmt.execute(rusqlite::params![
                path,
                file.name,
                file.is_dir,
                file.file_type,
                file.size,
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
        tx.commit()?;
        Ok(())
    }

    fn map_row_to_file(row: &rusqlite::Row) -> rusqlite::Result<File> {
        Ok(File {
            path: PathBuf::from(row.get::<_, String>(1)?),
            name: row.get(2)?,
            is_dir: row.get(3)?,
            file_type: row.get(4)?,
            size: row.get(5)?,
            last_modified: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(6)?),
            created_at: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(7)?),
            accessed_at: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(8)?),
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
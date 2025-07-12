use std::collections::HashSet;
use std::path::PathBuf;
use rusqlite::{Connection, Result as SqliteResult};
use crate::entities::file::File;
use crate::entities::stat::Stat;
use crate::ports::repository::FileRepository;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct Db {
    pub conn: Connection,
}

impl FileRepository for Db {

    fn new(path: &str) -> SqliteResult<Db> {
        let conn = Connection::open(path)?;        
        Ok(Self { conn })
    }

    fn init(&self) -> SqliteResult<()> {

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                is_dir INTEGER NOT NULL,
                file_type TEXT,
                size INTEGER,
                last_modified INTEGER,
                created_at INTEGER,
                is_indexed INTEGER NOT NULL DEFAULT 0,
                content_indexed INTEGER NOT NULL DEFAULT 0,
                is_indexable INTEGER NOT NULL DEFAULT 1,
                content_hash TEXT
            )",
            [],
        )?;

        let index_queries = vec![
            "CREATE INDEX IF NOT EXISTS idx_files_name ON files(name COLLATE NOCASE)",
            "CREATE INDEX IF NOT EXISTS idx_files_is_dir ON files(is_dir)",
            "CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type)",
            "CREATE INDEX IF NOT EXISTS idx_files_size ON files(size)",
            "CREATE INDEX IF NOT EXISTS idx_files_last_modified ON files(last_modified)",
            "CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_files_path ON files(path)",
        ];

        for query in index_queries {
            let _: Result<usize, _> = self.conn.execute(query, []);
        }

        self.conn.execute("CREATE TABLE IF NOT EXISTS types (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )", [])?;

        self.conn.execute("CREATE TABLE IF NOT EXISTS paths (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE
        )", [])?;

        Ok(())
    }

    fn search(
        &self, 
        query: &str, 
        file_types: &[String], 
        is_dir: bool, 
        folders: &[String], 
        size_limit: &[usize], 
        date_range: &[usize], 
        date_mode: &str, 
        in_content: bool
    ) -> SqliteResult<Vec<File>> 
    {
       
        let mut conditions = Vec::new();
        let mut params = Vec::new();
        let mut string_conditions = Vec::new();

        if !query.trim().is_empty() {
            let search_query = format!("%{}%", query);
            conditions.push("LOWER(name) LIKE LOWER(?)");
            params.push(search_query);
        }

        if size_limit.len() >= 2 && (size_limit[0] > 0 || size_limit[1] > 0) {
            let min = size_limit[0] as i64 * 1024 * 1024;
            let max = if size_limit[1] > 0 { 
                size_limit[1] as i64 * 1024 * 1024 
            } else { 
                i64::MAX
            };

            conditions.push("size >= ? AND size <= ?");
            params.push(min.to_string());
            params.push(max.to_string());
        }

        if in_content {
            conditions.push("content_indexed = 1");
            conditions.push("content_hash IS NOT NULL");
            conditions.push("content_hash LIKE ?");
            params.push(format!("%{}%", query));
        }

        if date_range.len() >= 2 && (date_range[0] > 0 || date_range[1] > 0) {
            let min = date_range[0] as i64;
            let max = if date_range[1] > 0 { 
                date_range[1] as i64 
            } else { 
                i64::MAX
            };

            if date_mode == "create" {
                conditions.push("created_at >= ? AND created_at <= ?");
            } else {
                conditions.push("last_modified >= ? AND last_modified <= ?");
            }

            params.push(min.to_string());
            params.push(max.to_string());
        }

        if is_dir {
            conditions.push("is_dir = 1");
        }
        
        if !file_types.is_empty() {
            let placeholders = file_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            string_conditions.push(format!("file_type IN ({})", placeholders));
            params.extend(file_types.iter().map(|s| s.to_string()));
        }
        
        if !folders.is_empty() {
            let folder_conditions: Vec<String> = folders.iter()
                .map(|folder| format!("path LIKE '%{}%'", folder))
                .collect();
            string_conditions.push(format!("({})", folder_conditions.join(" OR ")));
        }

        conditions.extend(string_conditions.iter().map(|s| s.as_str()));
        
        let where_clause = if conditions.is_empty() {
            "1=1".to_string()
        } else {
            conditions.join(" AND ")
        };
        
        let sql_query = format!(
            "SELECT * FROM files WHERE {} ORDER BY name COLLATE NOCASE LIMIT 10000",
            where_clause
        );
        
        let mut stmt = self.conn.prepare(&sql_query)?;

        let result = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(File {
                path: PathBuf::from(row.get::<_, String>(1)?),
                name: row.get(2)?,
                is_dir: row.get(3)?,
                file_type: row.get(4)?,
                size: row.get(5)?,
                last_modified: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(6)?),
                created_at: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(7)?),
                is_indexed: row.get(8)?,
                content_indexed: row.get(9)?,
                is_indexable: row.get(10)?,
                content_hash: row.get(11)?
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(result)
    }

    fn get_stat(&self) -> SqliteResult<Stat> {
        // Optimisation: une seule requête au lieu de trois
        let mut stmt = self.conn.prepare(
            "SELECT 
                COUNT(CASE WHEN is_dir = 0 THEN 1 END) as nb_files,
                COUNT(CASE WHEN is_dir = 1 THEN 1 END) as nb_folders,
                COALESCE(SUM(CASE WHEN is_dir = 0 THEN size ELSE 0 END), 0) as total_size
             FROM files"
        )?;
        
        let result = stmt.query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?
            ))
        })?;

        // Indexation des fichiers
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_indexed = 1 AND is_indexable = 1")?;
        let indexed_files: i64 = stmt.query_row([], |row| row.get(0))?;

        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_indexed = 0 AND is_indexable = 1")?;
        let unindexed_files: i64 = stmt.query_row([], |row| row.get(0))?;

        let indexed_percentage = (indexed_files as f64 / (indexed_files + unindexed_files) as f64) * 100.0;

        // Indexation du contenu
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_indexable = 0")?;
        let unindexable_files: i64 = stmt.query_row([], |row| row.get(0))?;


        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE content_indexed = 1 AND is_indexable = 1")?;
        let content_indexed_files: i64 = stmt.query_row([], |row| row.get(0))?;

        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE content_indexed = 0 AND is_indexable = 1")?;
        let uncontent_indexed_files: i64 = stmt.query_row([], |row| row.get(0))?;

        let content_indexed_percentage = (content_indexed_files as f64 / (content_indexed_files + uncontent_indexed_files + unindexable_files) as f64) * 100.0;

        Ok(Stat { 
            nb_files: result.0 as u32, 
            nb_folders: result.1 as u32, 
            total_size: result.2 as u64,
            indexed_files: indexed_files as u32,
            unindexed_files: unindexed_files as u32,
            indexed_percentage: indexed_percentage as f64,
            content_indexed_files: content_indexed_files as u32,
            uncontent_indexed_files: uncontent_indexed_files as u32,
            content_indexed_percentage: content_indexed_percentage as f64,
            unindexable_files: unindexable_files as u32
        })
    }

    fn get_all_types(&self) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM types")?;
        let types: Vec<String> = stmt.query_map([], |row| row.get(0))?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(types)
    }

    fn insert(&mut self, files: Vec<File>) -> SqliteResult<()> {

        let new_types = files.iter()
        .map(|file| file.file_type.clone().unwrap_or_default())
        .collect::<Vec<_>>();

        self.insert_type(new_types)?;
        self.insert_file(files)?;

        Ok(())
    }

    fn reset_data(&self) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM files", [])?;
        self.conn.execute("DELETE FROM types", [])?;
        self.conn.execute("DELETE FROM paths", [])?;
        Ok(())
    }

    fn get_all_folders(&self) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT name FROM files WHERE is_dir = 1")?;
        let folders: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(folders)
    }

    fn get_all_paths(&self) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT path FROM paths")?;
        let paths: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(paths)
    }

    fn insert_paths(&mut self, paths: Vec<String>) -> SqliteResult<()> {

        let tx = self.conn.transaction()?;

        tx.execute("DELETE FROM paths", [])?;

        for path in paths {
            tx.execute("INSERT INTO paths (path) VALUES (?)", [path])?;
        }
        tx.commit()?;
        Ok(())
    }

    fn get_uncontent_indexed_files(&self) -> SqliteResult<Vec<File>> {
        let mut stmt = self.conn.prepare("SELECT * FROM files WHERE content_indexed = 0 AND content_hash IS NULL AND is_indexable = 1")?;
        let files: Vec<File> = stmt.query_map([], |row| {
            Ok(File {
                path: PathBuf::from(row.get::<_, String>(1)?),
                name: row.get(2)?,
                is_dir: row.get(3)?,
                file_type: row.get(4)?,
                size: row.get(5)?,
                last_modified: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(6)?),
                created_at: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get(7)?),
                is_indexed: row.get(8)?,
                content_indexed: row.get(9)?,
                is_indexable: row.get(10)?,
                content_hash: row.get(11)?
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(files)
    }

    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE files SET content_indexed = ?, content_hash = ?, is_indexable = ? WHERE path = ?", 
            rusqlite::params![true, content_hash, is_indexable, file.path.to_str().unwrap()]
        )?;
        Ok(())
    }
}


impl Db {

    fn file_exist(&mut self, file: &File) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT EXISTS(SELECT 1 FROM files WHERE path = ? LIMIT 1)")?;
        let exists: i64 = stmt.query_row([file.path.to_str().unwrap()], |row| row.get(0))?;
        Ok(exists > 0)
    }

    fn type_exist(&self, type_name: &str) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT EXISTS(SELECT 1 FROM types WHERE name = ? LIMIT 1)")?;
        let exists: i64 = stmt.query_row([type_name], |row| row.get(0))?;
        Ok(exists > 0)
    }

    fn insert_type(&mut self, type_name: Vec<String>) -> SqliteResult<()> {      

        let mut new_types = HashSet::new();

        for type_name in &type_name {
            if !self.type_exist(type_name).unwrap_or(false) {
                new_types.insert(type_name.clone());
            }
        }

        if new_types.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        for type_name in &new_types {
            tx.execute("INSERT INTO types (name) VALUES (?)", [type_name])?;
        }

        tx.commit()?;
        Ok(())
    }
    
    fn insert_file(&mut self, files: Vec<File>) -> SqliteResult<()> {
        let mut new_files: Vec<File> = Vec::new();
        
        for file in files {
            if !self.file_exist(&file).unwrap_or(false) {
                new_files.push(file);
            }
        }
        
        if new_files.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;
        
        let mut stmt = tx.prepare("INSERT INTO files (path, name, is_dir, file_type, size, last_modified, created_at, is_indexed, content_indexed, is_indexable, content_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

        for file in &new_files {
            stmt.execute(rusqlite::params![
                file.path.to_str().unwrap(), 
                file.name, 
                file.is_dir, 
                file.file_type, 
                file.size, 
                file.last_modified.duration_since(UNIX_EPOCH).unwrap().as_secs(), 
                file.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                file.is_indexed,
                file.content_indexed,
                file.is_indexable,
                file.content_hash
            ])?;
        }

        drop(stmt);
        tx.commit()?;
        Ok(())
    }
}
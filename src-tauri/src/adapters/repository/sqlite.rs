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
                created_at INTEGER
            )",
            [],
        )?;


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

    fn search(&self, query: &str, file_types: &[String], is_dir: bool, folders: &[String], size_limit: &[usize], date_range: &[usize], date_mode: &str) -> SqliteResult<Vec<File>> {
       
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
            "SELECT * FROM files WHERE {} ORDER BY name COLLATE NOCASE",
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
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(result)
    }

    fn get_stat(&self) -> SqliteResult<Stat> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_dir = 0")?;
        let nb_files: i64 = stmt.query_row([], |row| row.get(0))?;
        
        // Gérer le cas où SUM(size) peut retourner NULL
        let mut stmt = self.conn.prepare("SELECT COALESCE(SUM(size), 0) FROM files WHERE is_dir = 0")?;
        let total_size: i64 = stmt.query_row([], |row| row.get(0))?;
        
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_dir = 1")?;
        let nb_folders: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(Stat { nb_files: nb_files as u32, nb_folders: nb_folders as u32, total_size: total_size as u64 })
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

        let mut new_paths = HashSet::new();

        for path in paths {
            if !self.path_exist(&path).unwrap_or(false) {
                new_paths.insert(path);
            }
        }

        if new_paths.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        for path in new_paths {
            tx.execute("INSERT INTO paths (path) VALUES (?)", [path])?;
        }
        tx.commit()?;
        Ok(())
    }
}


impl Db {

    fn file_exist(&mut self, file: &File) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE path = ?")?;
        let count: i64 = stmt.query_row([file.path.to_str().unwrap()], |row| row.get(0))?;
        Ok(count > 0)
    }

    fn type_exist(&self, type_name: &str) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM types WHERE name = ?")?;
        let count: i64 = stmt.query_row([type_name], |row| row.get(0))?;
        Ok(count > 0)
    }

    fn path_exist(&self, path: &str) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM paths WHERE path = ?")?;
        let count: i64 = stmt.query_row([path], |row| row.get(0))?;
        Ok(count > 0)
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

        for file in &new_files {
            tx.execute("INSERT INTO files (path, name, is_dir, file_type, size, last_modified, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)", 
            rusqlite::params![
                file.path.to_str().unwrap(), 
                file.name, 
                file.is_dir, 
                file.file_type, 
                file.size, 
                file.last_modified.duration_since(UNIX_EPOCH).unwrap().as_secs(), 
                file.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs()
            ]
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}
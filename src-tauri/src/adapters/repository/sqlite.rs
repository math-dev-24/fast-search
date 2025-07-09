use std::path::PathBuf;
use rusqlite::{Connection, Result as SqliteResult};
use crate::entities::file::File;
use crate::entities::stat::Stat;
use crate::ports::repository::FileRepository;

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
                last_modified TEXT,
                created_at TEXT
            )",
            [],
        )?;

        Ok(())
    }

    fn search(&self, query: &str, file_types: &[String], is_dir: bool, folders: &[String]) -> SqliteResult<Vec<File>> {
        let search_query = format!("%{}%", query);
        
        let mut conditions = vec!["LOWER(name) LIKE LOWER(?)"];
        let mut params = vec![search_query.as_str()];
        let mut string_conditions = Vec::new();

        if is_dir {
            conditions.push("is_dir = 1");
        }
        
        if !file_types.is_empty() {
            let placeholders = file_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            string_conditions.push(format!("file_type IN ({})", placeholders));
            params.extend(file_types.iter().map(|s| s.as_str()));
        }
        
        if !folders.is_empty() {
            let folder_conditions: Vec<String> = folders.iter()
                .map(|folder| format!("path LIKE '%{}%'", folder))
                .collect();
            string_conditions.push(format!("({})", folder_conditions.join(" OR ")));
        }
        

        conditions.extend(string_conditions.iter().map(|s| s.as_str()));
        
        let where_clause = conditions.join(" AND ");
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
                last_modified: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(result)
    }

    fn get_stat(&self) -> SqliteResult<Stat> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_dir = 0")?;
        let nb_files: i64 = stmt.query_row([], |row| row.get(0))?;
        let mut stmt = self.conn.prepare("SELECT SUM(size) FROM files WHERE is_dir = 0")?;
        let total_size: i64 = stmt.query_row([], |row| row.get(0))?;
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE is_dir = 1")?;
        let nb_folders: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(Stat { nb_files: nb_files as u32, nb_folders: nb_folders as u32, total_size: total_size as u64 })
    }

    fn get_type_files(&self) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT file_type FROM files WHERE file_type IS NOT NULL")?;
        let files_type: Vec<String> = stmt.query_map([], |row| row.get(0))?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(files_type)
    }

    fn insert(&mut self, files: &[File]) -> SqliteResult<()> {
        let new_files: Vec<_> = files.iter()
            .filter(|file| !self.file_exist(file).unwrap_or(false))
            .collect();
        
        if new_files.is_empty() {
            println!("Aucun nouveau fichier à insérer");
            return Ok(());
        }
        
        let tx = self.conn.transaction()?;
        for file in &new_files {
            match tx.execute("INSERT INTO files (path, name, is_dir, file_type, size, last_modified, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)", 
            rusqlite::params![
                file.path.to_str().unwrap(), 
                file.name.as_str(), 
                file.is_dir, 
                file.file_type.as_ref().map(|s| s.as_str()), 
                file.size, 
                file.last_modified.as_ref().map(|s| s.as_str()), 
                file.created_at.as_ref().map(|s| s.as_str())
            ]
            ) {
                Ok(_) => {},
                Err(e) => {
                    println!("Erreur lors de l'insertion du fichier {:?}: {}", file.path, e);
                    return Err(e);
                }
            }
        }
        tx.commit()?;
        println!("{} fichiers insérés dans la base de données", new_files.len());
        Ok(())
    }

    fn reset_data(&self) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM files", [])?;
        Ok(())
    }

    fn get_all_folders(&self) -> SqliteResult<Vec<File>> {
        let mut stmt = self.conn.prepare("SELECT * FROM files WHERE is_dir = 1")?;
        let files: Vec<File> = stmt.query_map([], |row| {
            Ok(File {
                path: PathBuf::from(row.get::<_, String>(1)?),
                name: row.get(2)?,
                is_dir: row.get(3)?,
                file_type: row.get(4)?,
                size: row.get(5)?,
                last_modified: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(files)
    }
}


impl Db {

    fn file_exist(&mut self, file: &File) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM files WHERE path = ?")?;
        let count: i64 = stmt.query_row([file.path.to_str().unwrap()], |row| row.get(0))?;
        Ok(count > 0)
    }
}
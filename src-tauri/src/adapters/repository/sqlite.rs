use std::collections::HashSet;
use std::path::PathBuf;
use rusqlite::{Connection, Result as SqliteResult};
use crate::entities::file::File;
use crate::entities::stat::Stat;
use crate::ports::repository::FileRepository;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::entities::search::{SearchQuery, DateMode, SortBy, SortOrder};

pub struct Db {
    pub conn: Connection,
}

impl FileRepository for Db {

    fn new(path: &str) -> SqliteResult<Db> {
        let conn = Connection::open(path).map_err(|e| e.to_string()).expect("Failed to open database");        
        Ok(Self { conn })
    }

    fn init(&self) -> SqliteResult<(), rusqlite::Error> {
        let init_sql = include_str!("../../../data/init.sql");
        match self.conn.execute_batch(init_sql) {
            Ok(_) => println!("Base de données initialisée avec succès"),
            Err(e) => {
                println!("Erreur d'initialisation de la base: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    fn search(
        &self,
        query: &SearchQuery,
    ) -> SqliteResult<Vec<File>> {
        let mut conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();
        let mut extra_conditions = Vec::new();
    
        // Recherche textuelle dans le nom ou le chemin
        if !query.text.trim().is_empty() {
            if query.search_in_content {
                // La recherche dans le contenu sera gérée par la jointure FTS
            } else {
                // Recherche dans le nom et le chemin
                conditions.push("(LOWER(name) LIKE LOWER(?) OR LOWER(path) LIKE LOWER(?))");
                params.push(format!("%{}%", query.text));
                params.push(format!("%{}%", query.text));
            }
        }
    
        // Filtres de type de fichier
        if query.filters.is_dir {
            conditions.push("is_dir = 1");
        }
    
        // Extensions de fichiers
        if !query.filters.file_types.is_empty() {
            let placeholders = query.filters.file_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            extra_conditions.push(format!("file_type IN ({})", placeholders));
            params.extend(query.filters.file_types.iter().cloned());
        }
    
        // Filtres par dossier
        if !query.filters.folders.is_empty() {
            let folder_conditions: Vec<String> = query.filters.folders
                .iter()
                .map(|folder| format!("path LIKE '%{}%'", folder))
                .collect();
            extra_conditions.push(format!("({})", folder_conditions.join(" OR ")));
        }
    
        // Filtres de taille
        if query.filters.size_limit.len() >= 2 && (query.filters.size_limit[0] > 0 || query.filters.size_limit[1] > 0) {
            let min = query.filters.size_limit[0] as i64 * 1024 * 1024; // Conversion en bytes
            let max = if query.filters.size_limit[1] > 0 {
                query.filters.size_limit[1] as i64 * 1024 * 1024
            } else {
                i64::MAX
            };
    
            conditions.push("size >= ? AND size <= ?");
            params.push(min.to_string());
            params.push(max.to_string());
        }
    
        // Filtres de dates
        if query.filters.date_range.len() >= 2 && (query.filters.date_range[0] > 0 || query.filters.date_range[1] > 0) {
            let min = query.filters.date_range[0] as i64;
            let max = if query.filters.date_range[1] > 0 {
                query.filters.date_range[1] as i64
            } else {
                i64::MAX
            };
    
            if query.filters.date_mode == DateMode::Create {
                conditions.push("created_at >= ? AND created_at <= ?");
            } else {
                conditions.push("last_modified >= ? AND last_modified <= ?");
            }
    
            params.push(min.to_string());
            params.push(max.to_string());
        }
    
        // Recherche par pattern de chemin
        if let Some(path_pattern) = &query.path_pattern {
            if !path_pattern.trim().is_empty() {
                conditions.push("path LIKE ?");
                params.push(format!("%{}%", path_pattern));
            }
        }
    
        // Combine toutes les conditions
        conditions.extend(extra_conditions.iter().map(|s| s.as_str()));
        let where_clause = if conditions.is_empty() {
            "1=1".to_string()
        } else {
            conditions.join(" AND ")
        };
    
        // Construction de l'ORDER BY avec COLLATE NOCASE pour les chaînes
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
    
        // Création de la requête SQL
        let sql_query = if query.search_in_content && !query.text.trim().is_empty() {
            // Requête avec jointure FTS pour la recherche dans le contenu
            format!(
                "SELECT files.* FROM files \
                 JOIN fts_content ON files.id = fts_content.file_id \
                 WHERE fts_content.content MATCH ? AND {} \
                 ORDER BY files.{} {} LIMIT {} OFFSET {}",
                where_clause, order_by, sort_order, query.limit, query.offset
            )
        } else {
            // Requête sans jointure
            format!(
                "SELECT * FROM files \
                 WHERE {} \
                 ORDER BY {} {} LIMIT {} OFFSET {}",
                where_clause, order_by, sort_order, query.limit, query.offset
            )
        };
    
        // Préparation de la requête
        let mut stmt = self.conn.prepare(&sql_query)?;
    
        // Insertion des paramètres
        let all_params: Vec<&dyn rusqlite::ToSql> = if query.search_in_content && !query.text.trim().is_empty() {
            let mut merged = vec![&query.text as &dyn rusqlite::ToSql];
            merged.extend(params.iter().map(|s| s as &dyn rusqlite::ToSql));
            merged  
        } else {
            params.iter().map(|s| s as &dyn rusqlite::ToSql).collect()
        };
    
        // Exécution et mapping
        let result = stmt
            .query_map(rusqlite::params_from_iter(all_params), |row| {
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

    fn insert_paths(&mut self, new_paths: Vec<String>) -> SqliteResult<Vec<String>> {

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

    fn get_uncontent_indexed_files(&self) -> SqliteResult<Vec<File>> {
        let mut stmt = self.conn.prepare("SELECT * FROM files WHERE content_indexed = 0 AND is_indexable = 1")?;
        let files: Vec<File> = stmt.query_map([], |row| {
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
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
        Ok(files)
    }

    fn update_file_index_status(&mut self, file: &File, content_hash: String, is_indexable: bool) -> SqliteResult<()> {

        let tx = self.conn.transaction()?;

        // D'abord, récupérer l'ID du fichier
        let file_id: i64 = tx.query_row(
            "SELECT id FROM files WHERE path = ?",
            [file.path.to_str().unwrap()],
            |row| row.get(0)
        )?;

        // Insérer le contenu dans la table FTS
        tx.execute(
            "INSERT INTO fts_content (content, file_id) VALUES (?, ?)",
            rusqlite::params![content_hash, file_id]
        )?;

        // Mettre à jour le statut d'indexation du fichier
        tx.execute(
            "UPDATE files SET content_indexed = ?, is_indexable = ? WHERE path = ?", 
            rusqlite::params![true, is_indexable, file.path.to_str().unwrap()]
        )?;

        tx.commit()?;
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

    fn delete_files_by_path_prefix(&mut self, path_prefix: &str) -> SqliteResult<()> {
        let tx = self.conn.transaction()?;
        
        // Supprimer d'abord le contenu FTS des fichiers qui commencent par le préfixe
        tx.execute(
            "DELETE FROM fts_content WHERE file_id IN (SELECT id FROM files WHERE path LIKE ?)",
            [format!("{}%", path_prefix)]
        )?;
        
        // Puis supprimer les fichiers
        tx.execute("DELETE FROM files WHERE path LIKE ?", [format!("{}%", path_prefix)])?;
        
        tx.commit()?;
        Ok(())
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
        
        let mut stmt = tx.prepare("INSERT INTO files (path, name, is_dir, file_type, size, last_modified, created_at, accessed_at, is_indexed, content_indexed, is_indexable, is_hidden, is_readonly, is_system, is_executable, is_symlink, permissions, owner, \"group\", mime_type, encoding, line_count, word_count, checksum, is_encrypted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

        for file in &new_files {
            stmt.execute(rusqlite::params![
                file.path.to_str().unwrap(), 
                file.name, 
                file.is_dir, 
                file.file_type, 
                file.size, 
                file.last_modified.duration_since(UNIX_EPOCH).unwrap().as_secs(), 
                file.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                file.accessed_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
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
}
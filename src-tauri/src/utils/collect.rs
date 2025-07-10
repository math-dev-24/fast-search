use std::path::{Path};
use std::time::{SystemTime};
use walkdir::WalkDir;
use crate::entities::file::File;

pub fn collect_files_and_folders(base_path: &Path) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    if !base_path.exists() {
        return files;
    }

    if !base_path.is_dir() {
        return files;
    }

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().to_string_lossy().contains("$RECYCLE.BIN")) // Ignorer la corbeille Windows
    {
        // Collect folders
        if entry.path().is_dir() {
            if let Some(file_name) = entry.path().file_name() {
                if let Some(name_str) = file_name.to_str() {

                    let last_modified = entry.metadata().unwrap().modified().unwrap_or_else(|_| SystemTime::now());
                    let created_at = entry.metadata().unwrap().created().unwrap_or_else(|_| SystemTime::now());

                    files.push(File {
                        path: entry.path().to_path_buf(),
                        name: name_str.to_string(),
                        is_dir: true,
                        file_type: None,
                        size: Some(0),
                        last_modified: last_modified,
                        created_at: created_at,
                    });
                }
            }
        } 
        // Collect files
        else {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    let file_type = match path.extension() {
                        Some(ext) => {
                            match ext.to_str() {
                                Some(ext_str) => ext_str.to_lowercase(),
                                None => "invalid_extension".to_string()
                            }
                        },
                        None => "no_extension".to_string()
                    };
                    
                    if let Ok(metadata) = entry.metadata() {
                        let last_modified = metadata.modified().unwrap_or_else(|_| SystemTime::now());
                        let created_at = metadata.created().unwrap_or_else(|_| SystemTime::now());

                        files.push(File {
                            path: path.to_path_buf(),
                            name: name_str.to_string(),
                            is_dir: false,
                            file_type: Some(file_type),
                            size: Some(metadata.len()),
                            last_modified: last_modified,
                            created_at: created_at,
                        });
                    }
                }
            }
        }
    }

    files
}
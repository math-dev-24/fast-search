use std::path::{Path};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use crate::entities::file::File;

pub fn collect_files_and_folders(base_path: &Path) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    println!("Début de la collecte pour le chemin: {:?}", base_path);

    if !base_path.exists() {
        println!("Le chemin n'existe pas: {:?}", base_path);
        return files;
    }

    if !base_path.is_dir() {
        println!("Le chemin n'est pas un dossier: {:?}", base_path);
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
                    files.push(File {
                        path: entry.path().to_path_buf(),
                        name: name_str.to_string(),
                        is_dir: true,
                        file_type: None,
                        size: None,
                        last_modified: None,
                        created_at: None,
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
                        let last_modified = metadata.modified()
                            .map(|modified| DateTime::from(modified))
                            .unwrap_or_else(|_| Utc::now());
                        let created_at = metadata.created()
                            .map(|created| DateTime::from(created))
                            .unwrap_or_else(|_| Utc::now());

                        files.push(File {
                            path: path.to_path_buf(),
                            name: name_str.to_string(),
                            is_dir: false,
                            file_type: Some(file_type),
                            size: Some(metadata.len()),
                            last_modified: Some(last_modified.to_rfc3339()),
                            created_at: Some(created_at.to_rfc3339()),
                        });
                    } else {
                        println!("Impossible de lire les métadonnées pour: {:?}", path);
                    }
                }
            }
        }
    }

    println!("Collecte terminée - Fichiers / Dossiers : {}", files.len());
    files
}
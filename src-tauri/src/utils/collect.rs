use std::path::{Path};
use std::time::{SystemTime};
use walkdir::WalkDir;
use crate::entities::file::File;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;


pub fn collect_files_and_folders<F>(base_path: &Path, progress_callback: F) -> Vec<File> 
where F: Fn(usize, &str) + Send + Sync + Clone
{
    if !base_path.exists() || !base_path.is_dir() {
        return Vec::new();
    }

    // Première phase: collecte des entrées (séquentielle)
    let entries: Vec<_> = WalkDir::new(base_path)
        .follow_links(false)
        .max_depth(50) // Limite raisonnable
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                eprintln!("Erreur d'accès: {}", err);
                None
            }
        })
        .filter(|e| !should_skip_entry(e))
        .collect();

    let total = entries.len();
    let processed = Arc::new(AtomicUsize::new(0));
    
    // Callback initial
    progress_callback(0, &format!("Début du traitement: {} éléments", total));
    
    // Deuxième phase: traitement parallèle
    let files: Vec<File> = entries
        .par_iter()
        .filter_map(|entry| {
            let result = process_entry(entry);
            
            let current = processed.fetch_add(1, Ordering::Relaxed);
            // Callback tous les 1000 éléments pour éviter le spam
            if current % 1000 == 0 {
                progress_callback(current, &format!("Traitement: {}", entry.path().display()));
            }
            
            result
        })
        .collect();

    // Callback final
    progress_callback(total, "Indexation terminée");
    files
}

fn extract_file_type(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "no_extension".to_string())
}

fn process_entry(entry: &walkdir::DirEntry) -> Option<File> {
    let path = entry.path();
    let file_name = path.file_name()?.to_str()?;
    
    // Récupération des métadonnées (une seule fois)
    let metadata = entry.metadata().ok()?;
    let last_modified = metadata.modified().unwrap_or_else(|_| SystemTime::now());
    let created_at = metadata.created().unwrap_or_else(|_| SystemTime::now());
    
    if path.is_dir() {
        Some(File {
            path: path.to_path_buf(),
            name: file_name.to_string(),
            is_dir: true,
            file_type: None,
            size: Some(0),
            last_modified,
            created_at,
        })
    } else {
        let file_type = extract_file_type(path);
        
        Some(File {
            path: path.to_path_buf(),
            name: file_name.to_string(),
            is_dir: false,
            file_type: Some(file_type),
            size: Some(metadata.len()),
            last_modified,
            created_at,
        })
    }
}

fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let path_str = entry.path().to_string_lossy();
    let file_name = entry.file_name().to_string_lossy();
    
    // Dossiers système Windows
    path_str.contains("$RECYCLE.BIN") ||
    path_str.contains("System Volume Information") ||
    path_str.contains("Windows\\System32") ||
    path_str.contains("AppData\\Local\\Temp") ||
    path_str.contains("ProgramData\\Microsoft") ||
    
    // Dossiers système Unix/Linux
    file_name.starts_with('.') ||
    path_str.contains("/proc/") ||
    path_str.contains("/sys/") ||
    path_str.contains("/dev/") ||
    
    // Dossiers temporaires communs
    path_str.contains("node_modules") ||
    path_str.contains(".git") ||
    path_str.contains("target/debug") ||
    path_str.contains("target/release") ||
    
    // Fichiers temporaires
    file_name.ends_with(".tmp") ||
    file_name.ends_with(".temp") ||
    file_name.starts_with("~$")
}

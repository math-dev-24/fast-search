use std::path::{Path};
use std::time::SystemTime;
use walkdir::WalkDir;
use crate::entities::file::File;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

pub fn collect_files_and_folders<F>(base_path: &Path, progress_callback: F) -> Vec<File> 
where F: Fn(usize, &str) + Send + Sync + Clone
{
    if !base_path.exists() || !base_path.is_dir() {
        eprintln!("[ERROR] Le chemin n'existe pas ou n'est pas un dossier: {}", base_path.display());
        return Vec::new();
    }

    let entries: Vec<_> = WalkDir::new(base_path)
        .follow_links(true)
        .max_depth(100) 
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                eprintln!("[WARNING] Erreur d'accès ignorée: {}", err);
                None
            }
        })
        .filter(|e| !should_skip_entry(e))
        .collect();

    let total = entries.len();

    println!("[INFO] Base path: {}. Trouvé {} entrées", base_path.display(), total);

    if total == 0 {
        progress_callback(0, "Aucun fichier trouvé dans ce répertoire");
        return Vec::new();
    }

    let processed = Arc::new(AtomicUsize::new(0));
    
    // Callback initial
    progress_callback(0, &format!("Début du traitement: {} éléments", total));
    
    // Deuxième phase: traitement parallèle avec gestion d'erreur améliorée
    let files: Vec<File> = entries
        .par_iter()
        .filter_map(|entry| {
            let result = process_entry_safe(entry);
            
            let current = processed.fetch_add(1, Ordering::Relaxed);
            // Callback tous les 500 éléments pour un meilleur feedback
            if current % 500 == 0 {
                progress_callback(current, &format!("Traitement: {} / {}", current, total));
            }
            
            result
        })
        .collect();

    progress_callback(total, &format!("Indexation terminée: {} fichiers traités", files.len()));
    files
}

fn extract_file_type(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "no_extension".to_string())
}

fn process_entry_safe(entry: &walkdir::DirEntry) -> Option<File> {
    let path = entry.path();
    
    // Vérification de sécurité basique
    if path.to_string_lossy().len() > 4096 {
        eprintln!("[WARNING] Chemin trop long ignoré: {}", path.display());
        return None;
    }
    
    let file_name = match path.file_name()?.to_str() {
        Some(name) => name,
        None => {
            eprintln!("[WARNING] Nom de fichier invalide ignoré: {}", path.display());
            return None;
        }
    };
    
    // Récupération des métadonnées avec gestion d'erreur
    let metadata = match entry.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("[WARNING] Impossible de lire les métadonnées pour {}: {}", path.display(), e);
            return None;
        }
    };
    
    let last_modified = metadata.modified().unwrap_or_else(|_| SystemTime::now());
    let created_at = metadata.created().unwrap_or_else(|_| SystemTime::now());
    let accessed_at = metadata.accessed().unwrap_or_else(|_| SystemTime::now());

    // Extraction des métadonnées étendues
    #[cfg(unix)]
    let permissions = {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode()
    };
    #[cfg(not(unix))]
    let permissions = 0u32;
    
    let is_readonly = metadata.permissions().readonly();
    let is_hidden = file_name.starts_with('.') || is_hidden_file(path);
    let is_system = is_system_file(path);
    
    #[cfg(unix)]
    let is_executable = {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    };
    #[cfg(not(unix))]
    let is_executable = false;
    
    let is_symlink = path.is_symlink();
    
    // Extraction des informations de propriétaire (Unix/Linux)
    let (owner, group) = extract_owner_info(&metadata);
    
    // Détermination du type MIME
    let mime_type = determine_mime_type(path);
    
    // Encodage (pour les fichiers texte)
    let encoding = if is_text_file(path) { Some("UTF-8".to_string()) } else { None };
    
    // Comptage des lignes et mots (pour les fichiers texte)
    let (line_count, word_count) = if is_text_file(path) {
        extract_text_stats(path)
    } else {
        (None, None)
    };

    if path.is_dir() {
        Some(File {
            path: path.to_path_buf(),
            name: file_name.to_string(),
            is_dir: true,
            file_type: None,
            size: Some(0),
            last_modified,
            created_at,
            accessed_at,
            is_indexed: true,
            content_indexed: true,
            is_indexable: true,
            is_hidden,
            is_readonly,
            is_system,
            is_executable,
            is_symlink,
            permissions: Some(permissions),
            owner,
            group,
            mime_type,
            encoding,
            line_count,
            word_count,
            checksum: None,
            is_encrypted: false,
        })
    } else {
        // Vérification de la taille du fichier
        let file_size = metadata.len();
        if file_size > MAX_FILE_SIZE {
            eprintln!("[WARNING] Fichier trop volumineux ignoré: {} ({} bytes)", path.display(), file_size);
            return None;
        }
        
        let file_type = extract_file_type(path);
        
        Some(File {
            path: path.to_path_buf(),
            name: file_name.to_string(),
            is_dir: false,
            file_type: Some(file_type),
            size: Some(file_size),
            last_modified,
            created_at,
            accessed_at,
            is_indexed: true,
            content_indexed: false,
            is_indexable: true,
            is_hidden,
            is_readonly,
            is_system,
            is_executable,
            is_symlink,
            permissions: Some(permissions),
            owner,
            group,
            mime_type,
            encoding,
            line_count,
            word_count,
            checksum: None,
            is_encrypted: false,
        })
    }
}

fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let path_str = entry.path().to_string_lossy();
    let file_name = entry.file_name().to_string_lossy();
    
    // Filtrage plus permissif - seulement les dossiers système critiques
    let should_skip = 
        // Dossiers système macOS critiques uniquement
        path_str.contains("/System/") ||
        path_str.contains("/private/") ||
        path_str.contains(".Trashes") ||
        path_str.contains(".fseventsd") ||
        path_str.contains(".TemporaryItems") ||
        path_str.contains(".db") ||
        
        // Dossiers système Windows critiques uniquement
        path_str.contains("$RECYCLE.BIN") ||
        path_str.contains("System Volume Information") ||
        path_str.contains("Windows\\System32\\") ||
        path_str.contains("AppData\\Local\\Temp\\") ||
        
        // Dossiers système Unix/Linux critiques uniquement
        path_str.contains("/proc/") ||
        path_str.contains("/sys/") ||
        
        // Dossiers de développement volumineux
        path_str.contains("/node_modules/") ||
        path_str.contains(".git/") ||
        path_str.contains(".vscode/") ||
        path_str.contains(".idea/") ||
        path_str.contains("/dist/") ||
        path_str.contains("/build/") ||
        path_str.contains("/target/") ||
        path_str.contains("/tmp/") ||
        path_str.contains("/var/") ||
        path_str.contains("/private/") ||

        // Fichiers temporaires système uniquement
        file_name.ends_with(".tmp") ||
        file_name.ends_with(".temp") ||
        file_name.starts_with("~$") ||
        file_name.ends_with(".DS_Store");

    
    should_skip
}

// Fonctions utilitaires pour l'extraction des métadonnées

fn is_hidden_file(path: &Path) -> bool {
    // Vérification des attributs cachés selon le système d'exploitation
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            return metadata.file_attributes() & 0x2 != 0; // FILE_ATTRIBUTE_HIDDEN
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Sur Unix/Linux, vérifier si le nom commence par un point
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }
}

fn is_system_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Fichiers système courants
    path_str.contains("system") ||
    path_str.contains("windows") ||
    path_str.contains("program files") ||
    path_str.contains("programdata") ||
    path_str.contains("/usr/") ||
    path_str.contains("/bin/") ||
    path_str.contains("/sbin/") ||
    path_str.contains("/lib/") ||
    path_str.contains("/etc/") ||
    path_str.contains("/var/") ||
    path_str.contains("/tmp/") ||
    path_str.contains("/proc/") ||
    path_str.contains("/sys/") ||
    path_str.contains(".system") ||
    path_str.contains(".sys")
}

fn extract_owner_info(metadata: &fs::Metadata) -> (Option<String>, Option<String>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        
        // Sur Unix, on peut essayer de récupérer les noms d'utilisateur et de groupe
        // Note: Cette implémentation est simplifiée, en production vous pourriez
        // utiliser une bibliothèque comme `users` pour une meilleure gestion
        let uid = metadata.uid();
        let gid = metadata.gid();
        
        // Pour l'instant, on retourne les IDs numériques
        (Some(uid.to_string()), Some(gid.to_string()))
    }
    
    #[cfg(not(unix))]
    {
        // Sur Windows, ces informations ne sont pas facilement accessibles
        (None, None)
    }
}

fn determine_mime_type(path: &Path) -> Option<String> {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    let mime_map: HashMap<&str, &str> = [
        // Images
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("png", "image/png"),
        ("gif", "image/gif"),
        ("webp", "image/webp"),
        ("svg", "image/svg+xml"),
        ("bmp", "image/bmp"),
        ("ico", "image/x-icon"),
        
        // Documents
        ("pdf", "application/pdf"),
        ("doc", "application/msword"),
        ("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        ("xls", "application/vnd.ms-excel"),
        ("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        ("ppt", "application/vnd.ms-powerpoint"),
        ("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        
        // Texte
        ("txt", "text/plain"),
        ("md", "text/markdown"),
        ("html", "text/html"),
        ("css", "text/css"),
        ("js", "application/javascript"),
        ("json", "application/json"),
        ("xml", "application/xml"),
        ("csv", "text/csv"),
        ("log", "text/plain"),
        
        // Code
        ("rs", "text/x-rust"),
        ("py", "text/x-python"),
        ("java", "text/x-java-source"),
        ("cpp", "text/x-c++src"),
        ("c", "text/x-csrc"),
        ("go", "text/x-go"),
        ("php", "text/x-php"),
        ("rb", "text/x-ruby"),
        ("swift", "text/x-swift"),
        ("kt", "text/x-kotlin"),
        
        // Archives
        ("zip", "application/zip"),
        ("rar", "application/vnd.rar"),
        ("7z", "application/x-7z-compressed"),
        ("tar", "application/x-tar"),
        ("gz", "application/gzip"),
        
        // Audio/Video
        ("mp3", "audio/mpeg"),
        ("wav", "audio/wav"),
        ("mp4", "video/mp4"),
        ("avi", "video/x-msvideo"),
        ("mkv", "video/x-matroska"),
        
        // Autres
        ("exe", "application/x-executable"),
        ("dll", "application/x-msdownload"),
        ("so", "application/x-sharedlib"),
        ("dylib", "application/x-mach-binary"),
    ].iter().cloned().collect();
    
    extension.and_then(|ext| mime_map.get(ext.as_str()).map(|&mime| mime.to_string()))
}

fn is_text_file(path: &Path) -> bool {
    let mime_type = determine_mime_type(path);
    mime_type.map(|mime| mime.starts_with("text/") || 
        mime.contains("json") || 
        mime.contains("xml") || 
        mime.contains("javascript") ||
        mime.contains("markdown") ||
        mime.contains("csv") ||
        mime.contains("rust") ||
        mime.contains("python") ||
        mime.contains("java") ||
        mime.contains("c++") ||
        mime.contains("c") ||
        mime.contains("go") ||
        mime.contains("php") ||
        mime.contains("ruby") ||
        mime.contains("swift") ||
        mime.contains("kotlin"))
        .unwrap_or(false)
}

fn extract_text_stats(path: &Path) -> (Option<u32>, Option<u32>) {
    // Limiter la taille pour éviter de lire des fichiers trop gros
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > 10 * 1024 * 1024 { // 10MB max
            return (None, None);
        }
    }
    
    match fs::File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut line_count = 0u32;
            let mut word_count = 0u32;
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    line_count += 1;
                    word_count += line.split_whitespace().count() as u32;
                    
                    // Limiter le traitement pour éviter les performances
                    if line_count > 100_000 {
                        break;
                    }
                }
            }
            
            (Some(line_count), Some(word_count))
        }
        Err(_) => (None, None)
    }
}

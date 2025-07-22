use std::{path::PathBuf, process::Command};

pub fn open_file_in_explorer(path: String) -> Result<(), String> {  

    #[cfg(target_os = "windows")]
    {
        println!("Ouverture du chemin: {}", path);
        
        // Check if the path is already in the extended-length path format (\\?\)
        let (path_to_use, is_unc_path) = if path.starts_with("\\\\?\\") {
            // Use the path as-is for extended paths
            (path.clone(), true)
        } else {
            // For regular paths, canonicalize to handle relative paths
            let path_buf = PathBuf::from(&path);
            match path_buf.canonicalize() {
                Ok(canonical_path) => (canonical_path.to_string_lossy().to_string(), false),
                Err(e) => {
                    // If canonicalization fails, try to use the original path
                    println!("Warning: Impossible de résoudre le chemin: {}", e);
                    (path.clone(), false)
                }
            }
        };
        
        // Check if the path is a directory or a file
        let is_dir = match std::fs::metadata(&path_to_use) {
            Ok(metadata) => metadata.is_dir(),
            Err(e) => {
                println!("Warning: Impossible d'obtenir les métadonnées: {}", e);
                // For UNC paths, assume it's a file if we can't determine
                false
            }
        };
        
        let mut command = Command::new("explorer");
        
        if is_dir {
            // For directories, open the directory directly
            println!("Ouverture du dossier: {}", path_to_use);
            command.arg(&path_to_use);
        } else {
            // For files, select the file in explorer
            println!("Ouverture du fichier: {}", path_to_use);
            
            // For UNC paths, try using the shell32 API via rundll32
            if is_unc_path {
                command = Command::new("rundll32");
                command.args(["shell32.dll,ShellExecute", &path_to_use]);
            } else {
                // Regular approach for normal paths
                command.arg("/select,");
                command.arg(&path_to_use);
            }
        }
        
        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                // If the command fails, try an alternative approach
                println!("Erreur lors de l'exécution de la commande: {}", e);
                
                // Try using ShellExecute for any path as a fallback
                let mut alt_command = Command::new("rundll32");
                alt_command.args(["shell32.dll,ShellExecute", &path_to_use]);
                
                alt_command.output()
                    .map_err(|e| format!("Erreur lors de l'exécution de la commande alternative: {}", e))?
            }
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let error_msg = if stderr.trim().is_empty() {
                format!("Explorer a retourné une erreur (code: {:?})", output.status.code())
            } else {
                format!("Explorer a retourné une erreur: {}", stderr)
            };
            return Err(error_msg);
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        let path_buf = PathBuf::from(&path);
    
        // Check if the path exists and is a directory
        let is_dir = std::fs::metadata(&path_buf)
            .map(|m| m.is_dir())
            .unwrap_or(false);
    
        if is_dir {
            // For directories, open the directory directly
            println!("Ouverture du dossier: {}", path);
            Command::new("open")
                .arg(&path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            // For files, reveal in Finder
            println!("Ouverture du fichier: {}", path);
            Command::new("open")
                .args(["-R", &path])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .args([&path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
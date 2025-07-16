use std::{path::PathBuf, process::Command};

pub fn open_file_in_explorer(path: String) -> Result<(), String> {  

    #[cfg(target_os = "windows")]
    {
        let path_buf = PathBuf::from(&path);
        let canonical_path = path_buf.canonicalize()
            .map_err(|e| format!("Impossible de résoudre le chemin: {}", e))?;
        
        let path_str = canonical_path.to_string_lossy();
        
        println!("Ouverture du fichier: {}", path_str);
        
        let mut command = Command::new("explorer");
        command.arg("/select,");
        command.arg(path_str.as_ref());
        
        let output = command.output()
            .map_err(|e| format!("Erreur lors de l'exécution d'explorer: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Explorer a retourné une erreur: {}", stderr));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
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
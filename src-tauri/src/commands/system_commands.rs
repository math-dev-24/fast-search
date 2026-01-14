use crate::domain::entities::stat::Stat;
use crate::shared::config::AppState;
use crate::shared::helpers::with_service_repository_readonly;

#[tauri::command]
pub fn get_stat(state: tauri::State<'_, AppState>) -> Result<Stat, String> {
    with_service_repository_readonly(&state, |repo| repo.get_stat())
}


#[tauri::command]
pub fn diagnose_scan_issues(paths: Vec<String>) -> Result<Vec<String>, String> {
    let mut issues = Vec::new();

    for path in paths {
        let path_obj = std::path::Path::new(&path);

        if !path_obj.exists() {
            issues.push(format!("Chemin inexistant: {}", path));
            continue;
        }

        if !path_obj.is_dir() {
            issues.push(format!("Le chemin n'est pas un dossier: {}", path));
            continue;
        }

        // Vérifier les permissions
        if let Err(e) = std::fs::metadata(path_obj) {
            issues.push(format!("Erreur d'accès au dossier {}: {}", path, e));
            continue;
        }

        // Vérifier la taille du dossier (approximative)
        let mut total_size = 0u64;
        let mut file_count = 0u64;

        if let Ok(entries) = std::fs::read_dir(path_obj) {
            for entry in entries.take(1000) {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            total_size += metadata.len();
                            file_count += 1;
                        }
                    }
                }
            }

            if total_size > 1024 * 1024 * 1024 { // 1GB
                issues.push(format!("Dossier très volumineux détecté: {} ({} fichiers, {} MB)",
                                    path, file_count, total_size / (1024 * 1024)));
            }
        }
    }

    if issues.is_empty() {
        issues.push("Aucun problème détecté avec les chemins fournis".to_string());
    }

    Ok(issues)
}
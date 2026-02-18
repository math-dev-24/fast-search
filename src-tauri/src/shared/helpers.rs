use crate::shared::config::AppState;
use crate::shared::errors::AppError;
use crate::domain::services::file_service::FileService;
use crate::infrastructure::repository::sqlite::Db;

/// Helper pour accéder au service repository de manière DRY (lecture seule)
pub fn with_service_repository_readonly<F, T>(state: &tauri::State<'_, AppState>, f: F) -> Result<T, String>
where
    F: FnOnce(&FileService<Db>) -> Result<T, AppError>,
{
    let repo = match state.service_repository.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("Service repository lock was poisoned (readonly), recovering to avoid permanent failure");
            poisoned.into_inner()
        }
    };
    f(&repo).map_err(|e| e.to_string())
}

/// Helper pour accéder au service repository de manière DRY (lecture/écriture)
pub fn with_service_repository<F, T>(state: &tauri::State<'_, AppState>, f: F) -> Result<T, String>
where
    F: FnOnce(&mut FileService<Db>) -> Result<T, AppError>,
{
    let mut repo = match state.service_repository.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("Service repository lock was poisoned (readwrite), recovering to avoid permanent failure");
            poisoned.into_inner()
        }
    };
    f(&mut repo).map_err(|e| e.to_string())
}

use tokio_retry::{strategy::ExponentialBackoff, Retry};
use std::time::Duration;
use crate::shared::errors::{AppError, AppResult};

/// Retry une opération avec backoff exponentiel
pub async fn retry_with_backoff<F, Fut, T>(operation: F) -> AppResult<T>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = AppResult<T>> + Send,
{
    let strategy = ExponentialBackoff::from_millis(100)
        .max_delay(Duration::from_secs(5))
        .take(3);

    Retry::spawn(strategy, || {
        let result = operation();
        async move { result.await }
    })
    .await
}

/// Exécute une opération avec un timeout
pub async fn with_timeout<F, T>(
    operation: F,
    timeout: Duration,
) -> AppResult<T>
where
    F: std::future::Future<Output = AppResult<T>>,
{
    match tokio::time::timeout(timeout, operation).await {
        Ok(result) => result,
        Err(_) => {
            tracing::warn!("Operation timed out after {:?}", timeout);
            Err(AppError::Internal(
                format!("Operation timed out after {:?}", timeout)
            ))
        }
    }
}

/// Exécute une opération DB synchrone avec retry et timeout (via tokio::task::spawn_blocking)
pub async fn db_operation_with_retry<F, T>(
    operation: F,
    timeout: Duration,
) -> AppResult<T>
where
    F: Fn() -> AppResult<T> + Send + Sync + 'static,
    T: Send + 'static,
{
    use std::sync::Arc;
    let op = Arc::new(operation);
    let op_clone = op.clone();
    
    with_timeout(
        async move {
            retry_with_backoff(move || {
                let op_inner = op_clone.clone();
                async move {
                    tokio::task::spawn_blocking(move || op_inner()).await
                        .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))?
                }
            }).await
        },
        timeout
    ).await
}

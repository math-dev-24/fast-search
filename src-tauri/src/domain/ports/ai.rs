use crate::domain::entities::ai::{AiRequest, AiResponse};
use async_trait::async_trait;
use crate::shared::errors::AppResult;

#[async_trait]
pub trait Ai: Send + Sync {
    async fn generate(&self, request: AiRequest) -> AppResult<AiResponse>;
    async fn list_models(&self) -> AppResult<Vec<String>>;
    async fn health_check(&self) -> AppResult<bool>;
}
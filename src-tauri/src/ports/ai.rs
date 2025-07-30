use crate::entities::ai::{AiRequest, AiResponse, AiError};
use async_trait::async_trait;


#[async_trait]
pub trait Ai: Send + Sync {
    async fn generate(&self, request: AiRequest) -> Result<AiResponse, AiError>;
    async fn list_models(&self) -> Result<Vec<String>, AiError>;
    async fn health_check(&self) -> Result<bool, AiError>;
}
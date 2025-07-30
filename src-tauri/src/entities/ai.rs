use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub model_used: String,
    pub tokens_used: Option<u32>,
}


#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
}
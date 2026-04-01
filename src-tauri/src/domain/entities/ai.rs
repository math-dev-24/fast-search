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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiProvider {
    LmStudio,
    Ollama,
    OpenAi,
    Anthropic,
    Mistral,
}

impl AiProvider {
    pub fn keyring_key(&self) -> &'static str {
        match self {
            AiProvider::LmStudio => "lmstudio",
            AiProvider::Ollama => "ollama",
            AiProvider::OpenAi => "openai",
            AiProvider::Anthropic => "anthropic",
            AiProvider::Mistral => "mistral",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCredentialsRef {
    pub credential_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub provider: AiProvider,
    pub endpoint: String,
    pub model: Option<String>,
    pub credential_ref: Option<CloudCredentialsRef>,
}


#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
}
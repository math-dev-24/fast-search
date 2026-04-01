use std::sync::Arc;

use crate::domain::entities::ai::{AiError, AiProvider, AiProviderConfig};
use crate::domain::ports::ai::Ai;
use crate::infrastructure::ai::{Anthropic, LmStudio, Mistral, Ollama, OpenAi};
use crate::shared::errors::{AppError, AppResult};

pub struct AiFactory;

impl AiFactory {
    pub fn create(config: &AiProviderConfig, api_key: Option<String>) -> AppResult<Arc<dyn Ai>> {
        let adapter: Arc<dyn Ai> = match config.provider {
            AiProvider::LmStudio => Arc::new(LmStudio::new(
                Some(config.endpoint.clone()),
                config.model.clone(),
            )),
            AiProvider::Ollama => Arc::new(Ollama::new(
                Some(config.endpoint.clone()),
                config.model.clone(),
            )),
            AiProvider::OpenAi => Arc::new(OpenAi::new(
                Some(config.endpoint.clone()),
                config.model.clone(),
                Self::require_api_key(api_key, "OpenAI")?,
            )),
            AiProvider::Anthropic => Arc::new(Anthropic::new(
                Some(config.endpoint.clone()),
                config.model.clone(),
                Self::require_api_key(api_key, "Anthropic")?,
            )),
            AiProvider::Mistral => Arc::new(Mistral::new(
                Some(config.endpoint.clone()),
                config.model.clone(),
                Self::require_api_key(api_key, "Mistral")?,
            )),
        };

        Ok(adapter)
    }

    fn require_api_key(api_key: Option<String>, provider_name: &str) -> AppResult<String> {
        match api_key {
            Some(key) if !key.trim().is_empty() => Ok(key),
            _ => Err(AppError::Ai(AiError::AuthError(format!(
                "{provider_name} API key is required"
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_config() -> AiProviderConfig {
        AiProviderConfig {
            provider: AiProvider::LmStudio,
            endpoint: "http://localhost:1234".to_string(),
            model: Some("local-model".to_string()),
            credential_ref: None,
        }
    }

    #[test]
    fn creates_local_adapter_without_api_key() {
        let adapter = AiFactory::create(&local_config(), None);
        assert!(adapter.is_ok());
    }

    #[test]
    fn fails_for_cloud_provider_without_api_key() {
        let config = AiProviderConfig {
            provider: AiProvider::OpenAi,
            endpoint: "https://api.openai.com".to_string(),
            model: Some("gpt-4o-mini".to_string()),
            credential_ref: None,
        };
        let adapter = AiFactory::create(&config, None);
        assert!(adapter.is_err());
    }
}

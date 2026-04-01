use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::domain::entities::ai::{AiError, AiRequest, AiResponse};
use crate::domain::ports::ai::Ai;
use crate::shared::errors::{AppError, AppResult};

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

pub struct Ollama {
    client: Client,
    base_url: String,
    default_model: String,
}

impl Ollama {
    pub fn new(base_url: Option<String>, default_model: Option<String>) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self {
            client,
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            default_model: default_model.unwrap_or_else(|| "llama3.2".to_string()),
        }
    }
}

#[async_trait]
impl Ai for Ollama {
    async fn generate(&self, request: AiRequest) -> AppResult<AiResponse> {
        let payload = OllamaRequest {
            model: request.model.unwrap_or_else(|| self.default_model.clone()),
            messages: vec![
                OllamaMessage {
                    role: "system".to_string(),
                    content: include_str!("../../../data/prompt.txt").to_string(),
                },
                OllamaMessage {
                    role: "user".to_string(),
                    content: request.prompt,
                },
            ],
            stream: false,
            options: OllamaOptions {
                temperature: request.temperature.unwrap_or(0.7),
                num_predict: request.max_tokens.unwrap_or(500),
            },
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Ollama request failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<OllamaResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        Ok(AiResponse {
            content: body.message.content,
            model_used: payload.model,
            tokens_used: None,
        })
    }

    async fn list_models(&self) -> AppResult<Vec<String>> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Ollama list models failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<OllamaTagsResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        Ok(body.models.into_iter().map(|m| m.name).collect())
    }

    async fn health_check(&self) -> AppResult<bool> {
        self.list_models().await.map(|_| true)
    }
}

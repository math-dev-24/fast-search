use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::entities::ai::{AiError, AiRequest, AiResponse};
use crate::domain::ports::ai::Ai;
use crate::shared::errors::{AppError, AppResult};

#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelsResponse {
    data: Vec<OpenAiModel>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModel {
    id: String,
}

pub struct OpenAi {
    client: Client,
    base_url: String,
    default_model: String,
    api_key: String,
}

impl OpenAi {
    pub fn new(base_url: Option<String>, default_model: Option<String>, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com".to_string()),
            default_model: default_model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
            api_key,
        }
    }
}

#[async_trait]
impl Ai for OpenAi {
    async fn generate(&self, request: AiRequest) -> AppResult<AiResponse> {
        let payload = OpenAiRequest {
            model: request.model.unwrap_or_else(|| self.default_model.clone()),
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: include_str!("../../../data/prompt.txt").to_string(),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: request.prompt,
                },
            ],
            temperature: request.temperature.unwrap_or(0.7),
            max_tokens: request.max_tokens.unwrap_or(500),
        };

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "OpenAI request failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<OpenAiResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        let choice = body
            .choices
            .first()
            .ok_or_else(|| AiError::ParsingError("No choices in OpenAI response".to_string()))?;

        Ok(AiResponse {
            content: choice.message.content.clone(),
            model_used: body.model.unwrap_or_else(|| "unknown".to_string()),
            tokens_used: body.usage.map(|u| u.total_tokens),
        })
    }

    async fn list_models(&self) -> AppResult<Vec<String>> {
        let response = self
            .client
            .get(format!("{}/v1/models", self.base_url))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "OpenAI list models failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<OpenAiModelsResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        Ok(body.data.into_iter().map(|m| m.id).collect())
    }

    async fn health_check(&self) -> AppResult<bool> {
        self.list_models().await.map(|_| true)
    }
}

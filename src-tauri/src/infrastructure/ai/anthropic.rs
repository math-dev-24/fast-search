use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::entities::ai::{AiError, AiRequest, AiResponse};
use crate::domain::ports::ai::Ai;
use crate::shared::errors::{AppError, AppResult};

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModelsResponse {
    data: Vec<AnthropicModel>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModel {
    id: String,
}

pub struct Anthropic {
    client: Client,
    base_url: String,
    default_model: String,
    api_key: String,
}

impl Anthropic {
    pub fn new(base_url: Option<String>, default_model: Option<String>, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.anthropic.com".to_string()),
            default_model: default_model.unwrap_or_else(|| "claude-3-5-sonnet-latest".to_string()),
            api_key,
        }
    }
}

#[async_trait]
impl Ai for Anthropic {
    async fn generate(&self, request: AiRequest) -> AppResult<AiResponse> {
        let payload = AnthropicRequest {
            model: request.model.unwrap_or_else(|| self.default_model.clone()),
            max_tokens: request.max_tokens.unwrap_or(500),
            temperature: request.temperature.unwrap_or(0.7),
            system: include_str!("../../../data/prompt.txt").to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: request.prompt,
            }],
        };

        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Anthropic request failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<AnthropicResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        let content = body
            .content
            .iter()
            .find(|part| part.content_type == "text")
            .and_then(|part| part.text.clone())
            .ok_or_else(|| AiError::ParsingError("No text content in Anthropic response".to_string()))?;

        Ok(AiResponse {
            content,
            model_used: body.model.unwrap_or_else(|| "unknown".to_string()),
            tokens_used: None,
        })
    }

    async fn list_models(&self) -> AppResult<Vec<String>> {
        let response = self
            .client
            .get(format!("{}/v1/models", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Anthropic list models failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<AnthropicModelsResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        let models: Vec<String> = body.data.into_iter().map(|m| m.id).collect();
        if models.is_empty() {
            return Ok(vec![self.default_model.clone()]);
        }
        Ok(models)
    }

    async fn health_check(&self) -> AppResult<bool> {
        Ok(!self.api_key.is_empty())
    }
}

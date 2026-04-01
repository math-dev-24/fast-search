use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::domain::entities::ai::{AiError, AiRequest, AiResponse};
use crate::domain::ports::ai::Ai;
use crate::shared::errors::{AppError, AppResult};

#[derive(Debug, Serialize)]
struct MistralRequest {
    model: String,
    messages: Vec<MistralMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MistralMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MistralResponse {
    choices: Vec<MistralChoice>,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralChoice {
    message: MistralMessage,
}

#[derive(Debug, Deserialize)]
struct MistralModelsResponse {
    data: Vec<MistralModel>,
}

#[derive(Debug, Deserialize)]
struct MistralModel {
    id: String,
}

pub struct Mistral {
    client: Client,
    base_url: String,
    default_model: String,
    api_key: String,
}

impl Mistral {
    pub fn new(base_url: Option<String>, default_model: Option<String>, api_key: String) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self {
            client,
            base_url: base_url.unwrap_or_else(|| "https://api.mistral.ai".to_string()),
            default_model: default_model.unwrap_or_else(|| "mistral-small-latest".to_string()),
            api_key,
        }
    }
    
    fn should_retry_status(status: reqwest::StatusCode) -> bool {
        status.as_u16() == 429 || status.is_server_error()
    }
}

#[async_trait]
impl Ai for Mistral {
    async fn generate(&self, request: AiRequest) -> AppResult<AiResponse> {
        let payload = MistralRequest {
            model: request.model.unwrap_or_else(|| self.default_model.clone()),
            messages: vec![
                MistralMessage {
                    role: "system".to_string(),
                    content: include_str!("../../../data/prompt.txt").to_string(),
                },
                MistralMessage {
                    role: "user".to_string(),
                    content: request.prompt,
                },
            ],
            temperature: request.temperature.unwrap_or(0.7),
            max_tokens: request.max_tokens.unwrap_or(500),
        };

        let mut attempt = 0u8;
        let response = loop {
            let resp = self
                .client
                .post(format!("{}/v1/chat/completions", self.base_url))
                .bearer_auth(&self.api_key)
                .json(&payload)
                .send()
                .await
                .map_err(|e| AiError::ConnectionError(e.to_string()))?;
            if Self::should_retry_status(resp.status()) && attempt < 2 {
                attempt += 1;
                tokio::time::sleep(Duration::from_millis(200 * u64::from(attempt))).await;
                continue;
            }
            break resp;
        };

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Mistral request failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<MistralResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        let choice = body
            .choices
            .first()
            .ok_or_else(|| AiError::ParsingError("No choices in Mistral response".to_string()))?;

        Ok(AiResponse {
            content: choice.message.content.clone(),
            model_used: body.model.unwrap_or_else(|| "unknown".to_string()),
            tokens_used: None,
        })
    }

    async fn list_models(&self) -> AppResult<Vec<String>> {
        let mut attempt = 0u8;
        let response = loop {
            let resp = self
                .client
                .get(format!("{}/v1/models", self.base_url))
                .bearer_auth(&self.api_key)
                .send()
                .await
                .map_err(|e| AiError::ConnectionError(e.to_string()))?;
            if Self::should_retry_status(resp.status()) && attempt < 2 {
                attempt += 1;
                tokio::time::sleep(Duration::from_millis(200 * u64::from(attempt))).await;
                continue;
            }
            break resp;
        };

        if !response.status().is_success() {
            return Err(AppError::Ai(AiError::RequestFailed(format!(
                "Mistral list models failed with status {}",
                response.status()
            ))));
        }

        let body = response
            .json::<MistralModelsResponse>()
            .await
            .map_err(|e| AiError::ParsingError(e.to_string()))?;

        Ok(body.data.into_iter().map(|m| m.id).collect())
    }

    async fn health_check(&self) -> AppResult<bool> {
        Ok(!self.api_key.is_empty())
    }
}

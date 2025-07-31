use crate::ports::ai::Ai;
use crate::entities::ai::{AiRequest, AiResponse, AiError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize)]
struct LMStudioRequest {
    model: String,
    messages: Vec<LMStudioMessage>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct LMStudioMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct LMStudioResponse {
    choices: Vec<LMStudioChoice>,
    model: String,
    usage: Option<LMStudioUsage>,
}

#[derive(Debug, Deserialize)]
struct LMStudioModel {
    id: String,
    object: String,
    owned_by: String,
}

#[derive(Debug, Deserialize)]
struct LMStudioModelsResponse {
    data: Vec<LMStudioModel>,
}

#[derive(Debug, Deserialize)]
struct LMStudioChoice {
    message: LMStudioMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LMStudioUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}


#[derive(Debug, Deserialize)]
struct LMStudioErrorResponse {
    error: LMStudioErrorDetail,
}

#[derive(Debug, Deserialize)]
struct LMStudioErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}


pub struct LmStudio {
    client: Client,
    base_url: String,
    default_model: String,
}

impl LmStudio {
    pub fn new(base_url: Option<String>, default_model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "http://localhost:1234".to_string()),
            default_model: default_model.unwrap_or_else(|| "local-model".to_string()),
        }
    }

    fn build_request(&self, request: AiRequest) -> LMStudioRequest {
        let prompt = include_str!("../../../data/prompt.txt").to_string();

        LMStudioRequest {
            model: request.model.unwrap_or_else(|| self.default_model.clone()),
            messages: vec![
                LMStudioMessage {
                    role: "system".to_string(),
                    content: prompt,
                },
            LMStudioMessage {
                role: "user".to_string(),
                content: request.prompt,
            }],
            temperature: request.temperature.unwrap_or(0.7),
            max_tokens: request.max_tokens.unwrap_or(500),
            stream: false,
        }
    }

    async fn handle_error_response(&self, response: reqwest::Response) -> AiError {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        
        if let Ok(error_response) = serde_json::from_str::<LMStudioErrorResponse>(&text) {
            AiError::RequestFailed(format!(
                "LM Studio error ({}): {}",
                status,
                error_response.error.message
            ))
        } else {
            AiError::RequestFailed(format!(
                "HTTP {} - {}",
                status,
                if text.is_empty() { "Unknown error" } else { &text }
            ))
        }
    }
}


#[async_trait]
impl Ai for LmStudio {
    async fn generate(&self, request: AiRequest) -> Result<AiResponse, AiError> {
        let lm_studio_request = self.build_request(request);

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Content-Type", "application/json")
            .json(&lm_studio_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    AiError::ConnectionError(format!(
                        "Cannot connect to LM Studio at {}: {}",
                        self.base_url, e
                    ))
                } else if e.is_timeout() {
                    AiError::ConnectionError("Request timeout".to_string())
                } else {
                    AiError::ConnectionError(format!("Network error: {}", e))
                }
            })?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }

        let response_body = response
            .json::<LMStudioResponse>()
            .await
            .map_err(|e| AiError::RequestFailed(format!("Failed to parse response: {}", e)))?;

        let choice = response_body
            .choices
            .first()
            .ok_or_else(|| AiError::ParsingError("No choices in response".to_string()))?;

        let content = choice.message.content.clone();

        Ok(AiResponse {
            content,
            model_used: response_body.model,
            tokens_used: response_body.usage.map(|usage| usage.total_tokens),
        })

    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        let response = self
            .client
            .get(format!("{}/v1/models", self.base_url))
            .send()
            .await
            .map_err(|e| AiError::ConnectionError(format!("Failed to list models: {}", e)))?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }
        
        let response_body = response
            .json::<LMStudioModelsResponse>()
            .await
            .map_err(|e| AiError::RequestFailed(format!("Failed to parse response: {}", e)))?;

        let models = response_body.data.iter().map(|model| model.id.clone()).collect();

        Ok(models)
    }

    async fn health_check(&self) -> Result<bool, AiError> {
        match self.list_models().await {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }
}
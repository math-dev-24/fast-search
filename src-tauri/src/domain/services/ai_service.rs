use std::sync::Arc;
use crate::domain::entities::ai::{AiError, AiRequest};
use crate::domain::entities::search::SearchQuery;
use crate::domain::ports::ai::Ai;
use crate::shared::errors::AppResult;

pub struct AiService {
    ai_port: Arc<dyn Ai>,
}


impl AiService {
    pub fn new(ai_port: Arc<dyn Ai>) -> Self {
        Self { ai_port }
    }

    pub async fn generate(&self, prompt: &str) -> AppResult<SearchQuery> {
        let request = AiRequest {
            prompt: prompt.to_string(),
            model: None,
            temperature: Some(0.7),
            max_tokens: Some(500),
        };

        let response = self.ai_port.generate(request).await?;

        println!("AI Response content: {}", response.content);

        let cleaned_content = response.content
            .trim()
            .replace("\\{", "{")
            .replace("\\}", "}")
            .replace("\\\"", "\"")
            .replace("\\n", "")
            .replace("\\t", "");

        println!("Cleaned content: {}", cleaned_content);

        let search_query = serde_json::from_str::<SearchQuery>(&cleaned_content)
            .map_err(|e| {
                println!("JSON parsing error: {}", e);
                println!("Failed to parse content: {}", cleaned_content);
                AiError::ParsingError(format!("Failed to parse AI response as SearchQuery: {}", e))
            })?;

        println!("Parsed search_query: {:?}", search_query);

        Ok(search_query)
    }

    pub async fn list_models(&self) -> AppResult<Vec<String>> {
        self.ai_port.list_models().await
    }

    pub async fn health_check(&self) -> AppResult<bool> {
        self.ai_port.health_check().await
    }

    pub async fn model_is_available(&self, model: &str) -> AppResult<bool> {
        let available_models = match self.ai_port.list_models().await {
            Ok(models) => models,
            Err(e) => return Err(e)
        };
        Ok(available_models.iter().any(|m| m == model))
    }
}
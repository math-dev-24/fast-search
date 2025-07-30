use crate::ports::ai::Ai;
use crate::entities::ai::{AiRequest, AiError};
use std::sync::Arc;
use crate::entities::search::SearchQuery;


pub struct AiService {
    ai_port: Arc<dyn Ai>,
}


impl AiService {
    pub fn new(ai_port: Arc<dyn Ai>) -> Self {
        Self { ai_port }
    }

    pub async fn generate(&self, prompt: &str) -> Result<SearchQuery, AiError> {
        let request = AiRequest {
            prompt: prompt.to_string(),
            model: None,
            temperature: Some(0.7),
            max_tokens: Some(500),
        };

        let response = self.ai_port.generate(request).await?;

        println!("AI Response content: {}", response.content);

        // Nettoyer la réponse JSON en enlevant les caractères d'échappement
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

    pub async fn list_models(&self) -> Result<Vec<String>, AiError> {
        self.ai_port.list_models().await
    }

    pub async fn health_check(&self) -> Result<bool, AiError> {
        self.ai_port.health_check().await
    }
}
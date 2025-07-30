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
        
        println!("response: {:?}", response);

        Ok(SearchQuery::default())
    }

    pub async fn list_models(&self) -> Result<Vec<String>, AiError> {
        self.ai_port.list_models().await
    }

    pub async fn health_check(&self) -> Result<bool, AiError> {
        self.ai_port.health_check().await
    }
}
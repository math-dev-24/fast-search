use std::sync::Arc;
use crate::infrastructure::ai::LmStudio;
use crate::domain::entities::search::SearchQuery;
use crate::domain::services::ai_service::AiService;

#[tauri::command]
pub async fn ai_search(natural_query: String, model: String, ai_url: String) -> Result<SearchQuery, String> {
    let ai_adapter = LmStudio::new(Some(ai_url), Some(model.clone()));
    let ai_service = AiService::new(Arc::new(ai_adapter));

    let check = ai_service.model_is_available(&model).await;

    if let Err(e) = check {
        return Err(format!("Model {} is not available: {}", model, e));
    }

    let search_query = ai_service.generate(&natural_query).await
        .map_err(|e| format!("AI generation failed: {}", e))?;
    Ok(search_query)
}

#[tauri::command]
pub async fn ai_health_check(ai_url: String) -> Result<bool, String> {
    let ai_adapter = LmStudio::new(Some(ai_url), None);
    let ai_service = AiService::new(Arc::new(ai_adapter));
    let health_check = ai_service.health_check().await
        .map_err(|e| format!("Health check failed: {}", e))?;
    Ok(health_check)
}

#[tauri::command]
pub async fn ai_list_models(ai_url: String) -> Result<Vec<String>, String> {
    let ai_adapter = LmStudio::new(Some(ai_url), None);
    let ai_service = AiService::new(Arc::new(ai_adapter));
    let models = ai_service.list_models().await
        .map_err(|e| format!("Failed to list models: {}", e))?;
    Ok(models)
}
use crate::services::ai_service::AiService;


pub async fn get_available_models(ai_service: &AiService) -> Result<Vec<String>, String> {
    let available_models = ai_service.list_models().await.unwrap();
    Ok(available_models)
}


pub fn check_model_available(model: &str, available_models: &Vec<String>) -> bool {
    available_models.iter().any(|m| m == model)
}






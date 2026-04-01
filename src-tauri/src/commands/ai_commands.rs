use crate::application::factories::ai_factory::AiFactory;
use crate::domain::entities::ai::{AiProvider, AiProviderConfig};
use crate::domain::entities::search::SearchQuery;
use crate::domain::services::ai_service::AiService;
use crate::infrastructure::ai::secrets::AiSecrets;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

type ModelCacheMap = HashMap<String, (Instant, Vec<String>)>;
static MODEL_CACHE: OnceLock<Mutex<ModelCacheMap>> = OnceLock::new();
const MODEL_CACHE_TTL: Duration = Duration::from_secs(30);
const MODEL_CACHE_MAX_ENTRIES: usize = 64;

fn model_cache() -> &'static Mutex<ModelCacheMap> {
    MODEL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[tauri::command]
pub async fn ai_search(natural_query: String, config: AiProviderConfig) -> Result<SearchQuery, String> {
    let api_key = AiSecrets::read_api_key(&config.provider).map_err(String::from)?;
    let ai_adapter = AiFactory::create(&config, api_key).map_err(String::from)?;
    let ai_service = AiService::new(ai_adapter);

    if let Some(model) = &config.model {
        let check = ai_service.model_is_available(model).await;
        if let Err(e) = check {
            return Err(format!("Model {} is not available: {}", model, e));
        }
    }

    let search_query = ai_service.generate(&natural_query).await
        .map_err(|e| format!("AI generation failed: {}", e))?;
    Ok(search_query)
}

#[tauri::command]
pub async fn ai_health_check(config: AiProviderConfig) -> Result<bool, String> {
    let api_key = AiSecrets::read_api_key(&config.provider).map_err(String::from)?;
    let ai_adapter = AiFactory::create(&config, api_key).map_err(String::from)?;
    let ai_service = AiService::new(ai_adapter);
    let health_check = ai_service.health_check().await
        .map_err(|e| format!("Health check failed: {}", e))?;
    Ok(health_check)
}

#[tauri::command]
pub async fn ai_list_models(config: AiProviderConfig) -> Result<Vec<String>, String> {
    let cache_key = format!("{:?}|{}", config.provider, config.endpoint);
    if let Ok(cache) = model_cache().lock() {
        if let Some((inserted_at, models)) = cache.get(&cache_key) {
            if inserted_at.elapsed() < MODEL_CACHE_TTL {
                return Ok(models.clone());
            }
        }
    }

    let api_key = AiSecrets::read_api_key(&config.provider).map_err(String::from)?;
    let ai_adapter = AiFactory::create(&config, api_key).map_err(String::from)?;
    let ai_service = AiService::new(ai_adapter);
    let models = ai_service.list_models().await
        .map_err(|e| format!("Failed to list models: {}", e))?;

    if let Ok(mut cache) = model_cache().lock() {
        // First clear stale entries, then enforce a hard cap.
        cache.retain(|_, (inserted_at, _)| inserted_at.elapsed() < MODEL_CACHE_TTL);
        while cache.len() >= MODEL_CACHE_MAX_ENTRIES {
            let oldest_key = cache
                .iter()
                .max_by_key(|(_, (inserted_at, _))| inserted_at.elapsed())
                .map(|(key, _)| key.clone());
            if let Some(key) = oldest_key {
                cache.remove(&key);
            } else {
                break;
            }
        }
        cache.insert(cache_key, (Instant::now(), models.clone()));
    }

    Ok(models)
}

#[tauri::command]
pub fn ai_save_api_key(provider: AiProvider, api_key: String) -> Result<(), String> {
    AiSecrets::save_api_key(&provider, &api_key).map_err(String::from)
}

#[tauri::command]
pub fn ai_has_api_key(provider: AiProvider) -> Result<bool, String> {
    AiSecrets::read_api_key(&provider)
        .map(|value| value.is_some())
        .map_err(String::from)
}

#[tauri::command]
pub fn ai_delete_api_key(provider: AiProvider) -> Result<(), String> {
    AiSecrets::delete_api_key(&provider).map_err(String::from)
}
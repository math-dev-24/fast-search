use keyring::Entry;

use crate::domain::entities::ai::{AiError, AiProvider};
use crate::shared::errors::{AppError, AppResult};

const SERVICE_NAME: &str = "fast-search-ai";

pub struct AiSecrets;

impl AiSecrets {
    fn entry(provider: &AiProvider) -> AppResult<Entry> {
        Entry::new(SERVICE_NAME, provider.keyring_key())
            .map_err(|e| AppError::Ai(AiError::ConfigError(format!("Keyring init failed: {e}"))))
    }

    pub fn save_api_key(provider: &AiProvider, api_key: &str) -> AppResult<()> {
        Self::entry(provider)?
            .set_password(api_key)
            .map_err(|e| AppError::Ai(AiError::AuthError(format!("Unable to save API key: {e}"))))
    }

    pub fn read_api_key(provider: &AiProvider) -> AppResult<Option<String>> {
        let entry = Self::entry(provider)?;
        match entry.get_password() {
            Ok(value) if !value.trim().is_empty() => Ok(Some(value)),
            Ok(_) => Ok(None),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AppError::Ai(AiError::AuthError(format!(
                "Unable to read API key: {e}"
            )))),
        }
    }

    pub fn delete_api_key(provider: &AiProvider) -> AppResult<()> {
        let entry = Self::entry(provider)?;
        match entry.delete_credential() {
            Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AppError::Ai(AiError::AuthError(format!(
                "Unable to delete API key: {e}"
            )))),
        }
    }
}

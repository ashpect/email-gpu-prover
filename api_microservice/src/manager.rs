use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::errors::ApiError;

#[derive(Clone, Debug)]
pub struct ApiKeyDetails {
    pub key: String,
    pub last_used: SystemTime,
    pub usage_count: u64,
    pub is_active: bool,
    pub max_usage_count: u64,
}

impl ApiKeyDetails {
    pub fn new(key: String) -> Self {
        Self {
            key,
            last_used: SystemTime::now(),
            usage_count: 0,
            is_active: true,
            max_usage_count: 1000,
        }
    }

    pub fn record_usage(&mut self) {
        self.last_used = SystemTime::now();
        self.usage_count += 1;
    }

    pub fn is_usage_limit_reached(&self) -> bool {
        self.usage_count >= self.max_usage_count
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}


#[derive(Debug, Clone)]
pub struct ApiKeyManager {
    keys: Arc<Mutex<HashMap<String, ApiKeyDetails>>>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_key(&self, api_key: String) -> Result<(), ApiError> {
        let mut keys = self.keys.lock().map_err(|e| {
            ApiError::InternalServerError(format!("Failed to acquire lock: {}", e))
        })?;
        
        keys.insert(api_key.clone(), ApiKeyDetails::new(api_key));
        Ok(())
    }

    pub fn validate_and_update_key(&self, api_key: &str) -> Result<bool, ApiError> {
        let mut keys = self.keys.lock().map_err(|e| {
            ApiError::InternalServerError(format!("Failed to acquire lock: {}", e))
        })?;

        if let Some(details) = keys.get_mut(api_key) {
            if !details.is_active() || details.is_usage_limit_reached() {
                return Ok(false);
            }
            details.record_usage();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_key_details(&self, api_key: &str) -> Result<Option<ApiKeyDetails>, ApiError> {
        let keys = self.keys.lock().map_err(|e| {
            ApiError::InternalServerError(format!("Failed to acquire lock: {}", e))
        })?;
        
        Ok(keys.get(api_key).cloned())
    }

    pub fn deactivate_key(&self, api_key: &str) -> Result<bool, ApiError> {
        let mut keys = self.keys.lock().map_err(|e| {
            ApiError::InternalServerError(format!("Failed to acquire lock: {}", e))
        })?;

        if let Some(details) = keys.get_mut(api_key) {
            details.is_active = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}


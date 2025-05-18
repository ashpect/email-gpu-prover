use axum::{
    Router,
    routing::{get, post},
    response::IntoResponse,
    Json,
    extract::State,
};
use slog::info;
use relayer_utils::LOG;
use std::sync::Arc;
use crate::manager::ApiKeyManager;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerifyPayload {
    api_key: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    is_valid: bool,
    message: String,
}

pub async fn health_check() -> impl IntoResponse {

    let json_response = serde_json::json!({
        "status": "success",
    });
    info!(LOG, "Health check");

    Json(json_response)

}

pub async fn verify(
    State(api_key_manager): State<Arc<ApiKeyManager>>,
    Json(payload): Json<VerifyPayload>
) -> impl IntoResponse {
    let validation_result = match api_key_manager.validate_and_update_key(&payload.api_key) {
        Ok(is_valid) => {
            info!(LOG, "API key validation result: {}", is_valid);
            Json(VerifyResponse {
                is_valid,
                message: "API key validation checked".to_string(),
            })
        },
        Err(_e) => {
            Json(VerifyResponse {
                is_valid: false,
                message: "Failed to verify API key".to_string(),
            })
        }
    };

    validation_result
}

pub fn create_router() -> Router {

    // This is for example purposes : 
    let api_key_manager = ApiKeyManager::new();
    for i in 1..=5 {
        let key = format!("mock-api-key-{}", i);
        api_key_manager.add_key(key).expect("Failed to add mock API key");
    }

    api_key_manager.add_key("b9a1f7c2-a4de-4f27-9e88-7d18313fa1f4".to_string())
        .expect("Failed to add UUID API key");
    
    let shared_manager = Arc::new(api_key_manager);
    
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/verify", post(verify))
        .with_state(shared_manager)
}
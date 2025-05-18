use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use relayer_utils::LOG;
use slog::info;
use reqwest;
use serde_json;
use crate::handler::{health_checker_handler, prove_handler};

pub fn create_router(api_verification_url: Arc<String>) -> Router {
    let protected_routes = Router::new()
        .route("/api/prove", post(prove_handler))
        .route_layer(middleware::from_fn_with_state(
            api_verification_url.clone(),
            api_key_middleware,
        ));

    Router::new()
        .route("/api/healthz", get(health_checker_handler))
        .merge(protected_routes)
}

pub async fn api_key_middleware(
    State(api_verification_url): State<Arc<String>>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    if let Some(api_key_header) = req.headers().get("x-api-key") {
        if let Ok(api_key) = api_key_header.to_str() {

            let client = reqwest::Client::new();
            let verify_response = client
                .post(api_verification_url.as_str())
                .json(&serde_json::json!({
                    "api_key": api_key.trim()
                }))
                .send()
                .await;

            match verify_response {
                Ok(response) => {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if json["is_valid"].as_bool().unwrap_or(false) {
                            return next.run(req).await;
                        }
                    }
                }
                Err(e) => {
                    info!(LOG, "Error verifying API key: {}", e);
                }
            }
        }
    }

    info!(LOG, "Invalid or missing API key");
    (StatusCode::UNAUTHORIZED, "Invalid or missing API key").into_response()
}

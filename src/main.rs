mod errors;
mod handler;
mod prove;
mod server;

use std::sync::Arc;

use anyhow::Result;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use dotenv::dotenv;
use relayer_utils::LOG;
use server::create_router;
use slog::info;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let api_verification_url = Arc::new(std::env::var("API_VERIFICATION_URL").expect("API_VERIFICATION_URL must be set"));

    std::fs::create_dir_all("artifacts")?;

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let prover = create_router(api_verification_url).layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    info!(LOG, "Serving prover on port: {}", port);
    axum::serve(listener, prover).await?;

    Ok(())
}

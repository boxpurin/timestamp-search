use crate::api::config::SERVER_CONFIG;
use api::app_state::AppState;
use api::middleware::{access_log_console, use_backet};
use api::route::router;
use api::service::TimeStampSearchService;
use axum::http::{Method, HeaderValue, header::{AUTHORIZATION}};
use leaky_bucket::RateLimiter;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod api;


fn use_cors() -> CorsLayer{
    CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(vec![
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_headers(vec![AUTHORIZATION])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let ts = Arc::new(
        meilisearch::repositories::timestamp_search::create_meilisearch_timestamp_search_repository(
        ),
    );
    let service = TimeStampSearchService::new(ts);

    tracing::trace!("Initialize RateLimiter.");
    let limiter = RateLimiter::builder()
        .interval(core::time::Duration::from_secs(100))
        .initial(500)
        .refill(50)
        .max(1000)
        .build();
    tracing::trace!("RateLimiter initialized. : {:?}", limiter);

    tracing::trace!("Initialize AppState.");
    let state = AppState::new(service, limiter);
    tracing::trace!("AppState initialized.");

    tracing::trace!("Initialize Router.");
    let app = router()
        .layer(use_cors())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            use_backet,
        ))
        .layer(axum::middleware::from_fn(access_log_console))
        .with_state(state.clone());
    tracing::trace!("Router initialized.");
    
    tracing::info!("server start listen addr : {}", SERVER_CONFIG.listen_addr());
    let listener = tokio::net::TcpListener::bind(SERVER_CONFIG.listen_addr()).await?;
    let ret = axum::serve(listener, app).await;

    if let Err(e) = ret {
        tracing::error!("server error: {}", e);
        return Err(e.into());
    }

    tracing::info!("Server shutdown.");
    Ok(())
}

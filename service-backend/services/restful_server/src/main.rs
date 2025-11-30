use crate::api::config::SERVER_CONFIG;
use api::app_state::AppState;
use api::middleware::{access_log_console, use_backet};
use api::route::router;
use api::service::TimeStampSearchService;
use axum::http::{Method, HeaderValue, header::{AUTHORIZATION}};
use leaky_bucket::RateLimiter;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::Level;

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
        .with_max_level(Level::DEBUG)
        .init();

    let ts = Arc::new(
        meilisearch::repositories::timestamp_search::create_meilisearch_timestamp_search_repository(
        ),
    );
    let service = TimeStampSearchService::new(ts);

    let limiter = RateLimiter::builder()
        .interval(core::time::Duration::from_secs(100))
        .initial(500)
        .refill(50)
        .max(1000)
        .build();

    let state = AppState::new(service, limiter);

    let app = router()
        .layer(use_cors())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            use_backet,
        ))
        .layer(axum::middleware::from_fn(access_log_console))
        .with_state(state.clone());
    
    tracing::info!("server start listen addr : {}", SERVER_CONFIG.listen_addr());
    let listener = tokio::net::TcpListener::bind(SERVER_CONFIG.listen_addr()).await?;
    let ret = axum::serve(listener, app).await;

    if let Err(e) = ret {
        tracing::error!("server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

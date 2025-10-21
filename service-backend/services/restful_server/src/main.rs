use crate::api::config::SERVER_CONFIG;
use api::app_state::AppState;
use api::middleware::{access_log_console, use_backet};
use api::route::router;
use api::service::TimeStampSearchService;
use leaky_bucket::RateLimiter;
use std::sync::Arc;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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

    let route = router();

    let app = router()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            use_backet,
        ))
        .layer(axum::middleware::from_fn(access_log_console))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(SERVER_CONFIG.listen_addr()).await?;
    let ret = axum::serve(listener, app).await;

    if let Err(e) = ret {
        tracing::error!("server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

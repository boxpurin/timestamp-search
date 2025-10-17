use crate::api::app_state::AppState;
use crate::api::handle::health::health_check;
use crate::api::handle::timestamp_search::search_timestamp;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    let router = Router::new()
        .merge(build_health_check())
        .merge(build_timestamp_search());

    Router::new().nest("/api/v1", router)
}

fn build_health_check() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

fn build_timestamp_search() -> Router<AppState> {
    Router::new().route("/timestamp/search", get(search_timestamp))
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::sync::Arc;

    use crate::api::middleware::{access_log_console, use_backet};
    use crate::api::service::TimeStampSearchService;
    use axum::{
        body::Body,
        http::{Method, StatusCode},
        middleware,
    };
    use domains::repositories::internal_timestamp_search_repository::{
        InternalVideoTimeStampSearchRepository, VideoTimestampSearchQuery,
        VideoTimestampSearchResult,
    };
    use domains::value_objects::page::Page;
    use domains::value_objects::per_page::PerPage;
    use errors::AppResult;
    use leaky_bucket::RateLimiter;
    use tower::ServiceExt;

    pub struct TestVideoTimeStampSearchRepository {}

    #[async_trait::async_trait]
    impl InternalVideoTimeStampSearchRepository for TestVideoTimeStampSearchRepository {
        async fn search_timestamps_by_query(
            &self,
            _: VideoTimestampSearchQuery,
        ) -> AppResult<VideoTimestampSearchResult> {
            Ok(VideoTimestampSearchResult {
                items: vec![],
                page: Page(1),
                per_page: PerPage(1),
                total_pages: 1,
                total_hits: 0,
            })
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn route_path_test() {
        let ts = Arc::new(TestVideoTimeStampSearchRepository {});
        let service = TimeStampSearchService::new(ts);

        let limiter = RateLimiter::builder()
            .interval(core::time::Duration::from_secs(100))
            .initial(500)
            .refill(50)
            .max(1000)
            .build();
        let state = AppState::new(service, limiter);

        let app = router()
            .layer(middleware::from_fn_with_state(state.clone(), use_backet))
            .layer(middleware::from_fn(access_log_console))
            .with_state(state.clone());

        tracing::debug!("Request test : /api/v1/health");
        let req = axum::http::Request::builder()
            .method(Method::GET)
            .uri("/api/v1/health")
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        tracing::debug!("Request test : /api/v1/timestamp/search");
        let req = axum::http::Request::builder()
            .method(Method::GET)
            .uri("/api/v1/timestamp/search?q=text&page=1&perPage=1")
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}

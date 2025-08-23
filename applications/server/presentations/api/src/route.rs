use axum::routing::get;
use axum::Router;
use crate::handle::health::health_check;


pub fn router() -> Router {
    let router = Router::new()
        .merge(build_health_check());

    Router::new().nest("/api/v1", router)
}

fn build_health_check() -> Router {
    Router::new().route("/health", get(health_check))
}


#[cfg(test)]
mod unit_tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::status::StatusCode;
    use tower::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn route_path_test() {
        let route = router();

        let req = Request::get("/api/v1/health").body(Body::empty()).unwrap();
        let res = route.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }
}
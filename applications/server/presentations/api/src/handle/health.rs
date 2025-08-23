use axum::response::Json;
use serde_json::{json, Value};

/// サーバーのヘルスチェック用
pub(crate) async fn health_check() -> Json<Value> {
    Json(json!( {"message" : "available"}))
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn health_check_test() {
        let ret = health_check().await;
        assert!(ret.get("message").is_some());
    }
}
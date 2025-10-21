use crate::api::app_state::AppState;
use crate::api::request::SearchTimeStampRequest;
use crate::api::response::SearchTimeStampResponse;
use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use errors::AppError;
use garde::Validate;

pub async fn search_timestamp(
    State(state): State<AppState>,
    Query(query): Query<SearchTimeStampRequest>,
) -> Result<Json<SearchTimeStampResponse>, Response> {
    tracing::debug!("Search timestamp: {:?}", Utc::now());
    query.validate().map_err(|e| {
        tracing::error!("Query validation failed. report : {}", e);
        AppError::InvalidInput(format!("invalid query parameter : {}", e)).into_response()
    })?;

    let r = state
        .timestamp_search
        .search_timestamp(query)
        .await;

    match r {
        Ok(r) => Ok(Json(r.into())),
        Err(e) => {
            tracing::error!("Search timestamp failed");
            Err(e.into_response())
        },
    }

}

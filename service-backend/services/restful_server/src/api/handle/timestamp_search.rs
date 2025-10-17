use std::sync::Arc;
use axum::extract::{Query, State, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use garde::Validate;
use errors::{AppError, AppResult};
use crate::api::app_state::AppState;
use crate::api::request::SearchTimeStampRequest;
use crate::api::response::SearchTimeStampResponse;

pub async fn search_timestamp(
    State(state): State<AppState>,
    Query(query): Query<SearchTimeStampRequest>
) -> Result<Json<SearchTimeStampResponse>, Response> {
    tracing::info!("Search timestamp: {:?}", Utc::now());
    query.validate().map_err(|e| {
        AppError::InvalidInput(
            format!("invalid query parameter : {}", e.to_string())
        ).into_response()
    })?;

    let r = state
        .timestamp_search
        .search_timestamp(query)
        .await
        .map_err(|e| {
            e.into_response()
        });

    if let Err(e) = r {
        return Err(AppError::InternalServerError("error".to_string()).into_response())
    };

    Ok(Json(r?.into()))
}
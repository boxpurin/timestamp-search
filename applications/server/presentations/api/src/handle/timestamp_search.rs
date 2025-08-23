use axum::extract::{Query, State};
use axum::Json;
use chrono::Utc;
use garde::Validate;
use errors::{AppError, AppResult};
use crate::app_state::AppState;
use crate::request::SearchTimeStampRequest;
use crate::response::SearchTimeStampResponse;

pub async fn search_timestamp(State(state): State<AppState>, Query(query): Query<SearchTimeStampRequest>) -> AppResult<Json<SearchTimeStampResponse>> {
    tracing::info!("Search timestamp: {:?}", Utc::now());
    query.validate().map_err(|e|
        AppError::InvalidInput(
            format!("invalid query parameter : {}", e.to_string())
        )
    )?;

    let r = state
        .timestamp_search
        .search_timestamp(query)
        .await?
        .into();

    Ok(Json(r))
}
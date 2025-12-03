use crate::api::app_state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum::http::header;
use errors::{AppResult, AppError};

pub async fn access_log_console(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    tracing::info!("Request method : {} , uri : {} ", req.method(), req.uri());

    let res = next.run(req).await;

    tracing::info!("Response StatusCode : {}", res.status());
    Ok(res)
}

/// API全体のRateLimitterのトークンの消費を実行する0
pub async fn use_backet(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    tracing::debug!("use_backet Request : {:?}", req);

    #[allow(clippy::collapsible_if)]
    if let Ok(l) = state.limiter.write() {
        if !l.try_acquire(1) {
            tracing::error!("No token left in limiter");
            return Err((StatusCode::TOO_MANY_REQUESTS, "No token left.").into_response());
        }
    }

    let res = next.run(req).await;
    Ok(res)
}

pub async fn check_auth(req: Request, next: Next) -> AppResult<Response> {
    let headers = req.headers();
    let provided = headers
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?;
    let provided = provided.to_str().map_err(|_| AppError::Unauthorized)?;
    
    let expected = "";
    if provided != expected {
        Ok(next.run(req).await)
    } else {
        Err(AppError::Unauthorized)
    }
}
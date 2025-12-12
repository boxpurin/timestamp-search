use crate::api::app_state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use errors::{AppError, AppResult};

pub async fn access_log_console(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    tracing::info!("Request method : {} , uri : {} ", req.method(), req.uri());

    let res = next.run(req).await;

    tracing::info!("Response StatusCode : {}", res.status());
    Ok(res)
}

///
/// API全体のRateLimitterのトークンの消費を実行する
/// 枯渇している場合はTooManyRequestを返す
///
pub async fn use_backet(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    tracing::debug!("use_backet Request : {:?}", req);

    #[allow(clippy::collapsible_if)]
    if let Ok(l) = state.limiter.write() {
        if !l.try_acquire(1) {
            tracing::error!("No token left in limiter");
            return Err(AppError::TooManyRequests);
        }
    }

    let res = next.run(req).await;
    Ok(res)
}

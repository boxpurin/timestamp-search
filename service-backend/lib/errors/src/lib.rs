use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use google_youtube3::Error as Youtube3Error;
use meilisearch_sdk::errors::Error::{Meilisearch, MeilisearchCommunication};
use meilisearch_sdk::errors::{Error as MeiliSearchError, ErrorType as MeiliSearchErrorType};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("500 Invalid input : input {0}")]
    /// BadRequest
    InvalidInput(String),

    #[error("500 Invalid domain . Domain : {0}")]
    /// InternalServerError
    DomainParseError(String),

    #[error("500 Entity build failed. object : {0}")]
    /// 500 InternalServerError
    EntityBuildFailed(String),

    #[error("Not found : {0}")]
    /// 404
    NotFound(String),

    #[error("500 Internal server error: {0}")]
    /// 500 サーバー内部エラー（予期しない例外等）
    InternalServerError(#[source] anyhow::Error),

    #[error("401 Unauthorized access")]
    /// 401 認証情報が不足または無効である。
    Unauthorized,

    #[error("403 Forbidden access")]
    /// 403 認証は成功しているが、当該リソースへのアクセス権が無い。
    Forbidden,
    
    #[error("429 Too Many Request")]
    /// 429 Too Many Requests
    TooManyRequests,

    #[error("502 Bad Gateway")]
    /// 外部APIやバックエンドサービスとの通信失敗
    BadGateway(#[source] anyhow::Error),

    #[error("503 Service unavailable.")]
    /// 503 サービス過負荷・メンテナンス等で一時的に利用不可
    ServiceUnavailable,
}


impl From<Youtube3Error> for AppError {
    fn from(e: Youtube3Error) -> Self {
        match e {
            _ => AppError::BadGateway(anyhow::anyhow!(e))
        }
    }
}

impl From<MeiliSearchError> for AppError {
    fn from(e: MeiliSearchError) -> AppError {
        match e {
            Meilisearch(e) => match e.error_type {
                MeiliSearchErrorType::InvalidRequest => AppError::InvalidInput(e.error_message),
                _ => AppError::BadGateway(anyhow::anyhow!(e)),
            },
            MeilisearchCommunication(e) => {
                tracing::error!("Communication error with Meilisearch. Status code : {}", e.status_code);
                AppError::BadGateway(anyhow::anyhow!(e))
            },
            MeiliSearchError::ParseError(e) => {
                tracing::error!("Parse error in Meilisearch response: {}", e);
                AppError::BadGateway(anyhow::anyhow!(e))
            },
            MeiliSearchError::Timeout => {
                tracing::error!("Timeout while communicating with Meilisearch");
                AppError::BadGateway(anyhow::anyhow!(e))
            },
            MeiliSearchError::InvalidRequest => {
                tracing::error!("Invalid request to Meilisearch");
                AppError::InvalidInput("Invalid request to Meilisearch".to_string())
            }
            MeiliSearchError::CantUseWithoutApiKey(_) => {
                tracing::error!("Can't use Meilisearch without an API key", );
                AppError::BadGateway(anyhow::anyhow!(e))
            }
            _ => {
                tracing::error!("An unknown error occurred in Meilisearch: {}", e);
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct ResponseBody {
    message : String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        let (status, message) = match self {
            AppError::InvalidInput(_) => {
                (StatusCode::BAD_REQUEST, "500 Bad Request. Invalid input.")
            },
            AppError::DomainParseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal Server Error.")
            },
            AppError::EntityBuildFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal Server Error.")
            },
            AppError::NotFound(_) => {
                (StatusCode::NOT_FOUND, "404 Not Found.")
            },
            AppError::InternalServerError(e) => {
                tracing::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal Server Error.")
            },
            AppError::TooManyRequests => {
                (StatusCode::TOO_MANY_REQUESTS, "429 Too Many Requests.")
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "401 Unauthorized.")
            },
            AppError::Forbidden => {
                (StatusCode::FORBIDDEN, "403 Forbidden.")
            },
            AppError::BadGateway(e) => {
                tracing::error!("{}", e);
                (StatusCode::BAD_GATEWAY, "502 Bad Gateway.")
            },
            AppError::ServiceUnavailable => {
                (StatusCode::SERVICE_UNAVAILABLE, "503 Service Unavailable.")
            },
        };


        (status, axum::Json(ResponseBody{
            message: String::from(message)
        })).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

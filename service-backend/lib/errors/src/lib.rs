use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use google_youtube3::Error as Youtube3Error;
use meilisearch_sdk::errors::Error::{Meilisearch, MeilisearchCommunication};
use meilisearch_sdk::errors::{Error as MeiliSearchError, ErrorType as MeiliSearchErrorType};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid input {0}")]
    InvalidInput(String),

    #[error("Invalid domain {0}")]
    DomainParseError(String),

    #[error("Entity build failed")]
    EntityBuildFailed(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(#[source] anyhow::Error),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Forbidden access: {0}")]
    Forbidden(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// バリデーション失敗時のエラー
    #[error("Validation failure. : {0}")]
    ValidationFailure(&'static str),

    /// 変換エラー
    #[error("Domain parse failure.")]
    ParseFailure,
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::ValidationFailure(_) => {
                tracing::error!("{}", e);
                AppError::InvalidInput(format!("{}", e))
            }
            DomainError::ParseFailure => {
                tracing::error!("Domain Parse failure: {}", e);
                AppError::DomainParseError(format!("{}", e))
            }
        }
    }
}

impl From<Youtube3Error> for AppError {
    fn from(e: Youtube3Error) -> Self {
        match e {
            Youtube3Error::HttpError(e) => {
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
            Youtube3Error::UploadSizeLimitExceeded(i, e) => {
                AppError::InvalidInput(format!(
                    "Upload size limit exceeded: {} bytes. Error: {}",
                    i, e
                ))
            }
            Youtube3Error::BadRequest(v) => {
                AppError::InvalidInput(format!("Bad request to YouTube API: {:?}", v))
            }
            Youtube3Error::MissingAPIKey => {
                tracing::error!("YouTube API key is missing");
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
            Youtube3Error::MissingToken(e) => {
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
            Youtube3Error::Cancelled => {
                tracing::error!("YouTube API request was cancelled");
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
            Youtube3Error::FieldClash(s) => {
                AppError::InvalidInput(format!("Field clash in YouTube API response: {}", s))
            }
            Youtube3Error::JsonDecodeError(s, e) => {
                AppError::InvalidInput(format!(
                    "JSON decode error in YouTube API response: {}. Error: {}",
                    s, e
                ))
            }
            Youtube3Error::Failure(r) => {
                AppError::ServiceUnavailable(format!("YouTube API failure: {:?}", r))
            }
            Youtube3Error::Io(e) => {
                tracing::error!("IO error in YouTube API: {}", e);
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
        }
    }
}

impl From<MeiliSearchError> for AppError {
    fn from(e: MeiliSearchError) -> AppError {
        match e {
            Meilisearch(e) => match e.error_type {
                MeiliSearchErrorType::InvalidRequest => AppError::InvalidInput(e.error_message),
                MeiliSearchErrorType::Auth => AppError::Unauthorized,
                MeiliSearchErrorType::Internal => AppError::InternalServerError(anyhow::anyhow!(e)),
                MeiliSearchErrorType::Unknown => AppError::InternalServerError(anyhow::anyhow!(e)),
                _ => AppError::InternalServerError(anyhow::anyhow!(e)),
            },
            MeilisearchCommunication(e) => match e.status_code {
                400 => AppError::InvalidInput(
                    e.message
                        .unwrap_or("Invalid request to meilisearch".to_string()),
                ),
                401 => AppError::Unauthorized,
                403 => AppError::Forbidden(
                    e.message
                        .unwrap_or("Forbidden access to meilisearch".to_string()),
                ),
                404 => AppError::NotFound(
                    e.message
                        .unwrap_or("Resource not found in meilisearch".to_string()),
                ),
                503 => AppError::ServiceUnavailable(
                    e.message
                        .unwrap_or("Service unavailable in meilisearch".to_string()),
                ),
                _ => AppError::InternalServerError(anyhow::anyhow!(e)),
            },
            MeiliSearchError::ParseError(e) => AppError::InvalidInput(e.to_string()),
            MeiliSearchError::Timeout => {
                tracing::error!("Meilisearch request timed out");
                AppError::ServiceUnavailable("Meilisearch request timed out".to_string())
            }
            MeiliSearchError::InvalidRequest => {
                tracing::error!("Invalid request to Meilisearch");
                AppError::InvalidInput("Invalid request to Meilisearch".to_string())
            }
            MeiliSearchError::CantUseWithoutApiKey(s) => {
                tracing::error!("Can't use Meilisearch without an API key: {}", s);
                AppError::Unauthorized
            }
            MeiliSearchError::TenantTokensInvalidApiKey => {
                AppError::Unauthorized
            }
            MeiliSearchError::TenantTokensExpiredSignature => {
                AppError::Unauthorized
            }
            MeiliSearchError::InvalidTenantToken(e) => {
                AppError::Unauthorized
            }
            MeiliSearchError::HttpError(e) => {
                tracing::error!("HTTP error in Meilisearch: {}", e);
                AppError::InternalServerError(anyhow::anyhow!(e))
            }
            MeiliSearchError::Yaup(e) => AppError::InvalidInput(
                "The library formatting the query parameters encountered an error.".to_string()
             ),
            MeiliSearchError::Uuid(e) => AppError::InvalidInput(format!("Invalid UUID4: {}", e)),
            MeiliSearchError::InvalidUuid4Version => {
                AppError::InvalidInput("Invalid UUID4 version".to_string())
            }
            MeiliSearchError::Other(e) =>
            // This is a catch-all for any other Meilisearch errors that don't fit the above categories
            // It should be used carefully, as it may mask specific issues.
            {
                tracing::error!("An unknown error occurred in Meilisearch: {}", e);
                AppError::InternalServerError(anyhow::anyhow!(e))
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
        let status = match self {
            AppError::InvalidInput(from) => {
                tracing::error!("{}", from);
                StatusCode::BAD_REQUEST
            },
            AppError::DomainParseError(domain) => {
                tracing::error!("{}", domain);
                StatusCode::BAD_REQUEST
            },
            AppError::EntityBuildFailed(msg) => {
                tracing::error!("{}", msg);
                StatusCode::BAD_REQUEST
            },
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        };


        (status, axum::Json(ResponseBody{
            message : status.to_string()
        })).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

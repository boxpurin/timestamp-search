use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use google_youtube3::Error as Youtube3Error;
use meilisearch_sdk::errors::Error::{Meilisearch, MeilisearchCommunication};
use meilisearch_sdk::errors::{Error as MeiliSearchError, ErrorType as MeiliSearchErrorType};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Invalid domain: {0}")]
    DomainParseError(String),

    #[error("Entity build failed: {0}")]
    EntityBuildFailed(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Forbidden access: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

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
                AppError::InvalidInput(format!("{}",e))
            }
            DomainError::ParseFailure => {
                tracing::error!("Domain Parse failure: {}", e);
                AppError::DomainParseError(format!("{}",e))
            }
        }
    }
}

impl From<Youtube3Error> for AppError {
    fn from(e: Youtube3Error) -> Self {
        match e {
            Youtube3Error::HttpError(e) => {
                tracing::error!("HTTP error in YouTube API: {}", e);
                AppError::InternalServerError(format!("HTTP error in YouTube API: {}", e))
            }
            Youtube3Error::UploadSizeLimitExceeded(i, e) => {
                tracing::error!("Upload size limit exceeded: {} bytes. Error: {}", i, e);
                AppError::InvalidInput(format!(
                    "Upload size limit exceeded: {} bytes. Error: {}",
                    i, e
                ))
            }
            Youtube3Error::BadRequest(v) => {
                tracing::error!("Bad request to YouTube API: {:?}", v);
                AppError::InvalidInput(format!("Bad request to YouTube API: {:?}", v))
            }
            Youtube3Error::MissingAPIKey => {
                tracing::error!("YouTube API key is missing");
                AppError::Unauthorized("YouTube API key is missing".to_string())
            }
            Youtube3Error::MissingToken(e) => {
                tracing::error!("YouTube API token is missing {}", e);
                AppError::Unauthorized("YouTube API token is missing.".to_string())
            }
            Youtube3Error::Cancelled => {
                tracing::error!("YouTube API request was cancelled");
                AppError::ServiceUnavailable("request was cancelled".to_string())
            }
            Youtube3Error::FieldClash(s) => {
                tracing::error!("Field clash in YouTube API response: {}", s);
                AppError::InvalidInput(format!("Field clash in YouTube API response: {}", s))
            }
            Youtube3Error::JsonDecodeError(s, e) => {
                tracing::error!(
                    "JSON decode error in YouTube API response: {}. Error: {}",
                    s,
                    e
                );
                AppError::InvalidInput(format!(
                    "JSON decode error in YouTube API response: {}. Error: {}",
                    s, e
                ))
            }
            Youtube3Error::Failure(r) => {
                tracing::error!("Failure in YouTube API response: {:?}", r);
                AppError::InternalServerError(format!("Failure in YouTube API response: {:?}", r))
            }
            Youtube3Error::Io(e) => {
                tracing::error!("IO error in YouTube API: {}", e);
                AppError::InternalServerError(format!("IO error in YouTube API: {}", e))
            }
        }
    }
}

impl From<MeiliSearchError> for AppError {
    fn from(e: MeiliSearchError) -> AppError {
        match e {
            Meilisearch(e) => match e.error_type {
                MeiliSearchErrorType::InvalidRequest => AppError::InvalidInput(e.error_message),
                MeiliSearchErrorType::Auth => AppError::Unauthorized(e.error_message),
                MeiliSearchErrorType::Internal => AppError::InternalServerError(e.error_message),
                MeiliSearchErrorType::Unknown => AppError::InternalServerError(e.error_message),
                _ => AppError::InternalServerError(e.error_message),
            },
            MeilisearchCommunication(e) => match e.status_code {
                400 => AppError::InvalidInput(
                    e.message
                        .unwrap_or("Invalid request to meilisearch".to_string()),
                ),
                401 => AppError::Unauthorized(
                    e.message
                        .unwrap_or("Unauthorized access to meilisearch".to_string()),
                ),
                403 => AppError::Forbidden(
                    e.message
                        .unwrap_or("Forbidden access to meilisearch".to_string()),
                ),
                404 => AppError::NotFound(
                    e.message
                        .unwrap_or("Resource not found in meilisearch".to_string()),
                ),
                409 => AppError::Conflict(
                    e.message
                        .unwrap_or("Conflict in meilisearch operation".to_string()),
                ),
                503 => AppError::ServiceUnavailable(
                    e.message
                        .unwrap_or("Service unavailable in meilisearch".to_string()),
                ),
                _ => AppError::InternalServerError(e.message.unwrap_or(
                    "Internal server error. error type : MeilisearchCommunication".to_string(),
                )),
            },
            MeiliSearchError::ParseError(e) => AppError::InvalidInput(e.to_string()),
            MeiliSearchError::Timeout => {
                AppError::ServiceUnavailable("Meilisearch request timed out".to_string())
            }
            MeiliSearchError::InvalidRequest => {
                AppError::InvalidInput("Invalid request to Meilisearch".to_string())
            }
            MeiliSearchError::CantUseWithoutApiKey(s) => {
                AppError::Unauthorized(format!("Meilisearch API key required: {}", s))
            }
            MeiliSearchError::TenantTokensInvalidApiKey => {
                AppError::Unauthorized("Invalid Meilisearch API key".to_string())
            }
            MeiliSearchError::TenantTokensExpiredSignature => {
                AppError::Unauthorized("Meilisearch API key signature expired".to_string())
            }
            MeiliSearchError::InvalidTenantToken(e) => {
                AppError::Unauthorized(format!("Invalid Meilisearch tenant token: {}", e))
            }
            MeiliSearchError::HttpError(e) => {
                AppError::InternalServerError(format!("HTTP error in Meilisearch: {}", e))
            }
            MeiliSearchError::Yaup(e) => AppError::InvalidInput(format!(
                "The library formatting the query parameters encountered an error.: {}",
                e
            )),
            MeiliSearchError::Uuid(e) => AppError::InvalidInput(format!("Invalid UUID4: {}", e)),
            MeiliSearchError::InvalidUuid4Version => {
                AppError::InvalidInput("Invalid UUID4 version".to_string())
            }
            MeiliSearchError::Other(e) =>
            // This is a catch-all for any other Meilisearch errors that don't fit the above categories
            // It should be used carefully, as it may mask specific issues.
            {
                AppError::InternalServerError(format!(
                    "An unknown error occurred in Meilisearch: {}",
                    e
                ))
            }
            _ => AppError::InternalServerError(format!(
                "An unknown error occurred in Meilisearch: {}",
                e
            )),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::DomainParseError(_) => StatusCode::BAD_REQUEST,
            AppError::EntityBuildFailed(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        };

        (status, self.to_string()).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

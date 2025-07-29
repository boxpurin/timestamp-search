use types::impl_string_value;
use errors::{AppError, AppResult};

impl_string_value!(VideoId);

impl VideoId {
    pub fn new(id: &str) -> AppResult<Self> {
        if id.is_empty() {
            return Err(AppError::InvalidInput("VideoId cannot be empty".to_string()));
        }
        // You can add more validation logic here if needed
        Ok(VideoId(id.to_string()))
    }
}

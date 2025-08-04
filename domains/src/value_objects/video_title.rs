use errors::{AppError, AppResult};
types::impl_string_value!(VideoTitle);

impl VideoTitle {
    pub fn new(title: &str) -> AppResult<Self> {
        if title.is_empty() {
            return Err(AppError::InvalidInput("Title cannot be empty".to_string()));
        }
        Ok(VideoTitle(title.to_string()))
    }
}

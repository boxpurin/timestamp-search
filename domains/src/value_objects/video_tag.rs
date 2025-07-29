use errors::{AppError, AppResult};

types::impl_string_value!(VideoTag);

impl VideoTag {
    pub fn new(tag: &str) -> AppResult<Self> {
        if tag.is_empty() {
            return Err(AppError::InvalidInput("Tag cannot be empty".to_string()));
        }
        Ok(VideoTag(tag.to_string()))
    }
}
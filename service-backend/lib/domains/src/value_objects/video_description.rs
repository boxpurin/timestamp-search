use errors::AppResult;

types::impl_string_value!(VideoDescription);
impl VideoDescription {
    pub fn new(description: &str) -> AppResult<Self> {
        Ok(VideoDescription(description.to_string()))
    }
}

impl Default for VideoDescription {
    fn default() -> Self {
        VideoDescription::new("").unwrap()
    }
}
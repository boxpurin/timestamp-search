use errors::AppResult;
use types::impl_string_value;

impl_string_value!(TimeStampDescription);

impl TimeStampDescription {
    pub fn new(description: &str) -> AppResult<Self> {
        Ok(TimeStampDescription(description.to_string()))
    }
}

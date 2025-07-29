use types::impl_string_value;

impl_string_value!(TimeStampDescription);

impl TimeStampDescription {
    pub fn new(description: &str) -> Self {
        TimeStampDescription(description.to_string())
    }
}
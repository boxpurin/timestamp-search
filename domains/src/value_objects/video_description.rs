use types::impl_string_value;

impl_string_value!(VideoDescription);
impl VideoDescription {
    pub fn new (description: &str) -> Self {
        VideoDescription(description.to_string())
    }
}
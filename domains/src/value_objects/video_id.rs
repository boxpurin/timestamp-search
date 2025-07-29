use types::impl_string_value;

impl_string_value!(VideoId);

impl VideoId {
    pub fn new(id: &str) -> Self {
        VideoId(id.to_string())
    }
}

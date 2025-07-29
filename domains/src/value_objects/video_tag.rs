use types::impl_string_value;

impl_string_value!(VideoTag);

impl VideoTag {
    pub fn new(tag: &str) -> Self {
        VideoTag(tag.to_string())
    }
}
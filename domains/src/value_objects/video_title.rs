use types::impl_string_value;

impl_string_value!(VideoTitle);

impl VideoTitle {
    pub fn new(title: &str) -> Self {
        VideoTitle(title.to_string())
    }
}
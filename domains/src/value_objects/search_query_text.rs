use types::impl_string_value;
impl_string_value!(SearchQueryText);

impl SearchQueryText {
    pub fn new(text: &str) -> Self {
        SearchQueryText(text.to_string())
    }
}
use types::impl_string_value;

impl_string_value!(ThumbnailUrl);
impl ThumbnailUrl {
    pub fn new(url: &str) -> Self {
        ThumbnailUrl(url.to_string())
    }
}
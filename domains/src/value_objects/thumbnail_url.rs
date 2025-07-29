use errors::{AppResult, AppError};
types::impl_string_value!(ThumbnailUrl);

impl ThumbnailUrl {
    pub fn new(url: &str) -> AppResult<Self> {
        if !url.starts_with("http://") || !url.starts_with("https://") {
            return Err(AppError::DomainParseError(url.to_string()));
        }
        Ok(ThumbnailUrl(url.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::AppError;

    #[test]
    fn test_thumbnail_url() {
        // Valid URLs
        assert!(ThumbnailUrl::new("http://example.com/image.jpg").is_ok());
        assert!(ThumbnailUrl::new("https://example.com/image.jpg").is_ok());

        // Invalid URLs
        assert!(matches!(
            ThumbnailUrl::new("ftp://example.com/image.jpg"),
            Err(AppError::DomainParseError(_))
        ));
        assert!(matches!(
            ThumbnailUrl::new("invalid-url"),
            Err(AppError::DomainParseError(_))
        ));

        // Valid URL creation
        let thumbnail_url = ThumbnailUrl::new("https://example.com/image.jpg").unwrap();
        assert_eq!(thumbnail_url.0, "https://example.com/image.jpg");
    }
}
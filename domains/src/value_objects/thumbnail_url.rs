use errors::{AppError, AppResult};
types::impl_string_value!(ThumbnailUrl);

impl ThumbnailUrl {
    pub fn new(url: &str) -> AppResult<Self> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::DomainParseError(url.to_string()));
        }

        if !url.ends_with(".jpg") && !url.ends_with(".png") {
            return Err(AppError::DomainParseError(url.to_string()));
        }
        Ok(ThumbnailUrl(url.to_string()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use errors::AppError;

    // Valid URLs
    #[rstest::rstest]
    #[test]
    #[case("http://example.com/image.jpg")]
    #[case("https://example.com/image.jpg")]
    #[case("https://example.com/image.png")]
    fn valid_thumbnail_url(#[case] url:&str) {
        assert!(ThumbnailUrl::new(url).is_ok());
    }

    #[rstest::rstest]
    #[test]
    #[case("")]
    #[case("ftp://example.com/image.jpg")]
    #[case("https://example.com/image.exe")]
    #[case("invalid-test")]
    fn invalid_thumbnail_url(#[case] url:&str) {
        assert!(matches!(
            ThumbnailUrl::new(url),
            Err(AppError::DomainParseError(_))
        ));
    }
}

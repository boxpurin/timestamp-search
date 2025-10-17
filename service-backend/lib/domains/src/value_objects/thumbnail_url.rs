use errors::{AppError::DomainParseError, AppResult};
use url::Url;
types::impl_string_value!(ThumbnailUrl);

impl ThumbnailUrl {
    pub fn new(url: &str) -> AppResult<Self> {
        match Url::parse(url) {
            Ok(u) => {
                let s = u.scheme();
                if s != "http" && s != "https" {
                    return Err(DomainParseError(
                        "expected scheme is http or https".to_string(),
                    ));
                }
                if let Some(p) = u.path_segments().map(|c| c.collect::<Vec<_>>()) {
                    let p = p.last().unwrap().to_string();
                    if !p.ends_with(".jpg") && !p.ends_with(".jpeg") && !p.ends_with(".png") {
                        return Err(DomainParseError("expected ext .jpg .jpeg .png".to_string()));
                    }
                }
            }
            Err(e) => return Err(DomainParseError(e.to_string())),
        };

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
    #[case("https://example.com/image.png?with_query=something")]
    fn valid_thumbnail_url(#[case] url: &str) {
        assert!(ThumbnailUrl::new(url).is_ok());
    }

    #[rstest::rstest]
    #[test]
    #[case("")]
    #[case("ftp://example.com/image.jpg")]
    #[case("https://example.com/image.exe")]
    #[case("invalid-test")]
    fn invalid_thumbnail_url(#[case] url: &str) {
        assert!(matches!(
            ThumbnailUrl::new(url),
            Err(AppError::DomainParseError(_))
        ));
    }
}

use errors::{AppError, AppResult};
use types::impl_string_value;

impl_string_value!(VideoId);

impl VideoId {
    pub fn new(id: &str) -> AppResult<Self> {
        if id.len() != 11 {
            return Err(AppError::InvalidInput(
                "Video ID must be 11 characters long".to_string(),
            ));
        }
        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(AppError::InvalidInput(
                "Video ID must contain only alphanumeric characters, hyphens, or underscores"
                    .to_string(),
            ));
        }
        Ok(VideoId(id.to_string()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[test]
    #[case("aqz-KE-bpKQ")]
    fn valid_video_id(#[case] valid_id: &str) {
        // Valid ID
        assert!(VideoId::new(valid_id).is_ok());
    }

    #[rstest]
    #[test]
    #[case::empty("")] // empty
    #[case::shortest("short")]    // invalid length
    #[case::invalid("aqz-KE-bpKQ!")] // invalid char
    fn invalid_video_id(#[case] invalid_id: &str) {
        assert!(VideoId::new(invalid_id).is_err());
    }
}

use errors::{AppError, AppResult};
types::impl_numeric_value!(VideoThumbnailHeight, u32);

/// Expected Resolution Height when fetch from Youtube Data API v3
///
/// https://developers.google.com/youtube/v3/docs/thumbnails
const STANDARD_HEIGHTS: &[u32] = &[
    90,    // default
    240,    // medium
    360,    // high
    480,    // standard
    720,   // maxres
];

impl VideoThumbnailHeight {
    pub fn new(height: u32) -> AppResult<Self> {
        if !STANDARD_HEIGHTS.contains(&height) {
            return Err(AppError::InvalidInput(
                format!("Invalid standard height: {} , Expected one of {:?}",height,STANDARD_HEIGHTS)
            ));
        }

        Ok(VideoThumbnailHeight(height))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[rstest::rstest]
    #[test]
    #[case(vec![90, 240, 360, 480, 720])]
    fn valid_height(#[case] heights: Vec<u32>) {
        for height in heights {
            assert!(VideoThumbnailHeight::new(height).is_ok());

            let h = VideoThumbnailHeight::new(height).unwrap();
            assert_eq!(h, height);
        }
    }

    #[rstest::rstest]
    #[test]
    #[case(vec![0, 120, 640, 1280, 1920])]
    fn invalid_height(#[case] heights: Vec<u32>)
    {
        for height in heights {
            assert!(VideoThumbnailHeight::new(height).is_err());
        }
    }
}

use errors::AppResult;
use errors::AppError::InvalidInput;

types::impl_numeric_value!(VideoThumbnailWidth, u32);

/// Expected Resolution Width when fetch from Youtube Data API v3
///
/// https://developers.google.com/youtube/v3/docs/thumbnails
const STANDARD_WIDTHS: &[u32] = &[
    120,    // default
    320,    // medium
    480,    // high
    640,    // standard
    1280,   // maxres
];

impl VideoThumbnailWidth {
    pub fn new(width: u32) -> AppResult<Self> {
        if !STANDARD_WIDTHS.contains(&width) {
            Err(InvalidInput(
                format!("Invalid standard width: {} , Expected one of {:?}",
                        width,
                        STANDARD_WIDTHS))
            )?;
        }

        Ok(VideoThumbnailWidth(width))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[rstest::rstest]
    #[test]
    #[case(vec![120, 320, 480, 640, 1280])]
    fn valid_width(#[case] widths: Vec<u32>) {
        for w in widths {
            assert!(VideoThumbnailWidth::new(w).is_ok());

            let width = VideoThumbnailWidth::new(w).unwrap();
            assert_eq!(width, width);
        }
    }

    #[rstest::rstest]
    #[test]
    #[case(vec![0, 240, 360, 1600, 777, 1920])]
    fn invalid_width(#[case] widths: Vec<u32>) {
        for w in widths {
            assert!(VideoThumbnailWidth::new(w).is_err());
        }
    }

}
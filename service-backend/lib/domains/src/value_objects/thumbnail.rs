use crate::value_objects::{
    height::VideoThumbnailHeight, thumbnail_url::ThumbnailUrl, width::VideoThumbnailWidth,
};
use errors::{AppError, AppResult};
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
pub struct Thumbnail {
    #[garde(url)]
    url: ThumbnailUrl,
    #[garde(skip)]
    width: VideoThumbnailWidth,
    #[garde(skip)]
    height: VideoThumbnailHeight,
}

impl Thumbnail {
    pub fn new<W: TryInto<VideoThumbnailWidth>, H: TryInto<VideoThumbnailHeight>>(
        url: ThumbnailUrl,
        width: W,
        height: H,
    ) -> AppResult<Self> {
        let width = width
            .try_into()
            .map_err(|_| AppError::DomainParseError("invalid width".to_string()))?;
        let height = height
            .try_into()
            .map_err(|_| AppError::DomainParseError("invalid height".to_string()))?;
        Ok(Self { url, width, height })
    }

    pub fn url(&self) -> &ThumbnailUrl {
        &self.url
    }

    pub fn width(&self) -> &VideoThumbnailWidth {
        &self.width
    }

    pub fn height(&self) -> &VideoThumbnailHeight {
        &self.height
    }
}

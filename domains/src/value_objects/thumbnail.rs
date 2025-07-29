use crate::value_objects::{
    thumbnail_url::ThumbnailUrl,
    width::Width,
    height::Height,
};
use garde::Validate;
use errors::{AppResult, AppError};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
pub struct Thumbnail {
    #[garde(url)]
    url: ThumbnailUrl,
    #[garde(skip)]
    width: Width,
    #[garde(skip)]
    height: Height,
}

impl Thumbnail {
    pub fn new<W: TryInto<Width>, H: TryInto<Height>>(
        url: ThumbnailUrl, width: W, height: H) -> AppResult<Self>
    {
        let width = width.try_into().map_err(|_| AppError::DomainParseError("".to_string()))?;
        let height = height.try_into().map_err(|_| AppError::DomainParseError("".to_string()))?;
        Ok(Self {
            url,
            width,
            height
        })
    }

    pub fn url(&self) -> &ThumbnailUrl {
        &self.url
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn height(&self) -> Height {
        self.height
    }
}
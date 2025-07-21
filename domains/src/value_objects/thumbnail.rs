use crate::value_objects::thumbnail_url::ThumbnailUrl;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
pub struct Thumbnail {
    #[garde(url)]
    url: ThumbnailUrl,
    #[garde(skip)]
    width: u32,
    #[garde(skip)]
    height: u32,
}

impl Thumbnail {
    pub fn new(url: ThumbnailUrl, width: u32, height: u32) -> Self {
        Self { url, width, height }
    }

    pub fn url(&self) -> &ThumbnailUrl {
        &self.url
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
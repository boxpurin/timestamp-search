use crate::value_objects::thumbnail_url::ThumbnailUrl;
use garde::{Validate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
pub struct Thumbnail {
    #[garde(url)]
    pub url: ThumbnailUrl,
    #[garde(skip)]
    pub width: u32,
    #[garde(skip)]
    pub height: u32,
}

use crate::value_objects::thumbnail_url::ThumbnailUrl;

pub struct Thumbnail{
    pub url: ThumbnailUrl,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
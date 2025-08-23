use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::value_objects::thumbnail_url::ThumbnailUrl;
use crate::value_objects::video_tag::VideoTag;
use crate::value_objects::video_title::VideoTitle;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VideoDetail{
    pub video_title: Option<VideoTitle>,
    pub video_tags: Option<Vec<VideoTag>>,
    pub thumbnail_url: Option<ThumbnailUrl>,
    pub published_at: Option<DateTime<Utc>>,
    pub actual_start_time: Option<DateTime<Utc>>,
}
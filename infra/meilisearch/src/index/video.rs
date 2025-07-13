use chrono::{DateTime, Utc};
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoIndex {
    pub id: VideoId,
    pub title: VideoTitle,
    pub description: VideoDescription,
    pub thumbnail_url: ThumbnailUrl,
    pub published_at: DateTime<Utc>,
}

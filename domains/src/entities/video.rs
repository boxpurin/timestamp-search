use crate::entities::channel::ChannelEntity;
use crate::value_objects::video_description::VideoDescription;
use crate::value_objects::video_id::VideoId;
use crate::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::value_objects::thumbnail::Thumbnail;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoEntity {
    pub id: VideoId,
    pub title: VideoTitle,
    pub description: VideoDescription,
    pub channel: ChannelEntity,
    pub thumbnail: Option<Thumbnail>,
    pub published_at: DateTime<Utc>,
    pub actual_start_time: Option<DateTime<Utc>>,
}

impl VideoEntity {
    pub fn new(
        id: VideoId,
        title: VideoTitle,
        description: VideoDescription,
        channel: ChannelEntity,
        thumbnail: Option<Thumbnail>,
        published_at: DateTime<Utc>,
        actual_start_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            channel,
            thumbnail,
            published_at,
            actual_start_time,
        }
    }
}

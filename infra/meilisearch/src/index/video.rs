use chrono::{DateTime, Utc};
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};
use domains::entities::video::VideoEntity;
use domains::value_objects::video_tag::VideoTag;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoIndex {
    pub video_id: VideoId,
    pub video_title: VideoTitle,
    pub video_tags: Vec<VideoTag>,
    pub video_description: VideoDescription,
    pub channel_id: ChannelId,
    pub channel_name: ChannelName,
    pub thumbnail_url: Option<ThumbnailUrl>,
    pub actual_start_time: Option<DateTime<Utc>>,
    pub published_at: DateTime<Utc>,
}

impl VideoIndex {
    pub fn new(
        id: VideoId,
        title: VideoTitle,
        tags: Vec<VideoTag>,
        description: VideoDescription,
        channel_id: ChannelId,
        channel_name: ChannelName,
        thumbnail_url: Option<ThumbnailUrl>,
        actual_start_time: Option<DateTime<Utc>>,
        published_at: DateTime<Utc>,
    ) -> Self {
        VideoIndex {
            video_id: id,
            video_title: title,
            video_tags: tags,
            video_description: description,
            channel_id,
            channel_name,
            thumbnail_url,
            actual_start_time,
            published_at,
        }
    }

    pub fn from_entity(video: VideoEntity) -> Self {
        VideoIndex::new(
            video.id,
            video.title,
            video.tags,
            video.description,
            video.channel.id,
            video.channel.name,
            video.thumbnail.map(|t| t.url().clone()),
            video.actual_start_time,
            video.published_at,
        )
    }
}

impl From<VideoEntity> for VideoIndex {
    fn from(video: VideoEntity) -> Self {
        VideoIndex::from_entity(video)
    }
}
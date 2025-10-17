use crate::config::CONFIG;
use crate::index::Index;
use chrono::DateTime;
use domains::entities::video::VideoEntity;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_tag::VideoTag;
use domains::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};
use domains::entities::channel::ChannelEntity;
use domains::value_objects::thumbnail::Thumbnail;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoIndex {
    pub video_id: VideoId,
    pub video_title: VideoTitle,
    pub video_tags: Vec<VideoTag>,
    pub video_description: VideoDescription,
    pub channel_id: ChannelId,
    pub channel_name: ChannelName,
    pub thumbnail_url: Option<ThumbnailUrl>,
    pub actual_start_time: Option<i64>,
    pub published_at: i64,
}

impl VideoIndex {
    pub fn from_entity(video: VideoEntity) -> Self {
        VideoIndex {
            video_id: video.id,
            video_title: video.title,
            video_tags: video.tags,
            video_description: video.description,
            channel_id : video.channel.id,
            channel_name : video.channel.name,
            thumbnail_url : video.thumbnail.map(|t| t.url().clone()),
            actual_start_time: video.actual_start_time.map(|t| t.timestamp()),
            published_at: video.published_at.timestamp(),
        }
    }
}

impl From<VideoEntity> for VideoIndex {
    fn from(video: VideoEntity) -> Self {
        VideoIndex::from_entity(video)
    }
}

impl From<VideoIndex> for VideoEntity {
    fn from(v: VideoIndex) -> VideoEntity {
        VideoEntity::build(
            v.video_id,
            v.video_title,
            ChannelEntity::new(v.channel_id, v.channel_name)
        )
            .with_tags(v.video_tags)
            .with_description(v.video_description)
            .with_thumbnail(v.thumbnail_url.map(|url| {
                Thumbnail::new(url, 320, 240).unwrap()
            }).unwrap())
            .with_published_at(
                DateTime::from_timestamp(v.published_at ,0).unwrap()
            )
            .with_actual_start_time(
                v.actual_start_time
                    .map(|t| DateTime::from_timestamp(t,0).unwrap()).unwrap()
            )
            .construct().unwrap()
    }
}

impl Index for VideoIndex {
    fn pid(&self) -> Option<&str> {
        Some(self.video_id.as_str())
    }

    fn pid_field() -> Option<&'static str> {
        Some("videoId")
    }

    fn name() -> &'static str {
        &CONFIG.video_index_name
    }
}

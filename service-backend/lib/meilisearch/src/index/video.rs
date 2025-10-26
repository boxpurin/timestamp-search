use crate::config::CONFIG;
use crate::index::Index;
use chrono::DateTime;
use domains::entities::channel::ChannelEntity;
use domains::entities::video::VideoEntity;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;
use domains::value_objects::thumbnail::Thumbnail;
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_tag::VideoTag;
use domains::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};

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
    pub actual_start_at: Option<i64>,
    pub published_at: i64,
}

impl VideoIndex {
    pub fn from_entity(video: VideoEntity) -> Self {
        VideoIndex {
            video_id: video.id,
            video_title: video.title,
            video_tags: video.tags,
            video_description: video.description,
            channel_id: video.channel.id,
            channel_name: video.channel.name,
            thumbnail_url: video.thumbnail.map(|t| t.url().clone()),
            actual_start_at: video.actual_start_at.map(|t| t.timestamp()),
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
        let mut builder = VideoEntity::build(
            v.video_id,
            v.video_title,
            ChannelEntity::new(v.channel_id, v.channel_name),
        )
        .with_tags(v.video_tags)
        .with_description(v.video_description)
        .with_published_at(DateTime::from_timestamp(v.published_at, 0).unwrap());

        if let Some(url) = v.thumbnail_url {
            builder = builder.with_thumbnail(Thumbnail::new(url, 320, 240).unwrap());
        }

        if let Some(t) = v.actual_start_at {
            builder = builder.with_actual_start_time(DateTime::from_timestamp(t, 0).unwrap());
        }
        builder.construct().unwrap()
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

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::index::video::VideoIndex;
    use domains::entities::channel::ChannelEntity;
    use domains::entities::video::VideoEntityBuilder;
    use domains::value_objects::channel_name::ChannelName;
    use domains::value_objects::video_description::VideoDescription;
    use domains::value_objects::video_id::VideoId;
    use rstest::rstest;

    #[rstest]
    fn conversion_index_entity_test() -> anyhow::Result<()> {
        let id = VideoId::new("abc-def-ghi")?;
        let title = VideoTitle::new("VideoTitle")?;
        let description = VideoDescription::new("Description")?;
        let channel_name = ChannelName::new("Channel Name")?;
        let channel = ChannelEntity::with_random_id(channel_name.clone());
        let channel_id = channel.id.clone();

        let entity = VideoEntityBuilder::new(id.clone(), title.clone(), channel.clone())
            .with_description(description)
            .construct()?;
        let index = VideoIndex::from_entity(entity.clone());

        // conversion check
        assert_eq!(index.video_id, entity.id);
        assert_eq!(index.video_tags, entity.tags);
        assert_eq!(index.video_title, entity.title);
        assert_eq!(index.video_description, entity.description);
        assert_eq!(
            index.thumbnail_url,
            entity.thumbnail.map(|t| t.url().clone())
        );
        assert_eq!(index.channel_id, channel_id);
        assert_eq!(index.channel_name, entity.channel.name);
        assert_eq!(index.published_at, entity.published_at.timestamp());
        assert_eq!(
            index.actual_start_at,
            entity.actual_start_at.map(|t| t.timestamp())
        );

        let entity = VideoEntity::from(index.clone());
        assert_eq!(index.video_id, entity.id);
        assert_eq!(index.video_tags, entity.tags);
        assert_eq!(index.video_title, entity.title);
        assert_eq!(index.video_description, entity.description);
        assert_eq!(
            index.thumbnail_url,
            entity.thumbnail.map(|t| t.url().clone())
        );
        assert_eq!(index.channel_id, channel_id);
        assert_eq!(index.channel_name, entity.channel.name);
        assert_eq!(index.published_at, entity.published_at.timestamp());
        assert_eq!(
            index.actual_start_at,
            entity.actual_start_at.map(|t| t.timestamp())
        );

        Ok(())
    }
}

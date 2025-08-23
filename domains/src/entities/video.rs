use crate::entities::channel::{ChannelEntity};
use crate::value_objects::thumbnail::Thumbnail;
use crate::value_objects::video_description::VideoDescription;
use crate::value_objects::video_id::VideoId;
use crate::value_objects::video_tag::VideoTag;
use crate::value_objects::video_title::VideoTitle;
use chrono::{DateTime, Utc};
use rand::distr::Alphanumeric;
use rand::Rng;
use rstest::rstest;
use serde::{Deserialize, Serialize};
use errors::AppResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoEntity {
    pub id: VideoId,
    pub title: VideoTitle,
    pub tags: Vec<VideoTag>,
    pub description: VideoDescription,
    pub channel: ChannelEntity,
    pub thumbnail: Option<Thumbnail>,
    pub published_at: DateTime<Utc>,
    pub actual_start_time: Option<DateTime<Utc>>,
}

impl VideoEntity {
    pub fn build(id: VideoId, title:VideoTitle, channel: ChannelEntity) -> VideoEntityBuilder {
        VideoEntityBuilder::new(id, title, channel)
    }

    pub fn with_random_id(title: VideoTitle, channel: ChannelEntity) -> VideoEntityBuilder {
        let mut rng = rand::rng();
        let v : String = (0..11).map(|_| rng.sample(Alphanumeric) as char).collect();
        let id = VideoId::new(&v).unwrap();
        VideoEntityBuilder::new(id, title, channel)
    }
}


#[derive(Clone, Debug)]
pub struct VideoEntityBuilder {
    id: VideoId,
    title: VideoTitle,
    tags: Vec<VideoTag>,
    description: VideoDescription,
    channel: ChannelEntity,
    thumbnail: Option<Thumbnail>,
    published_at: DateTime<Utc>,
    actual_start_time: Option<DateTime<Utc>>,
}

impl VideoEntityBuilder {
    pub fn new(id: VideoId, title:VideoTitle, channel: ChannelEntity) -> Self {
        Self {
            id,
            title,
            tags: Vec::new(),
            description: VideoDescription::default(),
            channel,
            thumbnail: None,
            published_at: Utc::now(),
            actual_start_time: None,
        }
    }

    pub fn with_tags(mut self, tags: Vec<VideoTag>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_description(
        mut self,
        description: VideoDescription,
    ) -> Self {
        self.description = description;
        self
    }

    pub fn with_thumbnail(mut self, thumbnail: Thumbnail) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn with_published_at(mut self, published_at: DateTime<Utc>) -> Self {
        self.published_at = published_at;
        self
    }

    pub fn with_actual_start_time(mut self, actual_start_time: DateTime<Utc>) -> Self {
        self.actual_start_time = Some(actual_start_time);
        self
    }

    pub fn construct(self) -> AppResult<VideoEntity> {
        Ok(VideoEntity {
            id: self.id,
            title: self.title,
            tags: self.tags,
            description: self.description,
            channel: self.channel,
            thumbnail: self.thumbnail,
            published_at: self.published_at,
            actual_start_time: self.actual_start_time,
        })
    }
}
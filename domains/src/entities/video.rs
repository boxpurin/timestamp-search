use crate::entities::channel::ChannelEntity;
use crate::value_objects::video_description::VideoDescription;
use crate::value_objects::video_id::VideoId;
use crate::value_objects::video_title::VideoTitle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoEntity {
    pub id: VideoId,
    pub title: VideoTitle,
    pub description: VideoDescription,
    pub channel: ChannelEntity,
}

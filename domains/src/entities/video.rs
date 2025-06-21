use google_youtube::v3::video::Video;
use serde::{Deserialize, Serialize};
use crate::value_objects::video_id::VideoId;
use crate::value_objects::video_title::VideoTitle;
use crate::value_objects::video_description::VideoDescription;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoEntity {
    pub id: VideoId,
    pub title: VideoTitle,
    pub descriotipn:VideoDescription
}
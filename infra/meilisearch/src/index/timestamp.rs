use domains::value_objects::timestamp_id::TimestampId;
use domains::value_objects::timestamp_description::TimeStampDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_title::VideoTitle;
use domains::value_objects::video_tag::VideoTag;
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc };
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::entities::video::VideoEntity;
use errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStampIndex {
    pub pid: TimestampId,
    pub video_id: VideoId,
    pub description: TimeStampDescription,
    pub start_time: u64, // 秒単位
    pub video_details: Option<VideoTimeStampDetails>,
}

impl TimeStampIndex {
    pub fn new(
        pid: TimestampId,
        video_id: VideoId,
        description: TimeStampDescription,
        start_time: u64,
        video_details: Option<VideoTimeStampDetails>,
    ) -> Self {
        TimeStampIndex {
            pid,
            video_id,
            description,
            start_time,
            video_details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTimeStampDetails {
    pub video_title: VideoTitle,
    pub tags: Vec<VideoTag>,
    pub published_at: DateTime<Utc>,
    pub actual_start_time: Option<DateTime<Utc>>,
}
impl VideoTimeStampDetails {
    pub fn new(
        video_title: VideoTitle,
        tags: Vec<VideoTag>,
        published_at: DateTime<Utc>,
        actual_start_time: Option<DateTime<Utc>>,
    ) -> Self {
        VideoTimeStampDetails {
            video_title,
            tags,
            published_at,
            actual_start_time,
        }
    }
}

impl From<VideoEntity> for VideoTimeStampDetails {
    fn from(video: VideoEntity) -> Self {
        VideoTimeStampDetails {
            video_title: video.title,
            tags: video.tags,
            published_at: video.published_at,
            actual_start_time: video.actual_start_time,
        }
    }
}

impl TryFrom<TimeStampIndex> for VideoTimeStampDetails {
    type Error = AppError;
    fn try_from(index: TimeStampIndex) -> AppResult<Self> {
        index
            .video_details
            .ok_or(AppError::DomainParseError("VideoTimeStampDetails is missing".to_string()))
    }
}
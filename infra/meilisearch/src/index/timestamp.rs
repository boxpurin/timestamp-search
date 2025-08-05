use crate::config::CONFIG;
use crate::index::Index;
use chrono::{DateTime, Utc};
use domains::entities::video::VideoEntity;
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::value_objects::seconds::Seconds;
use domains::value_objects::timestamp::TimeStamp;
use domains::value_objects::timestamp_description::TimeStampDescription;
use domains::value_objects::timestamp_id::TimestampId;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_tag::VideoTag;
use domains::value_objects::video_title::VideoTitle;
use errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};

// 想定されている型変換は
// VideoEntity と TimeStamp の２つから TimeStampIndex（とVideoTimeStampDetails）を作り出す
// 逆に取得する時は TimeStampIndex から VideoTimestampEntity への変換
// VideoEntity へ変換する場合は TimeStampIndex ではなく VideoId等を使って VideoIndex を MeiliSearch から取得する
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeStampIndex {
    pub pid: TimestampId,
    pub video_id: VideoId,
    pub description: TimeStampDescription,
    pub start_time: Seconds, // 秒単位
    pub video_details: Option<VideoTimeStampDetails>,
}

impl TimeStampIndex {
    pub fn new<S: Into<Seconds>>(
        pid: TimestampId,
        video_id: VideoId,
        description: TimeStampDescription,
        start_time: S,
        video_details: Option<VideoTimeStampDetails>,
    ) -> Self {
        TimeStampIndex {
            pid,
            video_id,
            description,
            start_time: start_time.into(),
            video_details,
        }
    }

    pub fn from_entity(video: VideoEntity, timestamp: TimeStamp) -> Self {
        TimeStampIndex::new(
            TimestampId::new(&video.id, &timestamp).unwrap(),
            video.id.clone(),
            timestamp.description,
            timestamp.seconds,
            Some(VideoTimeStampDetails::from_entity(video)),
        )
    }

    pub fn take_video_details(self) -> AppResult<VideoTimeStampDetails> {
        self.video_details.ok_or(AppError::DomainParseError(
            "VideoTimeStampDetails is missing".to_string(),
        ))
    }

    pub fn into_entity(self) -> VideoTimestampEntity {
        VideoTimestampEntity::new(
            self.video_id,
            TimeStamp::new(self.start_time, self.description).unwrap(),
        )
    }

    pub fn into_timestamp(self) -> TimeStamp {
        TimeStamp::new(self.start_time, self.description).unwrap()
    }
}

impl Into<VideoTimestampEntity> for TimeStampIndex {
    fn into(self) -> VideoTimestampEntity {
        VideoTimestampEntity::new(
            self.video_id,
            TimeStamp::new(self.start_time, self.description).unwrap(),
        )
    }
}

impl Index for TimeStampIndex {
    fn pid(&self) -> Option<&str> {
        Some(self.pid.as_str())
    }

    fn pid_field() -> Option<&'static str> {
        Some("pid")
    }

    fn name() -> &'static str {
        &CONFIG.timestamp_index_name
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

    pub fn from_entity(video: VideoEntity) -> Self {
        VideoTimeStampDetails::new(
            video.title,
            video.tags,
            video.published_at,
            video.actual_start_time,
        )
    }
}

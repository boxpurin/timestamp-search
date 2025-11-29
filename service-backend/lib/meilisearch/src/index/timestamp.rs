use crate::config::CONFIG;
use crate::index::Index;
use chrono::DateTime;
use domains::entities::video::VideoEntity;
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::value_objects::elapsed_time::ElapsedTime;
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use domains::value_objects::timestamp::TimeStamp;
use domains::value_objects::timestamp_description::TimeStampDescription;
use domains::value_objects::timestamp_id::TimestampId;
use domains::value_objects::video_detail::VideoDetail;
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
    pub elapsed_time: ElapsedTime, // 秒単位
    pub video_details: Option<VideoTimeStampDetails>,
}

impl TimeStampIndex {
    pub fn new<S: Into<ElapsedTime>>(
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
            elapsed_time: start_time.into(),
            video_details,
        }
    }

    pub fn from_entity(video: VideoEntity, timestamp: TimeStamp) -> Self {
        TimeStampIndex::new(
            TimestampId::new(&video.id, &timestamp).unwrap(),
            video.id.clone(),
            timestamp.description,
            timestamp.elapsed_time,
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
            TimeStamp::new(self.elapsed_time, self.description).unwrap(),
        )
    }

    pub fn into_timestamp(self) -> TimeStamp {
        TimeStamp::new(self.elapsed_time, self.description).unwrap()
    }
}

impl From<TimeStampIndex> for VideoTimestampEntity {
    fn from(v: TimeStampIndex) -> VideoTimestampEntity {
        VideoTimestampEntity::with_details(
            v.video_id,
            TimeStamp::new(v.elapsed_time, v.description).unwrap(),
            v.video_details.map(|d| VideoDetail {
                video_title: d.video_title,
                video_tags: d.video_tags,
                thumbnail_url: d.thumbnail_url,
                published_at: d.published_at.map(|t| DateTime::from_timestamp(t,0)).unwrap_or(None),
                actual_start_at: d.actual_start_at.map(|t| DateTime::from_timestamp(t,0)).unwrap_or(None),
            }),
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
#[serde(rename_all = "camelCase")]
pub struct VideoTimeStampDetails {
    pub video_title: Option<VideoTitle>,
    pub video_tags: Option<Vec<VideoTag>>,
    pub thumbnail_url: Option<ThumbnailUrl>,
    pub published_at: Option<i64>,
    pub actual_start_at: Option<i64>,
}
impl VideoTimeStampDetails {
    pub fn new(
        video_title: Option<VideoTitle>,
        video_tags: Option<Vec<VideoTag>>,
        thumbnail_url: Option<ThumbnailUrl>,
        published_at: Option<i64>,
        actual_start_at: Option<i64>,
    ) -> Self {
        VideoTimeStampDetails {
            video_title,
            video_tags,
            thumbnail_url,
            published_at,
            actual_start_at,
        }
    }

    pub fn from_entity(video: VideoEntity) -> Self {
        VideoTimeStampDetails::new(
            Some(video.title),
            Some(video.tags),
            video.thumbnail.map(|t| t.url().clone()),
            Some(video.published_at.timestamp()),
            video.actual_start_at.map(|t| t.timestamp()),
        )
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::index::timestamp::TimeStampIndex;
    use domains::entities::channel::ChannelEntity;
    use domains::entities::video::VideoEntityBuilder;
    use domains::value_objects::timestamp::TimeStamp;
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
        let _channel_id = channel.id.clone();

        let video = VideoEntityBuilder::new(id.clone(), title.clone(), channel.clone())
            .with_description(description)
            .construct()?;

        let timestamp = TimeStamp {
            description: TimeStampDescription::new("test")?,
            elapsed_time: ElapsedTime::new(100)?,
        };

        let index = TimeStampIndex::from_entity(video, timestamp.clone());

        assert_eq!(index.pid, TimestampId::new(&id, &timestamp)?);
        assert_eq!(index.video_id, id);
        assert_eq!(index.description, timestamp.description);
        assert_eq!(index.elapsed_time, timestamp.elapsed_time);

        index.video_details.as_ref().map(|d| {
            assert_eq!(d.video_title, Some(title));
            assert_eq!(d.video_tags, Some(vec![]));
            assert_eq!(d.thumbnail_url, None);
            assert!(d.published_at.is_some()); // published_atは生成時に自動でなにか入っている
            assert_eq!(d.actual_start_at, None);
        });

        let e = index.clone().into_entity();
        let e2 = VideoTimestampEntity::from(index.clone());

        assert_eq!(e, e2);

        Ok(())
    }
}

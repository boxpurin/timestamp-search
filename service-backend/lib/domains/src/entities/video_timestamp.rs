use crate::value_objects::{timestamp::TimeStamp, video_id::VideoId, video_detail::VideoDetail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct VideoTimestampEntity {
    pub video_id: VideoId,
    pub timestamp: TimeStamp,
    pub video_detail: Option<VideoDetail>
}

impl VideoTimestampEntity {
    pub fn new(video_id: VideoId, timestamp: TimeStamp) -> Self {
        VideoTimestampEntity {
            video_id,
            timestamp,
            video_detail: None
        }
    }

    pub fn with_details(video_id: VideoId, timestamp: TimeStamp, details: VideoDetail) -> Self {
        VideoTimestampEntity {
            video_id,
            timestamp,
            video_detail: Some(details)
        }
    }
}

impl PartialEq for VideoTimestampEntity {
    fn eq(&self, other: &Self) -> bool {
        self.video_id == other.video_id &&
            self.timestamp == other.timestamp
    }
}
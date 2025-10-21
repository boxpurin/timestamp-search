use crate::value_objects::{timestamp::TimeStamp, video_detail::VideoDetail, video_id::VideoId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct VideoTimestampEntity {
    pub video_id: VideoId,
    pub timestamp: TimeStamp,
    pub video_details: Option<VideoDetail>,
}

impl VideoTimestampEntity {
    pub fn new(video_id: VideoId, timestamp: TimeStamp) -> Self {
        VideoTimestampEntity {
            video_id,
            timestamp,
            video_details: None,
        }
    }

    pub fn with_details(video_id: VideoId, timestamp: TimeStamp, details: Option<VideoDetail>) -> Self {
        VideoTimestampEntity {
            video_id,
            timestamp,
            video_details: details,
        }
    }
}

impl PartialEq for VideoTimestampEntity {
    fn eq(&self, other: &Self) -> bool {
        self.video_id == other.video_id && self.timestamp == other.timestamp
    }
}

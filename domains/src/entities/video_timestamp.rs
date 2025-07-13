use crate::value_objects::{timestamp::TimeStamp, video_id::VideoId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoTimestampEntity {
    pub video_id: VideoId,
    pub timestamp: TimeStamp,
}

impl VideoTimestampEntity {
    pub fn new(video_id: VideoId, timestamp: TimeStamp) -> Self {
        VideoTimestampEntity {
            video_id,
            timestamp,
        }
    }
}

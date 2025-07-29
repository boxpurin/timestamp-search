use crate::entities::video::VideoEntity;
use crate::value_objects::timestamp::TimeStamp;
use crate::value_objects::video_id::VideoId;
use errors::{AppResult, AppError};
use sha2::{Digest, Sha224};

types::impl_string_value!(TimestampId);

impl TimestampId {
    pub fn new(video_id: &VideoId, timestamp: &TimeStamp) -> AppResult<Self> {
        // read hash digest and consume hasher
        let result = Sha224::digest(timestamp.description.as_bytes());
        Ok(
            TimestampId(
                format!("{}-{}-{:x}", video_id, timestamp.seconds, result)
            )
        )
    }
}


#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::value_objects::timestamp_description::TimeStampDescription;
    use crate::value_objects::seconds::Seconds;

    #[test]
    fn test_timestamp_id() {
        let video_id = VideoId::new("video123").unwrap();
        let timestamp = TimeStamp::new(Seconds::new(60).unwrap(), TimeStampDescription::new("Test description").unwrap()).unwrap();

        let timestamp_id = TimestampId::new(&video_id, &timestamp).unwrap();

        assert!(!timestamp_id.0.is_empty());
        assert!(timestamp_id.0.contains(video_id.as_str()));
        assert!(timestamp_id.0.contains(&timestamp.seconds.0.to_string()));
    }
}
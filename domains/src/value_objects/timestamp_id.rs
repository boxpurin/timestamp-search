use crate::entities::video::VideoEntity;
use crate::value_objects::timestamp::TimeStamp;
use sha2::Digest;
use sha2::Sha224;
use types::impl_string_value;

impl_string_value!(TimestampId);

impl TimestampId {
    pub fn new(id: &str) -> Self {
        TimestampId(id.to_string())
    }
    
    pub fn from_timestamp(video: &VideoEntity, timestamp: &TimeStamp) -> Self {
        // read hash digest and consume hasher
        let result = Sha224::digest(timestamp.description.as_bytes());
        TimestampId::new(&format!("{}-{}-{:x}", video.id, timestamp.seconds, result))
    }
}

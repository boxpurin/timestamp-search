use std::io::Read;
use types::impl_string_value;
use crate::value_objects::timestamp::TimeStamp;
use crate::entities::video::VideoEntity;
use sha2::Sha224;
use sha2::Digest;
use hex_literal::hex;

impl_string_value!(TimestampId);

impl TimestampId {
    pub fn from_timestamp(video: &VideoEntity , timestamp: &TimeStamp) -> Self {
        // read hash digest and consume hasher
        let result = Sha224::digest(timestamp.description.as_bytes());
        TimestampId::new(format!("{}-{}-{:x}", video.id, timestamp.seconds, result))
    }
}
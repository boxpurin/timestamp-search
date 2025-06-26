use types::impl_string_value;
use crate::value_objects::Timestamp;
use crate::entities::VideoEntity;
use sha2::Digest;

impl_string_value!(TimestampId);

impl TimestampId {
    pub fn from_timestamp(video: &VideoEntity , timestamp: &Timestamp) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(timestamp.to_string());
        let hash = hasher.finalize();
        let hex_string = hex::encode(hash);
        TimestampId::new(hex_string)
    }
}
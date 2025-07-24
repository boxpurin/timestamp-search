use crate::value_objects::timestamp_description::TimeStampDescription;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub seconds: u64,
    pub description: TimeStampDescription,
}

impl TimeStamp {
    pub fn new(seconds: u64, description: TimeStampDescription) -> Self {
        TimeStamp {
            seconds,
            description,
        }
    }

    pub fn to_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.seconds as i64, 0)
            .single()
            .expect("Invalid timestamp")
    }
}

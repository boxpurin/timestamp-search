use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use crate::value_objects::timestamp_description::TimeStampDescription;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub seconds: i64,
    pub description: TimeStampDescription,
}

impl TimeStamp {
    pub fn new(seconds: i64, description: TimeStampDescription) -> Self {
        TimeStamp { seconds, description }
    }

    pub fn to_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.seconds, 0)
            .single()
            .expect("Invalid timestamp")
    }
}
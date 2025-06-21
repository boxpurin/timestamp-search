use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::value_objects::timestamp_description::TimestampDescription;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp {
    pub seconds: i64,
    pub description: TimestampDescription,
}
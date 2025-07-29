use crate::value_objects::timestamp_description::TimeStampDescription;
use serde::{Deserialize, Serialize};
use crate::value_objects::seconds::Seconds;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub seconds: Seconds,
    pub description: TimeStampDescription,
}

impl TimeStamp {
    pub fn new<S: Into<Seconds>>(seconds: S, description: TimeStampDescription) -> Self {
        TimeStamp {
            seconds: seconds.into(),
            description,
        }
    }
}

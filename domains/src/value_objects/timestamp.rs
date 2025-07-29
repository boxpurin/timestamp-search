use crate::value_objects::timestamp_description::TimeStampDescription;
use serde::{Deserialize, Serialize};
use errors::{AppResult, AppError};
use crate::value_objects::seconds::Seconds;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub seconds: Seconds,
    pub description: TimeStampDescription,
}

impl TimeStamp {
    pub fn new<S: TryInto<Seconds>>(seconds: S, description: TimeStampDescription) -> AppResult<Self> {
        Ok(TimeStamp {
            seconds: seconds
                .try_into()
                .map_err(|_| AppError::DomainParseError("Invalid seconds".to_string()))?,
            description,
        })
    }
}

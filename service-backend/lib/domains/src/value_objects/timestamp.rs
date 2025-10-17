use crate::value_objects::elapsed_time::ElapsedTime;
use crate::value_objects::timestamp_description::TimeStampDescription;
use errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeStamp {
    pub elapsed_time: ElapsedTime,
    pub description: TimeStampDescription,
}

impl TimeStamp {
    pub fn new<S: TryInto<ElapsedTime>>(
        seconds: S,
        description: TimeStampDescription,
    ) -> AppResult<Self> {
        Ok(TimeStamp {
            elapsed_time: seconds
                .try_into()
                .map_err(|_| AppError::DomainParseError("Invalid seconds".to_string()))?,
            description,
        })
    }
}

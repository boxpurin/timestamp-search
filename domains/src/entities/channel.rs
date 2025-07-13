use crate::value_objects::channel_id::ChannelId;
use crate::value_objects::channel_name::ChannelName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelEntity {
    pub id: ChannelId,
    pub name: ChannelName,
}

impl ChannelEntity {
    pub fn new(id: ChannelId, name: ChannelName) -> Self {
        ChannelEntity { id, name }
    }
}

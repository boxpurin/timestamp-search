use serde::{Deserialize, Serialize};
use crate::value_objects::channel_id::ChannelId;
use crate::value_objects::channel_name::ChannelName;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelEntity {
    pub id: ChannelId,
    pub name: ChannelName,
}
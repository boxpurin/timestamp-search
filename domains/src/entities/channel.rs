use serde::{Deserialize, Serialize};
use crate::value_objects::channel_id::ChannelId;
use crate::value_objects::channel_name::ChannelName;
use google_youtube3::api::Channel as YouTubeChannel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelEntity {
    pub id: ChannelId,
    pub name: ChannelName,
}

impl TryFrom<YouTubeChannel> for ChannelEntity {
    type Error = String;

    fn try_from(value: YouTubeChannel) -> Result<Self, Self::Error> {
        let snippet = value.snippet.ok_or("Channel snippet is missing")?;
        let id = value.id.ok_or("Channel ID is missing")?;
        let name = snippet.title.ok_or("Channel name is missing")?;
        Ok(ChannelEntity {
            id: ChannelId::from(id),
            name: ChannelName::from(name),
        })
    }
}
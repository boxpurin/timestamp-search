use google_youtube3::api::Video as YouTubeVideo;
use serde::{Deserialize, Serialize};
use crate::value_objects::video_id::VideoId;
use crate::value_objects::video_title::VideoTitle;
use crate::value_objects::video_description::VideoDescription;
use crate::entities::channel::ChannelEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoEntity {
    pub id: VideoId,
    pub title: VideoTitle,
    pub description:VideoDescription,
    pub channel: ChannelEntity,
}

impl TryFrom<YouTubeVideo> for VideoEntity {
    type Error = &'static str;

    fn try_from(value: YouTubeVideo) -> Result<Self, Self::Error> {
        let id = value.id.ok_or("Video ID is missing")?;
        let snippet = value.snippet.ok_or("Video snippet is missing")?;
        let title = snippet.title.ok_or("Video title is missing")?;
        let description = snippet.description.ok_or("Video description is missing")?;
        let channel_id = snippet.channel_id.ok_or("Channel ID is missing")?;
        let channel_title = snippet.channel_title.ok_or("Channel title is missing")?;
        let channel = ChannelEntity {
            id: crate::value_objects::channel_id::ChannelId::from(channel_id),
            name: crate::value_objects::channel_name::ChannelName::from(channel_title),
        };

        Ok(VideoEntity {
            id: VideoId::from(id),
            title: VideoTitle::from(title),
            description: VideoDescription::from(description),
            channel,
        })
    }
}
use domains::entities::channel::ChannelEntity;
use domains::entities::video::VideoEntity;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_title::VideoTitle;
use google_youtube3::api::{PlaylistItem, Video};

/// Converter from YouTube Video to VideoEntity
pub struct VideoEntityConverter(pub Video);
pub struct PlayListItemToVideoEntityConverter(pub PlaylistItem);

impl TryInto<VideoEntity> for VideoEntityConverter {
    type Error = String;

    fn try_into(self) -> Result<VideoEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or("Video snippet is missing")?;
        let id = inner.id.ok_or("Video ID is missing")?;
        let title = snippet.title.ok_or("Video title is missing")?;
        let description = snippet.description.ok_or("Video description is missing")?;

        let channel_id = snippet.channel_id.ok_or("Channel ID is missing")?;
        let channel_name = snippet.channel_title.ok_or("Channel name is missing")?;

        Ok(VideoEntity {
            id: VideoId::from(id),
            title: VideoTitle::from(title),
            description: VideoDescription::from(description),
            channel: ChannelEntity::new(
                ChannelId::from(channel_id),
                ChannelName::from(channel_name),
            ),
        })
    }
}

impl TryInto<VideoEntity> for PlayListItemToVideoEntityConverter {
    type Error = String;

    fn try_into(self) -> Result<VideoEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or("Playlist item snippet is missing")?;
        let id = inner.id.ok_or("Playlist item ID is missing")?;
        let title = snippet.title.ok_or("Playlist item title is missing")?;
        let description = snippet
            .description
            .ok_or("Playlist item description is missing")?;

        let channel_id = snippet.channel_id.ok_or("Channel ID is missing")?;
        let channel_name = snippet.channel_title.ok_or("Channel name is missing")?;

        Ok(VideoEntity {
            id: VideoId::from(id),
            title: VideoTitle::from(title),
            description: VideoDescription::from(description),
            channel: ChannelEntity::new(
                ChannelId::from(channel_id),
                ChannelName::from(channel_name),
            ),
        })
    }
}

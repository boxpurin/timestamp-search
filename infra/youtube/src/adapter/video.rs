use domains::entities::channel::ChannelEntity;
use domains::entities::video::VideoEntity;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_title::VideoTitle;
use google_youtube3::api::{PlaylistItem, Video};
use domains::value_objects::thumbnail::Thumbnail;
use crate::adapter::thumbnail::ThumbnailsToThumbnailConverter;

/// Converter from YouTube Video to VideoEntity
pub struct VideoEntityConverter(pub Video);

impl TryInto<VideoEntity> for VideoEntityConverter {
    type Error = String;

    fn try_into(self) -> Result<VideoEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or("Video snippet is missing")?;
        let id = inner.id.ok_or("Video ID is missing")?;
        let title = snippet.title.ok_or("Video title is missing")?;
        let description = snippet.description.ok_or("Video description is missing")?;

        let thumbnails = snippet.thumbnails.ok_or("Thumbnails is missing")?;
        let t : Option<Thumbnail> = ThumbnailsToThumbnailConverter(thumbnails)
            .try_into().ok();

        let channel_id = snippet.channel_id.ok_or("Channel ID is missing")?;
        let channel_name = snippet.channel_title.ok_or("Channel name is missing")?;

        let published_at =  snippet.published_at.ok_or("Published date is missing")?;

        let  ls = inner.live_streaming_details.ok_or("Live streaming details are missing")?;
        let a= ls.actual_start_time;

        Ok(VideoEntity::new(
            VideoId::from(id),
            VideoTitle::from(title),
            VideoDescription::from(description),
            ChannelEntity::new(
                ChannelId::from(channel_id),
                ChannelName::from(channel_name),
            ),
            t,
            published_at,
            a,
        ))
    }
}
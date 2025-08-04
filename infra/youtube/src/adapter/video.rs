use crate::adapter::thumbnail::ThumbnailsToThumbnailConverter;
use domains::entities::channel::ChannelEntity;
use domains::entities::video::VideoEntity;
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;
use domains::value_objects::thumbnail::Thumbnail;
use domains::value_objects::video_description::VideoDescription;
use domains::value_objects::video_id::VideoId;
use domains::value_objects::video_tag::VideoTag;
use domains::value_objects::video_title::VideoTitle;
use errors::AppError::{self, DomainParseError};
use google_youtube3::api::Video;

/// Converter from YouTube Video to VideoEntity
pub struct VideoEntityConverter(pub Video);

impl TryInto<VideoEntity> for VideoEntityConverter {
    type Error = AppError;

    fn try_into(self) -> Result<VideoEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner
            .snippet
            .ok_or(DomainParseError("Video snippet is missing".to_string()))?;
        let id = inner
            .id
            .ok_or(DomainParseError("Video id is missing".to_string()))?;
        let title = snippet
            .title
            .ok_or(DomainParseError("Video title is missing".to_string()))?;
        let description = snippet
            .description
            .ok_or(DomainParseError("Video description is missing".to_string()))?;
        let thumbnails = snippet
            .thumbnails
            .ok_or(DomainParseError("Thumbnails is missing".to_string()))?;
        let t: Option<Thumbnail> = ThumbnailsToThumbnailConverter(thumbnails).try_into().ok();

        let tags = snippet
            .tags
            .unwrap_or_default()
            .iter()
            .map(|tag| VideoTag::new(tag).unwrap())
            .collect::<Vec<VideoTag>>();

        let channel_id = snippet
            .channel_id
            .ok_or(DomainParseError("Channel ID is missing".to_string()))?;
        let channel_name = snippet
            .channel_title
            .ok_or(DomainParseError("Channel name is missing".to_string()))?;

        let published_at = snippet
            .published_at
            .ok_or(DomainParseError("Published date is missing".to_string()))?;

        let ls = inner.live_streaming_details.ok_or(DomainParseError(
            "Live streaming details are missing".to_string(),
        ))?;
        let a = ls.actual_start_time;

        Ok(VideoEntity::new(
            VideoId::from(id),
            VideoTitle::from(title),
            tags,
            VideoDescription::from(description),
            ChannelEntity::new(ChannelId::from(channel_id), ChannelName::from(channel_name)),
            t,
            published_at,
            a,
        ))
    }
}

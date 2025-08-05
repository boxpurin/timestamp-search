use crate::repositories::external_video_repository::ExternalVideoRepository;
use crate::repositories::internal_video_repository::InternalVideoRepository;
use crate::value_objects::channel_id::ChannelId;
use errors::AppResult;

pub struct VideoFetchService<E: ExternalVideoRepository, I: InternalVideoRepository> {
    external_video_repository: E,
    internal_video_repository: I,
}

impl<E: ExternalVideoRepository, I: InternalVideoRepository> VideoFetchService<E, I> {
    pub fn new(external_video_repository: E, internal_video_repository: I) -> Self {
        VideoFetchService {
            external_video_repository,
            internal_video_repository,
        }
    }

    pub async fn fetch_recent_video_by_channel_id(
        &self,
        channel_id: &ChannelId,
        count: u32,
    ) -> AppResult<()> {
        let v = self
            .external_video_repository
            .fetch_recent_video_by_channel_id(channel_id, count)
            .await?;

        tracing::info!("Downloaded videos count {}", v.len());
        for video in &v {
            if !self
                .internal_video_repository
                .find_video_entity_by_id(&video.id)
                .await?
            {
                self.internal_video_repository
                    .add_video_entity(video)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn fetch_all_videos_by_channel_id(&self, channel_id: &ChannelId) -> AppResult<()> {
        let v = self
            .external_video_repository
            .fetch_all_videos_by_channel_id(channel_id)
            .await?;

        tracing::info!("Downloaded videos count {}", v.len());
        for video in &v {
            let ret = self
                .internal_video_repository
                .find_video_entity_by_id(&video.id)
                .await?;
            if !ret
            {
                self.internal_video_repository
                    .add_video_entity(video)
                    .await?;
            }
        }

        Ok(())
    }
}

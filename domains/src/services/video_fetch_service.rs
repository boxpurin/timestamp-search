use crate::repositories::external_video_repository::ExternalVideoRepository;
use crate::repositories::internal_video_repository::InternalVideoRepository;
use crate::value_objects::channel_id::ChannelId;
use errors::AppResult;
use crate::entities::video::VideoEntity;
use std::sync::Arc;

pub struct VideoFetchService<E: ExternalVideoRepository> {
    external_video_repository: Arc<E>,
}

impl<E: ExternalVideoRepository> VideoFetchService<E> {
    pub fn new(external_video_repository: Arc<E>) -> Self {
        VideoFetchService {
            external_video_repository,
        }
    }

    pub async fn fetch_recent_video_by_channel_id(
        &self,
        channel_id: &ChannelId,
        count: u32,
    ) -> AppResult<Vec<VideoEntity>> {
        let v = self
            .external_video_repository
            .fetch_recent_video_by_channel_id(channel_id, count)
            .await?;

        tracing::info!("Downloaded videos count {}", v.len());
        Ok(v)
    }

    pub async fn fetch_all_videos_by_channel_id(&self, channel_id: &ChannelId) -> AppResult<Vec<VideoEntity>> {
        let v = self
            .external_video_repository
            .fetch_all_videos_by_channel_id(channel_id)
            .await?;

        tracing::info!("Downloaded videos count {}", v.len());
        Ok(v)
    }
}

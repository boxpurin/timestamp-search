use errors::AppResult;
use domains::entities::video::VideoEntity;
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::repositories::internal_timestamp_repository::InternalVideoTimeStampRepository;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use std::sync::Arc;

pub struct TimeStampIndexingService<I:InternalVideoTimeStampRepository, V:InternalVideoRepository>{
    v_repo: Arc<I>,
    ts_repo: Arc<V>,
}

impl<I:InternalVideoTimeStampRepository, V:InternalVideoRepository> TimeStampIndexingService<I, V> {
    pub fn new(repo: Arc<I>, video_repo: Arc<V>) -> Self {
        Self{
            v_repo: repo,
            ts_repo: video_repo
        }
    }

    pub async fn add_or_update_timestamp(&self, v: &VideoEntity, ts: &VideoTimestampEntity) -> AppResult<()> {
        if self.ts_repo.find_video_entity_by_id(&v.id).await? {
            self.v_repo.add_video_timestamp_entity(v, ts).await?;
        }
        Ok(())
    }
    pub async fn add_or_update_timestamps(&self, v: &VideoEntity, tss: &[VideoTimestampEntity]) -> AppResult<()> {
        if self.ts_repo.find_video_entity_by_id(&v.id).await? {
            self.v_repo.add_video_timestamp_entities(v, tss).await?;
        }
        Ok(())
    }
}


use errors::AppResult;
use domains::entities::video::VideoEntity;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use std::sync::Arc;
pub struct VideoIndexingService<I: InternalVideoRepository> {
    pub repo: Arc<I>
}

impl<I: InternalVideoRepository> VideoIndexingService<I> {
    pub fn new(repo: Arc<I>) -> Self {
        Self { repo }
    }

    pub async fn add_or_update_video_entities(&self, entities: &[VideoEntity]) -> AppResult<()>{
        self.repo.update_video_entities(entities).await?;
        Ok(())
    }
}
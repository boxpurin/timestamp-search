use domains::entities::video::VideoEntity;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use domains::value_objects::video_id::VideoId;

use crate::client::ApiClient;
use crate::index::Index;
use crate::index::video::VideoIndex;
use crate::repositories::MeiliSearchCrudApi;
use errors::{AppError, AppResult};

pub struct MeiliSearchVideoCrudRepository<
    T: MeiliSearchCrudApi<VideoIndex, VideoEntity> + Send + Sync,
> {
    client: T,
}

pub fn create_video_crud_repository() -> MeiliSearchVideoCrudRepository<ApiClient> {
    MeiliSearchVideoCrudRepository {
        client: ApiClient::new(),
    }
}

#[async_trait::async_trait]
impl<T: MeiliSearchCrudApi<VideoIndex, VideoEntity> + Send + Sync> InternalVideoRepository
    for MeiliSearchVideoCrudRepository<T>
{
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for adding a video entity to MeiliSearch
        self.client
            .add_entity(VideoIndex::name(), video_entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        self.client
            .add_entities(VideoIndex::name(), video_entities)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for updating a video entity in MeiliSearch
        self.client
            .update_entity(VideoIndex::name(), video_entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        // Implementation for updating multiple video entities in MeiliSearch
        self.client
            .update_entities(VideoIndex::name(), video_entities)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<bool> {
        // Implementation for finding a video entity by ID in MeiliSearch
        let exists = self
            .client
            .find_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(exists)
    }

    async fn get_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<Option<VideoEntity>> {
        // Implementation for getting a video entity by ID from MeiliSearch
        let result = self
            .client
            .get_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(result.map(|entity| entity.into()))
    }

    async fn get_all_video_entities(&self) -> AppResult<Vec<VideoEntity>> {
        // Implementation for getting all video entities from MeiliSearch
        let result = self
            .client
            .get_all_entities(VideoIndex::name())
            .await
            .map_err(AppError::from)?;
        Ok(result.into_iter().map(|entity| entity.into()).collect())
    }

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<()> {
        // Implementation for deleting a video entity by ID from MeiliSearch
        self.client
            .delete_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn delete_all_video_entities(&self) -> AppResult<()> {
        // Implementation for deleting all video entities from MeiliSearch
        self.client
            .delete_all_entities(VideoIndex::name())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

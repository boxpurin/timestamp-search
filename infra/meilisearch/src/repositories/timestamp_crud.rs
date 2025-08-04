use crate::client::ApiClient;
use crate::index::Index;
use crate::index::timestamp::{TimeStampIndex, VideoTimeStampDetails};
use crate::repositories::MeiliSearchCrudApi;
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::repositories::internal_timestamp_repository::InternalVideoTimeStampRepository;
use domains::value_objects::timestamp_id::TimestampId;
use domains::value_objects::video_id::VideoId;
use errors::{AppError, AppResult};

pub struct MeiliSearchVideoCrudRepository<
    T: MeiliSearchCrudApi<TimeStampIndex, VideoTimestampEntity> + Send + Sync,
> {
    client: T,
}

pub fn create_timestamp_crud_repository() -> MeiliSearchVideoCrudRepository<ApiClient> {
    MeiliSearchVideoCrudRepository {
        client: ApiClient::new(),
    }
}

#[async_trait::async_trait]
impl<T: MeiliSearchCrudApi<TimeStampIndex, VideoTimestampEntity> + Send + Sync>
    InternalVideoTimeStampRepository for MeiliSearchVideoCrudRepository<T>
{
    async fn add_video_timestamp_entity(&self, entity: &VideoTimestampEntity) -> AppResult<()> {
        self.client
            .add_entity(TimeStampIndex::name(), entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn add_video_timestamp_entities(
        &self,
        entities: &[VideoTimestampEntity],
    ) -> AppResult<()> {
        self.client
            .add_entities(TimeStampIndex::name(), entities)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_timestamp_entity(&self, entity: &VideoTimestampEntity) -> AppResult<()> {
        self.client
            .update_entity(TimeStampIndex::name(), entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_timestamp_entities(
        &self,
        entities: &[VideoTimestampEntity],
    ) -> AppResult<()> {
        self.client
            .update_entities(TimeStampIndex::name(), entities)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    async fn find_video_timestamp_entity_by_id(&self, video_id: &TimestampId) -> AppResult<bool> {
        let exists = self
            .client
            .find_entity_by_id(TimeStampIndex::name(), video_id.as_str())
            .await?;
        Ok(exists)
    }

    async fn get_video_timestamp_entity_by_id(
        &self,
        video_id: &TimestampId,
    ) -> AppResult<Option<VideoTimestampEntity>> {
        let entity = self
            .client
            .get_entity_by_id(TimeStampIndex::name(), video_id.as_str())
            .await?;
        Ok(entity)
    }

    async fn get_all_video_timestamp_entities(&self) -> AppResult<Vec<VideoTimestampEntity>> {
        let entities = self.client.get_all_entities(TimeStampIndex::name()).await?;
        Ok(entities)
    }

    async fn delete_video_timestamp_entity_by_id(&self, video_id: &VideoId) -> AppResult<()> {
        self.client
            .delete_entity_by_id(TimeStampIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn delete_video_timestamp_entity_by_video_id(&self, video_id: &VideoId) -> AppResult<()> {
        self.client
            .delete_entity_by_id(TimeStampIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn delete_all_video_timestamp_entities(&self) -> AppResult<()> {
        self.client
            .delete_all_entities(TimeStampIndex::name())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

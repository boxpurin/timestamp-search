use crate::client::ApiClient;
use crate::index::Index;
use crate::index::timestamp::TimeStampIndex;
use crate::repositories::MeilisearchCrudApi;
use domains::entities::video::VideoEntity;
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::repositories::internal_timestamp_repository::InternalVideoTimeStampRepository;
use domains::value_objects::timestamp_id::TimestampId;
use domains::value_objects::video_id::VideoId;
use errors::{AppError, AppResult};

pub struct MeilisearchVideoCrudRepository<T: MeilisearchCrudApi<TimeStampIndex> + Send + Sync> {
    client: T,
}

impl<T: MeilisearchCrudApi<TimeStampIndex> + Send + Sync> MeilisearchVideoCrudRepository<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

pub fn create_timestamp_crud_repository() -> MeilisearchVideoCrudRepository<ApiClient> {
    MeilisearchVideoCrudRepository {
        client: ApiClient::new(),
    }
}

#[async_trait::async_trait]
impl<T: MeilisearchCrudApi<TimeStampIndex> + Send + Sync> InternalVideoTimeStampRepository
    for MeilisearchVideoCrudRepository<T>
{
    async fn add_video_timestamp_entity(
        &self,
        video_entity: &VideoEntity,
        timestamp_entity: &VideoTimestampEntity,
    ) -> AppResult<()> {
        let i =
            TimeStampIndex::from_entity(video_entity.clone(), timestamp_entity.timestamp.clone());

        self.client.add_entity(TimeStampIndex::name(), &i).await?;
        Ok(())
    }

    async fn add_video_timestamp_entities(
        &self,
        video_entity: &VideoEntity,
        entities: &[VideoTimestampEntity],
    ) -> AppResult<()> {
        let v = entities
            .iter()
            .map(|e| TimeStampIndex::from_entity(video_entity.clone(), e.timestamp.clone()))
            .collect::<Vec<TimeStampIndex>>();

        // Implementation for adding a video entity to MeiliSearch
        self.client
            .add_entities(TimeStampIndex::name(), v.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_timestamp_entity(
        &self,
        video_entity: &VideoEntity,
        entity: &VideoTimestampEntity,
    ) -> AppResult<()> {
        let i = TimeStampIndex::from_entity(video_entity.clone(), entity.timestamp.clone());

        self.client
            .update_entity(TimeStampIndex::name(), &i)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_timestamp_entities(
        &self,
        video_entity: &VideoEntity,
        entities: &[VideoTimestampEntity],
    ) -> AppResult<()> {
        let v = entities
            .iter()
            .map(|e| TimeStampIndex::from_entity(video_entity.clone(), e.timestamp.clone()))
            .collect::<Vec<TimeStampIndex>>();

        self.client
            .update_entities(TimeStampIndex::name(), &v)
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
        Ok(entity.map(|i| i.into()))
    }

    async fn get_all_video_timestamp_entities(&self) -> AppResult<Vec<VideoTimestampEntity>> {
        let entities = self.client.get_all_entities(TimeStampIndex::name()).await?;
        Ok(entities.into_iter().map(|i| i.into()).collect())
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

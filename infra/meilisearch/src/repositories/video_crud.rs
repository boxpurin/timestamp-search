use domains::entities::video::VideoEntity;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use domains::value_objects::video_id::VideoId;

use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::Error as MeilisearchError;
use serde::Serialize;
use errors::{AppResult, AppError};

pub struct MeiliSearchVideoCrudRepository<T: MeiliSearchApi + Send + Sync> {
    client: T,
}

pub struct ApiClient {
    pub client: Client,
}

impl ApiClient {
    fn new() -> Self {
        let client = Client::new("http://localhost:7700", Some("masterKey"))
            .expect("Error creating meilisearch client");

        ApiClient { client }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MeiliSearchApi {
    async fn add_entity<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError>;

    async fn add_entities<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        entities: &[T],
    ) -> Result<(), MeilisearchError>;

    async fn update_entity<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError>;

    async fn update_entities<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        entities: &[T],
    ) -> Result<(), MeilisearchError>;

    async fn find_entity_by_id<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<bool, MeilisearchError>;

    async fn find_entities_by_id<T: Serialize + Send + Sync + 'static>
        (&self, index_name: &str, ids: &[String])
        -> Result<Vec<T>, MeilisearchError>;

    async fn get_entity_by_id<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<Option<T>, MeilisearchError>;

    async fn get_all_entities<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
    ) -> Result<Vec<T>, MeilisearchError>;

    async fn delete_entity_by_id<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<(), MeilisearchError>;

    async fn delete_all_entities<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
    ) -> Result<(), MeilisearchError>;
}

#[async_trait::async_trait]
impl<T: MeiliSearchApi + Send + Sync> InternalVideoRepository
    for MeiliSearchVideoCrudRepository<T>
{
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for adding a video entity to MeiliSearch
        self.client.add_entity("video_index", video_entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        // Implementation for adding multiple video entities to MeiliSearch
        Ok(())
    }

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for updating a video entity in MeiliSearch
        Ok(())
    }

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        // Implementation for updating multiple video entities in MeiliSearch
        Ok(())
    }

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<bool> {
        // Implementation for finding a video entity by ID in MeiliSearch
        Ok(true)
    }

    async fn get_video_entity_by_id(
        &self,
        video_id: &VideoId,
    ) -> AppResult<Option<VideoEntity>> {
        // Implementation for getting a video entity by ID from MeiliSearch
        Ok(None)
    }

    async fn get_all_video_entities(&self) -> AppResult<Vec<VideoEntity>> {
        // Implementation for getting all video entities from MeiliSearch
        Ok(vec![])
    }

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<()> {
        // Implementation for deleting a video entity by ID from MeiliSearch
        Ok(())
    }

    async fn delete_all_video_entities(&self) -> AppResult<()> {
        // Implementation for deleting all video entities from MeiliSearch
        Ok(())
    }
}

#[async_trait::async_trait]
impl MeiliSearchApi for ApiClient {
    async fn add_entity<T: Serialize + Send + Sync + 'static>(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_documents(&[entity], Some("id"))
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn add_entities<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, entities: &[T]) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_documents(entities, Some("id"))
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;
        Ok(())
    }

    async fn update_entity<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, entity: &T) -> Result<(), MeilisearchError> {
        todo!()
    }

    async fn update_entities<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, entities: &[T]) -> Result<(), MeilisearchError> {
        todo!()
    }

    async fn find_entity_by_id<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, id: &str) -> Result<bool, MeilisearchError> {
        todo!()
    }

    async fn find_entities_by_id<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, ids: &[String]) -> Result<Vec<T>, MeilisearchError> {
        todo!()
    }

    async fn get_entity_by_id<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, id: &str) -> Result<Option<T>, MeilisearchError> {
        todo!()
    }

    async fn get_all_entities<T: Serialize + Send + Sync + 'static>(&self, index_name: &str) -> Result<Vec<T>, MeilisearchError> {
        todo!()
    }

    async fn delete_entity_by_id<T: Serialize + Send + Sync + 'static>(&self, index_name: &str, id: &str) -> Result<(), MeilisearchError> {
        todo!()
    }

    async fn delete_all_entities<T: Serialize + Send + Sync + 'static>(&self, index_name: &str) -> Result<(), MeilisearchError> {
        todo!()
    }
}

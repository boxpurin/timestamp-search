use domains::entities::video::VideoEntity;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use domains::value_objects::video_id::VideoId;

use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::{Error as MeilisearchError, ErrorCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use errors::{AppResult, AppError};
use crate::index::Index;
use crate::index::video::VideoIndex;
use crate::repositories::MeiliSearchApi;

pub struct MeiliSearchVideoCrudRepository<T: MeiliSearchApi<VideoIndex, VideoEntity> + Send + Sync> {
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

#[async_trait::async_trait]
impl<T: MeiliSearchApi<VideoIndex, VideoEntity> + Send + Sync> InternalVideoRepository
    for MeiliSearchVideoCrudRepository<T>
{
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for adding a video entity to MeiliSearch
        self.client.add_entity(VideoIndex::name(), video_entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        self.client.add_entities(VideoIndex::name(), video_entities)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
        // Implementation for updating a video entity in MeiliSearch
        self.client.update_entity(VideoIndex::name(), video_entity)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
        // Implementation for updating multiple video entities in MeiliSearch
        self.client.update_entities(VideoIndex::name(), video_entities)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<bool> {
        // Implementation for finding a video entity by ID in MeiliSearch
        let exists = self.client.find_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(exists)
    }

    async fn get_video_entity_by_id(
        &self,
        video_id: &VideoId,
    ) -> AppResult<Option<VideoEntity>> {
        // Implementation for getting a video entity by ID from MeiliSearch
        let result = self.client.get_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(result.map(|entity| entity.into()))
    }

    async fn get_all_video_entities(&self) -> AppResult<Vec<VideoEntity>> {
        // Implementation for getting all video entities from MeiliSearch
        let result = self.client.get_all_entities(VideoIndex::name())
            .await
            .map_err(AppError::from)?;
        Ok(result.into_iter().map(|entity| entity.into()).collect())
    }

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<()> {
        // Implementation for deleting a video entity by ID from MeiliSearch
        self.client.delete_entity_by_id(VideoIndex::name(), video_id.as_str())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn delete_all_video_entities(&self) -> AppResult<()> {
        // Implementation for deleting all video entities from MeiliSearch
        self.client.delete_all_entities(VideoIndex::name())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<I: Index + Serialize + Sync + Send + 'static,
    T: Serialize + DeserializeOwned + Send + Sync + 'static> MeiliSearchApi<I, T> for ApiClient {
    async fn add_entity(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_documents(&[entity], I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn add_entities(&self, index_name: &str, entities: &[T]) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_documents(entities, I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn update_entity(&self, index_name: &str, entity: &T) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_or_update(&[entity], I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn update_entities(&self, index_name: &str, entities: &[T]) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_or_update(&entities, I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn find_entity_by_id(&self, index_name: &str, id: &str) -> Result<bool, MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let result = i.get_document::<T>(id).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if let MeilisearchError::Meilisearch(e) = &e {
                    if e.error_code == ErrorCode::DocumentNotFound {
                        return Ok(false);
                    }
                }
                Err(e)
            },
        }
    }

    async fn get_entity_by_id(&self, index_name: &str, id: &str) -> Result<Option<T>, MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let result = i.get_document::<T>(id).await;

        match result {
            Ok(document) => Ok(Some(document)),
            Err(e) => {
                if let MeilisearchError::Meilisearch(e) = &e {
                    if e.error_code == ErrorCode::DocumentNotFound {
                        return Ok(None);
                    }
                }
                Err(e)
            },
        }
    }

    async fn get_all_entities(&self, index_name: &str) -> Result<Vec<T>, MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let result = i.get_documents::<T>().await;

        match result {
            Ok(documents) => Ok(documents.results),
            Err(e) => Err(e),
        }
    }

    async fn delete_entity_by_id(&self, index_name: &str, id: &str) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i.delete_document(id).await?;
        Ok(())
    }

    async fn delete_all_entities(&self, index_name: &str) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i.delete_all_documents().await?;
        Ok(())
    }
}

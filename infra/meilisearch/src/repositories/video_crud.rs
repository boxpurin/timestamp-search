use domains::entities::video::VideoEntity;
use domains::repositories::internal_video_repository::InternalVideoRepository;
use domains::value_objects::video_id::VideoId;

use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::{Error as MeilisearchError, ErrorCode, ErrorType};
use meilisearch_sdk::macro_helper::async_trait;
use serde::Serialize;

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
    async fn add_entity<T: Serialize + Send + Sync>(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError>;
}

#[async_trait::async_trait]
impl<T: MeiliSearchApi + Send + Sync> InternalVideoRepository
    for MeiliSearchVideoCrudRepository<T>
{
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> Result<(), String> {
        // Implementation for adding a video entity to MeiliSearch
        Ok(())
    }

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> Result<(), String> {
        // Implementation for adding multiple video entities to MeiliSearch
        Ok(())
    }

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> Result<(), String> {
        // Implementation for updating a video entity in MeiliSearch
        Ok(())
    }

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> Result<(), String> {
        // Implementation for updating multiple video entities in MeiliSearch
        Ok(())
    }

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> Result<bool, String> {
        // Implementation for finding a video entity by ID in MeiliSearch
        Ok(true)
    }

    async fn get_video_entity_by_id(
        &self,
        video_id: &VideoId,
    ) -> Result<Option<domains::entities::video::VideoEntity>, String> {
        // Implementation for getting a video entity by ID from MeiliSearch
        Ok(None)
    }

    async fn get_all_video_entities(&self) -> Result<Vec<VideoEntity>, String> {
        // Implementation for getting all video entities from MeiliSearch
        Ok(vec![])
    }

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> Result<(), String> {
        // Implementation for deleting a video entity by ID from MeiliSearch
        Ok(())
    }

    async fn delete_all_video_entities(&self) -> Result<(), String> {
        // Implementation for deleting all video entities from MeiliSearch
        Ok(())
    }
}

#[async_trait::async_trait]
impl MeiliSearchApi for ApiClient {
    async fn add_entity<T: Serialize + Send + Sync>(
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
}

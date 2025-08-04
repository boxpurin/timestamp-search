use serde::de::DeserializeOwned;
use serde::Serialize;
use meilisearch_sdk::errors::{Error as MeilisearchError};
use crate::index::Index;
pub mod video_crud;
pub mod timestamp_crud;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MeiliSearchCrudApi<
    I: Serialize + Index + Send + Sync + 'static, 
    T: Serialize + DeserializeOwned + Send + Sync + 'static> {
    async fn add_entity(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError>;

    async fn add_entities(
        &self,
        index_name: &str,
        entities: &[T],
    ) -> Result<(), MeilisearchError>;

    async fn update_entity(
        &self,
        index_name: &str,
        entity: &T,
    ) -> Result<(), MeilisearchError>;

    async fn update_entities(
        &self,
        index_name: &str,
        entities: &[T],
    ) -> Result<(), MeilisearchError>;

    async fn find_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<bool, MeilisearchError>;

    async fn get_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<Option<T>, MeilisearchError>;

    async fn get_all_entities(
        &self,
        index_name: &str,
    ) -> Result<Vec<T>, MeilisearchError>;

    async fn delete_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<(), MeilisearchError>;

    async fn delete_all_entities(
        &self,
        index_name: &str,
    ) -> Result<(), MeilisearchError>;
}
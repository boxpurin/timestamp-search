use crate::index::Index;
use domains::repositories::internal_timestamp_search_repository::VideoTimestampSearchQuery;
use errors::AppResult;
use meilisearch_sdk::errors::Error as MeilisearchError;
use meilisearch_sdk::search::SearchResults;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod timestamp_crud;
pub mod timestamp_search;
pub mod video_crud;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MeilisearchCrudApi<I: Serialize + Index + DeserializeOwned + Send + Sync + 'static> {
    async fn add_entity(&self, index_name: &str, index: &I) -> Result<(), MeilisearchError>;

    async fn add_entities(&self, index_name: &str, entities: &[I]) -> Result<(), MeilisearchError>;

    async fn update_entity(&self, index_name: &str, entity: &I) -> Result<(), MeilisearchError>;

    async fn update_entities(
        &self,
        index_name: &str,
        entities: &[I],
    ) -> Result<(), MeilisearchError>;

    async fn find_entity_by_id(&self, index_name: &str, id: &str)
    -> Result<bool, MeilisearchError>;

    async fn get_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<Option<I>, MeilisearchError>;

    async fn get_all_entities(&self, index_name: &str) -> Result<Vec<I>, MeilisearchError>;

    async fn delete_entity_by_id(&self, index_name: &str, id: &str)
    -> Result<(), MeilisearchError>;

    async fn delete_all_entities(&self, index_name: &str) -> Result<(), MeilisearchError>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MeilisearchSearchApi<I: Serialize + Index + DeserializeOwned + Send + Sync + 'static> {
    async fn search_by_query(
        &self,
        search_query: VideoTimestampSearchQuery,
    ) -> AppResult<SearchResults<I>>;
}

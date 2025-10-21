use crate::config::CONFIG;
use crate::index::Index;
use crate::index::timestamp::TimeStampIndex;
use crate::repositories::{MeilisearchCrudApi, MeilisearchSearchApi};
use domains::repositories::internal_timestamp_search_repository::{
    Part, VideoTimestampSearchQuery,
};
use errors::AppResult;
use itertools::Itertools;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::{Error as MeilisearchError, ErrorCode};
use meilisearch_sdk::search::{SearchQuery as MeilisearchSearchQuery, SearchResults, Selectors};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashSet;

pub struct ApiClient {
    pub client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        tracing::info!(
            "Create Connection to MeiliSearch at {}",
            CONFIG.connection_addr
        );

        let client = Client::new(&CONFIG.connection_addr, Some(&CONFIG.master_key))
            .expect("Error creating meilisearch client");

        Self { client }
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<I: Index + Serialize + DeserializeOwned + Sync + Send + 'static> MeilisearchCrudApi<I>
    for ApiClient
{
    async fn add_entity(&self, index_name: &str, entity: &I) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;

        let task = i
            .add_documents(&[entity], I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        if task.is_failure() {
            let f = task.unwrap_failure();
            tracing::error!("task failure : {}", f.error_message);
        }

        Ok(())
    }

    async fn add_entities(&self, index_name: &str, entities: &[I]) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;

        let _ = i
            .add_documents(entities, I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn update_entity(&self, index_name: &str, entity: &I) -> Result<(), MeilisearchError> {
        tracing::debug!("update_entity. index_name : {}", index_name);
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_or_update(&[entity], I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn update_entities(
        &self,
        index_name: &str,
        entities: &[I],
    ) -> Result<(), MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let _ = i
            .add_or_update(entities, I::pid_field())
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    async fn find_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<bool, MeilisearchError> {
        tracing::info!("find entity by id");
        let i = self.client.get_index(index_name).await;
        let result: Result<I, MeilisearchError> = i?.get_document::<I>(id).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                #[allow(clippy::collapsible_if)]
                if let MeilisearchError::Meilisearch(e) = &e {
                    if e.error_code == ErrorCode::DocumentNotFound {
                        tracing::info!("DocumentNotFound");
                        return Ok(false);
                    }
                }
                tracing::error!("error");
                Err(e)
            }
        }
    }

    async fn get_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<Option<I>, MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let result = i.get_document::<I>(id).await;

        match result {
            Ok(document) => Ok(Some(document)),
            Err(e) => {
                #[allow(clippy::collapsible_if)]
                if let MeilisearchError::Meilisearch(e) = &e {
                    if e.error_code == ErrorCode::DocumentNotFound {
                        return Ok(None);
                    }
                }
                Err(e)
            }
        }
    }

    async fn get_all_entities(&self, index_name: &str) -> Result<Vec<I>, MeilisearchError> {
        let i = self.client.get_index(index_name).await?;
        let result = i.get_documents::<I>().await;

        match result {
            Ok(documents) => Ok(documents.results),
            Err(e) => Err(e),
        }
    }

    async fn delete_entity_by_id(
        &self,
        index_name: &str,
        id: &str,
    ) -> Result<(), MeilisearchError> {
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

#[async_trait::async_trait]
impl MeilisearchSearchApi<TimeStampIndex> for ApiClient {
    async fn search_by_query(
        &self,
        search_query: VideoTimestampSearchQuery,
    ) -> AppResult<SearchResults<TimeStampIndex>> {
        let i = self.client.get_index(TimeStampIndex::name()).await?;
        let mut q = MeilisearchSearchQuery::new(&i);

        tracing::debug!("Health check");
        self.client.health().await?;
        tracing::debug!("available.");

        // set query
        q.with_query(search_query.query.as_str());

        // construct filter
        let filter_text = {
            let mut v = Vec::<String>::new();

            #[allow(clippy::collapsible_if)]
            if let Some(ids) = search_query.video_ids {
                if ids.is_empty() {
                    v.push(format!("videoId IN [{}]", ids.join(" , ")));
                }
            };

            #[allow(clippy::collapsible_if)]
            if let Some(tags) = search_query.video_tags {
                if tags.is_empty() {
                    v.push(format!("tagId IN [{}]", tags.join(" , ")));
                }
            }

            if let Some(at) = search_query.actual_start_at {
                v.push(format!("actualStartAt = {}", at.timestamp()));
            } else {
                if let Some(from) = search_query.actual_start_from {
                    v.push(format!("actualStartAt <= {}", from.timestamp()));
                };

                if let Some(to) = search_query.actual_start_to {
                    v.push(format!("actualStartAt >= {}", to.timestamp()));
                }
            }
            v.into_iter().join(" AND ")
        };

        if !filter_text.is_empty() {
            tracing::debug!("filter : {:?}", filter_text);
            q.with_filter(&filter_text);
        }

        // set attributes_to_search_on
        let mut a = HashSet::new();
        a.insert("pid");
        a.insert("videoId");
        a.insert("description");
        a.insert("elapsedTime");

        tracing::debug!("parts : {:?}", search_query.parts);
        if let Some(parts) = search_query.parts {
            for part in parts {
                match part {
                    Part::VideoDetails => {
                        a.insert("videoDetails.videoTitle");
                        a.insert("videoDetails.videoTags");
                        a.insert("videoDetails.thumbnailUrl");
                        a.insert("videoDetails.actualStartAt");
                        a.insert("videoDetails.publishedAt");
                    }
                    Part::VideoTitle => {
                        a.insert("videoDetails.videoTitle");
                    }
                    Part::VideoTags => {
                        a.insert("videoDetails.videoTags");
                    }
                    Part::ThumbnailUrl => {
                        a.insert("videoDetails.thumbnailUrl");
                    }
                    Part::ActualStartAt => {
                        a.insert("videoDetails.actualStartAt");
                    }
                    Part::PublishedAt => {
                        a.insert("videoDetails.publishedAt");
                    }
                };
            }
        }

        let a = Vec::from_iter(a);
        tracing::debug!("with_attributes_to_retrieve : {:?}", a);
        q.with_attributes_to_retrieve(Selectors::Some(&a));

        tracing::debug!("Search page : {:?}", search_query.page);
        q.with_page(search_query.page.into());
        tracing::debug!("Search per_page : {:?}", search_query.per_page);
        q.with_hits_per_page(search_query.per_page.into());
        tracing::debug!("Search limit : {:?}", search_query.limit);
        q.with_limit(search_query.limit.into());

        tracing::debug!("Search query : {:?}", search_query.query);
        Ok(q.execute().await?)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn create_client() {
        let _hub = ApiClient::new();
    }
}

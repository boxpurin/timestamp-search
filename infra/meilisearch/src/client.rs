use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::{ErrorCode, Error as MeilisearchError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::index::Index;
use crate::repositories::MeiliSearchCrudApi;
use crate::config::CONFIG;

pub struct ApiClient {
    pub client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        tracing::info!("Create Connection to MeiliSearch at {}", CONFIG.connection_addr);

        let client = Client::new(&CONFIG.connection_addr, Some(&CONFIG.master_key))
            .expect("Error creating meilisearch client");

        Self { client }
    }
}


#[async_trait::async_trait]
impl<I: Index + Serialize + Sync + Send + 'static,
    T: Serialize + DeserializeOwned + Send + Sync + 'static> MeiliSearchCrudApi<I, T> for ApiClient {
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

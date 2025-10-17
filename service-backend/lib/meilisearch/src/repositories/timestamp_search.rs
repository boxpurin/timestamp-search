use domains::repositories::internal_timestamp_search_repository::{InternalVideoTimeStampSearchRepository, VideoTimestampSearchQuery, VideoTimestampSearchResult};
use errors::AppResult;
use crate::adapter::SearchResultConverter;
use crate::client::ApiClient;
use crate::repositories::MeilisearchSearchApi;

pub struct MeilisearchTimestampSearchRepository {
    client: ApiClient,
}

pub fn create_meilisearch_timestamp_search_repository() -> MeilisearchTimestampSearchRepository {
    MeilisearchTimestampSearchRepository{
        client: ApiClient::new(),
    }
}

#[async_trait::async_trait]
impl InternalVideoTimeStampSearchRepository for MeilisearchTimestampSearchRepository {
    async fn search_timestamps_by_query(&self, query: VideoTimestampSearchQuery) -> AppResult<VideoTimestampSearchResult> {
        let ret = self
            .client
            .search_by_query(query)
            .await?;

        SearchResultConverter::convert_to_domain(ret)
    }
}
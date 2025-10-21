use crate::api::request::SearchTimeStampRequest;
use domains::repositories::internal_timestamp_search_repository::{
    InternalVideoTimeStampSearchRepository, VideoTimestampSearchQuery, VideoTimestampSearchResult,
};
use errors::AppResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct TimeStampSearchService {
    search_repository: Arc<dyn InternalVideoTimeStampSearchRepository + Send + Sync>,
}

impl TimeStampSearchService {
    pub fn new(
        search_repository: Arc<dyn InternalVideoTimeStampSearchRepository + Send + Sync>,
    ) -> Self {
        Self { search_repository }
    }

    pub async fn search_timestamp(
        &self,
        req: SearchTimeStampRequest,
    ) -> AppResult<VideoTimestampSearchResult> {
        tracing::debug!("restful_server::api::service::TimeStampSearchService::search_timestamp");
        let query = VideoTimestampSearchQuery::try_from(req)?;

        tracing::debug!("begin search_timestamps_by_query");
        let v = self
            .search_repository
            .search_timestamps_by_query(query)
            .await?;
        Ok(v)
    }
}
#[allow(dead_code)]
pub struct RestApiService {}

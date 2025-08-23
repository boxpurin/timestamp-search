use domains::repositories::internal_timestamp_search_repository::{InternalVideoTimeStampSearchRepository, VideoTimestampSearchQuery, VideoTimestampSearchResult};
use errors::AppResult;
use crate::request::SearchTimeStampRequest;

pub struct TimeStampSearchService {
    search_repository: Box<dyn InternalVideoTimeStampSearchRepository>,
}

impl TimeStampSearchService {
    pub fn new(search_repository: Box<dyn InternalVideoTimeStampSearchRepository>) -> Self {
        Self { search_repository }
    }

    pub async fn search_timestamp(&self, req: SearchTimeStampRequest) -> AppResult<VideoTimestampSearchResult> {
        let query = VideoTimestampSearchQuery::try_from(req)?;
        let v = self.search_repository.search_timestamps_by_query(query).await?;
        Ok(v)
    }
}
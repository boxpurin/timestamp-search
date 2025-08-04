use crate::entities::video::VideoEntity;
use errors::AppResult;

pub struct SearchQuery {
    pub query: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoSearchRepository {
    async fn search_videos_by_query(&self, query: &SearchQuery) -> AppResult<Vec<VideoEntity>>;
}

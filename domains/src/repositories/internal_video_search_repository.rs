use crate::entities::video::VideoEntity;
use async_trait::async_trait;

pub struct SearchQuery {
    pub query: String,
}

#[async_trait]
pub trait InternalVideoSearchRepository {
    async fn search_videos_by_query(&self, query: &SearchQuery)
    -> Result<Vec<VideoEntity>, String>;
}

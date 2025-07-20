use crate::entities::video::VideoEntity;

pub struct SearchQuery {
    pub query: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoSearchRepository {
    async fn search_videos_by_query(&self, query: &SearchQuery)
    -> Result<Vec<VideoEntity>, String>;
}

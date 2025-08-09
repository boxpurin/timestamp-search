use crate::entities::video::VideoEntity;
use errors::AppResult;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct SearchQuery {
    pub query: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoSearchRepository {
    async fn search_videos_by_query(&self, query: &SearchQuery) -> AppResult<Vec<VideoEntity>>;
}

#[cfg(test)]
mod unit_tests{
    use crate::repositories::internal_video_search_repository::MockInternalVideoSearchRepository;
    use super::*;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn mock_internal_video_search(){
        let mut mock = MockInternalVideoSearchRepository::new();
        let query = SearchQuery{ query : "query".to_string() };

        mock.expect_search_videos_by_query()
            .with(eq(query.clone()))
            .returning(|q| Ok(vec![]));

        let v = mock.search_videos_by_query(&query).await;
        assert!(v.is_ok());
    }
}
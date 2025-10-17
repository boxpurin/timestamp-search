use crate::entities::video_timestamp::VideoTimestampEntity;
use crate::value_objects::search_query_text::SearchQueryText;
use crate::value_objects::{
    video_id::VideoId,
    video_tag::VideoTag,
    limit::Limit
};
use chrono::{DateTime, Utc};
use errors::{AppError, AppResult};
use std::str::FromStr;
use crate::value_objects::page::Page;
use crate::value_objects::per_page::PerPage;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoTimeStampSearchRepository {
    async fn search_timestamps_by_query(&self, query: VideoTimestampSearchQuery) -> AppResult<VideoTimestampSearchResult>;
}


/// 検索クエリ
///
pub struct VideoTimestampSearchQuery {
    pub query: SearchQueryText,
    pub video_ids: Option<Vec<VideoId>>,
    pub video_tags: Option<Vec<VideoTag>>,
    pub actual_start_from: Option<DateTime<Utc>>, // Unix timestamp in seconds
    pub actual_start_to: Option<DateTime<Utc>>,   // Unix timestamp in seconds
    pub actual_start_at: Option<DateTime<Utc>>,
    pub parts: Option<Vec<Part>>,
    pub limit: Limit,
    pub page: Page,
    pub per_page: PerPage,
}

pub struct VideoTimestampSearchResult {
    pub items: Vec<VideoTimestampEntity>,
    pub page: Page,
    pub per_page: PerPage,
    pub total_pages: usize,
    pub total_hits: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Part{
    VideoDetail,
    VideoTitle,
    VideoTags,
    ThumbnailUrl,
    ActualStartAt,
    PublishedAt,
}

impl FromStr for Part {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "videoDetail" => Ok(Part::VideoDetail),
            "videoTitle" => Ok(Part::VideoTitle),
            "videoTags" => Ok(Part::VideoTags),
            "thumbnailUrl" => Ok(Part::ThumbnailUrl),
            "actualStartAt" => Ok(Part::ActualStartAt),
            "publishedAt" => Ok(Part::PublishedAt),
            _ => Err(AppError::InvalidInput(
                format!(r#"Invalid part. \
                    required 'videoDetail',\
                    'videoTitle',\
                    'videoTags',\
                    'thumbnailUrl',\
                    'actualStartAt',\
                    'publishedAt'.\
                    input : {}"#, s)
            ))
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use mockall::predicate::eq;
    use crate::repositories::internal_video_search_repository::{MockInternalVideoSearchRepository, SearchQuery, InternalVideoSearchRepository};

    #[tokio::test]
    async fn internal_timestamp_search_repository_mock(){
        let mut mock = MockInternalVideoSearchRepository::new();
        let query = SearchQuery{ query : "query".to_string() };

        mock.expect_search_videos_by_query()
            .with(eq(query.clone()))
            .returning(|_| Ok(vec![]));

        let v = mock.search_videos_by_query(&query).await;
        assert!(v.is_ok());
    }

}
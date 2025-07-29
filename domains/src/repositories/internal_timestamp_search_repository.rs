use crate::entities::video::VideoEntity;
use errors::AppResult;
use chrono::{DateTime, Utc};
use crate::value_objects::video_id::VideoId;
use crate::value_objects::search_query_text::SearchQueryText;
/// 検索クエリ
///
pub struct SearchQuery {
    pub query: SearchQueryText,
    pub video_id: Option<VideoId>,
    pub actual_start_from: Option<DateTime<Utc>>, // Unix timestamp in seconds
    pub actual_start_to: Option<DateTime<Utc>>, // Unix timestamp in seconds
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoTimeStampSearchRepository {
    async fn search_timestamps_by_query(&self, query: SearchQuery)
                                    -> AppResult<Vec<VideoEntity>>;
}

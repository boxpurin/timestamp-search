use crate::index::timestamp::TimeStampIndex;
use domains::repositories::internal_timestamp_search_repository::VideoTimestampSearchResult;
use domains::value_objects::page::Page;
use domains::value_objects::per_page::PerPage;
use errors::{AppError, AppResult};
use meilisearch_sdk::search::SearchResults;

pub struct SearchResultConverter;

impl SearchResultConverter {
    pub fn convert_to_domain(
        results: SearchResults<TimeStampIndex>,
    ) -> AppResult<VideoTimestampSearchResult> {
        Ok(VideoTimestampSearchResult {
            items: results
                .hits
                .into_iter()
                .map(|v| v.result.into())
                .collect::<_>(),
            page: Page::new(
                results
                    .page
                    .ok_or(AppError::InvalidInput("page is missing".to_owned()))?,
            )?,
            per_page: PerPage::new(
                results
                    .hits_per_page
                    .ok_or(AppError::InvalidInput("perpage is missing".to_owned()))?,
            )?,
            total_pages: results
                .total_pages
                .ok_or(AppError::InvalidInput("total pages is missing".to_owned()))?,
            total_hits: results
                .total_hits
                .ok_or(AppError::InvalidInput("total hits is missing".to_owned()))?,
        })
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::index::timestamp::TimeStampIndex;
    use domains::entities::channel::ChannelEntity;
    use domains::entities::video::VideoEntity;
    use domains::value_objects::channel_name::ChannelName;
    use domains::value_objects::timestamp::TimeStamp;
    use domains::value_objects::video_title::VideoTitle;
    use domains::value_objects::{
        elapsed_time::ElapsedTime, page::Page, per_page::PerPage,
        timestamp_description::TimeStampDescription,
    };
    use meilisearch_sdk::search::{SearchResult, SearchResults};

    #[test]
    fn test_convert_to_domain_success() {
        let c = ChannelEntity::with_random_id(ChannelName::new("channel name").unwrap());

        let v = VideoEntity::with_random_id(VideoTitle::new("Video 1").unwrap(), c.clone())
            .construct()
            .unwrap();

        let ts = TimeStamp {
            elapsed_time: ElapsedTime(1),
            description: TimeStampDescription::new("description").unwrap(),
        };

        let i = TimeStampIndex::from_entity(v, ts);

        // Arrange: Create a mock SearchResults from Meilisearch.
        // This simulates a successful search result from the Meilisearch client.
        let search_results = SearchResults {
            hits: vec![SearchResult {
                result: i,
                formatted_result: None,
                matches_position: None,
                ranking_score: None,
                ranking_score_details: None,
                federation: None,
            }],
            offset: Some(0),
            limit: Some(1),
            estimated_total_hits: None,
            processing_time_ms: 5,
            query: "test query".to_string(),
            facet_distribution: None,
            total_hits: Some(1),
            hits_per_page: Some(1),
            page: Some(1),
            total_pages: Some(1),
            facet_stats: None,
            index_uid: None,
        };

        // Act: Call the function to test.
        let result = SearchResultConverter::convert_to_domain(search_results);

        // Assert: Check if the conversion was successful and the data is correct.
        assert!(result.is_ok());
        let domain_result = result.unwrap();

        assert_eq!(domain_result.items.len(), 1);
        assert_eq!(domain_result.page, Page::new(1).unwrap());
        assert_eq!(domain_result.per_page, PerPage::new(1).unwrap());
        assert_eq!(domain_result.total_pages, 1);
        assert_eq!(domain_result.total_hits, 1);
    }
}

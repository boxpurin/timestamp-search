use chrono::{DateTime, Utc};
use domains::repositories::internal_timestamp_search_repository::{
    Part, VideoTimestampSearchQuery,
};
use domains::value_objects::limit::Limit;
use domains::value_objects::page::Page;
use domains::value_objects::per_page::PerPage;
use domains::value_objects::search_query_text::SearchQueryText;
use domains::value_objects::{video_id::VideoId, video_tag::VideoTag};
use garde::Validate;
use serde::Deserialize;
use std::str::FromStr;

///
/// フロントエンドから受け取るタイムスタンプ検索リクエスト
/// # Params
/// - keyword:    検索キーワード
/// - ids:        ビデオID(option)
/// - tags:       タグ検索(option)
/// - actual_start_time_from  : 配信開始時間区間指定（開始）(option)
/// - actual_start_time_to    : 配信開始時間区間指定（終端）(option)
/// - actual_start_time_at    : 配信開始時間区間指定（指定）(option)
/// - page                    : 頁数
/// - per_page                : ページ毎の数
///
#[derive(Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchTimeStampRequest {
    #[garde(length(min = 1, max = 100))]
    #[serde(rename = "q")]
    pub keyword: String,
    #[garde(skip)]
    pub ids: Option<Vec<VideoId>>,
    #[garde(skip)]
    pub tags: Option<Vec<VideoTag>>,
    #[garde(skip)]
    #[serde(rename = "startFrom")]
    pub actual_start_from: Option<DateTime<Utc>>,
    #[garde(skip)]
    #[serde(rename = "startTo")]
    pub actual_start_to: Option<DateTime<Utc>>,
    #[garde(skip)]
    #[serde(rename = "startAt")]
    pub actual_start_at: Option<DateTime<Utc>>,
    #[garde(skip)]
    pub parts: Option<String>,
    #[garde(range(min = 1, max = 1000))]
    pub page: Option<usize>,
    #[garde(range(min = 1, max = 100))]
    pub per_page: Option<usize>,
}

impl TryFrom<SearchTimeStampRequest> for VideoTimestampSearchQuery {
    type Error = errors::AppError;
    fn try_from(
        search_time_stamp: SearchTimeStampRequest,
    ) -> Result<VideoTimestampSearchQuery, Self::Error> {
        let parts = if let Some(parts) = search_time_stamp.parts {
            let parts = parts.split(",");
            let mut p = Vec::new();
            for part in parts {
                if let Ok(part) = Part::from_str(part) {
                    p.push(part);
                }
            }
            Some(p)
        } else {
            None
        };
        Ok(Self {
            query: SearchQueryText::new(&search_time_stamp.keyword).unwrap(),
            video_ids: search_time_stamp.ids,
            video_tags: search_time_stamp.tags,
            actual_start_from: search_time_stamp.actual_start_from,
            actual_start_to: search_time_stamp.actual_start_to,
            actual_start_at: search_time_stamp.actual_start_at,
            parts,
            limit: Limit::new(1000)?,
            page: Page::new(search_time_stamp.page.unwrap_or(1))?,
            per_page: PerPage::new(search_time_stamp.per_page.unwrap_or(25))?,
        })
    }
}

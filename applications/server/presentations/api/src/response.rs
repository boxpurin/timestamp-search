use serde::{Serialize, Deserialize};
use domains::value_objects::{
    video_id::VideoId,
    video_title::VideoTitle,
    video_tag::VideoTag,
    elapsed_time::ElapsedTime,
    thumbnail_url::ThumbnailUrl,
    timestamp_description::TimeStampDescription
};
use chrono::{DateTime, Utc};
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::repositories::internal_timestamp_search_repository::VideoTimestampSearchResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SearchTimeStampResponse {
    items: Vec<ResponseTimeStamp>,
    page: usize,
    per_page: usize,
    total_pages: usize,
    total_hits: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ResponseTimeStamp {
    video_id: VideoId,
    elapsed_time: ElapsedTime,
    description: TimeStampDescription,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_details: Option<ResponseTimeStampVideoDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ResponseTimeStampVideoDetails{
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<VideoTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<ThumbnailUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<VideoTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_start_time: Option<DateTime<Utc>>,
}

impl From<VideoTimestampSearchResult> for SearchTimeStampResponse {
    fn from(result: VideoTimestampSearchResult) -> Self {
        let items = result
            .items
            .into_iter()
            .map(|e| e.into()).collect();
        SearchTimeStampResponse {
            items,
            page: result.page.into(),
            per_page: result.per_page.into(),
            total_pages: result.total_pages,
            total_hits: result.total_hits,
        }
    }
}

impl From<VideoTimestampEntity> for ResponseTimeStamp {
    fn from(entity: VideoTimestampEntity) -> Self {
        ResponseTimeStamp{
            video_id: entity.video_id.clone(),
            elapsed_time: entity.timestamp.elapsed_time.clone(),
            description: entity.timestamp.description.clone(),
            video_details: ResponseTimeStampVideoDetails::try_from(entity).ok(),
        }
    }
}

impl TryFrom<VideoTimestampEntity> for ResponseTimeStampVideoDetails{
    type Error = ();
    fn try_from(entity: VideoTimestampEntity) -> Result<Self, Self::Error> {
        if let Some(detail) = entity.video_detail {
            return Ok(Self {
                title: detail.video_title,
                thumbnail_url: detail.thumbnail_url,
                tags: detail.video_tags,
                published_at: detail.published_at,
                actual_start_time: detail.actual_start_time,
            })
        };

        Err(())
    }
}
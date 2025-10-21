use chrono::{DateTime, Utc};
use domains::entities::video_timestamp::VideoTimestampEntity;
use domains::repositories::internal_timestamp_search_repository::VideoTimestampSearchResult;
use domains::value_objects::{
    elapsed_time::ElapsedTime, thumbnail_url::ThumbnailUrl,
    timestamp_description::TimeStampDescription, video_id::VideoId, video_tag::VideoTag,
    video_title::VideoTitle,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTimeStampResponse {
    items: Vec<ResponseTimeStamp>,
    page: usize,
    per_page: usize,
    total_pages: usize,
    total_hits: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTimeStamp {
    video_id: VideoId,
    elapsed_time: ElapsedTime,
    description: TimeStampDescription,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_details: Option<ResponseTimeStampVideoDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTimeStampVideoDetails {
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
        tracing::debug!("{:?}", result);
        let items = result.items.into_iter().map(|e| e.into()).collect();
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
        ResponseTimeStamp {
            video_id: entity.video_id.clone(),
            elapsed_time: entity.timestamp.elapsed_time,
            description: entity.timestamp.description.clone(),
            video_details: ResponseTimeStampVideoDetails::try_from(entity).ok(),
        }
    }
}

impl TryFrom<VideoTimestampEntity> for ResponseTimeStampVideoDetails {
    type Error = ();
    fn try_from(entity: VideoTimestampEntity) -> Result<Self, Self::Error> {
        if let Some(detail) = entity.video_details {
            return Ok(Self {
                title: detail.video_title,
                thumbnail_url: detail.thumbnail_url,
                tags: detail.video_tags,
                published_at: detail.published_at,
                actual_start_time: detail.actual_start_at,
            });
        };

        Err(())
    }
}

use std::sync::Arc;
use rstest::{rstest, fixture};
use domains::services::timestamp_indexing_service::TimeStampIndexingService;
use domains::services::timestamp_parser_service::TimeStampParserService;
use domains::services::video_fetch_service::VideoFetchService;
use domains::services::video_indexing_service::VideoIndexingService;
use domains::value_objects::channel_id::ChannelId;
use meilisearch::repositories::timestamp_crud::{create_timestamp_crud_repository, MeilisearchVideoCrudRepository};
use meilisearch::repositories::video_crud::create_video_crud_repository;
use youtube::repositories::youtube_video::{create_youtube_video_repository};

#[rstest]
#[tokio::test]
async fn fetch_and_update_video_timestamps() -> anyhow::Result<()> {
    let yt = Arc::new(create_youtube_video_repository().await);
    let fetch =  VideoFetchService::new(yt.clone());

    let channel = ChannelId::new("abc-def-ghi")?;
    let vs = fetch.fetch_recent_video_by_channel_id(&channel, 10).await?;

    if !vs.is_empty() {
        let vis = VideoIndexingService::new(Arc::new(fetch));
        vis.add_or_update_video_entities(&vs).await?;

        let parser = TimeStampParserService::new();
        let v_repo = Arc::new(create_video_crud_repository());
        let ts_repo = Arc::new(create_timestamp_crud_repository());
        let tis = TimeStampIndexingService::new(
            ts_repo.clone(),
            v_repo.clone()
        );

        for v in vs.iter() {
            let tss = parser.parse_video(v).await?;
            tis.add_or_update_timestamps(v, tss).await?;
        }
    }

    Ok(())
}
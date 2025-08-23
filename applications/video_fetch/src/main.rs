use domains::services::{
    video_fetch_service::VideoFetchService,
    video_indexing_service::VideoIndexingService,
    timestamp_parser_service::TimeStampParserService,
    timestamp_indexing_service::TimeStampIndexingService,
};
use domains::value_objects::channel_id::ChannelId;
use meilisearch::repositories::{
    video_crud::create_video_crud_repository,
    timestamp_crud::create_timestamp_crud_repository,
};
use youtube::repositories::youtube_video::create_youtube_video_repository;
use std::sync::Arc;
use errors::{AppError, AppResult};

#[tokio::main]
async fn main() -> AppResult<()>{
    tracing_subscriber::fmt::init();

    let ext_repo = Arc::new(create_youtube_video_repository().await);
    let int_repo = Arc::new(create_video_crud_repository());
    let tss_repo = Arc::new(create_timestamp_crud_repository());

    let video_fetch_service = VideoFetchService::new(ext_repo.clone());
    let parser = TimeStampParserService::new();
    let video_indexing = VideoIndexingService::new(int_repo.clone());
    let ts_indexing = TimeStampIndexingService::new(tss_repo.clone(), int_repo.clone());

    let channel_id = ChannelId::new("TSSEARCH_TARGET_CHANNEL_ID").expect("Invalid channel ID");
    // Replace with the actual channel ID you want to fetch videos for
    let vs = video_fetch_service
        .fetch_all_videos_by_channel_id(&channel_id)
        .await?;

    if let Err(_) = video_indexing.add_or_update_video_entities(&vs).await{
        return Err(AppError::InvalidInput("Failed to add video entities".to_string()));
    }

    for v in vs {
        if let Ok(tss) = parser.parse_video(&v){
            ts_indexing.add_or_update_timestamps(&v, tss.as_slice()).await?;
        }
    }

    Ok(())
}

use domains::services::video_fetch_service::VideoFetchService;
use domains::value_objects::channel_id::ChannelId;
use youtube::repositories::youtube_video::create_youtube_video_repository;
use meilisearch::repositories::video_crud::create_video_crud_repository;

#[tokio::main]
async fn main() {
    let ext_repo = create_youtube_video_repository().await;
    let int_repo = create_video_crud_repository();
    let video_fetch_service = VideoFetchService::new(ext_repo, int_repo);
    
    let channel_id = ChannelId::new("YOUR_CHANNEL_ID").expect("Invalid channel ID");
    // Replace with the actual channel ID you want to fetch videos for
    let v =video_fetch_service.fetch_all_videos_by_channel_id(&channel_id).await.unwrap();
    
}


#[cfg(test)]
mod integral_tests {
    use youtube::repositories::youtube_video::create_youtube_video_repository;
    use domains::repositories::external_video_repository::ExternalVideoRepository;
    use domains::value_objects::channel_id::ChannelId;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_youtube_video_repository() {
        let repository = create_youtube_video_repository().await;
        let channel_id = ChannelId::new("UCf2I8ZGg0uS4FfFTdypJ5Xg".to_owned());

        // Test fetching recent video by channel ID
        let recent_videos = repository.fetch_recent_video_by_channel_id(&channel_id, 10).await;
        assert!(recent_videos.is_ok());

        let videos = recent_videos.unwrap();
        assert_eq!(videos.len(), 10);

        // Test fetching all videos by channel ID
        //let result = repository.fetch_all_videos_by_channel_id(&channel_id).await;
        //assert!(result.is_ok());
        //tracing::info!("total videos: {}", result.unwrap().len());
    }
}
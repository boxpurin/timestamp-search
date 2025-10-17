use usecase::timestamp_parser_service::TimeStampParserService;
use domains::value_objects::channel_id::ChannelId;
use meilisearch::repositories::{
    timestamp_crud::create_timestamp_crud_repository,
    video_crud::create_video_crud_repository,
};
use youtube::repositories::youtube_video::create_youtube_video_repository;
use std::sync::Arc;
use errors::{AppError, AppResult};
use clap::Parser;
use std::fs::File;
use domains::entities::video::VideoEntity;
use usecase::timestamp_indexing_service::TimeStampIndexingService;
use usecase::video_fetch_service::VideoFetchService;
use usecase::video_indexing_service::VideoIndexingService;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "false")]
    pub in_external: bool,

    #[arg(short, long, conflicts_with = "in_external")]
    pub in_json: Option<String>,

    #[arg(long, default_value = "false")]
    pub out_internal: bool,

    #[arg(short, long, conflicts_with = "out_internal")]
    pub out_json: Option<String>,
}

#[tokio::main]
async fn main() -> AppResult<()>{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();
    let videos = if args.in_external {
        tracing::info!("Load video from external service.");
        let channel_id = std::env::var("TSS_TARGET_CHANNEL_ID").expect("TSSEARCH_TARGET_CHANNEL_ID");
        let ext_repo = Arc::new(create_youtube_video_repository().await);
        let video_fetch_service = VideoFetchService::new(ext_repo.clone());
        let channel_id = ChannelId::new(&channel_id)?;

        video_fetch_service.fetch_recent_video_by_channel_id(&channel_id, 10).await?
    } else {
        tracing::info!("Load video entity from local json file.");
        let mut videos = Vec::new();
        if let Some(in_json) = args.in_json {
            let file = File::open(in_json).expect("Unable to open file");
            let v : Vec<VideoEntity> = serde_json::from_reader(file).expect("Invalid JSON file");
            videos.extend(v);
        } else {
            panic!("Input json file is not set.");
        }
        videos
    };

    if args.out_internal {
        tracing::info!("Output video and timestamps from internal meilisearch.");
        let int_repo = Arc::new(create_video_crud_repository());
        let tss_repo = Arc::new(create_timestamp_crud_repository());

        let parser = TimeStampParserService::new();
        let video_indexing = VideoIndexingService::new(int_repo.clone());
        let ts_indexing = TimeStampIndexingService::new(tss_repo.clone(), int_repo.clone());

        if video_indexing.add_or_update_video_entities(&videos).await.is_err() {
            return Err(AppError::InvalidInput("Failed to add video entities".to_string()));
        }

        for v in videos {
            if let Ok(tss) = parser.parse_video(&v){
                ts_indexing.add_or_update_timestamps(&v, tss.as_slice()).await?;
            }
        }
    } else {
        tracing::info!("Output video to json file.");
        if let Some(file) = args.out_json {
            let file = File::create(file).expect("Unable to open file");
            serde_json::to_writer_pretty(file, &videos).expect("Unable to write to file");
        } else {
            println!("Output to json file is not set.");
        }
    }

    Ok(())
}

use crate::adapter::video::VideoEntityConverter;
use crate::config::YOUTUBE_CLIENT;
use domains::entities::video::VideoEntity;
use domains::repositories::external_video_repository::ExternalVideoRepository;
use domains::value_objects::channel_id::ChannelId;
use google_youtube3::{YouTube, hyper_rustls, hyper_util, yup_oauth2};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait YouTubeApi {
    async fn fetch_channel_uploads_from_api(
        &self,
        channel_id: &ChannelId,
    ) -> Result<String, String>;

    async fn fetch_playlist_videos_from_api(
        &self,
        playlist_id: &str,
        max_results: u32,
        page_token: &Option<String>,
    ) -> Result<(Vec<VideoEntity>, Option<String>), String>;
}

pub struct YoutubeVideoRepository {
    api_client: Box<dyn YouTubeApi + Send + Sync>,
}

pub async fn create_youtube_video_repository() -> YoutubeVideoRepository {
    let secret = yup_oauth2::read_application_secret(&YOUTUBE_CLIENT.google_client_secret_path)
        .await
        .expect("TSSEARCH_GOOGLE_CLIENT_SECRET_PATH FILE NOT FOUND");

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(&YOUTUBE_CLIENT.persistent_token_path)
    .build()
    .await
    .expect("InstalledFlowAuthenticator: builder");

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    let hub = YouTube::new(client, auth);
    let api_impl = YouTubeApiImpl { hub };
    YoutubeVideoRepository::new(Box::new(api_impl))
}

#[async_trait::async_trait]
impl ExternalVideoRepository for YoutubeVideoRepository {
    /// YouTube APIを使用して、指定されたチャンネルIDのすべてのビデオを取得します。
    ///
    /// # Arguments
    /// * `channel_id` - チャンネルID
    ///
    /// # Returns
    /// * `Result<Vec<VideoEntity>, String>` - ビデオ情報のリストまたはエラーメッセージ
    async fn fetch_all_videos_by_channel_id(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<VideoEntity>, String> {
        let uploads = self
            .api_client
            .fetch_channel_uploads_from_api(channel_id)
            .await?;
        let mut videos = Vec::new();
        let (mut v, mut p) = self
            .api_client
            .fetch_playlist_videos_from_api(&uploads, 50, &None)
            .await?;
        videos.extend(v);
        while p.is_some() {
            (v, p) = self
                .api_client
                .fetch_playlist_videos_from_api(&uploads, 50, &p)
                .await?;
            videos.extend(v);
        }

        Ok(videos)
    }

    /// YouTube APIを使用して、指定されたチャンネルIDの最新のビデオを取得します。
    ///
    /// * `channel_id` - チャンネルID
    /// * `count` - 取得するビデオの数
    ///
    /// # Returns
    /// * `Result<Vec<VideoEntity>, String>` - ビデオ情報のリストまたはエラーメッセージ
    async fn fetch_recent_video_by_channel_id(
        &self,
        channel_id: &ChannelId,
        max_results: u32,
    ) -> Result<Vec<VideoEntity>, String> {
        tracing::debug!("fetching recent video from api");
        let uploads = self
            .api_client
            .fetch_channel_uploads_from_api(channel_id)
            .await?;
        let (v, _) = self
            .api_client
            .fetch_playlist_videos_from_api(&uploads, max_results, &None)
            .await?;
        Ok(v)
    }
}

impl YoutubeVideoRepository {
    pub fn new(api_client: Box<dyn YouTubeApi + Sync + Send>) -> Self {
        YoutubeVideoRepository { api_client }
    }
}

struct YouTubeApiImpl {
    hub: YouTube<HttpsConnector<HttpConnector>>,
}

impl YouTubeApiImpl {
    async fn convert_playlist_item_to_video(
        &self,
        item: google_youtube3::api::PlaylistItem,
    ) -> Result<google_youtube3::api::Video, String> {
        let id = item
            .snippet
            .as_ref()
            .and_then(|s| s.resource_id.as_ref())
            .and_then(|r| r.video_id.as_ref())
            .ok_or("Video ID not found in playlist item")?;
        let v = self
            .hub
            .videos()
            .list(&vec![
                "snippet".to_string(),
                "contentDetails".to_string(),
                "liveStreamingDetails".to_string(),
            ])
            .add_id(id)
            .doit()
            .await;

        match v {
            Ok((_, videos)) => {
                if let Some(video) = videos.items.and_then(|items| items.into_iter().next()) {
                    Ok(video)
                } else {
                    Err("No video found for the given ID".to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

#[async_trait::async_trait]
impl YouTubeApi for YouTubeApiImpl {
    async fn fetch_channel_uploads_from_api(
        &self,
        channel_id: &ChannelId,
    ) -> Result<String, String> {
        let result = self
            .hub
            .channels()
            .list(&vec!["contentDetails".to_string()])
            .add_id(channel_id)
            .doit()
            .await
            .map_err(|e| e.to_string())?;
        let (_response, channels) = result;
        if let Some(channels) = channels.items {
            if channels.is_empty() {
                return Err("Channel not found".to_string());
            }
            let channel = channels.into_iter().next().unwrap();
            let content_details = channel.content_details.ok_or("Content details not found")?;
            let related_playlists = content_details
                .related_playlists
                .ok_or("Related playlists not found")?;
            let uploads = related_playlists.uploads.ok_or("Uploads not found")?;
            Ok(uploads)
        } else {
            Err("Channel not found".to_string())
        }
    }

    async fn fetch_playlist_videos_from_api(
        &self,
        playlist_id: &str,
        max_results: u32,
        page_token: &Option<String>,
    ) -> Result<(Vec<VideoEntity>, Option<String>), String> {
        let mut req = self
            .hub
            .playlist_items()
            .list(&vec!["snippet".to_string()])
            .playlist_id(playlist_id)
            .max_results(max_results); // You can adjust the number of results as needed

        if let Some(token) = page_token {
            req = req.page_token(&token);
        };

        let (items, page_token) =
            req.doit()
                .await
                .map_err(|e| e.to_string())
                .and_then(|(_, items)| {
                    let page_token = items.next_page_token;
                    let items = items
                        .items
                        .ok_or("No items found in playlist".to_string())?;
                    Ok((items, page_token))
                })?;
        tracing::debug!(
            "Fetched {} items from playlist {}",
            items.len(),
            playlist_id
        );

        let items = items
            .into_iter()
            .filter(|item| {
                if let Some(snippet) = item.snippet.as_ref() {
                    snippet.resource_id.as_ref().map_or(false, |r| {
                        r.kind.as_ref().map_or("", |k| k) == "youtube#video".to_owned()
                    })
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        let mut videos: Vec<VideoEntity> = Vec::new();
        for item in items {
            // Convert PlayListItem to YouTubeVideo
            let v = self.convert_playlist_item_to_video(item).await?;
            let v = VideoEntityConverter(v);
            match v.try_into() {
                Ok(video) => videos.push(video),
                Err(_e) => {
                    return Err("Failed to convert playlist item to video entity".to_string());
                }
            };
        }
        // Implement the logic to fetch videos from YouTube API
        Ok((videos, page_token))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    use domains::entities::video::VideoEntity;
    use domains::value_objects::video_description::VideoDescription;
    use domains::value_objects::video_id::VideoId;
    use domains::value_objects::video_title::VideoTitle;

    use domains::entities::channel::ChannelEntity;
    use domains::value_objects::channel_id::ChannelId;
    use domains::value_objects::channel_name::ChannelName;

    use super::MockYouTubeApi;
    use mockall::predicate::eq;

    #[tokio::test]
    #[rstest::rstest]
    #[case(vec![])]
    #[case(vec![
        VideoEntity::new(
            VideoId::new("video1".to_string()),
            VideoTitle::new("Video 1".to_string()),
            Vec::new(),
            VideoDescription::new("Description 1".to_string()),
            ChannelEntity::new(
                ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw".to_owned()),
                ChannelName::new("Channel 1".to_string())
            ),
            None,
            chrono::Utc::now(),
            None,
        )
    ]
    )]
    async fn test_fetch_all_videos_by_channel_id(#[case] expected: Vec<VideoEntity>) {
        let channel_id = ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw".to_owned());
        let len = expected.len();
        let mut mock_api = MockYouTubeApi::new();
        mock_api
            .expect_fetch_channel_uploads_from_api()
            .with(eq(channel_id.clone()))
            .returning(|_| Ok("PLBCF2DAC6FFB574DE".to_string()));

        mock_api
            .expect_fetch_playlist_videos_from_api()
            .with(eq("PLBCF2DAC6FFB574DE"), eq(50), eq(&None))
            .returning(move |_, _, _| Ok((expected.clone(), None)));

        let repository = YoutubeVideoRepository::new(Box::new(mock_api));
        let result = repository.fetch_all_videos_by_channel_id(&channel_id).await;
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(v.len(), len);
    }

    #[tokio::test]
    async fn test_fetch_recent_video_by_channel_id() {
        let channel_id = ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw".to_owned());
        let mut mock_api = MockYouTubeApi::new();
        mock_api
            .expect_fetch_channel_uploads_from_api()
            .with(eq(channel_id.clone()))
            .returning(|_| Ok("PLBCF2DAC6FFB574DE".to_string()));

        mock_api
            .expect_fetch_playlist_videos_from_api()
            .with(eq("PLBCF2DAC6FFB574DE"), eq(10), eq(&None))
            .returning(|_, _, _| Ok((vec![], None)));

        let repository = YoutubeVideoRepository::new(Box::new(mock_api));
        let result = repository
            .fetch_recent_video_by_channel_id(&channel_id, 10)
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}

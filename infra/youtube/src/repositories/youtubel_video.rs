use crate::adapter::video::PlayListItemToVideoEntityConverter;
use domains::entities::video::VideoEntity;
use domains::repositories::external_video_repository::ExternalVideoRepository;
use domains::value_objects::channel_id::ChannelId;
use google_youtube3::{YouTube, hyper_rustls, hyper_util, yup_oauth2};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;

pub struct YoutubeVideoRepository {
    hub: YouTube<HttpsConnector<HttpConnector>>,
}

pub async fn create_youtube_video_repository() -> YoutubeVideoRepository {
    let youtube_client_secret_path = std::env::var("TSSEARCH_GOOGLE_CLIENT_SECRET_PATH")
        .expect("GOOGLE_CLIENT_SECRET_PATH not set");

    let persistent_token_path = std::env::var("TSSEARCH_PERSISTENT_TOKEN_PATH")
        .expect("TSSEARCH_PERSISTENT_TOKEN_PATH not set");

    let secret = yup_oauth2::read_application_secret(youtube_client_secret_path)
        .await
        .expect("GOOGLE_CLIENT_SECRET FILE NOT FOUND");

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(persistent_token_path)
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

    YoutubeVideoRepository::new(hub)
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
        let uploads = self.fetch_channel_uploads_from_api(channel_id).await?;
        let mut videos = Vec::new();
        let (mut v, mut p) = self
            .fetch_playlist_videos_from_api(&uploads, 50, &None)
            .await?;
        videos.extend(v);
        while p.is_some() {
            (v, p) = self
                .fetch_playlist_videos_from_api(&uploads, 50, &p)
                .await?;
            videos.extend(v);
        }

        Ok(videos)
    }

    /// YouTube APIを使用して、指定されたチャンネルIDの最新のビデオを取得します。
    ///
    /// # Arguments
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
        let uploads = self.fetch_channel_uploads_from_api(channel_id).await?;
        let (v, _) = self
            .fetch_playlist_videos_from_api(&uploads, max_results, &None)
            .await?;
        Ok(v)
    }
}

impl YoutubeVideoRepository {
    pub fn new(hub: YouTube<HttpsConnector<HttpConnector>>) -> Self {
        YoutubeVideoRepository { hub }
    }

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
        let (response, channels) = result;
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
            .add_id(playlist_id)
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

        let mut videos: Vec<VideoEntity> = Vec::new();
        for item in items {
            let item = PlayListItemToVideoEntityConverter(item);
            match item.try_into() {
                Ok(video) => videos.push(video),
                Err(e) => {}
            };
        }
        // Implement the logic to fetch videos from YouTube API
        Ok((videos, page_token))
    }
}

#[cfg(test)]
mod tests {}
